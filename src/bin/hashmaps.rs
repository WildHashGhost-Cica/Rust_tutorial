use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

use std::collections::HashMap;

fn main(){
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for(k, v) in heroes.iter(){
        println!("{} = {}", k, v);

    }
    println!("Length: {}", heroes.len());

    if heroes.contains_key(&"Batman"){
        let the_batman =  heroes.get(&"Batman");
        match the_batman{
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }
}