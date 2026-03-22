# VASM, Vintage Assembler

## Registers

AL/H
BL/H
CL/H
DL/H

SP (24-bit)
PC (24-bit)
RET (24-bit)
STAT (8-bit)

## Opcodes

Opcodes and operands are to be aligned in 16 bit chunks. To fix alignment, NOPs
will be inserted.

### Misc Operations
| Hex Value | Opcode | Operand Type | Description |
| 0x00 | NOP | N/A | Performs no operation |
| 0x01 | SYS | IM-U8 | Performs system call that corresponds to the 8-bit int operand |
| 0x02 | SWPA | N/A | Swaps the high and low bytes of A |
| 0x03 | SWPAB | N/A | Swaps A and B |
| 0x04 | SWPABCD | N/A | Swaps AB and CD | 
| 0x05 | PUSHRET | N/A | Pushes RET to the stack |
| 0x06 | POPRET | N/A | Pops RET from the stack |
| 0x07 | CLRSTAT | N/A | Clears the status register |

### Jump Operations
| Hex Value | Opcode | Operand Type | Description |
| 0x10 | JP8 | IM-I8 | Moves the PC to the RET register and adds the 8-bit signed int to the program counter if true flag is set |
| 0x11 | JP24 | IM-24 | Moves the PC to the RET register and loads the 24-bit signed int into the program counter if true flag is set |
| 0x12 | JPCD | N/A | Moves the PC to the RET register and the lower 24 bits of the CD register into the PC register if the true flag is set |
| 0x12 | JPRET | N/A | Moves the RET register into the PC register if the true flag is set |

### Status Operations
| Hex Value | Opcode | Operand Type | Description |
| = | = | = |
| 0x21 | STT | N/A | Sets the true flag high |
| 0x22 | SINV | N/A | Sets the true flag to the inverse of what it is now |
| 0x23 | STEQ | N/A | Sets the true flag if the previous eval set the equal flag, otherwise set it to false |
| 0x24 | STGT | N/A | Sets the true flag if the previous eval set the greater than flag, otherwise set it to false |
| 0x25 | STLT | N/A | Sets the true flag if the previous eval set the less than flag, otherwise set it to false |
| 0x26 | STNZ | N/A | Sets the true flag if the previous load, arithmatic or bitwise operation's result was non-zero |
| 0x27 | STOF | N/A | Sets the true flag if the previous arithmatic operation resulted in an overflow |

### Compare Operations 
| Hex Value | Opcode | Operand Type | Description |
| = | = | = |
| 0x30 | CMPA | N/A | Clears the status register and compares the low and high registers of A |
| 0x34 | CMPAB | N/A | Clears the status register and compares A to B |
| 0x36 | CMPABCD | N/A | Clears the status register and compares AB to CD |

### Data Transfer Operations
| Hex Value | Opcode | Operand Type | Description |
| = | = | = |
| 0x40 | LAL8 | IM-8 | Loads a byte into the low register of A |
| 0x41 | LA16 | IM-16 | Loads two bytes into the A register |
| 0x42 | LAB24 | IM-24 | Loads three bytes into the ABL register |
| 0x43 | LAB32 | IM-32 | Loads four bytes into the AB register |
| 0x44 | LALICD | N/A | Loads the value pointed to by CD into AL |
| 0x45 | LAICD | N/A | Loads the value pointed to by CD into A |
| 0x46 | LABICD | N/A | Loads the value pointed to by CD into AB |
| 0x47 | PSHAL | N/A | Push the low byte of A onto the stack |
| 0x48 | PSHA | N/A | Push A onto the stack |
| 0x49 | PSHAB | Push AB onto the stack |
| 0x4A | POPAL | N/A | Pop the stack into AL |
| 0x4B | POPA | N/A | Pop the stack into A |
| 0x4C | POPAB | Pop the stack into AB |
| 0x4D | SALICD | N/A | Stores AL to the address pointed to by CD |
| 0x4E | SAICD | N/A | Stores A to the address pointed to by CD |
| 0x4F | SABICD | N/A | Stores AB to the address pointed to by CD |

### Arithmatic Operations
| Hex Value | Opcode | Operand Type | Description |
| = | = | = |
| 0x50 | ADDA | N/A | Adds AL to AH and stores the result in AL |
| 0x51 | ADDAB | N/A | Adds A to B and stores the result in A |
| 0x52 | ADDABCD | N/A | Adds AB to CD and stores the result in AB |
| 0x53 | SADDA | N/A | Signed-adds AL to AH and stores the result in AL |
| 0x54 | SADDAB | N/A | Signed-adds A to B and stores the result in A |
| 0x55 | SADDABCD | N/A | Signed-adds AB to CD and stores the result in AB |
| 0x56 | MULA | N/A | Multiplies AL by AH and stores the result in AL |
| 0x57 | MULAB | N/A | Multiplies A by B and stores the result in A |
| 0x58 | MULABCD | N/A | Multiplies AB by CD and stores the result in AB |
| 0x59 | SMULA | N/A | Signed-multiplies AL by AH and stores the result in AL |
| 0x5A | SMULAB | N/A | Signed-multiplies A by B and stores the result in A |
| 0x5B | SMULABCD | N/A | Signed-multiplies AB by CD and stores the result in AB |

### Bitwise Operations 
| Hex Value | Opcode | Operand Type | Description |
| = | = | = |
| 0x60 | ANDA | N/A | Performs bitwise-AND on AL and AH and stores the result in AL |
| 0x61 | ANDAB | N/A | Performs bitwise-AND on A and B and stores the result in A |
| 0x62 | ANDABCD | N/A | Performs bitwise-AND on AB and CD and stores the result in AB |
| 0x63 | ORA | N/A | Performs bitwise-OR on AL and AH and stores the result in AL |
| 0x64 | ORAB | N/A | Performs bitwise-OR on A and B and stores the result in A |
| 0x65 | ORABCD | N/A | Performs bitwise-OR on AB and CD and stores the result in AB |
| 0x66 | NORA | N/A | Performs bitwise-NOR on AL and AH and stores the result in AL |
| 0x67 | NORAB | N/A | Performs bitwise-NOR on A and B and stores the result in A |
| 0x68 | NORABCD | N/A | Performs bitwise-NOR on AB and CD and stores the result in AB |
| 0x69 | INVAL | N/A | Performs bitwise inverse on AL |
| 0x6A | INVA | N/A | Performs bitwise inverse on A |
| 0x6B | INVAB | N/A | Performs bitwise inverse on AB |
| 0x70 | SHLAL | N/A | Logically left-shifts AL by 1 bit |
| 0x71 | SHLA | N/A | Logically left-shifts A by 1 bit |
| 0x72 | SHLAB | N/A | Logically left-shifts AB by 1 bit |
| 0x73 | SHRAL | N/A | Logically right-shifts AL by 1 bit |
| 0x74 | SHRA | N/A | Logically right-shifts A by 1 bit |
| 0x75 | SHRAB | N/A | Logically right-shifts AB by 1 bit |
| 0x76 | RTLAL | N/A | Left-rotates AL by 1 bit |
| 0x77 | RTLA | N/A | Left-rotates A by 1 bit |
| 0x78 | RTLAB | N/A | Left-rotates AB by 1 bit |
| 0x79 | RTRAL | N/A | Right-rotates AL by 1 bit |
| 0x7A | RTRA | N/A | Right-rotates A by 1 bit |
| 0x7B | RTRAB | N/A | Right-rotates AB by 1 bit |
| 0x7C | ASRAL | N/A | Arithmatically right-shifts AL by 1 bit |
| 0x7D | ASRA | N/A | Arithmatically right-shifts A by 1 bit |
| 0x7E | ASRAB | N/A | Arithmatically right-rotates AB by 1 bit |


