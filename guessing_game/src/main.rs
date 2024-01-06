
use std::io; //standard io librabry 
fn main(){
    println!("Geussing game start");
    println!("Please input your guess. ");

    let mut guess = String::new();  //mutabale variable having str value 
    io::stdin()
        .read_line(&mut guess) //input to read_line function and safe reference (&)to mut guess 
        .expect("Failed to read line");  // readline return a value ok or err to handle err we write expect cand be handled in a better way
    println!("You have guessed :{guess}") //{} like f strings in py  for passing variable



}