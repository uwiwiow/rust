use std::io;
fn main() {
    println!("INGRESE LA FORMA DE LA ECUACION");
    let mut ecuacionCompleta = String::new();
    io::stdin().read_line(&mut ecuacionCompleta).expect("FAILED TO READ LINE");
    let ecuacion = find_eq(&ecuacionCompleta);
    println!("{}", ecuacion.1);
}
fn find_eq (ecuacion: &str) -> (bool, &str) {
    let byte = ecuacion.as_bytes();
    for (i, &item) in byte.iter().enumerate() {
        if item == b'=' {
            return (true, &ecuacion[i+1..].trim())
        }
    }
    return (false, &ecuacion[..])
}
fn find_term (ecuacion: &str) -> [usize; 6] {
    let byte = ecuacion.as_bytes();
    let mut espacios: [usize; 6] = [0,0,0,0,0,0];
    let mut iter = 0;
    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            espacios[iter] = i;
        }
        iter = iter + 1;
    }
    return espacios
}
fn find_a (ecuacion: &str) -> (bool, usize) {
    let byte = ecuacion.as_bytes();
    for (i, &item) in byte.iter().enumerate() {
        if item == b'a' {
            return (true, i)
        }
    }
    return (false, 0)
}