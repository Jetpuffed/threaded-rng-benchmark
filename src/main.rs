use core::arch::x86_64::
{
    __cpuid,
    _rdrand16_step,
    _rdrand32_step,
    _rdrand64_step,
};

use std::
{
    sync::{Arc, Mutex},
    thread
};

fn rand_u16() -> u16
{
    let mut n = 0;
    unsafe { _rdrand16_step(&mut n) };

    return n
}

fn rand_u32() -> u32
{
    let mut n = 0;
    unsafe { _rdrand32_step(&mut n) };

    return n
}

fn rand_u64() -> u64
{
    let mut n = 0;
    unsafe { _rdrand64_step(&mut n) };

    return n
}

/// Uses the x86-64 instruction `CPUID` to obtain the amount
/// of logical cores in the current machine.
fn get_logical_cores() -> u32
{
    // Selects leaf EAX=1 and gets "additional information" from register EBX
    let cpuid = unsafe { __cpuid(1).ebx };

    // Isolate bits 16 ..= 23 with bitwise AND then right shift
    // by 16 to get the maximum number of addressable logical cores.
    return (cpuid & 0xFF0000) >> 16
}

fn main() {}
