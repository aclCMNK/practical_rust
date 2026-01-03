/*
 * TITLE: Lectura de argumentos entrantes desde la ejecución del programa por cli
*/
fn main() {
    // capturamos los argumentos por cli
    let _args: Vec<String> = std::env::args().collect();
    // se obtiene el tamaño de argumentos
    let length: usize = _args.len();
    if length < 2 {
        println!("No name provided!");
        return;
    }
    // capturamos el primer argumento
    let name: &String = &_args[1];
    // lo imprimimos en pantalla
    println!("Hello, world!, {}", name);
}
