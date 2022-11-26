use crate::mir;
use std::io::Write;

impl mir::MirRegister {
    pub fn x8664_gen(&self) -> u8 /* only four bits are used, biggest byte is for special x86 64 registers. (r8 etc) */
    {
        match self.cls {
            mir::AL_CLASS => 0b00000000,
            mir::CL_CLASS => 0b00000001,
            _ => unreachable!(),
        }
    }
}

impl mir::X8664MovArg {
    pub fn x8664_gen(&self) -> Vec<u8> {
        match self {
            mir::X8664MovArg::Immediate32(v) => v.to_le_bytes().to_vec(),
            mir::X8664MovArg::Immediate64(v) => v.to_le_bytes().to_vec(),
            mir::X8664MovArg::Register(r) => vec![r.x8664_gen()],
        }
    }
}

impl mir::X8664Instruction {
    pub fn x8664_gen(&self) -> Vec<u8> {
        match self {
            mir::X8664Instruction::Mov { dest, src } => {
                assert_eq!(dest.is_64bit(), src.is_64bit());
                let is_64bit = dest.is_64bit();
                assert!(!is_64bit, "No support for 64-bit.");
                let dest_byte = dest.x8664_gen();

                if dest_byte & (1 << 7) == 1 {
                    panic!("No support for special 64-bit registers. (dest)");
                }

                match src {
                    mir::X8664MovArg::Immediate64(_) => panic!("No support for 64-bit."),
                    mir::X8664MovArg::Immediate32(v) => {
                        let mut modrm = 0b11000000;
                        modrm |= dest_byte;
                        let mut vec = vec![0xC7, modrm];
                        vec.extend(v.to_le_bytes());
                        vec
                    }
                    mir::X8664MovArg::Register(src) => {
                        let src_byte = src.x8664_gen();
                        if src_byte & (1 << 7) == 1 {
                            panic!("No support for special 64-bit registers. (src)");
                        }
                        let mut modrm = 0b11000000;
                        modrm |= dest_byte << 3;
                        modrm |= src_byte << 0;
                        vec![0x8B, modrm]
                    }
                }
            }
            mir::X8664Instruction::RetNear => vec![0xC3],
        }
    }
}
