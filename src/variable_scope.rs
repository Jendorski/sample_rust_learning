fn var_scope() {//s is not valid here
  let s = "hello";//s is valid from this point forward
  //do stuff with s
}//this scope is now over, s is no longer valid.

fn mutated_string(){
  let mut s = String::from("hello");
  s.push_str(", world!");
  println!("{s}");
}

mutated_string();