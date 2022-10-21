use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let mut arr_it =  [ 1, 2, 3, 4];
    for val in arr_it.iter(){
        println!("{}", val);
    }
    let mut iter1 = arr_it.iter();
    println!("1st:{:?}", iter1.next());
}