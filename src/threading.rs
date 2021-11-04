use core::arch::x86_64::__cpuid;

use std::
{
    sync::mpsc,
    thread,
};

use crate::rng::hw_rand_u64;

/// Uses the x86-64 instruction `CPUID` to obtain the amount
/// of logical cores in the current machine.
pub fn get_logical_cores() -> usize
{
    // Selects leaf EAX=1 and gets "additional information" from register EBX
    let cpuid = unsafe { __cpuid(1).ebx };

    // Isolate bits 16 ..= 23 with bitwise AND then right shift
    // by 16 to get the maximum number of addressable logical cores.
    return ((cpuid & 0xFF0000) >> 16) as usize
}

pub fn threaded_mpsc(threads: usize, total_size: usize) -> Vec<Vec<u64>>
{
    if threads == 0
    {
        let mut rd_vec = Vec::with_capacity(total_size);

        for _ in 0 .. total_size
        {
            rd_vec.push(hw_rand_u64());
        }

        return vec![rd_vec]
    }
    else
    {
        let mut rd_vec = Vec::with_capacity(total_size);
        let mut workers = Vec::with_capacity(threads);
        let job_size = total_size / threads;
        let (tx, rx) = mpsc::channel();

        for _ in 0 .. threads
        {
            let tx = tx.clone();
            let worker = thread::spawn(
                move ||
                {
                    let mut tmp = Vec::with_capacity(job_size);

                    for _ in 0 .. job_size
                    {
                        tmp.push(hw_rand_u64());
                    }

                    tx.send(tmp).unwrap();
                }
            );
            
            workers.push(worker);
        }
        
        for worker in workers
        {
            worker.join().unwrap();
        }

        while let Ok(recv) = rx.try_recv()
        {
            rd_vec.push(recv);
        }

        return rd_vec
    }
}
