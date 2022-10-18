use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let arr_1: [i32; 10] =  [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut loop_idx: usize =  0;
    while loop_idx < arr_1.len() {
        println!("Arr :{}", arr_1[loop_idx]);
        loop_idx += 1;
    }
  
}
