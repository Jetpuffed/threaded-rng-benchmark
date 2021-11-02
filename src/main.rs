use core::arch::x86_64::{
    _rdrand16_step,
    _rdrand32_step,
    _rdrand64_step,
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

fn main() {}
