use std::io;

fn main() {
    let mut input = String::new();
    let mut number : i64 = 1;

    while input.is_empty(){
        println!("Please enter a whole number between 1 and 20.");
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        number = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a whole number!", input.trim());
                input.clear();
                1
            },
        };

        if number > 20 {
            println!("Number entered must be 20 or less!");
            number = 0;
            input.clear();
        }else if number < 1 {
            println!("Number entered must be 1 or greater!");
            number = 0;
            input.clear();
        }
    }

    println!("Factorial of {} is {}.", input.trim(), factorial(number));   
}

fn factorial(num: i64) -> u64{
    let mut answer : u64 = 1;
    let mut num : u64 = num as u64;

    while num > 1 {
        answer *= num;
        num -= 1;
    }

    answer
}
