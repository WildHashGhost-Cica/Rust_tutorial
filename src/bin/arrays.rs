use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let arr_1: [i32; 10] =  [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("2nd: {}", arr_1[1]);

    println!("Length: {}", arr_1.len());
    let mut loop_idx: usize = 0;
    loop{
        if arr_1[loop_idx] %2 ==  0{
            loop_idx += 1;
            continue;
        }
        if arr_1[loop_idx] == 9 {
            break;
        }
        println!("val :{}", arr_1[loop_idx]);
        loop_idx += 1;
    }
}