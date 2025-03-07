use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivina el número!");

    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    println!("El número secreto es: {}", numero_secreto);

    loop {
        println!("Por favor, introduce tu adivinanza.");

        let mut adivinanza = String::new();

        io::stdin()
            .read_line(&mut adivinanza)
            .expect("Fallo al leer la línea");

        let adivinanza: u32 = match adivinanza.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, introduce un número!");
                continue;
            }
        };

        println!("Tu adivinanza es: {}", adivinanza);

        match adivinanza.cmp(&numero_secreto) {
            Ordering::Less => println!("Demasiado bajo!"),
            Ordering::Greater => println!("Demasiado alto!"),
            Ordering::Equal => {
                println!("Has adivinado!");
                break;
            }
        }
    }
}