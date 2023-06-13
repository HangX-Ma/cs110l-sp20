use crossbeam_channel;
#[allow(unused)]
use core::num;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    // we need to resize output_vec, otherwise, the index will out of bounds,
    // on account of the disordered threads
    output_vec.resize_with(input_vec.len(), Default::default);
    // TODO: implement parallel map!
    let (input_sender, input_receiver) = crossbeam_channel::unbounded();
    let (output_sender, output_receiver) = crossbeam_channel::unbounded();
    let mut threads = Vec::new();

    for _ in 0..num_threads {
        let input_receiver = input_receiver.clone();
        let output_sender = output_sender.clone();
        threads.push(thread::spawn(move || {
            while let Ok(elem_pair) = input_receiver.recv() {
                // receive the tuple and send back the processed pair
                let (idx, data) = elem_pair;
                output_sender
                    .send((idx, f(data)))
                    .expect("Tried writing to 'output' channel, but there are no receivers!");
            }
        }));
    }

    for i in (0..input_vec.len()).rev() {
        input_sender
            .send((i, input_vec.pop().unwrap()))
            .expect("Tried writing to 'input' channel, but there are no receivers!");
    }

    drop(input_sender);
    drop(output_sender);

    while let Ok(elem_pair) = output_receiver.recv() {
        let (idx, data) = elem_pair;
        output_vec[idx] = data;
    }

    // join the threads until all work finishes
    for thread in threads {
        thread.join().expect("Panic occurred in thread!");
    }

    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
