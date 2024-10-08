use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Угадайте число!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Секретное число равно {}", secret_number);
    loop {
        println!("Пожалуйста, введите свою догадку.");

        let mut guess = String::new();

        let msg = "He получилось прочитать строку";
        io::stdin().read_line(&mut guess).expect(msg);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Вы загадали: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком малое число!"),
            Ordering::Greater => println!("Слишком большое число!"),
            Ordering::Equal => {
                println!("Вы угадали!");
                break;
            }
        }
    }
}
