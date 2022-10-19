use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = vec![1, 2, 3,4];
    vec2.push(5);
    println!("2nd: {}", vec2[1]);
    let second: &i32 = &vec2[1];

    match vec2.get(1){
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd value"),
    }

    for i in &mut vec2{
        *i *= 2;
        println!("Double value: {}", i);
    }

    /*for i in &vec2{
        println!("{}",i);
    }*/

    println!("Length of Vec: {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());


    

}