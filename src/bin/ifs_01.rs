use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let mut my_age = 47;
    let can_vote: bool = if my_age >= 18 {
        true
    }else{
        false
    };
    println!("Can vote: {}", can_vote);
}