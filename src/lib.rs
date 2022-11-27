#![allow(dead_code)]
pub mod asm;
pub mod mir;
mod reg;
pub mod x8664gen;
pub use reg::*;

#[derive(Debug)]
pub enum Number {
    Int32(i32),
    Int64(i64),
    UInt32(u32),
    UInt64(u64),
}

#[derive(Debug)]
pub enum RValue {
    Register(Register),
    Number(Number),
}

impl From<Register> for RValue {
    fn from(r: Register) -> Self {
        RValue::Register(r)
    }
}

impl From<Number> for RValue {
    fn from(n: Number) -> Self {
        RValue::Number(n)
    }
}

#[derive(Debug)]
pub struct OpSAlloc {
    dest: Register,
    bytes: usize,
}

#[derive(Debug)]
pub struct OpMvr {
    dest: Register,
    val: RValue,
}

#[derive(Debug)]
pub struct OpMvp {
    ptr: Register,
    val: RValue,
}

#[derive(Debug)]
pub struct OpAdd {
    dest: Register,
    a: RValue,
    b: RValue,
}

#[derive(Debug)]
pub enum Op {
    SAlloc(OpSAlloc),
    Mvr(OpMvr),
    Mvp(OpMvp),
    Add(OpAdd),
}

#[derive(Debug, Default)]
pub struct Builder {
    ops: Vec<Op>,
    next_reg: usize,
}

impl Builder {
    #[inline]
    fn next_gen(&mut self) -> Register {
        let reg = Register::Pseudo(self.next_reg);
        self.next_reg += 1;
        reg
    }

    #[inline]
    pub fn salloc(&mut self, bytes: usize) -> Register {
        let dest = self.next_gen();
        self.ops.push(Op::SAlloc(OpSAlloc { dest, bytes }));
        dest
    }

    #[inline]
    pub fn copy<R: Into<RValue>>(&mut self, rhs: R) -> Register {
        let dest = self.next_gen();
        self.ops.push(Op::Mvr(OpMvr {
            dest,
            val: rhs.into(),
        }));
        dest
    }

    #[inline]
    pub fn add<A: Into<RValue>, B: Into<RValue>>(&mut self, a: A, b: B) -> Register {
        let dest = self.next_gen();
        self.ops.push(Op::Add(OpAdd {
            dest,
            a: a.into(),
            b: b.into(),
        }));
        dest
    }
}
