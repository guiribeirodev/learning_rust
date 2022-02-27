use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número meu príncipe!!!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Chuta um número amigão:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Seu palpite foi: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Este não é um número valido");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno meu amigo!"),
            Ordering::Greater => println!("Opa, passou, muito grande!"),
            Ordering::Equal => {
                println!("Parabéns, você é bom ein, acertou!");
                break;
            }
        }
    }
}
