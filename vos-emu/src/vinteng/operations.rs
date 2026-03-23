use crate::vinteng::VintEngine;
use crate::vinteng::VintError;
use crate::vinteng::syscalls::VINT_SYSCALL;

type VintEngineOperation = fn(&mut VintEngine, &[u8; 2]) -> Result<(), VintError>;

fn nop(_vinteng: &mut VintEngine, _buffer: &[u8; 2]) -> Result<(), VintError> {
    return Ok(());
}

fn sys(vinteng: &mut VintEngine, buffer: &[u8; 2]) -> Result<(), VintError> {
    vinteng.invalid_buffer = true;

    let syscall = buffer[1];
    vinteng.pc += 1;

    VINT_SYSCALL[syscall as usize](vinteng)?;

    return Ok(());
}

fn swpa(vinteng: &mut VintEngine, _buffer: &[u8; 2]) -> Result<(), VintError> {
    vinteng.a.reverse();

    return Ok(());
}

fn swpab(vinteng: &mut VintEngine, _buffer: &[u8; 2]) -> Result<(), VintError> {
    let new_a = vinteng.b;
    vinteng.b = vinteng.a;
    vinteng.a = new_a;

    return Ok(());
}

fn swpabcd(vinteng: &mut VintEngine, _buffer: &[u8; 2]) -> Result<(), VintError> {
    let new_a = vinteng.c;
    let new_b = vinteng.d;
    vinteng.c = vinteng.a;
    vinteng.d = vinteng.b;
    vinteng.a = new_a;
    vinteng.b = new_b;

    return Ok(());
}

fn jp8(vinteng: &mut VintEngine, buffer: &[u8; 2]) -> Result<(), VintError> {
    vinteng.invalid_buffer = true;

    let operand = i8::from_le_bytes([buffer[1]]);
    vinteng.pc += 1;
    vinteng.ret = vinteng.pc;

    vinteng.pc += operand as u32;

    return Ok(());
}

fn jp24(vinteng: &mut VintEngine, buffer: &[u8; 2]) -> Result<(), VintError> {
    vinteng.invalid_buffer = true;

    let mut operand_bytes = [0u8; 4];

    operand_bytes[0] = buffer[1];
    vinteng.pc += 1;

    let window = vinteng.mem_map.access(vinteng.pc, 2)?;
    operand_bytes[1..3].copy_from_slice(window);

    let operand = u32::from_le_bytes(operand_bytes);

    vinteng.pc += 2;
    vinteng.ret = vinteng.pc;

    vinteng.pc = operand;

    return Ok(());
}

fn jpret(vinteng: &mut VintEngine, _buffer: &[u8; 2]) -> Result<(), VintError> {
    vinteng.invalid_buffer = true;

    vinteng.pc = vinteng.ret;

    return Ok(());
}

fn exit(vinteng: &mut VintEngine, _buffer: &[u8; 2]) -> Result<(), VintError> {
    vinteng.invalid_buffer = true;

    vinteng.halt_req = true;
    return Ok(());
}

fn lal8(vinteng: &mut VintEngine, buffer: &[u8; 2]) -> Result<(), VintError> {
    vinteng.invalid_buffer = true;

    let operand = buffer[1];
    vinteng.pc += 1;

    vinteng.a[0] = operand;

    Ok(())
}
fn lab24(vinteng: &mut VintEngine, buffer: &[u8; 2]) -> Result<(), VintError> {
    vinteng.invalid_buffer = true;

    let mut operand_bytes = [0u8; 4];

    operand_bytes[0] = buffer[1];
    vinteng.pc += 1;

    let window = vinteng.mem_map.access(vinteng.pc, 2)?;
    operand_bytes[1..3].copy_from_slice(window);

    vinteng.a.copy_from_slice(&operand_bytes[..2]);
    vinteng.b.copy_from_slice(&operand_bytes[2..]);

    vinteng.pc += 2;

    Ok(())
}

fn lab32(vinteng: &mut VintEngine, _buffer: &[u8; 2]) -> Result<(), VintError> {
    vinteng.invalid_buffer = true;

    let window_1 = vinteng.mem_map.access(vinteng.pc, 2)?;
    vinteng.a.copy_from_slice(window_1);
    vinteng.pc += 2;
    let window_2 = vinteng.mem_map.access(vinteng.pc, 2)?;
    vinteng.b.copy_from_slice(window_2);
    vinteng.pc += 2;

    Ok(())
}

fn lalicd(vinteng: &mut VintEngine, _buffer: &[u8; 2]) -> Result<(), VintError> {
    let window = vinteng.mem_map.access(vinteng.cd_as_u32(), 1)?;

    vinteng.a[0] = window[0];

    Ok(())
}

fn laicd(vinteng: &mut VintEngine, _buffer: &[u8; 2]) -> Result<(), VintError> {
    let window = vinteng.mem_map.access(vinteng.cd_as_u32(), 2)?;

    vinteng.a.copy_from_slice(window);

    Ok(())
}

fn pshal(vinteng: &mut VintEngine, _buffer: &[u8; 2]) -> Result<(), VintError> {
    vinteng.push_u8(vinteng.a[0])
}

fn pshab(vinteng: &mut VintEngine, _buffer: &[u8; 2]) -> Result<(), VintError> {
    vinteng.push_u32(vinteng.ab_as_u32())
}

fn saddial(vinteng: &mut VintEngine, buffer: &[u8; 2]) -> Result<(), VintError> {
    vinteng.invalid_buffer = true;

    let operand = i8::from_le_bytes([buffer[1]]);
    vinteng.pc += 1;

    let mut a = i8::from_le_bytes([vinteng.a[0]]);

    a += operand;

    vinteng.a[0] = a.to_le_bytes()[0];

    Ok(())
}

fn saddiab(vinteng: &mut VintEngine, _buffer: &[u8; 2]) -> Result<(), VintError> {
    vinteng.invalid_buffer = true;

    let mut operand = [0; 4];

    let window_1 = vinteng.mem_map.access(vinteng.pc, 2)?;
    operand[..2].copy_from_slice(window_1);
    vinteng.pc += 2;
    let window_2 = vinteng.mem_map.access(vinteng.pc, 2)?;
    operand[2..].copy_from_slice(window_2);
    vinteng.pc += 2;

    let sum = vinteng.ab_as_i32() + i32::from_le_bytes(operand);

    let sum_bytes = sum.to_le_bytes();

    vinteng.a.copy_from_slice(&sum_bytes[..2]);
    vinteng.b.copy_from_slice(&sum_bytes[2..]);

    Ok(())
}

fn unknown(vinteng: &mut VintEngine, buffer: &[u8; 2]) -> Result<(), VintError> {
    let i_opcode = (vinteng.pc - 1) % 2;
    return Err(VintError::UnknownOperation(buffer[i_opcode as usize]));
}

pub static VINT_OPERATIONS: [VintEngineOperation; 256] = [
    nop, sys, swpa, swpab, swpabcd, unknown, unknown, unknown, exit, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, jp8, jp24, unknown, jpret, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, lal8, unknown, lab24, lab32, lalicd, laicd, unknown, pshal, unknown, pshab, unknown,
    unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown, unknown,
    unknown, saddial, unknown, saddiab, unknown, unknown, unknown, unknown, unknown, unknown,
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
    unknown,
];
