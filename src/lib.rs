use std::thread;

#[no_mangle]
pub extern fn  process(threads: usize, count: u64) -> u64 {
    // create OS native threads and collect thread handles into vector
    // each thread do the some counting...
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            thread::spawn(move || {
                let mut x = 0u64;
                for _ in 0..count {
                    x += 1
                }
                x
            })
        })
        .collect();

    // wait for each thread completion, print result and return sum of all results
    // NOTE: join(mut self) - consumes thread handles, so we must use into_iter(), not iter()!
    handles
        .into_iter()
        .map(|h| {
            let res = h.join().map_err(|_| "Cant thread assess!").unwrap();
            println!("Thread completed with count={}", res);
            res
        })
        .sum()
}