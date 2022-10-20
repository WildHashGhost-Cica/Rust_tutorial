use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;



fn main(){
    #[derive(Debug)]
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob = Customer{
        name: String::from("Bob Smith"),
        address: String::from("125 New Street"),
        balance: 234.50,
    };
    //bob.address = String::from("505 New Street");
    println!("{} {} {}",bob.name, bob.address, bob.balance);
}