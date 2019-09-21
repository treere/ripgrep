extern crate crossbeam_channel as channel;
extern crate ignore;

use std::env;
use std::thread;

use ignore::WalkBuilder;

fn main() {
    let mut path = env::args().nth(1).unwrap();
    let mut parallel = false;
    let (tx, rx) = channel::bounded::<Result<ignore::DirEntry,ignore::Error>>(100);
    if path == "parallel" {
        path = env::args().nth(2).unwrap();
        parallel = true;
    } else if path == "walkdir" {
        path = env::args().nth(2).unwrap();
    }

    let stdout_thread = thread::spawn(move || {
        for dent in rx {
            println!("{:?}",dent);
        }
    });

    if parallel {
        let walker = WalkBuilder::new(path).threads(1).build_parallel();
        walker.run(|| {
            let tx = tx.clone();
            Box::new(move |result| {
                use ignore::WalkState::*;

                tx.send(result).unwrap();
                Continue
            })
        });
    } else {
        let walker = WalkBuilder::new(path).build();
        for result in walker {
            tx.send(result).unwrap();
        }
    }
    drop(tx);
    stdout_thread.join().unwrap();
}
