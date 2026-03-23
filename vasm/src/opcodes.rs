#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum OperandType {
    None,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    AddrRel,
    AddrAbs,
}

impl OperandType {
    pub fn size(&self) -> usize {
        match self {
            Self::None => 0,
            Self::I8 | Self::U8 | Self::AddrRel => 1,
            Self::I16 | Self::U16 => 2,
            Self::AddrAbs => 3,
            Self::I32 | Self::U32 => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Alignment {
    Even,
    Odd,
}

pub struct OpcodeSpec {
    pub hex: u8,
    pub string: &'static str,
    pub operand_type: OperandType,
}

impl OpcodeSpec {
    pub fn required_alignment(&self) -> Option<Alignment> {
        if self.operand_type.size() == 0 {
            return None;
        }

        let alignment = match self.operand_type.size() % 2 == 0 {
            true => Alignment::Odd,
            false => Alignment::Even,
        };

        Some(alignment)
    }
}

pub static OPCODE_SPECS: [OpcodeSpec; 5] = [
    OpcodeSpec {
        hex: 0x00,
        string: "NOP",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x01,
        string: "SYS",
        operand_type: OperandType::U8,
    },
    OpcodeSpec {
        hex: 0x10,
        string: "JP8",
        operand_type: OperandType::AddrRel,
    },
    OpcodeSpec {
        hex: 0x11,
        string: "JP24",
        operand_type: OperandType::AddrAbs,
    },
    OpcodeSpec {
        hex: 0x12,
        string: "JPRET",
        operand_type: OperandType::None,
    },
];
