use gil::asm;
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

#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
unsafe fn mmap_raw(
    addr: *mut u8,
    length: usize,
    prot: u64,
    flags: u64,
    fd: i64,
    offset: usize,
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

#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
unsafe fn munmap_raw(addr: *mut u8, length: usize) {
    asm!(
        "mov rax, 11",
        "syscall",
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        in("rdi") addr,
        in("rsi") length,
    );
}

#[cfg(not(all(target_os = "linux", target_pointer_width = "64")))]
compile_error! { "Only 64-bit linux is supported." }

struct MemHandle {
    buf: *mut u8,
    map_sz: usize,
}

impl MemHandle {
    pub unsafe fn alloc(size: usize, prot: u64, flags: u64, fd: i64, offset: usize) -> MemHandle {
        let ptr = mmap_raw(ptr::null_mut(), size, prot, flags, fd, offset);
        MemHandle {
            buf: ptr,
            map_sz: size,
        }
    }

    pub unsafe fn alloc_copy<T>(
        size: usize,
        prot: u64,
        flags: u64,
        fd: i64,
        offset: usize,
        value: &[T],
    ) -> MemHandle {
        let handle = Self::alloc(size, prot, flags, fd, offset);
        ptr::copy_nonoverlapping(value.as_ptr(), handle.buf as *mut T, value.len());
        handle
    }

    pub fn ptr(&self) -> *mut u8 {
        self.buf
    }
}

impl std::ops::Drop for MemHandle {
    fn drop(&mut self) {
        unsafe { munmap_raw(self.buf, self.map_sz) }
    }
}

fn main() {
    let mut builder = Builder::default();
    let _v0 = builder.copy(gil::Number::UInt32(10));
    dbg!(builder);

    let instructions = vec![
        asm::X8664Instruction::Mov {
            dest: gil::REG_X86_64_ECX,
            src: asm::X8664MovArg::Immediate32(42),
        },
        asm::X8664Instruction::Mov {
            dest: gil::REG_X86_64_EAX,
            src: asm::X8664MovArg::Register(gil::REG_X86_64_ECX),
        },
        asm::X8664Instruction::RetNear,
    ];
    let code: Vec<u8> = instructions
        .iter()
        .flat_map(|ins| ins.x8664_gen())
        .collect();

    println!("CODE: ");
    for b in &code {
        println!("0b{0:08b} | {0:#x}", b);
    }

    let handle = unsafe {
        MemHandle::alloc_copy(
            4096,
            PROT_EXEC | PROT_READ | PROT_WRITE,
            MAP_ANONYMOUS | MAP_PRIVATE,
            -1,
            0,
            code.as_slice(),
        )
    };
    let function: extern "C" fn() -> u32 = unsafe { mem::transmute(handle.ptr()) };
    dbg!(function());
}
