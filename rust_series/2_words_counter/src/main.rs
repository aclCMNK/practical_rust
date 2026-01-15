//Contador de Palabras: 
//Lee un texto y cuenta la frecuencia de cada palabra usando HashMap. Practica iteradores y ownership de Strings.
//Casos de uso: Parrafo de lorem ipsum o cualquier texto.

use std::collections::HashMap;

fn count_words(paragraph: &mut String) -> HashMap<String, i32> {
    let mut dicc: HashMap<String, i32> = HashMap::new();
    for word in paragraph.split_whitespace() {
        if !dicc.contains_key(word) {
            dicc.insert(word.to_string(), 0);
        }
        dicc.insert(word.to_string(), dicc[word] + 1);
    }
    return dicc;
}

fn main() {
    println!("Escribe un parrafo para contar las palabras:");
    let mut paragraph: String = String::new();
    std::io::stdin().read_line(&mut paragraph).unwrap();
    let words: Vec<&str> = paragraph.split_whitespace().collect();
    println!("El parrafo tiene {} palabras", words.len());
    if words.len() <= 0 {
        println!("El parrafo debe tener al menos una palabra");
        return;
    }
    let dicc: HashMap<String, i32> = count_words(&mut paragraph);
    for (word, count) in dicc {
        println!("{}: {}", word, count);
    }
}
