extern crate rand;
use std::io;
fn main() {

    let abe = String::from("abcdefghijklmnopqrstuvwxyz");
    let abe = abe.as_bytes();

    println!("ingrese la ecuacion");
    let mut ecuacion = String::new();
    io::stdin().read_line(&mut ecuacion).expect("Failed to read line");
    let ecuacion = ecuacion.trim();
    let find = find_bytes(ecuacion, abe);
    let var_loc = find_x(ecuacion, find);
    println!("{}", &ecuacion[var_loc..var_loc+1]);

    let mut numero = String::new();
    io::stdin().read_line(&mut numero).expect("failed to read line");
    let numero = numero.trim();
    let indeces = to_index(numero);
    println!("{}", indeces);


} //modificacion
fn find_bytes (ecuacion: &str, abe: &[u8]) -> [u8; 8] {
    let bytes = ecuacion.as_bytes();
    let mut enc: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut it = 0;
    for (_i, &item) in bytes.iter().enumerate() {
        for (_h, &letra) in abe.iter().enumerate() {
            if item == letra {
                enc[it] = item;
                it += 1;
            }
        }
    }
    return enc
}
fn find_x (termino: &str, variable: [u8; 8]) -> usize {
    let bytes = termino.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        for (_h, &var) in variable.iter().enumerate() {
            if item == var {
                return i
            }
        }
    }
    return termino.len()+1
}
fn to_index (numero: &str) -> String {
    let mut fin = String::new();
    let bytes = numero.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'1' {
            fin.push_str("¹");
        }
        if item == b'2' {
            fin.push_str("²");
        }
        if item == b'3' {
            fin.push_str("³");
        }
        if item == b'4' {
            fin.push_str("⁴");
        }
        if item == b'5' {
            fin.push_str("⁵");
        }
        if item == b'6' {
            fin.push_str("⁶");
        }
        if item == b'7' {
            fin.push_str("⁷");
        }
        if item == b'8' {
            fin.push_str("⁸");
        }
        if item == b'9' {
            fin.push_str("⁹");
        }
        if item == b'0' {
            fin.push_str("⁰");
        }
    }
    return fin
}