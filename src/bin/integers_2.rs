use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let num_1:f32 = 1.11111111;
    println!("f32: {}",num_1 + 0.111111);

    let num_2:f64 = 1.11111111;
    println!("f32: {}",num_2 + 0.111111);
}