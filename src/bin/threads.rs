use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::thread;
use std::time::Duration;

fn main(){
    let thread1 = thread::spawn(|| {
        for i in 1..25{
            println!("spawned thread : {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..20{
        println!("Main thread: {}",i);
        thread::sleep(Duration::from_millis(1));

    }
    thread1.join().unwrap();
}