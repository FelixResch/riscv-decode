use crate::*;

pub fn decode_q00(i: u32) -> DecodingResult {
    match i >> 13 {
        0b000 if i == 0 => Ok(Instruction::Illegal),
        0b000 => Err(DecodingError::Unimplemented), // C.ADDI4SPN
        0b001 => Err(DecodingError::Unimplemented), // C.FLD
        0b010 => Ok(Instruction::Lw(IType(
            ((i & 0x1c00) << 13)      // imm[5:3]
            | ((i & 0x380) << 8)      // rs1[2:0]
            | ((i & 0x40) << 16)      // imm[2]
            | ((i & 0x20) << 21)      // imm[6]
            | ((i & 0x1c) << 5)       // rd[2:0]
            | 0b_01000_010_01000_0000011,
        ))),
        0b011 => Ok(Instruction::Ld(IType(
            // C.LD (C.FLW in RV32)
            ((i & 0x1c00) << 13)      // imm[5:3]
            | ((i & 0x380) << 8)      // rs1[2:0]
            | ((i & 0x60) << 21)      // imm[7:6]
            | ((i & 0x1c) << 5)       // rd[2:0]
            | 0b_01000_011_01000_0000011,
        ))),
        0b100 => Err(DecodingError::Unimplemented), // reserved
        0b101 => Err(DecodingError::Unimplemented), // C.FSD
        0b110 => Ok(Instruction::Sw(SType(
            // C.SW
            ((i & 0x1000) << 13)      // imm[5]
            | ((i & 0xc00))           // imm[4:3]
            | ((i & 0x380) << 8)      // rs1[2:0]
            | ((i & 0x40) << 3)       // imm[2]
            | ((i & 0x20) << 21)      // imm[6]
            | ((i & 0x1c) << 18)      // rs2[2:0]
            | 0b_01000_01000_010_00000_0100011,
        ))),
        0b111 => Ok(Instruction::Sd(SType(
            // C.SD (C.FSW in RV32)
            ((i & 0x1000) << 13)      // imm[5]
            | ((i & 0xc00))           // imm[4:3]
            | ((i & 0x380) << 8)      // rs1[2:0]
            | ((i & 0x60) << 21)      // imm[7:6]
            | ((i & 0x1c) << 18)      // rs2[2:0]
            | 0b_01000_01000_011_00000_0100011,
        ))),
        _ => Err(DecodingError::Unimplemented),
    }
}

pub fn decode_q01(i: u32) -> DecodingResult {
    match i >> 13 {
        0b011 => match (i >> 7) & 0b11111 {
            0b00 => Ok(Instruction::Illegal),
            0b10 => Ok(Instruction::CAddi16Sp(CAddi16SpType(i as u16))),
            rd => Err(DecodingError::Unimplemented),
        },
        _ => Err(DecodingError::Unimplemented),
    }
}

pub fn decode_q10(i: u32) -> DecodingResult {
    match i >> 13 {
        0b100 => match i >> 12 & 0b1 {
            0b0 => match i >> 7 & 0x1f {
                0 => Ok(Instruction::Illegal),
                _ => match i >> 2 &0x1f {
                    0 => Ok(Instruction::CJr(CRType(i as u16))),
                    _ => Ok(Instruction::CMv(CRType(i as u16))),
                }
            }
            _ => match i >> 7 & 0x1f {
                0 => match i >> 2 & 0x1f {
                    0 => Ok(Instruction::CEBreak(CRType(i as u16))),
                    _ => Ok(Instruction::Illegal),
                }
                _ => match i >> 2 & 0x1f {
                    0 => Ok(Instruction::CJalr(CRType(i as u16))),
                    _ => Ok(Instruction::CAdd(CRType(i as u16))),
                }
            },
        }
        _ => Err(DecodingError::Unimplemented),
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction::*;
    use super::*;

    #[test]
    fn q00() {
        assert_eq!(decode_q00(0x6188).unwrap(), Ld(IType(0x0005b503))); // ld a0,0(a1)
        assert_eq!(decode_q00(0x75e0).unwrap(), Ld(IType(0x0e85b403))); // ld s0,232(a1)
        assert_eq!(decode_q00(0x43b0).unwrap(), Lw(IType(0x0407a603))); // lw a2,64(a5)
        assert_eq!(decode_q00(0xe188).unwrap(), Sd(SType(0x00a5b023))); // sd a0,0(a1)
        assert_eq!(decode_q00(0xf5e0).unwrap(), Sd(SType(0x0e85b423))); // sd s0,232(a1)
        assert_eq!(decode_q00(0xc3b0).unwrap(), Sw(SType(0x04c7a023))); // sw a2,64(a5)
    }
}
