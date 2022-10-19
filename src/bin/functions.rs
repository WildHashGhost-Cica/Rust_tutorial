use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn say_hello(){
    println!("Hello WildhashGhost!");
}

fn get_sum(x: i32, y:i32){
    println!("{} + {} = {}", x, y, x+y );
}

fn get_sum2(x:i32, y:i32) ->i32 {
    x+y
}

fn get_sum3(x:i32, y:i32) -> i32 {
    return x + y;
}
fn get_sum4(x:i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn get_2(x:i32) -> (i32, i32){
    return (x+1, x+2);
}

fn sum_list(list: &[i32]) -> i32 {
     let mut sum: i32 = 0;
     for &val in list.iter(){
        sum += &val;
     }
     sum
}

fn main(){
    say_hello();
    get_sum(5, 10);

    println!("{:?}", get_sum2(4,8));
    println!("{:?}", get_sum3(2, 5));
    println!("{:?}", get_sum4(21));

    let (val_1, val_2) = get_2(13);
    println!("Nums: {} {}",val_1, val_2 );

    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(&num_list));
}