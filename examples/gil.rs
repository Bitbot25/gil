use gil::mir;
use gil::Builder;
use std::arch::asm;
use std::{mem, ptr};

const PROT_READ: u64 = 0x1;
const PROT_WRITE: u64 = 0x2;
const PROT_EXEC: u64 = 0x4;
const PROT_NONE: u64 = 0x0;

const MAP_SHARED: u64 = 0x01;
const MAP_PRIVATE: u64 = 0x02;
const MAP_ANONYMOUS: u64 = 0x20;

unsafe fn mmap_raw(
    addr: *mut u8,
    length: u64,
    prot: u64,
    flags: u64,
    fd: i64,
    offset: u64,
) -> *mut u8 {
    let mut result: u64 = 9;
    asm!(
         "syscall",
         inout("rax") result,
         out("rcx") _,
         out("r11") _,

         in("rdi") addr,
         in("rsi") length,
         in("rdx") prot,
         in("r10") flags,
         in("r8") fd,
         in("r9") offset,
    );
    result as *mut u8
}

fn main() {
    let mut builder = Builder::default();
    let v0 = builder.copy(gil::Number::UInt32(10));
    dbg!(builder);

    let instructions = vec![
        mir::X8664Instruction::Mov {
            dest: mir::MIR_X86_64_ECX,
            src: mir::X8664MovArg::Immediate32(42),
        },
        mir::X8664Instruction::Mov {
            dest: mir::MIR_X86_64_EAX,
            src: mir::X8664MovArg::Register(mir::MIR_X86_64_ECX),
        },
        mir::X8664Instruction::RetNear,
    ];
    let code: Vec<u8> = instructions
        .iter()
        .flat_map(|ins| ins.x8664_gen())
        .collect();

    println!("CODE: ");
    for b in &code {
        println!("0b{0:08b} | {0:#x}", b);
    }

    let exec = unsafe {
        mmap_raw(
            ptr::null_mut(),
            4096,
            PROT_READ | PROT_WRITE | PROT_EXEC,
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        )
    };
    unsafe { ptr::copy_nonoverlapping(code.as_ptr(), exec, code.len()) }
    let fn_ptr: extern "C" fn() -> u32 = unsafe { mem::transmute(exec) };
    dbg!(fn_ptr());
}
