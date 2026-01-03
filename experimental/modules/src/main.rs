/*
 * TITLE: En este ejemplo aprendemos a cargar modulos desde main
 * Cargaremos modulos como archivos y como dorectorios
*/

// file's name as module 
mod ext_fn;

// directory's name as module
mod mods;

// directory's name as module
mod utils{
    // sub-directory's as sub-module
    pub mod core;
}

fn main() {
    println!("Hello, world!");
    //---------------------------------------------------------
    // Invocar funciones publicas desde el modulo "ext_fn".
    // El nombre del archivo influye en el nombre del modulo    
    ext_fn::test1();
    //---------------------------------------------------------
    // el archivo de este modulo debe llamarse "mod.rs"
    // si se cambia el nombre diferente a mod.rs no compila
    mods::test1();
    //---------------------------------------------------------
    // se llama la función de acuerdo a la ruta que tenga
    utils::core::test1();
}
