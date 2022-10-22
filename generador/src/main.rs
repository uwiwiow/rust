extern crate rand;
use std::io;
use rand::Rng;
fn main() {
    println!("cuantos digitos quiere en la contraseña?");
    let mut digitos = String::new();
    io::stdin().read_line(&mut digitos).expect("Failed to read line");
    let digitos: u32 = digitos.trim().parse().expect("not a number");

    println!("Cuantas contraseñas desea generar?");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    let num: u32 = num.trim().parse().expect("not a number");

    let mut array: [String; 256];

    for i in array.iter().enumerate() {
        let letra: char = i.0;

    }
}