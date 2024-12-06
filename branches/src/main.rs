mod variable_scope;
mod structs;

fn main(){
    //let references_to_nothing = dangle();
    let mut s = String::from("hello world");
    let word = first_words(&s);//word will get the value 5

    s.clear(); // this empties the string, making it equal to ""

    //word still has the value 5 here, but,
    //there is no more string that we could meaningfully use the value 5 with, word is now totally invalid
    println!("the first word is: {word}");
}

fn string_slices () {
    let s = String::from("hello world");

    let len = s.len();

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2];
    let slice = &s[..2];
    let slice = &s[3..len];
    let slice = &s[3..];
    let slice = &s[0..len];
    let slice = &s[..];

    let my_string = String::from("hello world");

    //`first_word` works on slices of `String`s, whether partial or whole
    let word = first_words(&my_string[0..6]);
    let word = first_words((&my_string[..]));

    //`first_word` also works on references to `String`s which are equivalent to whole slices of `String`s
    let word = first_words((&my_string));
    let my_string_literal = "hello world";

    //`first_word` works on slices of string literals
    //whether partial or whole.
    let word = first_words(&my_string_literal[0..6]);
    let word = first_words((&my_string_literal[..]));

    //Because string literals *are* string slices already,
    //this works too, without the slice syntax!
    let word = first_words(my_string_literal);

}

fn other_slices (){
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);
}

fn first_words(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn fifth_main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}
fn fourth_main(){
    let s1 = String::from("hello");

    let ( len) = calculate_length(&s1);

    //println!("The length of '{s2}' is {len}")
}

fn calculate_length (s: &String) -> (usize) {
    let length = s.len();
     length
}

fn third_main(){
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back (a_string: String) -> String {
    a_string
}
fn second_main() {
    let s = String::from("hello world");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{some_string}")
}

fn makes_copy (some_integer: i32) {
    println!("{some_integer}")
}
fn first_main() {
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

    interact_on_move()
}

fn interact_on_move(){
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}")
}