use std::io;
fn main() {

    let abe = String::from("abcdfghijkmopqrstuvwxyz");
    let abe = abe.as_bytes();

    println!("De la funcion que desea derivar");
    let mut funcion = String::new();
    io::stdin().read_line(&mut funcion).expect("Failed to read line");
    let funcion = funcion.trim();
    let ecuacion = find_eq(funcion);

    let mut termino = find_termino(ecuacion);

    let mut find = find_var(termino.0, abe);

    let variable_princ = find_x(termino.0, find.0);
    let mut derivada = String::from("f'(");
    if variable_princ.0 {
        derivada.push_str(&termino.0[variable_princ.1..variable_princ.1+1]);
    } else {
        derivada.push_str("x");
    }
    derivada.push_str(") = ");

    let mut index = termino.1;

    if termino.1 == ecuacion.len() {
        let fun_x = find_x(termino.0, find.0);
        let (is_x, in_x) = fun_x;

        let fun_pw = find_pw(termino.0);
        let (is_pw, in_pw, f_pw, s_pw) = fun_pw;

        let fun_div = find_div(termino.0);
        let (is_div, in_div, _f_div, s_div) = fun_div;

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
        if is_x && in_x != 0 && is_pw && in_x == f_pw.len()-1 {
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
        //saber si es x^-n
        if is_x && in_x == 0 && is_pw && &s_pw[..1] == "-" {
            println!("xd");
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
        }
        //saber si es kx^-n
        if is_x && in_x != 0 && is_pw && &s_pw[..1] == "-" && !is_div && in_x == f_pw.len()-1 {
            derivada.push_str("-");
            derivada.push_str(&f_pw[..in_x]);
            derivada.push_str("/(");
            derivada.push_str(&s_pw[1..]);
            derivada.push_str(&termino.0[in_x..in_x+1]);
            let ene = &s_pw[1..];
            let ene: u32 = ene.trim().parse().expect("type a number");
            let ene = ene - 1;
            let ene = ene.to_string();
            if ene != "1" {
                derivada.push_str("^");
                derivada.push_str(&ene);
            }
            derivada.push_str(")");
        }
        // saber si es x^a/b
        if is_x && is_pw && is_div && f_pw.len() == 1 && f_pw == "x" && &s_pw[..1] != "-" {
            let a = &termino.0[in_pw+1..in_div].trim();
            let a: f32 = a.trim().parse().expect("type a number");
            let b = &s_div[..].trim();
            let b: f32 = b.trim().parse().expect("type a number");
            let pw_up = a - b;

            let b_in = to_index(b.to_string());
            let a_in = to_index(pw_up.to_string());

            //saber si es x^a/b y a/b == 1/1
            if a == b {
                derivada.push_str("1");
            }
            if a == 0.0 {
                derivada.push_str("1");
            }
            if b == 0.0 {
                derivada = String::from("Error, no se puede dividir entre 0.");
            }
            //saber si es x^a/b y saber si en y' a/b es menor que 0. == y'=-a/b(^b√x^a)
            if pw_up / b < 0.0 {
                derivada.push_str("[-");
                derivada.push_str(&a.to_string());
                derivada.push_str("/");
                derivada.push_str(&b.to_string());
                derivada.push_str("(");
                derivada.push_str(&b_in);
                derivada.push_str("√");
                derivada.push_str(f_pw);
                derivada.push_str(&a_in);
                derivada.push_str(")]");
            }
            //saber si es x^a/b y saber si en y' a/b, a es mayor que b == ax^((a/b)-1)/b
            derivada.push_str("(");
            derivada.push_str(&a.to_string());
            derivada.push_str(f_pw);
            derivada.push_str(&a_in);
            derivada.push_str("ᐟ");
            derivada.push_str(&b_in);
            derivada.push_str("/");
            derivada.push_str(&b.to_string());
            derivada.push_str(")");
        }

        // saber si es cx^a/b
        if is_x && is_pw && is_div && f_pw.len() > 1 && f_pw != "x" && &s_pw[..1] != "-" {
            let a = &termino.0[in_pw+1..in_div].trim();
            let a: f32 = a.trim().parse().expect("type a number");
            let b = &s_div[..].trim();
            let b: f32 = b.trim().parse().expect("type a number");
            let pw_up = a - b;

            let b_in = to_index(b.to_string());
            let a_in = to_index(pw_up.to_string());

            //saber si es x^a/b y a/b == 1/1
            if a == b {
                derivada.push_str("1");
            }
            if a == 0.0 {
                derivada.push_str("1");
            }
            if b == 0.0 {
                derivada = String::from("Error, no se puede dividir entre 0.");
            }
            //saber si es x^a/b y saber si en y' a/b es menor que 0. == y'=-a/b(^b√x^a)
            if pw_up / b < 0.0 {
                derivada.push_str("[-");
                derivada.push_str(&a.to_string());
                derivada.push_str("/");
                derivada.push_str(&b.to_string());
                derivada.push_str("(");
                derivada.push_str(&b_in);
                derivada.push_str("√");
                derivada.push_str(f_pw);
                derivada.push_str(&a_in);
                derivada.push_str(")]");
            }
            //saber si es x^a/b y saber si en y' a/b, a es mayor que b == ax^((a/b)-1)/b
            derivada.push_str("(");
            derivada.push_str(&a.to_string());
            derivada.push_str(f_pw);
            derivada.push_str(&a_in);
            derivada.push_str("ᐟ");
            derivada.push_str(&b_in);
            derivada.push_str("/");
            derivada.push_str(&b.to_string());
            derivada.push_str(")");
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
            termino = find_termino(&ecuacion[index..].trim());
            if boolen {
                index = index + termino.0.len();
            }
            find = find_var(termino.0, abe);
        }

        let fun_x = find_x(termino.0, find.0);
        let (is_x, in_x) = fun_x;

        let fun_pw = find_pw(termino.0);
        let (is_pw, in_pw, f_pw, s_pw) = fun_pw;

        let fun_div = find_div(termino.0);
        let (is_div, in_div, _f_div, s_div) = fun_div;

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
        if termino.0 == "/" {
            derivada.push_str("/");
            continue;
        }
        if termino.0 == "(" {
            derivada.push_str("(");
            continue;
        }
        if termino.0 == ")" {
            derivada.push_str(")");
            continue;
        }
        //saber si es k
        if !is_x && termino.0 != " " && !is_pw && !is_div {
            derivada.push_str("0");
            continue;
        }
        //saber si es x
        if is_x && in_x == 0 && termino.0.len() == 1 && !is_pw && !is_div {
            derivada.push_str("1");
            continue;
        }
        //saber si es kx
        if in_x == termino.0.len()-1 && !is_pw && !is_div {
            derivada.push_str(&termino.0[..in_x]);
            continue;
        }
        //saber si es x^n
        if is_x && in_x == 0 && is_pw && &s_pw[..1] != "-" && !is_div {
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
        if is_x && in_x != 0 && is_pw && &s_pw[..1] != "-" && !is_div && in_x == f_pw.len()-1 {
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
        if is_x && in_x == 0 && is_pw && &s_pw[..1] == "-" && !is_div {
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
        //saber si es kx^-n
        if is_x && in_x != 0 && is_pw && &s_pw[..1] == "-" && !is_div && in_x == f_pw.len()-1 {
            derivada.push_str("-");
            derivada.push_str(&f_pw[..in_x]);
            derivada.push_str("/(");
            derivada.push_str(&s_pw[1..]);
            derivada.push_str(&termino.0[in_x..in_x+1]);
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
        // saber si es x^a/b
        if is_x && is_pw && is_div && f_pw.len() == 1 && f_pw == "x" && &s_pw[..1] != "-" {
            let a = &termino.0[in_pw+1..in_div].trim();
            let a: f32 = a.trim().parse().expect("type a number");
            let b = &s_div[..].trim();
            let b: f32 = b.trim().parse().expect("type a number");
            let pw_up = a - b;

            let b_in = to_index(b.to_string());
            let a_in = to_index(pw_up.to_string());

            //saber si es x^a/b y a/b == 1/1
            if a == b {
                derivada.push_str("1");
                continue;
            }
            if a == 0.0 {
                derivada.push_str("1");
                continue;
            }
            if b == 0.0 {
                derivada = String::from("Error, no se puede dividir entre 0.");
                break;
            }
            //saber si es x^a/b y saber si en y' a/b es menor que 0. == y'=-a/b(^b√x^a)
            if pw_up / b < 0.0 {
                derivada.push_str("[-");
                derivada.push_str(&a.to_string());
                derivada.push_str("/");
                derivada.push_str(&b.to_string());
                derivada.push_str("(");
                derivada.push_str(&b_in);
                derivada.push_str("√");
                derivada.push_str(f_pw);
                derivada.push_str(&a_in);
                derivada.push_str(")]");
                continue;
            }
            //saber si es x^a/b y saber si en y' a/b, a es mayor que b == ax^((a/b)-1)/b
            derivada.push_str("(");
            derivada.push_str(&a.to_string());
            derivada.push_str(f_pw);
            derivada.push_str(&a_in);
            derivada.push_str("ᐟ");
            derivada.push_str(&b_in);
            derivada.push_str("/");
            derivada.push_str(&b.to_string());
            derivada.push_str(")");
            continue;
        }

        // saber si es cx^a/b
        if is_x && is_pw && is_div && f_pw.len() > 1 && f_pw != "x" && &s_pw[..1] != "-" {
            let a = &termino.0[in_pw+1..in_div].trim();
            let a: f32 = a.trim().parse().expect("type a number");
            let b = &s_div[..].trim();
            let b: f32 = b.trim().parse().expect("type a number");
            let pw_up = a - b;

            let b_in = to_index(b.to_string());
            let a_in = to_index(pw_up.to_string());

            //saber si es x^a/b y a/b == 1/1
            if a == b {
                derivada.push_str("1");
                continue;
            }
            if a == 0.0 {
                derivada.push_str("1");
                continue;
            }
            if b == 0.0 {
                derivada = String::from("Error, no se puede dividir entre 0.");
                break;
            }
            //saber si es x^a/b y saber si en y' a/b es menor que 0. == y'=-a/b(^b√x^a)
            if pw_up / b < 0.0 {
                derivada.push_str("[-");
                derivada.push_str(&a.to_string());
                derivada.push_str("/");
                derivada.push_str(&b.to_string());
                derivada.push_str("(");
                derivada.push_str(&b_in);
                derivada.push_str("√");
                derivada.push_str(f_pw);
                derivada.push_str(&a_in);
                derivada.push_str(")]");
                continue;
            }
            //saber si es x^a/b y saber si en y' a/b, a es mayor que b == ax^((a/b)-1)/b
            derivada.push_str("(");
            derivada.push_str(&a.to_string());
            derivada.push_str(f_pw);
            derivada.push_str(&a_in);
            derivada.push_str("ᐟ");
            derivada.push_str(&b_in);
            derivada.push_str("/");
            derivada.push_str(&b.to_string());
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
fn find_x (termino: &str, variable: [u8; 8]) -> (bool, usize) {
    let bytes = termino.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        for (_h, &letra) in variable.iter().enumerate() {
            if item == letra {
                return (true, i)
            }
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
fn find_div (termino: &str) -> (bool, usize, &str, &str){
    let bytes = termino.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'/' {
            return (true, i, &termino[..i], &termino[i+1..])
        }
    }
    return (false, bytes.len(), &termino[..], &termino[..])
}
fn find_var (ecuacion: &str, abe: &[u8]) -> ([u8; 8], bool) {
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
    let mut is_var = false;
    if it > 0 {
        is_var = true;
    }

    return (enc, is_var)
}
fn to_index (numero: String) -> String {
    let mut fin = String::new();
    let bytes = numero.as_bytes();
    for (_i, &item) in bytes.iter().enumerate() {
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
        if item == b'-' {
            fin.push_str("⁻");
        }
    }
    return fin
}