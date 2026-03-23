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

pub static OPCODE_SPECS: [OpcodeSpec; 25] = [
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
        hex: 0x02,
        string: "SWPA",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x03,
        string: "SWPAB",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x04,
        string: "SWPABCD",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x08,
        string: "EXIT",
        operand_type: OperandType::None,
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
        hex: 0x13,
        string: "JPRET",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x40,
        string: "LAL8",
        operand_type: OperandType::U8,
    },
    OpcodeSpec {
        hex: 0x41,
        string: "LA16",
        operand_type: OperandType::U16,
    },
    OpcodeSpec {
        hex: 0x42,
        string: "LAB24",
        operand_type: OperandType::AddrAbs,
    },
    OpcodeSpec {
        hex: 0x43,
        string: "LAB32",
        operand_type: OperandType::U32,
    },
    OpcodeSpec {
        hex: 0x44,
        string: "LALICD",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x45,
        string: "LAICD",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x46,
        string: "LABICD",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x47,
        string: "PSHAL",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x48,
        string: "PSHA",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x49,
        string: "PSHAB",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x4A,
        string: "POPAL",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x4B,
        string: "POPA",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x4C,
        string: "POPAB",
        operand_type: OperandType::None,
    },
    OpcodeSpec {
        hex: 0x56,
        string: "SADDIAL",
        operand_type: OperandType::I8,
    },
    OpcodeSpec {
        hex: 0x57,
        string: "SADDIA",
        operand_type: OperandType::I16,
    },
    OpcodeSpec {
        hex: 0x58,
        string: "SADDIAB",
        operand_type: OperandType::I32,
    },
];
