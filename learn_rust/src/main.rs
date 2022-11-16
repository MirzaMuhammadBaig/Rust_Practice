use rand ::{thread_rng,Rng};
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("Secret number is :{}",secret_number);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        println!("Your guessed:{}", guess);
        let guess:u32 = guess.trim().parse().expect("Type integer");
        println!("{}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You are putting less value than value"),
            Ordering::Greater => println!("You are putting greater value than value"),
            Ordering::Equal => { println!("You have successfuly wrote exect value"); break;},
    }
}
}
