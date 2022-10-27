use std::usize;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(1..5);
    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 11 {
            break counter * 2;
        }
    };
    println!("Result = {}", result);
    while_func(x);
    for_func(x);
    iter_func();
}

fn while_func(x: i32) {
    let mut number = x;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("START!");

}

fn for_func(x: i32) {
    let a = [22, 222, 2222, 22222];
    let mut index = x as usize;

    while index != 0 {
        println!("Value in for_func = {}", a[index]);
        index -= 1;
    }
}

fn iter_func() {
    let a = [2, 22, 222, 2222, 22222];

    for element in a.iter() {
        println!("Value IterFunc = {}", element);
    }

}
















