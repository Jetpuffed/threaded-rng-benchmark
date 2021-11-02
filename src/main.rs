use core::arch::x86_64::{
    _rdrand16_step,
    _rdrand32_step,
    _rdrand64_step,
};

fn rand_u16() -> u16
{
    let mut n = 0;
    unsafe { _rdrand16_step(n) };

    return n
}

fn main() {}
