use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let arr_1: [i32; 10] =  [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut loop_idx = 0;

    for val in arr_1.iter(){
        println!("Val: {}", val);
    }
 }