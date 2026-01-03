/**
 * Conceptos de estudio:
 * - Strings sin uso del objeto String
 * - Prestamos y referencias (se usa con &)
 * - Mutabilidad (se usa con mut)
 * - Apuntadores (se representa con *)
 * - Lifetime. Con el lifetime garantizamos que no se haga referencia a una posición de memoria de
 * manera invalida
 *
 * Explicación:
 * - Lifetime: El lifetime de las variables se indican con "'[whatever]"
 *      - Usar <'a> para indicar lifetime en la función
 *      - Usar &'a para indicar lifetime en la variable de entrada en la función
 *      - Usar &'a [data type] para indicar lifetime en la variable de salida en la función
*/

fn borrow_string2(txt: &mut &str) {
    *txt = "in function 2"; // modificamos el valor de txt que viene de fuera
}

fn borrow_string<'a>(txt: &'a mut &str) -> &'a str {
    let mut txt2: &str = txt; // capturamos la referencia de txt
    println!("3. text = {}", txt2); // imprime hello
    *txt = "in function"; // a la variable txt de fuera le capturamos el apuntador y le modificamos
                          // el valor
    println!("4. txt = {}", txt); // imprime "in function"
    borrow_string2(txt);
    //txt2 = r#"weiufubw"#;
    txt2 = "weiufuw";
    println!("5. txt2 = {}", txt2); // imprime "weiufuw"
    txt2 //retornamos el txt2
}
fn main() {
    let mut text: &str = "";
    println!("1. text = {}", text); // imprime vacio
    text = "hello";
    println!("2. text = {}", text); // imprime "hello"
    borrow_string(&mut text);
    println!("6. text = {}", text); // imprime "in function 2"
    text = "world";
    println!("7. text = {}", text); // imprime world
}
