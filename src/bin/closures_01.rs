use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let mut samp1 = 5;
    let print_var = || println!("Samp1 = {}", samp1);
    print_var();

    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 ={}", samp1);
    samp1 = 10;
    println!("samp1 ={}",samp1);
}