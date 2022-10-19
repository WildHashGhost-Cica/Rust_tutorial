use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

use std::ops::Add;
//if we want to use different data type 
fn get_sum_gen<T:Add<Output = T>>(x: T, y:T) -> T {
   return x + y;
}

fn main(){
   println!("5 + 4 = {}", get_sum_gen(5,4));
   println!("5.2 + 4.3 = {}", get_sum_gen(5.2, 4.3));
}