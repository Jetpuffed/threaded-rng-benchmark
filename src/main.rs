use core::arch::x86_64::
{
    __cpuid,
    _rdrand16_step,
    _rdrand32_step,
    _rdrand64_step,
};

use std::
{
    marker::Send,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

trait Random
{
    fn random() -> Self;
}

impl Random for u16
{
    fn random() -> u16
    {
        let mut n = 0;
        unsafe { _rdrand16_step(&mut n) };
        return n
    }
}

impl Random for u32
{
    fn random() -> u32
    {
        let mut n = 0;
        unsafe { _rdrand32_step(&mut n) };
        return n
    }
}

impl Random for u64
{
    fn random() -> u64
    {
        let mut n = 0;
        unsafe { _rdrand64_step(&mut n) };
        return n
    }
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

/// Creates Arc<Mutex<`data`>> from a function to increase readability.
fn create_new_mutex<T>(data: T) -> Arc<Mutex<T>>
{
    return Arc::new(Mutex::new(data))
}

/// Evaluates the time it takes to generate a vector of length `size`
/// with random `T` type values. The amount of tests directly
/// corresponds to the amount of logical cores in your machine.
fn eval_threading<T: 'static + Random + Send>(size: usize)
{
    for threads in 0 .. get_logical_cores()
    {
        let vec_t = create_new_mutex::<Vec<T>>(Vec::with_capacity(size));
        let mut handles = Vec::new();

        for _ in 0 .. threads
        {
            let vec_t = Arc::clone(&vec_t);
            let handle = thread::spawn(move || {
                for _ in 0 .. (size / threads as usize)
                {
                    let mut target = vec_t.lock().unwrap();
                    target.push(T::random());
                }
            });

            handles.push(handle);
        }

        let now = Instant::now();
        for handle in handles
        {
            handle.join().unwrap();
        }
        println!("{} threaded operation took {} second(s) to complete.", threads, now.elapsed().as_secs());
    }
}

fn main() {}
