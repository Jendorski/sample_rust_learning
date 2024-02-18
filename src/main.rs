mod variable_scope;

fn main() {
    println!("Hello, world!");

    let mut number = 3;
    if number < 5 {
        println!("Condition is true")
    }
    else {
        println!("Condition is false")
    }

    number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    }else if number % 3 == 0{
        println!("number is divisible by 3")
    }else if number % 2 == 0 {
        println!("number is divisible by 2")
    }else {
        println!("number is not divisible by 4, 3 or 2")
    }

    if number != 0{
        print!("")
    }
}
