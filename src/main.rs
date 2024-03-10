use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!(" jogo de Adivinhação!");
    println!("--adivinhe o número secreto--");
    println!("==============================");
    println!("Regras: ");
    println!("1. O número secreto é um número entre 1 e 100.");
    println!("2. Você tem que adivinhar o número secreto.");
    println!("3. Seu palpite será comparado com o número secreto.");
    println!("4. Seu palpite será classificado como 'menor', 'maior' ou 'igual' ao número secreto.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

loop {
    println!("\nDigite seu palpite:");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler a linha.");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Muito pequeno!"),
        Ordering::Greater => println!("Muito grande"),
        Ordering::Equal => {
            println!("Você acertou!");
            break;
        }
    }
}


}
