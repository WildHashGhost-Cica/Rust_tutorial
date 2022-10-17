use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let num_1: u32 = 12;
    let num_2: u32 = 4;
    println!("12 + 4 = {}", num_1 + num_2);
    println!("12 - 4 = {}", num_1 - num_2);
    println!("12 / 4 = {}", num_1 / num_2);
    println!("12 % 4 = {}", num_1 % num_2);
}