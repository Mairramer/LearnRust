use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let numero_secreto = rand::thread_rng().gen_range(1, 30);

    loop {
        println!("Digite um numero:");
        let mut advinhe = String::new();

        io::stdin()
            .read_line(&mut advinhe)
            .expect("Falha ao ler entrada");

        let advinhe: u32 = match advinhe.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Voce digitou: {}", advinhe);

        match advinhe.cmp(&numero_secreto) {
            Ordering::Less => println!("Numero muito pequeno!"),
            Ordering::Greater => println!("Numero muito grande!"),
            Ordering::Equal => {
                println!("Acertou!");
                break;
            }
        }
    }
}
