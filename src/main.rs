use threaded_number_generator_rs::threading::{get_logical_cores, threaded_mpsc};
use std::time::Instant;

fn main() 
{
    let mut min_threads = 1;
    let max_threads = get_logical_cores();
    let mut input = String::new();

    println!("How many threads would you like to start with? [{} ..= {}]", min_threads, max_threads);
    std::io::stdin().read_line(&mut input).unwrap();
    min_threads = input.trim().parse::<usize>().unwrap_or_else(|_x| 1);

    let mut execution_timings = Vec::new();

    for thread_n in min_threads ..= max_threads
    {
        for size_n in 1 .. 32
        {
            let total_size = 1 << size_n;
            let now = Instant::now();
            threaded_mpsc(thread_n, total_size);
            execution_timings.push(now.elapsed().as_secs_f64());
            println!("[{} thread(s) | {:.4}s | {} ({})]\n", thread_n, now.elapsed().as_secs_f64(), size_n, total_size);
        }
    }
}
