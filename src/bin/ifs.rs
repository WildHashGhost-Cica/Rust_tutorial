use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    let age: i32 = 8;
    if (age >= 1) && (age <= 18 ){
        println!("Important Birthday");
    }else if ( age == 21) || (age == 50){
        println!("Important Birthday");
    }else if age >= 65 {
        println!("Important Birthday");
    }else{
        println!("Not an Important Birtday");
    }
}