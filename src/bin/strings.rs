use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let mut st1: String =  String::new();
    st1.push('A');
    st1.push_str(" world");
    for word in st1.split_whitespace(){
        println!(" {}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);
}