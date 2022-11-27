use crate::reg;

pub enum X8664Instruction {
    Mov {
        dest: reg::HardRegister,
        src: X8664MovArg,
    },
    RetNear,
    RetFar,
}

pub enum X8664MovArg {
    Immediate64(u64),
    Immediate32(u32),
    Register(reg::HardRegister),
}

impl X8664MovArg {
    pub fn is_64bit(&self) -> bool {
        match self {
            X8664MovArg::Immediate32(_) => false,
            X8664MovArg::Immediate64(_) => true,
            X8664MovArg::Register(reg) => reg.is_64bit(),
        }
    }

    pub fn is_immediate(&self) -> bool {
        match self {
            X8664MovArg::Register(_) => false,
            _ => true,
        }
    }

    pub fn is_register(&self) -> bool {
        !self.is_immediate()
    }
}
