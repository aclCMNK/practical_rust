// Validador de Paréntesis:
// Verifica si los paréntesis/llaves están balanceados en una expresión usando un Vec como stack.
// Casos de uso:
// "{{{{{{{{{}}}}}}}}}}"  → true
// "(((((((((()))))))))"  → false (falta un cierre)
// "{[({[({[]})]})]}"    → true (anidamiento múltiple)
// "[[[[[[[[[[]]]]]]]]]" → false (falta un cierre)
// "fn process(data: Vec<String>) -> Result<HashMap<String, i32>, Error> { Ok(HashMap::new()) }"
// → true
// "if (x > 0) { while (y < 10) { arr[i] = map.get(&key).unwrap(); } }"
// → true
// "let result = vec![1, 2, 3].iter().map(|x| { x * 2 }).collect();"
// → true
// "fn broken() { if true { println!(\"test\"); }"
// → false (falta cerrar función)
//
// MAS CASOS:
/*
 * r#"{"users": [{"name": "Ana", "tags": ["rust", "go"]}, {"name": "Bob"}]}"#
→ true

r#"{"config": {"db": {"host": "localhost", "port": 5432}}"#
→ false (falta cerrar objetos)

r#"[[[{"deep": [1, [2, [3]]]}]]]"#
→ true

r#"println!("Texto con (paréntesis) dentro")"#
→ true (ignora paréntesis dentro de strings)

r#"let s = "string con {llaves} y [corchetes]"; if true { }"#
→ true

"// comentario con ( sin cerrar\nfn main() { }"
→ true (ignora comentarios)

r#"fn main() {
    let data = vec![
        (1, 2),
        (3, 4)
    ];
    
    for (x, y) in data {
        println!("{}, {}", x, y);
    }
}"#
→ true
*/

use std::collections::HashMap;

fn check_clousures(input: String) -> bool {
    let mut valid = true;
    let mut vector: Vec<char> = Vec::new();
    let dict: HashMap<char, i8> = HashMap::from([
        ('(', 1),
        (')', -1),
        ('{', 2),
        ('}', -2),
        ('[', 3),
        (']', -3),
        ('<', 4),
        ('>', -4),
    ]);

    for c in input.chars() {
        if dict.contains_key(&c) {
            let val: i8 = dict.get(&c).unwrap().clone();
            if val > 0 {
                vector.push(c);
            } else if val < 0 && vector.len() > 0 {
                let last_char = vector[vector.len() - 1];
                let last_val: i8 = dict.get(&last_char).unwrap().clone();
                if last_val + val == 0 {
                    vector.pop();
                } else {
                    valid = false;
                    break;
                }
            }
        }
    }

    if vector.len() > 0 {
        valid = false;
    }

    return valid;
}

fn main() {
    println!("Write an expression to validate!");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let result = check_clousures(input);
    println!("{}", result);
}
