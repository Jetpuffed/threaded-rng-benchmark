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

const SEED: u64 = 42;

#[derive(Clone, Copy)]
struct PCG
{
    state: u64,
    inc: u64,
}

trait Random
{
    fn random() -> Self;
    fn pcg(rng: PCG) -> Self;
}

impl Random for u16
{
    fn random() -> u16
    {
        let mut n = 0;
        unsafe { _rdrand16_step(&mut n) };
        return n
    }

    fn pcg(mut rng: PCG) -> u16
    {
        let old_state = rng.state;
        rng.state = old_state.wrapping_mul(6364136223846793005) + (rng.inc | 1);
        let xor_shifted = ((old_state >> 18) ^ old_state) >> 27;
        let rot = old_state >> 59;
        return ((xor_shifted >> rot) | (xor_shifted << (rot.wrapping_neg() & 15))) as u16
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

    fn pcg(mut rng: PCG) -> u32
    {
        let old_state = rng.state;
        rng.state = old_state.wrapping_mul(6364136223846793005) + (rng.inc | 1);
        let xor_shifted = ((old_state >> 18) ^ old_state) >> 27;
        let rot = old_state >> 59;
        return ((xor_shifted >> rot) | (xor_shifted << (rot.wrapping_neg() & 31))) as u32
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

    fn pcg(mut rng: PCG) -> u64
    {
        let old_state = rng.state;
        rng.state = old_state.wrapping_mul(6364136223846793005) + (rng.inc | 1);
        let xor_shifted = ((old_state >> 18) ^ old_state) >> 27;
        let rot = old_state >> 59;
        return (xor_shifted >> rot) | (xor_shifted << (rot.wrapping_neg() & 63))
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
    let rng: PCG = PCG { state: SEED, inc: SEED };

    for threads in 1 ..= get_logical_cores()
    {
        let vec_t = create_new_mutex::<Vec<T>>(Vec::with_capacity(size));
        let mut handles = Vec::new();
        let chunk_size: usize = size / threads as usize;

        for _ in 0 .. threads
        {
            let vec_t = Arc::clone(&vec_t);
            let handle = thread::spawn(move || {
                let mut dst = vec_t.lock().unwrap();
                let mut tmp: Vec<T> = Vec::with_capacity(chunk_size);

                for _ in 0 .. chunk_size
                {
                    tmp.push(T::pcg(rng));
                }

                for _ in 0 .. chunk_size
                {
                    dst.push(tmp.pop().unwrap())
                }
            });

            handles.push(handle);
        }

        let now = Instant::now();
        for handle in handles
        {
            handle.join().unwrap();
        }
        println!("{} threaded operation took {:#.4} second(s) to complete.", threads, now.elapsed().as_secs_f64());
    }
}

fn main()
{
    let mut t_buf = String::new();
    let mut n_buf = String::new();
    println!("Enter type (u16 | u32 | u64):\n");
    std::io::stdin().read_line(&mut t_buf).unwrap();
    println!("Enter size (1 << #):\n");
    std::io::stdin().read_line(&mut n_buf).unwrap();

    match t_buf.as_str().trim()
    {
        "u16" => { eval_threading::<u16>(1 << n_buf.trim().parse::<usize>().unwrap()) }
        "u32" => { eval_threading::<u32>(1 << n_buf.trim().parse::<usize>().unwrap()) }
        "u64" => { eval_threading::<u64>(1 << n_buf.trim().parse::<usize>().unwrap()) }
        _ => panic!("That type is not valid!")
    }
    
}
