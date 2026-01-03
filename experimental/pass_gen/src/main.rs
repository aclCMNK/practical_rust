/*
* TITLE: En este ejercicio crearemos un generador de constraseñas
*/


use std::any::type_name;

use rand::Rng;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let mut offset: i8;
    let mut head: i8 = 0;
    let mut pass: String = String::new();
    loop {
        println!("Type the number of repetitions between 1 and 5:");
        let mut offset_str: String = String::new();
        // capturamos datos desde pantalla y el valor se lo asignamos a offset_str
        std::io::stdin()
            .read_line(&mut offset_str)
            .expect("Failed to read line");
        // --------------------------
        // CAST a string to a number
        // Convertimos de cadena de texto a numero
        // --------------------------
        offset = offset_str.trim().parse().expect("Failed to parse()");
        // --------------------------
        println!("Repetitions: {}", offset);
        if offset > 5 {
            println!("Too many repetitions");
        } else if offset < 1 {
            println!("Too few repetitions");
        } else {
            break;
        }
    }
    // iniciamos un ciclo para ir generando los caracteres de la clave
    loop {
        // generamos un numero aleatorio
        let rnd: i128 = rand::rng().random_range(0..100000000000);
        // lo convertimos de numero a hexa
        let hex: String = format!("{:x}", rnd);
        // lo insertamos a la cadena final
        pass.push_str(&hex);
        head += 1;
        if head >= offset {
            break;
        }
    }
    println!("{}", pass);
}
