use threaded_number_generator_rs::threading::{get_logical_cores, threaded_mpsc};
use std::time::Instant;

fn main() 
{
    let max_threads = get_logical_cores() - 1;
    let mut input = String::new();

    println!("How many threads would you like to start with? [0 .. {}]", max_threads);
    std::io::stdin().read_line(&mut input).unwrap();
    let min_threads = input.trim().parse::<usize>().unwrap_or_else(|_x| 0);

    let mut all_execution_timings = Vec::with_capacity(max_threads);

    for thread_n in min_threads ..= max_threads
    {
        let mut execution_timings = Vec::with_capacity(32);

        for size_n in 1 .. 32
        {
            let total_size = 1 << size_n;
            let now = Instant::now();
            threaded_mpsc(thread_n, total_size);
            execution_timings.push(now.elapsed().as_secs_f64());
            println!("[{} thread(s) | {:.4}s | {} ({})]\n", thread_n + 1, now.elapsed().as_secs_f64(), size_n, total_size);
        }

        all_execution_timings.push(execution_timings);
    }

    let mut i = min_threads;
    for timings in all_execution_timings
    {
        let mut n = 0.0;

        for timing in timings
        {
            n += timing;
        }

        println!("{} threaded average: {}s", i + 1, n / 32.0);
        i += 1;
    }
}
