pub struct MirRegister {
    pub cls: u32,
    pub id: u32,
}

impl MirRegister {
    pub fn is_64bit(&self) -> bool {
        const RAX: u32 = MIR_X86_64_RAX.id;
        const RCX: u32 = MIR_X86_64_RCX.id;
        match self.id {
            RAX | RCX => true,
            _ => false,
        }
    }
}

pub const AL_CLASS: u32 = 0;
pub const CL_CLASS: u32 = 1;

pub const MIR_X86_64_AL: MirRegister = MirRegister {
    cls: AL_CLASS,
    id: 0,
};
pub const MIR_X86_64_AX: MirRegister = MirRegister {
    cls: AL_CLASS,
    id: 1,
};
pub const MIR_X86_64_EAX: MirRegister = MirRegister {
    cls: AL_CLASS,
    id: 2,
};
pub const MIR_X86_64_RAX: MirRegister = MirRegister {
    cls: AL_CLASS,
    id: 3,
};
pub const MIR_X87_64_ST0: MirRegister = MirRegister {
    cls: AL_CLASS,
    id: 4,
};
pub const MIR_X86_64_MMX0: MirRegister = MirRegister {
    cls: AL_CLASS,
    id: 5,
};
pub const MIR_X86_64_XMM0: MirRegister = MirRegister {
    cls: AL_CLASS,
    id: 6,
};
pub const MIR_X86_64_YMM0: MirRegister = MirRegister {
    cls: AL_CLASS,
    id: 7,
};

pub const MIR_X86_64_CL: MirRegister = MirRegister {
    cls: CL_CLASS,
    id: 8,
};
pub const MIR_X86_64_CX: MirRegister = MirRegister {
    cls: CL_CLASS,
    id: 9,
};
pub const MIR_X86_64_ECX: MirRegister = MirRegister {
    cls: CL_CLASS,
    id: 10,
};
pub const MIR_X86_64_RCX: MirRegister = MirRegister {
    cls: CL_CLASS,
    id: 11,
};
pub const MIR_X87_64_ST1: MirRegister = MirRegister {
    cls: CL_CLASS,
    id: 12,
};
pub const MIR_X86_64_MMX1: MirRegister = MirRegister {
    cls: CL_CLASS,
    id: 13,
};
pub const MIR_X86_64_XMM1: MirRegister = MirRegister {
    cls: CL_CLASS,
    id: 14,
};
pub const MIR_X86_64_YMM1: MirRegister = MirRegister {
    cls: CL_CLASS,
    id: 15,
};

pub const MIR_LAST_HARD_REGISTER: u32 = MIR_X86_64_YMM0.id;

pub enum MirInstruction {
    X8664(X8664Instruction),
}

pub enum X8664Instruction {
    Mov { dest: MirRegister, src: X8664MovArg },
    RetNear,
}

pub enum X8664MovArg {
    Immediate64(u64),
    Immediate32(u32),
    Register(MirRegister),
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
