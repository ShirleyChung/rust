extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let sn = 1;
    let en = 9999;
    println!("Welcome! Guess number");
    
    let myans = rand::thread_rng().gen_range(sn, en);
    
    loop { // 無窮迴圈
        println!("Come on! enter a number between({}~{})", sn, en);
        
        let mut guess = String::new(); // mutable string;(宣告)
        
        // 輸入字串
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");
        
        println!("You guess: {}", guess);

        // 轉換型別, 並用match處理Result回傳值
        let guess: u32 = match guess.trim().parse() 
        {
            Ok(num) => num,
            Err(_) => continue,
        };
//          .expect("Please enter a number..");
        
        match guess.cmp(&myans){
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal      => {
                println!("Yes you got it!");
                break;
            }
        }
    }
    
    println!("My secret number is {}", myans);

}
