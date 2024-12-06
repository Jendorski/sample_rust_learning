use std::io;

fn main() {
    println!("Hello, world!");
    let  x= 5;
    println!("The x is: {x}");
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The values of x is : {x}");

    let c = 'z';
    let z: char = 'Z';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x,y,z) = tup;
    println!("The value of y is {y}");

    let months = ["January", "February", "March"];

    let a: [i32; 5] = [1,2,3,4,5];

    let b =  [3; 5];
    let b = [3,3,3,3,3];

    let a = [1,2,3,4,5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

    let index: usize = index.
    trim()
    .parse()
    .expect("Index is not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    another_function();

}

fn another_function () {
    println!("Another function");
}

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;