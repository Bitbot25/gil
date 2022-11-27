#[derive(Copy, Clone, Debug)]
pub enum Register {
    Pseudo(usize),
    Hard(HardRegister),
}

impl Default for Register {
    fn default() -> Self {
        Register::Pseudo(0)
    }
}

impl Register {
    pub fn is_64bit(&self) -> bool {
        match self {
            Register::Pseudo(_) => panic!("No register."),
            Register::Hard(h) => h.is_64bit(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct HardRegister {
    pub cls: RegisterClass,
    pub id: u32,
}

impl HardRegister {
    pub fn is_64bit(&self) -> bool {
        const RAX: u32 = REG_X86_64_RAX.id;
        const RCX: u32 = REG_X86_64_RCX.id;
        match self.id {
            RAX | RCX => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RegisterClass {
    Al,
    Cl,
}

pub const REG_X86_64_AL: HardRegister = HardRegister {
    cls: RegisterClass::Al,
    id: 0,
};
pub const REG_X86_64_AX: HardRegister = HardRegister {
    cls: RegisterClass::Al,
    id: 1,
};
pub const REG_X86_64_EAX: HardRegister = HardRegister {
    cls: RegisterClass::Al,
    id: 2,
};
pub const REG_X86_64_RAX: HardRegister = HardRegister {
    cls: RegisterClass::Al,
    id: 3,
};
pub const REG_X87_64_ST0: HardRegister = HardRegister {
    cls: RegisterClass::Al,
    id: 4,
};
pub const REG_X86_64_MMX0: HardRegister = HardRegister {
    cls: RegisterClass::Al,
    id: 5,
};
pub const REG_X86_64_XMM0: HardRegister = HardRegister {
    cls: RegisterClass::Al,
    id: 6,
};
pub const REG_X86_64_YMM0: HardRegister = HardRegister {
    cls: RegisterClass::Al,
    id: 7,
};

pub const REG_X86_64_CL: HardRegister = HardRegister {
    cls: RegisterClass::Cl,
    id: 8,
};
pub const REG_X86_64_CX: HardRegister = HardRegister {
    cls: RegisterClass::Cl,
    id: 9,
};
pub const REG_X86_64_ECX: HardRegister = HardRegister {
    cls: RegisterClass::Cl,
    id: 10,
};
pub const REG_X86_64_RCX: HardRegister = HardRegister {
    cls: RegisterClass::Cl,
    id: 11,
};
pub const REG_X87_64_ST1: HardRegister = HardRegister {
    cls: RegisterClass::Cl,
    id: 12,
};
pub const REG_X86_64_MMX1: HardRegister = HardRegister {
    cls: RegisterClass::Cl,
    id: 13,
};
pub const REG_X86_64_XMM1: HardRegister = HardRegister {
    cls: RegisterClass::Cl,
    id: 14,
};
pub const REG_X86_64_YMM1: HardRegister = HardRegister {
    cls: RegisterClass::Cl,
    id: 15,
};

pub const LAST_HARD_REGISTER: u32 = REG_X86_64_YMM0.id;
