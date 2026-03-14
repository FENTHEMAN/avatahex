use std::thread::spawn;

use crossbeam::channel::unbounded;

use crate::{artist::Operation, utils::byte::parse_byte};

enum Work {
    Task((usize, u8)),
    Finished
}

const N_THREADS: u8 = 2;

pub fn queue_parse(input: &str) -> Vec<Operation> {
    let (todo_tx, todo_rx) = unbounded::<Work>();
    let (results_tx, results_rx) = unbounded::<(usize, Operation)>();

    let mut n_bytes = 0;

    for (i, byte) in input.bytes().enumerate() {
        todo_tx.send(Work::Task((i, byte))).unwrap();
        n_bytes += 1;
    }

    for _ in 0..N_THREADS {
        todo_tx.send(Work::Finished).unwrap();
    }

    for _ in 0..N_THREADS {
        let todo = todo_rx.clone();
        let results = results_tx.clone();
        spawn(move || loop {
            let task = todo.recv();

            let result = match task {
                Err(_) => break,
                Ok(Work::Finished) => break,
                Ok(Work::Task((i, byte))) => (i, parse_byte(byte))
            };

            results.send(result).unwrap();
        });
    }

    let mut operations = vec![Operation::Noop(0); n_bytes];
    for _ in 0..n_bytes {
        let (i, operation) = results_rx.recv().unwrap();
        operations[i] = operation;
    }

    operations
}