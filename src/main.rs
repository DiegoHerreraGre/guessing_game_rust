use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main () {
    println! ("Adivinemos un número");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println! ("El número secreto es: {}", secret_number);
    loop {
        println! ("Por favor ingrese su número");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fallo al leer la línea");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println! ("Muy pequeño"),
            Ordering::Greater => println! ("Muy grande"),
            Ordering::Equal => {
                println! ("Acertaste");
                break;
            }
        }
    }
}