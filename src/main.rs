use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
    println!("Advinhe o número!");

    let mut rng = thread_rng();
    let numero_secreto = rng.gen_range(0..101);

    let mut tentativas = 0;

    print!("{}[2J", 27 as char);

    loop {
        println!("Digite o seu palpite.");

        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_err) => {
                println!("'{}' é invalido!", palpite.trim());
                continue;
            }
        };

        println!("Você disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => {
                println!("Muito baixo!");
                tentativas += 1;
            },
            Ordering::Greater => {
                println!("Muito alto!");
                tentativas += 1;
            },
            Ordering::Equal => {
                println!("O número secreto era: {}", numero_secreto);
                println!("Você acertou!");
                println!("Você usou {} tentativas", tentativas);
                break;
            },
        }
    }
}
