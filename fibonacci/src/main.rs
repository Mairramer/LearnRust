use std::io;
fn main() {
    println!("Digite \"sair\" para finalizar");

    loop {
        let mut number = String::new();
        println!("Digite um numero:");
        io::stdin().read_line(&mut number).expect("Falha");

        if number.trim() == "sair" {
            break;
        }

        let number: i64 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("{}", fibonaci(number));
    }
}

fn fibonaci(number: i64) -> i64 {
    if number < 2 {
        return number;
    }
    return fibonaci(number - 1) + fibonaci(number - 2);
}
