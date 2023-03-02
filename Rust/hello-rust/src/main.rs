use ferris_says::say; //Imporing the external crates 
use std::io::{stdout, BufWriter}; //Importing the standard crates

fn main(){
 let stdout = stdout(); 
 let message = String::from("Welcome to rust");
 let width = message.chars().count(); //Getting the width of the message.
 let mut writer =  BufWriter::new(stdout.lock());
 say(message.as_bytes(), width, &mut writer).unwrap();
}
