use std::io;
fn main() {

    let abe = String::from("abcdefghijklmnopqrstuvwxyz");
    let abe = abe.as_bytes();

    println!("De la funcion que desea derivar");
    let mut funcion = String::new();
    io::stdin().read_line(&mut funcion).expect("Failed to read line");
    let funcion = funcion.trim();
    let ecuacion = find_eq(funcion);

    let incognita = find_var(ecuacion, abe);

    let mut termino = find_termino(ecuacion);

    let mut index = termino.1;

    let mut derivada = String::from("f'(x) = ");
    if termino.1 == ecuacion.len() {
        let fun_x = find_x(termino.0);
        let (is_x, in_x) = fun_x;

        let fun_pw = find_pw(termino.0);
        let (is_pw, _in_pw, f_pw, s_pw) = fun_pw;

        if termino.0 == " " {
            derivada.push_str(" ");
        }

        //saber si es +
        if termino.0 == "+" {
            derivada.push_str("+");
        }
        //saber si es -
        if termino.0 == "-" {
            derivada.push_str("-");
        }
        //saber si es k
        if !is_x && termino.0 != " " {
            derivada.push_str("0");
        }
        //saber si es x
        if is_x && in_x == 0 && termino.0.len() == 1 {
            derivada.push_str("1");
        }
        //saber si es kx
        if in_x == termino.0.len()-1 && !is_pw {
            derivada.push_str(&termino.0[..in_x]);
        }
        //saber si es x^n
        if is_x && in_x == 0 && is_pw && &s_pw[..1] != "-" {
            derivada.push_str(s_pw);
            derivada.push_str(f_pw);
            let ene = s_pw;
            let ene: u32 = ene.trim().parse().expect("please type a number");
            let ene = ene - 1;
            let ene = ene.to_string();
            if ene != "1" {
                derivada.push_str("^");
                derivada.push_str(&ene);
            }
        }
        //saber si es kx^n
        if is_x && in_x != 0 && is_pw {
            let ka = &termino.0[..in_x];
            let ka: u32 = ka.trim().parse().expect("please type a number");
            let ene = s_pw;
            let ene: u32 = ene.trim().parse().expect("please type a number");
            let equis = ka * ene;
            let equis = equis.to_string();
            derivada.push_str(&equis);
            derivada.push_str(&f_pw[f_pw.len()-1..]);
            let ene = ene - 1;
            let ene = ene.to_string();
            if ene != "1" {
                derivada.push_str("^");
                derivada.push_str(&ene);
            }
        }

    }


    let mut iterador = 0;
    while termino.1 != ecuacion.len() {

        iterador += 1;
        if iterador > 1 {
            derivada.push_str(" ");

            if termino.2 == -1 {
                break;
            }

            index = index + 1;
            let mut boolen = false;
            if &ecuacion[index..].trim().len() != &ecuacion[index..].len() {
                boolen = true;
            }
            termino = find_termino(&ecuacion[index..].trim()); //if trim index + 1
            if boolen {
                index = index + termino.0.len();
            }
        }

        let fun_x = find_x(termino.0);
        let (is_x, in_x) = fun_x;

        let fun_pw = find_pw(termino.0);
        let (is_pw, _in_pw, f_pw, s_pw) = fun_pw;

        if termino.0 == " " {
            derivada.push_str(" ");
            continue;
        }

        //saber si es +
        if termino.0 == "+" {
            derivada.push_str("+");
            continue;
        }
        //saber si es -
        if termino.0 == "-" {
            derivada.push_str("-");
            continue;
        }
        //saber si es k
        if !is_x && termino.0 != " " && !is_pw {
            derivada.push_str("0");
            continue;
        }
        //saber si es x
        if is_x && in_x == 0 && termino.0.len() == 1 && !is_pw {
            derivada.push_str("1");
            continue;
        }
        //saber si es kx
        if in_x == termino.0.len()-1 && !is_pw {
            derivada.push_str(&termino.0[..in_x]);
            continue;
        }
        //saber si es x^n
        if is_x && in_x == 0 && is_pw && &s_pw[..1] != "-" {
            derivada.push_str(s_pw);
            derivada.push_str(f_pw);
            let ene = s_pw;
            let ene: u32 = ene.trim().parse().expect("please type a number");
            let ene = ene - 1;
            let ene = ene.to_string();
            if ene != "1" {
                derivada.push_str("^");
                derivada.push_str(&ene);
            }
            continue;
        }
        //saber si es kx^n
        if is_x && in_x != 0 && is_pw && &s_pw[..1] != "-" {
            let ka = &termino.0[..in_x];
            let ka: u32 = ka.trim().parse().expect("please type a number");
            let ene = s_pw;
            let ene: u32 = ene.trim().parse().expect("please type a number");
            let equis = ka * ene;
            let equis = equis.to_string();
            derivada.push_str(&equis);
            derivada.push_str(&f_pw[f_pw.len()-1..]);
            let ene = ene - 1;
            let ene = ene.to_string();
            if ene != "1" {
                derivada.push_str("^");
                derivada.push_str(&ene);
            }
            continue;
        }
        //saber si es x^-n
        if is_x && in_x == 0 && is_pw && &s_pw[..1] == "-" {
            derivada.push_str("-1/(");
            derivada.push_str(&s_pw[1..]);
            derivada.push_str(f_pw);
            let ene = &s_pw[1..];
            let ene: u32 = ene.trim().parse().expect("type a number");
            let ene = ene - 1;
            let ene = ene.to_string();
            if ene != "1" {
                derivada.push_str("^");
                derivada.push_str(&ene);
            }
            derivada.push_str(")");
            continue;
        }

    }
    println!("{}", derivada);
}
fn find_eq (funcion: &str) -> &str {
    let bytes = funcion.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'=' {
            return &funcion[i+1..].trim()
        }
    }
    return &funcion[..].trim()
}
fn find_termino (funcion: &str) -> (&str, usize, isize) {
    let bytes = funcion.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&funcion[..i].trim(), i, 0)
        }
    }
    return (&funcion[..].trim(), bytes.len(), -1)
}
fn find_x (termino: &str) -> (bool, usize) {
    let bytes = termino.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'x' {
            return (true, i)
        }
    }
    return (false, termino.len()+1)
}
fn find_pw (termino: &str) -> (bool, usize, &str, &str) {
    let bytes = termino.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'^' {
            return (true, i, &termino[..i], &termino[i+1..])
        }
    }
    return (false, bytes.len(), &termino[..], &termino[..])
}
fn find_var (ecuacion: &str, abe: &[u8]) -> ([usize; 8], usize) {
    let bytes = ecuacion.as_bytes();
    let mut enc: [usize; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut it = 0;
    for (i, &item) in bytes.iter().enumerate() {
        for (_h, &letra) in abe.iter().enumerate() {
            if item == letra {
                enc[it] = i;
                it += 1;
            }
        }
    }
    return (enc, it)
}