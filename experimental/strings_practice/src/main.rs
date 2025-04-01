/**
 * Conceptos de estudio:
 * - Strings sin uso del objeto String
 * - Prestamos y referencias
 * - Mutabilidad
 * - Apuntadores
 * - Lifetime
 *
 * Explicación:
 * - Lifetime:
 *      - Usar <'a> para indicar lifetime en la función
 *      - Usar &'a para indicar lifetime en la variable de entrada en la función
 *      - Usar &'a [data type] para indicar lifetime en la variable de salida en la función
*/

fn borrow_string2(txt: &mut &str) {
    *txt = "in function 2";
}

fn borrow_string<'a>(txt: &'a mut &str) -> &'a str {
    let mut txt2: &str = txt;
    println!("{}", txt2);
    *txt = "in function";
    println!("{}", txt);
    borrow_string2(txt);
    //txt2 = r#"weiufubw"#;
    txt2 = "weiufubw";
    println!("{}", txt2);
    txt2
}
fn main() {
    let mut text: &str = "";
    println!("{}", text);
    text = "hello";
    println!("{}", text);
    borrow_string(&mut text);
    println!("{}", text);
    text = "world";
    println!("{}", text);
}
