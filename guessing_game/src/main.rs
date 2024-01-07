

use std::io; //standard io library 
use rand::Rng; //Rng trait defines the random number generator implements
use std::cmp::Ordering;
//main begins 
fn main(){
    println!("Geussing game start");
    
    let secretnumber = rand::thread_rng().gen_range(1..=100);// rand::thread_rng function that gives us the particular random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system.
    //println!("The secret number is {secretnumber}");
    loop {
            println!("Please input your guess. ");

        let mut guess = String::new();  //mutabale variable having str value 
        io::stdin()
            .read_line(&mut guess) //input to read_line function and safe reference (&)to mut guess 
            .expect("Failed to read line");  // readline return a value ok or err to handle err we write expect cand be handled in a better way
        
        //let guess:u32= guess.trim().parse().expect("Please type a number !");//SAhadowing a variable 
        
        let guess:u32= match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>continue,
        };

        println!("You have guessed :{guess}") ;//{} like f strings in py  for passing variable

        match guess.cmp(&secretnumber){
            Ordering::Less=>println!("too small"),
            Ordering::Greater=>println!("greater "),
            Ordering::Equal=>{
                println!("You win");
                break;
            }
        }
    }
    

}