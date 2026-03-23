use std::collections::LinkedList;
use std::fmt::Debug;
use std::fmt::Display;
use std::io::Error as IoError;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use crate::vinteng::operations::VINT_OPERATIONS;

mod operations;
mod syscalls;

#[derive(Debug)]
pub enum VintError {
    SegFault(u32),
    IoError(IoError),
    UnknownOperation(u8),
    UnknownSyscall,
}

impl Display for VintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SegFault(addr) => write!(f, "segmentation fault accessing {:#08X}", addr),
            Self::IoError(e) => write!(f, "io error: {}", e),
            Self::UnknownOperation(op) => write!(f, "unknown op: {:#02x}", op),
            Self::UnknownSyscall => write!(f, "unknown syscall"),
        }
    }
}

impl From<IoError> for VintError {
    fn from(value: IoError) -> Self {
        Self::IoError(value)
    }
}

#[derive(Debug)]
pub struct MemoryMapEntry {
    pub addr_start: u32,
    pub addr_end: u32,
    pub read_only: bool,
    pub data: Vec<u8>,
}

impl MemoryMapEntry {
    pub fn new(addr_start: u32, addr_end: u32) -> Self {
        let size = addr_end - addr_start;

        Self {
            addr_start,
            addr_end,
            read_only: false,
            data: vec![0; size as usize],
        }
    }
}

pub struct MemoryMap {
    pub entries: LinkedList<MemoryMapEntry>,
}

impl Debug for MemoryMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.entries.iter() {
            writeln!(
                f,
                "\tEntry: {:#08X}->{:#08X}",
                entry.addr_start, entry.addr_end
            )?;
        }
        Ok(())
    }
}

impl MemoryMap {
    pub fn new() -> Self {
        Self {
            entries: LinkedList::new(),
        }
    }

    /// allocates an entry and returns the starting address
    pub fn alloc(&mut self, size: u32) -> u32 {
        // Make size a multiple of two
        let size = size + (size % 2);
        let mut entries_iter = self.entries.iter().enumerate();

        let mut entry_a = match entries_iter.next() {
            Some((_, entry_a)) => entry_a,
            None => {
                let addr_start = 0;
                let addr_end = addr_start + size;

                let entry = MemoryMapEntry::new(addr_start, addr_end);

                self.entries.push_back(entry);

                return addr_start;
            }
        };

        loop {
            let entry_b_opt = entries_iter.next();

            if entry_b_opt.is_none() {
                let addr_start_rough = entry_a.addr_end;
                let addr_start = addr_start_rough + (addr_start_rough % 2);
                let addr_end = addr_start + size;

                let entry = MemoryMapEntry::new(addr_start, addr_end);

                self.entries.push_back(entry);

                return addr_start;
            }

            let (i_entry, entry_b) = entry_b_opt.unwrap();

            let free_space = entry_b.addr_start - entry_a.addr_end;

            if free_space > size {
                let addr_start_rough = entry_a.addr_end;
                let addr_start = addr_start_rough + (addr_start_rough % 2);
                let addr_end = addr_start + size;

                let entry = MemoryMapEntry::new(addr_start, addr_end);

                let mut back_half = self.entries.split_off(i_entry);
                back_half.push_front(entry);
                self.entries.append(&mut back_half);

                return addr_start;
            }

            entry_a = entry_b;
        }
    }

    pub fn access<'a>(&'a mut self, addr: u32, size: u32) -> Result<&'a mut [u8], VintError> {
        for entry in self.entries.iter_mut() {
            if entry.addr_start <= addr && entry.addr_end > addr {
                let i_start = addr - entry.addr_start;
                let addr_end = addr + size;
                let i_end = i_start + size;

                if addr_end > entry.addr_end {
                    log::error!(
                        "attempted to access address {:#08X} where entry ends at {:#08X}",
                        addr_end,
                        entry.addr_end
                    );
                    return Err(VintError::SegFault(addr + size));
                }

                return Ok(&mut entry.data[i_start as usize..i_end as usize]);
            }
        }

        log::error!("address {:#08X} out of bounds", addr);
        return Err(VintError::SegFault(addr));
    }

    pub fn mark_ro(&mut self, addr: u32) -> Result<(), VintError> {
        for entry in self.entries.iter_mut() {
            if entry.addr_start == addr {
                entry.read_only = true;
                return Ok(());
            }
        }

        return Err(VintError::SegFault(addr));
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct StatRegister {
    inner: u8,
}

/// Single-threaded VINT bytecode interpreter
pub struct VintEngine {
    mem_map: MemoryMap,
    a: [u8; 2],
    b: [u8; 2],
    c: [u8; 2],
    d: [u8; 2],
    sp: u32,
    pc: u32,
    ret: u32,
    stat: StatRegister,
    halt_req: bool,
    invalid_buffer: bool,
}

impl Debug for VintEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Memory Map:\n{:?}", self.mem_map)?;
        let a_int = u16::from_le_bytes(self.a);
        let b_int = u16::from_le_bytes(self.b);
        let c_int = u16::from_le_bytes(self.c);
        let d_int = u16::from_le_bytes(self.d);
        writeln!(f, "Registers:")?;
        writeln!(
            f,
            "\tA: {:#06X}\tB: {:#06X}\tC: {:#06X}\tD: {:#06X}",
            a_int, b_int, c_int, d_int
        )?;
        writeln!(
            f,
            "\tPC: {:#08X}\tSP: {:#08X}\tRET: {:#08X}",
            self.pc, self.sp, self.ret
        )?;

        return Ok(());
    }
}

impl VintEngine {
    pub fn new<In: Read + Seek>(mut in_stream: In, stack_size: u32) -> Result<Self, VintError> {
        let in_size = in_stream.seek(SeekFrom::End(0))?;
        in_stream.seek(SeekFrom::Start(0))?;

        let mut in_buffer = vec![0; in_size as usize];

        in_stream.read_exact(&mut in_buffer)?;

        let mut mem_map = MemoryMap::new();

        let pc = mem_map.alloc(in_size as u32);
        mem_map.mark_ro(pc)?;
        let mem_window = mem_map.access(pc, in_size as u32)?;

        mem_window.copy_from_slice(&in_buffer);

        // Stack grows down.
        let stack_origin = mem_map.alloc(stack_size);
        let sp = stack_origin + (stack_size - 1);

        let ret = pc;
        let stat = Default::default();

        Ok(Self {
            mem_map,
            sp,
            pc,
            ret,
            stat,
            a: [0; 2],
            b: [0; 2],
            c: [0; 2],
            d: [0; 2],
            halt_req: false,
            invalid_buffer: false,
        })
    }

    pub fn run(&mut self) -> Result<(), VintError> {
        let mut buffer = [0u8; 2];
        loop {
            let window = self.mem_map.access(self.pc, 2)?;

            buffer.copy_from_slice(&window);
            self.invalid_buffer = false;

            for opcode in buffer {
                self.pc += 1;
                VINT_OPERATIONS[opcode as usize](self, &buffer)?;

                if self.invalid_buffer {
                    break;
                }
            }

            if self.halt_req {
                return Ok(());
            }
        }
    }

    pub fn ab_as_u32(&self) -> u32 {
        let a = &self.a;
        let b = &self.b;
        u32::from_le_bytes([a[0], a[1], b[0], b[1]])
    }

    pub fn ab_as_i32(&self) -> i32 {
        let a = &self.a;
        let b = &self.b;
        i32::from_le_bytes([a[0], a[1], b[0], b[1]])
    }

    pub fn cd_as_u32(&self) -> u32 {
        let c = &self.c;
        let d = &self.d;
        u32::from_le_bytes([c[0], c[1], d[0], d[1]])
    }

    pub fn push_from(&mut self, slice: &[u8]) -> Result<(), VintError> {
        let size = slice.len() as u32;

        self.sp -= size;
        let window = self.mem_map.access(self.sp, size)?;
        window.copy_from_slice(slice);

        Ok(())
    }

    pub fn push_u8(&mut self, val: u8) -> Result<(), VintError> {
        self.push_from(&val.to_le_bytes())
    }

    pub fn push_u16(&mut self, val: u16) -> Result<(), VintError> {
        self.push_from(&val.to_le_bytes())
    }

    pub fn push_u32(&mut self, val: u32) -> Result<(), VintError> {
        self.push_from(&val.to_le_bytes())
    }

    pub fn pop_into(&mut self, slice: &mut [u8]) -> Result<(), VintError> {
        let size = slice.len() as u32;
        let window = self.mem_map.access(self.sp, size)?;

        slice.copy_from_slice(window);

        self.sp += size;

        Ok(())
    }

    pub fn pop_u8(&mut self) -> Result<u8, VintError> {
        let mut bytes = [0; 1];

        self.pop_into(&mut bytes)?;

        Ok(u8::from_le_bytes(bytes))
    }

    pub fn pop_u16(&mut self) -> Result<u16, VintError> {
        let mut bytes = [0; 2];

        self.pop_into(&mut bytes)?;

        Ok(u16::from_le_bytes(bytes))
    }

    pub fn pop_u32(&mut self) -> Result<u32, VintError> {
        let mut bytes = [0; 4];

        self.pop_into(&mut bytes)?;

        Ok(u32::from_le_bytes(bytes))
    }
}
