fn main() {
    // comentario single line en Rust
    /* Comentario Multi-line en Rust.
        Igualitos que los de C/C++*/
    println!("--- Prints ---");
    println!("Hello, world!");
    println!("Las macros en Rust son raras!");
    println!("Como se printara un entero...?");
    print!("Tambien se puede usar print!()\n");
    print!("Lo cual no anyade \\n al final de cada linea\n\n");

    println!("--- Variables ---");
    // En Rust, los tipos de variables se definen segun el valor que les asignamos al inicializarlas. Como en Python :)
    // Hay alguna diferencia, como con los enteros y los floats. En este caso podemos declarar de que tamanyo los queremos

    let nombre: &str = "John Doe";
    let numero: i8 = 3; // i8, i16, i32, i64, i128
    let numero_f: f32 = 3.14; // f32, f64, f128
    let cond: bool = false;

    print!("Esto es un string {}\nEsto es un numero entero {}\nEsto es un numero de punto flotante {}\nTodas las variables se definen con 'let'\nEn un print, para formatear variables se hace con {{}}\n", nombre, numero, numero_f);

    /*
    Error: Las variables en Rust son inmutables por defecto a no ser que las definamos con el operador mut
    Ej:
        let mut test = 30;
        test = 40;
        Okay!
        
        let test = 30;
        test = 40;
        Error! 
    */

    // las constantes se declaran como las variables pero con const delante. La unica diferencia es que ES OBLIGATORIO que las constantes tengan un tipo delante
    const FILENAME: &str = "main.rs"; 
    // const FILENAME = "main.rs" -> ERROR!

    // Operadores aritmeticos clasicos: + - * / %
    let add = 1 + 1; // = 7
    let sub = 3 - 1;
    let mul = 2 * 3;
    let div = 25 / 5;
    let rem = 10 % 2;

    // Operadores de asignacion: =, +=, -=, *=, /=, %=
    // Operadores de comparacion: ==, !=, >, <, >=, <=
    // Operadores logicos: &&, ||, !

    // if-else
    let n1 = 5;
    let n2 = 8;
    let mut acc = 0;

    if n1>n2 {
        acc = n1 + n2;
    }else{
        acc = n2 - n1;
    }
    
    // En rust, los ifs se pueden usar como una expresion. Equivalente a lo anterior

    acc = if n1 > n2 {
        n1 + n2 // No se pone ;, da error
    }else{
        n2 - n1
    };

    acc = if n1 > n2 { n1 + n2 } else { n2 - n1 }; // Equivalente a las dos formas anteriores

    // match: switch-case de c/c++

    let day = 4;
    match day{
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wendnesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Dia invalido"), // default
    }

    // Para juntar mas de un valor en un solo caso

    match day{
        1 | 2 | 3 | 4 | 5  => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    // Con match se puede hacer lo mismo que con el if - else

    let res = match day{
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wendnesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Dia invalido", // default
    };
    println!("{}", res);

}
