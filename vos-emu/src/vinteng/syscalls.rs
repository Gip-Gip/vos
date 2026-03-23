use crate::vinteng::VintEngine;
use crate::vinteng::VintError;

type VintEngineSyscall = fn(&mut VintEngine) -> Result<(), VintError>;

/// Args:
///     | fhandle: u8
///     | size: u32,
///     V ptr: u32,
///
/// Returns:
///     V size_written: Result<u32, IoError>,
fn writef(vinteng: &mut VintEngine) -> Result<(), VintError> {
    let fhandle = vinteng.pop_u8()?;
    let size = vinteng.pop_u32()?;
    let ptr = vinteng.pop_u32()?;

    let result = match fhandle {
        0 | 1 => {
            todo!()
        }
        2 => {
            let str_window = vinteng.mem_map.access(ptr, size)?;

            println!("{}", str::from_utf8(str_window).unwrap());

            str_window.len()
        }
        _ => {
            panic!("{}", fhandle);
        }
    };

    // !TODO! error handling
    vinteng.push_u32(result as u32)?;
    // 0 to signify no error
    vinteng.push_u8(0)?;

    Ok(())
}

fn unknown(vinteng: &mut VintEngine) -> Result<(), VintError> {
    return Err(VintError::UnknownSyscall);
}

pub static VINT_SYSCALL: [VintEngineSyscall; 256] = [
    writef, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown,
];
