// Gestor de Tareas Simple: 
// Crea un Vec de tareas (strings) con funciones para agregar, eliminar y listar. 
// Practica pasar referencias mutables e inmutables.

fn add_task(tasks: &mut Vec<String>, task: String) {
    tasks.push(task);
}

fn list_tasks(tasks: &Vec<String>) {
    for (index, task) in tasks.iter().enumerate() {
        println!("{}: {}", index, task);
    }
}

fn delete_task(tasks: &mut Vec<String>, index: usize) {
    tasks.remove(index);
}

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    loop {
        println!("1. Agregar tarea");
        println!("2. Eliminar tarea");
        println!("3. Listar tareas");
        println!("4. Salir");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Ingrese la tarea:");
                let mut task = String::new();
                std::io::stdin().read_line(&mut task).unwrap();
                add_task(&mut tasks, task.trim().to_string());
            }
            "2" => {
                println!("Ingrese el índice de la tarea a eliminar:");
                let mut index_str = String::new();
                std::io::stdin().read_line(&mut index_str).unwrap();
                let index: usize = index_str.trim().parse().unwrap();
                delete_task(&mut tasks, index);
            }
            "3" => list_tasks(&tasks),
            "4" => break,
            _ => println!("Opción no válida"),
        }
    }
}
