use std::collections::HashMap;
use std::fmt::Display;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

use crate::opcodes::Alignment;
use crate::opcodes::OPCODE_SPECS;
use crate::opcodes::OperandType;

use std::io::Error as IoError;

pub const LABEL_DELEM: char = ':';
pub const COMMENT_DELEM: &str = "//";

pub enum AssemblyErrorId {
    IoError(IoError),
    EmptyLabel,
    DuplicateLabel,
    InvalidLabel,
    UnknownOpcode,
    UnknownLabel,
    ExcessOperands,
    InvalidOperand,
    DistantJump,
}

impl Display for AssemblyErrorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::IoError(e) => return write!(f, "io error: {}", e),
            Self::EmptyLabel => "Empty label",
            Self::DuplicateLabel => "Duplicate label",
            Self::InvalidLabel => "Invalid label",
            Self::UnknownOpcode => "Unknown opcode",
            Self::UnknownLabel => "Unknown label",
            Self::ExcessOperands => "Too many operands",
            Self::InvalidOperand => "Invalid operand",
            Self::DistantJump => "Jump is too far",
        };

        write!(f, "{}", string)
    }
}

pub struct AssemblyError {
    pub line_number: usize,
    pub id: AssemblyErrorId,
}

impl AssemblyError {
    pub fn from_io_error(line_number: usize, e: IoError) -> Self {
        Self {
            line_number,
            id: AssemblyErrorId::IoError(e),
        }
    }

    pub fn empty_label(line_number: usize) -> Self {
        Self {
            line_number,
            id: AssemblyErrorId::EmptyLabel,
        }
    }

    pub fn duplicate_label(line_number: usize) -> Self {
        Self {
            line_number,
            id: AssemblyErrorId::DuplicateLabel,
        }
    }

    pub fn invalid_label(line_number: usize) -> Self {
        Self {
            line_number,
            id: AssemblyErrorId::InvalidLabel,
        }
    }

    pub fn unknown_opcode(line_number: usize) -> Self {
        Self {
            line_number,
            id: AssemblyErrorId::UnknownOpcode,
        }
    }

    pub fn excess_operands(line_number: usize) -> Self {
        Self {
            line_number,
            id: AssemblyErrorId::ExcessOperands,
        }
    }

    pub fn invalid_operand(line_number: usize) -> Self {
        Self {
            line_number,
            id: AssemblyErrorId::InvalidOperand,
        }
    }

    pub fn unknown_label(line_number: usize) -> Self {
        Self {
            line_number,
            id: AssemblyErrorId::UnknownLabel,
        }
    }

    pub fn distant_jump(line_number: usize) -> Self {
        Self {
            line_number,
            id: AssemblyErrorId::DistantJump,
        }
    }
}

impl Display for AssemblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}: {}", self.line_number, self.id)
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    None,
    U8(u8),
    AddrRel(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    AddrAbs(u32),
    U32(u32),
    I32(i32),
    Label(String),
}

pub struct ByteMapEntry {
    pub addr: usize,
    pub line_number: usize,
    pub i_opcode: usize,
    pub operand: Operand,
}

pub struct ByteMap {
    // Each entry refers to an index in the opcode specification
    opcode_map: HashMap<&'static str, usize>,
    label_map: HashMap<String, usize>,
    entries: Vec<ByteMapEntry>,
    out_counter: usize,
}

impl ByteMap {
    pub fn new() -> Self {
        let opcode_spec_iter = OPCODE_SPECS
            .iter()
            .enumerate()
            .map(|(i_spec, spec)| (spec.string, i_spec));
        Self {
            opcode_map: HashMap::from_iter(opcode_spec_iter),
            label_map: HashMap::with_capacity(16),
            entries: Vec::with_capacity(128),
            out_counter: 0,
        }
    }

    pub fn process_in<In: Read>(&mut self, in_stream: &mut In) -> Result<(), AssemblyError> {
        let mut in_stream = BufReader::new(in_stream);
        let mut in_buffer = String::with_capacity(16);
        let mut line_number: usize = 0;

        self.parse_opcode("JP24", Some("_start"), 0)?;

        loop {
            line_number += 1;
            in_buffer.clear();

            let read_size = in_stream
                .read_line(&mut in_buffer)
                .map_err(|e| AssemblyError::from_io_error(line_number, e))?;

            if read_size == 0 {
                break;
            }

            self.process_line(&in_buffer, line_number)?;
        }

        Ok(())
    }
    /// Sequentially process a line
    fn process_line(&mut self, line: &str, line_number: usize) -> Result<(), AssemblyError> {
        let trimmed_line_opt = line.trim().split(COMMENT_DELEM).next();

        let trimmed_line = match trimmed_line_opt {
            Some(line) => line,
            None => {
                return Ok(());
            }
        };

        if trimmed_line.is_empty() {
            return Ok(());
        }

        let mut tokens = trimmed_line.split_whitespace();
        let mut label_opt = None;
        let opcode_opt;
        let operand_opt;

        // There should at least be one token if the string isn't empty
        let first_token = tokens.next().unwrap();

        if first_token.contains(LABEL_DELEM) {
            // Trim the label delemiter from the label and store it
            let (label, _) = first_token.split_at(first_token.len() - 1);

            label_opt = Some(label);

            opcode_opt = tokens.next();
            operand_opt = tokens.next();
        } else {
            opcode_opt = Some(first_token);
            operand_opt = tokens.next();
        }

        if tokens.next().is_some() {
            return Err(AssemblyError::excess_operands(line_number));
        }

        if let Some(label) = label_opt {
            self.parse_label(label, self.out_counter, line_number)?;
        }

        let opcode_str = match opcode_opt {
            Some(opcode_str) => opcode_str,
            None => {
                return Ok(());
            }
        };

        self.parse_opcode(opcode_str, operand_opt, line_number)?;

        Ok(())
    }

    /// Generate and write data to out stream
    pub fn generate_data<Out: Write>(self, out_stream: &mut Out) -> Result<usize, AssemblyError> {
        let mut out_stream = BufWriter::with_capacity(128, out_stream);

        for entry in self.entries.into_iter() {
            let opcode_spec = &OPCODE_SPECS[entry.i_opcode];

            out_stream
                .write(&[opcode_spec.hex])
                .map_err(|e| AssemblyError::from_io_error(entry.line_number, e))?;

            match entry.operand {
                Operand::None => {
                    continue;
                }
                Operand::U8(x) => out_stream.write(&x.to_le_bytes()),
                Operand::I8(x) => out_stream.write(&x.to_le_bytes()),
                Operand::U16(x) => out_stream.write(&x.to_le_bytes()),
                Operand::I16(x) => out_stream.write(&x.to_le_bytes()),
                Operand::U32(x) => out_stream.write(&x.to_le_bytes()),
                Operand::I32(x) => out_stream.write(&x.to_le_bytes()),
                Operand::AddrRel(x) => out_stream.write(&x.to_le_bytes()),
                Operand::AddrAbs(x) => out_stream.write(&x.to_le_bytes()[..3]),
                Operand::Label(label) => {
                    let label_addr = self
                        .label_map
                        .get(label.as_str())
                        .ok_or(AssemblyError::unknown_label(entry.line_number))?;
                    if matches!(opcode_spec.operand_type, OperandType::AddrRel) {
                        let rel_addr_i32 = *label_addr as i32 - entry.addr as i32;

                        let rel_addr: i8 = rel_addr_i32
                            .try_into()
                            .map_err(|_| AssemblyError::distant_jump(entry.line_number))?;

                        out_stream.write(&rel_addr.to_le_bytes())
                    } else {
                        out_stream.write(&label_addr.to_le_bytes()[..3])
                    }
                }
            }
            .map_err(|e| AssemblyError::from_io_error(entry.line_number, e))?;
        }

        Ok(self.out_counter)
    }

    fn parse_label(
        &mut self,
        label: &str,
        address: usize,
        line_number: usize,
    ) -> Result<(), AssemblyError> {
        Self::check_label(label, line_number)?;

        if self.label_map.insert(label.to_owned(), address).is_some() {
            return Err(AssemblyError::duplicate_label(line_number));
        }

        Ok(())
    }

    fn check_label(label: &str, line_number: usize) -> Result<(), AssemblyError> {
        if label.is_empty() {
            return Err(AssemblyError::empty_label(line_number));
        }

        if label.contains(LABEL_DELEM) {
            return Err(AssemblyError::invalid_label(line_number));
        }

        // Labels cannot start with numbers
        if label.starts_with(|c: char| c.is_numeric()) {
            return Err(AssemblyError::invalid_label(line_number));
        }

        // Labels can only contain letters, numbers, and underscores
        for character in label.chars() {
            if !character.is_alphanumeric() && character != '_' {
                return Err(AssemblyError::invalid_label(line_number));
            }
        }

        Ok(())
    }

    fn parse_opcode(
        &mut self,
        opcode_str: &str,
        operand_opt: Option<&str>,
        line_number: usize,
    ) -> Result<(), AssemblyError> {
        let opcode_str_ucase = opcode_str.to_uppercase();

        let i_opcode = *self
            .opcode_map
            .get(opcode_str_ucase.as_str())
            .ok_or(AssemblyError::unknown_opcode(line_number))?;

        let opcode_spec = &OPCODE_SPECS[i_opcode];

        // Realign operation if needed
        if opcode_spec
            .required_alignment()
            .unwrap_or(self.current_alignment())
            != self.current_alignment()
        {
            self.parse_opcode("NOP", None, line_number)?;
        }

        let operand_type = opcode_spec.operand_type;

        let operand_str = operand_opt.unwrap_or_default();

        let is_operand_label = !operand_str.starts_with(|c: char| c.is_numeric());

        let operand = match operand_type {
            OperandType::None => {
                if operand_opt.is_some() {
                    return Err(AssemblyError::excess_operands(line_number));
                }

                Operand::None
            }
            OperandType::I8 => Operand::I8(
                operand_str
                    .parse()
                    .map_err(|_| AssemblyError::invalid_operand(line_number))?,
            ),
            OperandType::U8 => Operand::U8(
                operand_str
                    .parse()
                    .map_err(|_| AssemblyError::invalid_operand(line_number))?,
            ),
            OperandType::I16 => Operand::I16(
                operand_str
                    .parse()
                    .map_err(|_| AssemblyError::invalid_operand(line_number))?,
            ),
            OperandType::U16 => Operand::U16(
                operand_str
                    .parse()
                    .map_err(|_| AssemblyError::invalid_operand(line_number))?,
            ),
            OperandType::AddrRel => match is_operand_label {
                true => Operand::Label(operand_str.to_string()),
                false => Operand::AddrRel(
                    operand_str
                        .parse()
                        .map_err(|_| AssemblyError::invalid_operand(line_number))?,
                ),
            },
            OperandType::AddrAbs => match is_operand_label {
                true => Operand::Label(operand_str.to_string()),
                false => Operand::AddrAbs(
                    operand_str
                        .parse()
                        .map_err(|_| AssemblyError::invalid_operand(line_number))?,
                ),
            },
            OperandType::I32 => Operand::I32(
                operand_str
                    .parse()
                    .map_err(|_| AssemblyError::invalid_operand(line_number))?,
            ),
            OperandType::U32 => Operand::U32(
                operand_str
                    .parse()
                    .map_err(|_| AssemblyError::invalid_operand(line_number))?,
            ),
        };

        let entry = ByteMapEntry {
            addr: self.out_counter,
            line_number,
            i_opcode,
            operand,
        };

        self.entries.push(entry);

        self.out_counter += 1 + operand_type.size();

        Ok(())
    }

    fn current_alignment(&self) -> Alignment {
        match self.out_counter % 2 == 0 {
            true => Alignment::Even,
            false => Alignment::Odd,
        }
    }
}

pub struct Assembler<In: Read, Out: Write> {
    in_stream: In,
    out_stream: Out,
}

impl<In: Read, Out: Write> Assembler<In, Out> {
    pub fn new(in_stream: In, out_stream: Out) -> Self {
        Self {
            in_stream,
            out_stream,
        }
    }

    /// Returns the bytes written on success and the error on error
    pub fn assemble(mut self) -> Result<usize, AssemblyError> {
        let mut byte_map = ByteMap::new();

        byte_map.process_in(&mut self.in_stream)?;

        if log::log_enabled!(log::Level::Debug) {
            log::debug!("Labels:");
            for label in byte_map.label_map.iter() {
                log::debug!("\t{}:\t{:#06x}", label.0, label.1);
            }
        }

        byte_map.generate_data(&mut self.out_stream)
    }
}
