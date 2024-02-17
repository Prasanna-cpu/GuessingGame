use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main(){

    println!("Guess the number");
    loop{


        println!("Input the number you think");

        let secret_no=rand::thread_rng().gen_range(1..=101);

        let mut guess:String=String::new();  //Creates a new empty string (mutable)

        io::stdin().read_line(&mut guess).expect("Failed to read line");


        let guess:u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };


        println!("You guessed : {}",guess);
        println!("Secret Number :{}",secret_no);

        match guess.cmp(&secret_no){
            Ordering::Equal=>{
                println!("{}","You win".green());
                break;
            },
            Ordering::Greater=>println!("{}","Too big".red()),
            Ordering::Less=>println!("{}","Too small".yellow())
        }




    }
        

    


}

            