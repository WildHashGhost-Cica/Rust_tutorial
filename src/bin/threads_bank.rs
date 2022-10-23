use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::thread;
use std::time::Duration;

fn main(){
    pub struct Bank {
        balance: f32
    }
    //we will get an error what will fix with reference smart pointer
    fn withdraw(the_bank: &mut Bank, amt:f32){
        the_bank.balance -= amt;
    }
    let mut bank = Bank{balance: 100.0};
    withdraw(&mut bank, 5.00);
    println!("Balance: {}", bank.balance);

    fn customer(the_bank: &mut Bank){
        withdraw(the_bank, 5.00);
    }
    thread::spawn(||{
        customer(&mut bank)
    }).join().unwrap();
}