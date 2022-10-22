use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    //let var_name = |parameters| -> return_type{BODY}

    let can_vote = |age:i32| {
        age >= 18
    };
    println!(" Can vote: {}", can_vote(8));
}