use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);
}