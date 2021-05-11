use std::io;

fn main() {
    println!("Digite a temperatura:");
    let mut temperatura = String::new();

    io::stdin()
        .read_line(&mut temperatura)
        .expect("Falha ao ler numero!.");

    let temperatura: f64 = temperatura
        .trim()
        .parse()
        .expect("Erro na entrada de dados");

    println!("Digite c para converter em Celsius ou f para converter em Farenheit");

    let mut resultado = String::new();

    io::stdin()
        .read_line(&mut resultado)
        .expect("Falha na leitura de dados");

    let resultado = resultado.trim();

    let mut result = 0.0;

    if resultado == "c" {
        result = farenheit_para_celsius(temperatura);
    } else if resultado == "f" {
        result = celsius_para_farenheit(temperatura);
    } else {
        println!("Opção Invalida");
    }

    if resultado == "c" {
        println!("Temperatura em Farenheit para Celsius.");
        println!("Resultado {}°{}", result, resultado);
    } else if resultado == "f" {
        println!("Temperatura em Celsius  para Farenheit .");
        println!("Resultado {}°{}", result, resultado);
    }
}

fn celsius_para_farenheit(temp: f64) -> f64 {
    (temp * (9.0 / 5.0)) + 32.0
}

fn farenheit_para_celsius(temp: f64) -> f64 {
    (temp - 32.0) * (5.0 / 9.0)
}
