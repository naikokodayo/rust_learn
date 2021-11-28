// 猜数游戏  
use std::io; // prelude
use std::cmp::Ordering; 
use rand::Rng; // trait

fn main() {
    println!("猜数!!");

    let secret_number = rand::thread_rng().gen_range(1..101); // i32 u32 i64

    // println!("神秘数字是: {}", secret_number);

    loop {
        println!("猜猜看");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("无法读取行");
    
        // shadow 
         // \n trim 去掉输入时按下回车键 空格
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("你猜测的数字是:{}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("猜对了！");
                break;
            },
        }
    }
}
