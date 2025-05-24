use std::io;
// use std::vec;

// Estaria guapo que para el caso 1, 2 y 3 se pudiese hacer con mas de un numero
// https://doc.rust-lang.org/book/foreword.html -> importante
fn print_menu(){
        println!("--- Select one option ---");
        println!("--- 1. Add two numbers ---");
        println!("--- 2. Subtract two numbers ---");
        println!("--- 3. Multiply two numbers ---");
        println!("--- 4. Divide two numbers ---");
        println!("--- 5. Exit ---");
        print!("\n\n");
}

fn read_number() -> i32{
    // se asume que la entrada siempre es int
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Error leyendo linea");
    
    return num
        .trim()
        .parse()
        .expect("EL numero no es un entero");
}


fn calc(){
    println!("Inserta un numero: ");
    // Entrada hasta que el usuario introduzca un caracter cualquiera
    let mut v1: Vec<i32> = Vec::new();
    let mut aux = String::new();
    loop{
        io::stdin
            .read_line(&mut aux)
            .expect("Error al leer la linea");

        let aux = aux.trim() // Trimeamos espacios y \n

        let inp = aux.parse::<i32>();

        // control de errores recuperables con Result<> -> locura

        if let Ok(numero) = inp {
            // Ok -> true: es un i32
            v1.push(numero);
        }else if inp.len() == 1{
            let c = inp.chars().next().unwrap();
            // caracter
        }else{
            //caso string
        }
    }
    // Siempre deben ser int
    let n1_parsed: i32 = read_number();
    let n2_parsed: i32 = read_number();
    // loop -> bucle infinito por defecto
    loop{
        print_menu();
        let mut inp = String::new();
        let mut inp_parsed: i8 = -1;

        while inp_parsed < 0 || inp_parsed > 5{

            io::stdin()
                .read_line(&mut inp)
                .expect("el input no es un string");
            inp_parsed = inp
                .trim()
                .parse()
                .expect("El input no es un numero entero");
        }

        match inp_parsed{
            1 => println!("{}", n1_parsed + n2_parsed),
            2 => println!("{}", n1_parsed - n2_parsed),
            3 => println!("{}", n1_parsed * n2_parsed),
            4 => println!("{}", n1_parsed / n2_parsed),
            5 => break,
            _ => continue,
        }
    }
}

fn main() {
    calc();
}
