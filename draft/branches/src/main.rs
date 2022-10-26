use rand::Rng;

fn main() {

    let mut rng = rand::thread_rng();

    let x = rng.gen_range(0..100);
    let y = rng.gen_range(0..100);
    //let num2= rand::thread_rng();

    if x < y {
        println!("*** {} < {} ***", x, y);
        println!("*** TRUE ***");
    } else {
        println!("*** {} > {} ***", x, y);
        println!("*** FALSE ***");
    }

    second_funct(x);
    if_in_let(x);
}

fn second_funct(num: i32) {
    
    if num % 4 == 0 {
        println!("Num is devisible by 4 ");
    } else if num % 3 == 0 {
        println!("Num is devisible by 3 ");
    } else if num % 2 == 0 {
        println!("Num is devisible by 2 ");
    } else {
        println!("Num is not devisible by 4, 3, 2.")
    }
}


fn if_in_let(condition: i32) {
    //let condition = true;
    let number = if condition != 22 {
        22
    } else {
        13
    };

    println!("Значение числа равно {}", number);
}    
