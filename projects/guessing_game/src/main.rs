extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数字を当ててみて！");
    let secret_number = rand::thread_rng().gen_range(1,101);
    // println!("秘密の数字は: {}", secret_number);

    loop {
        println!("予想値を入力してね");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("入力の読み込みに失敗しました");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("予想値は: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("小さすぎ！"),
            Ordering::Greater => println!("大きすぎ！"),
            Ordering::Equal   => {
                println!("正解！！");
                break;
            }
        }
    }
}
