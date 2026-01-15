use chrono::Local;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

fn generate_unique_id() -> String {
    let id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("ID-{}", id)
}

#[derive(Debug, Default, Clone)]
struct Book {
    ID: String,
    title: String,
    author: String,
    year: u32,
    available: bool,
    status: bool,
}

#[derive(Debug, Default, Clone)]
struct Library {
    books: HashMap<String, Book>,
}

#[derive(Debug, Default, Clone)]
struct BorrowedBook {
    ID: String,
    borrowing_date: String,
    due_date: String,
}

#[derive(Debug, Default, Clone)]
struct User {
    name: String,
    nit: String,
    books: HashMap<String, BorrowedBook>,
}

fn add_book(library: &mut Library) {
    let mut name: String = String::new();
    let mut author: String = String::new();
    let mut year: String = String::new();

    println!("-----------------------------");
    println!("Ingrese el titulo del libro");
    std::io::stdin()
        .read_line(&mut name)
        .expect("Error al leer el titulo");
    println!("-----------------------------");
    println!("Ingrese el autor del libro");
    std::io::stdin()
        .read_line(&mut author)
        .expect("Error al leer el autor");
    println!("-----------------------------");
    println!("Ingrese el año del libro");
    std::io::stdin()
        .read_line(&mut year)
        .expect("Error al leer el año");
    println!("-----------------------------");

    let mut book: Book = Book::default();
    book.ID = generate_unique_id();
    book.title = name.trim().to_string();
    book.author = author.trim().to_string();
    book.year = year.trim().parse().unwrap();
    book.available = true;
    book.status = true;
    library.books.insert(book.ID.clone(), book);
}

fn list_books(library: &Library) {
    println!("-----------------------------");
    println!("-----------------------------");
    for (id, book) in library.books.iter() {
        println!("ID: {}", id);
        println!("Titulo: {}", book.title);
        println!("Autor: {}", book.author);
        println!("Año: {}", book.year);
        println!("Disponible: {}", book.available);
        println!("-----------------------------");
    }
    println!("-----------------------------");
}

fn list_available_books(library: &Library) {
    println!("-----------------------------");
    println!("-----------------------------");
    for (id, book) in library.books.iter() {
        if !book.available {
            continue;
        }
        println!("ID: {}", id);
        println!("Titulo: {}", book.title);
        println!("Autor: {}", book.author);
        println!("Año: {}", book.year);
        println!("Disponible: {}", book.available);
        println!("-----------------------------");
    }
    println!("-----------------------------");
}

fn list_borrowed_books(library: &Library) {
    println!("-----------------------------");
    println!("-----------------------------");
    for (id, book) in library.books.iter() {
        if book.available {
            continue;
        }
        println!("ID: {}", id);
        println!("Titulo: {}", book.title);
        println!("Autor: {}", book.author);
        println!("Año: {}", book.year);
        println!("Disponible: {}", book.available);
        println!("-----------------------------");
    }
    println!("-----------------------------");
}

fn add_user(users: &mut HashMap<String, User>) {
    println!("-----------------------------");
    println!("Ingrese el nombre del usuario");
    let mut name: String = String::new();
    let mut nit: String = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Error al leer el nombre");
    println!("Ingrese el número de identificación del usuario");
    std::io::stdin()
        .read_line(&mut nit)
        .expect("Error al leer el NIT");
    println!("-----------------------------");
    let mut user: User = User::default();
    user.name = name.trim().to_string();
    user.nit = nit.trim().to_string();
    users.insert(user.nit.clone(), user);
}

fn list_users(users: &HashMap<String, User>) {
    println!("-----------------------------");
    println!("-----------------------------");
    for (nit, user) in users.iter() {
        println!("NIT: {}", nit);
        println!("Nombre: {}", user.name);
        println!("-----------------------------");
    }
    println!("-----------------------------");
}

fn borrow_book(bookID: String, nit: String, library: &Library, users: &mut HashMap<String, User>) {
    println!("-----------------------------");
    println!("Ingresa el nit del usuario:");
    let mut nit: String = String::new();
    std::io::stdin()
        .read_line(&mut nit)
        .expect("Error al leer el NIT");
    println!("-----------------------------");
    println!("Ingresa el ID del libro:");
    let mut bookID: String = String::new();
    std::io::stdin()
        .read_line(&mut bookID)
        .expect("Error al leer el ID");
    println!("-----------------------------");
    let mut user: User = User::default();
    if let Some(user_exists) = users.get(nit.trim()) {
        user = user_exists.clone();
    } else {
        println!("El usuario no existe");
        println!("-----------------------------");
        return;
    }
    let mut book: Book = Book::default();
    if let Some(book_exists) = library.books.get(bookID.trim()) {
        book = book_exists.clone();
    } else {
        println!("El libro no existe");
        println!("-----------------------------");
        return;
    }
    let mut borrowed_book: BorrowedBook = BorrowedBook::default();
    borrowed_book.ID = book.ID.clone();
    let date: String = Local::now().format("%Y-%m-%d").to_string();
    let due_date: String = (Local::now() + chrono::Duration::days(14))
        .format("%Y-%m-%d")
        .to_string();
    borrowed_book.borrowing_date = date;
    borrowed_book.due_date = due_date;
    user.books.insert(book.ID.clone(), borrowed_book);
    book.available = false;

    println!("Libro prestado exitosamente");
    println!("-----------------------------");
}

fn main() {
    let borrowing_limit: u32 = 2;
    let mut library: Library = Library::default();
    let mut users: HashMap<String, User> = HashMap::new();

    loop {
        println!("Bienvenido a la biblioteca Rustaniana!");
        println!("Seleccione una accion:");
        println!("-----------------------------");
        println!("1. Agregar un libro");
        println!("2. Deshabilitar un libro");
        println!("3. Buscar un libro");
        println!("4. Listar libros");
        println!("5. Listar libros disponibles");
        println!("6. Listar libros prestados");
        println!("7. Prestar un libro");
        println!("8. Devolver un libro");
        println!("9. Listar usuarios");
        println!("10. Agregar un usuario");
        println!("11. Listar usuarios");
        println!("12. Deshabilitar un usuario");
        println!("13. Salir");
        println!("-----------------------------");
        let mut action: String = String::new();
        std::io::stdin()
            .read_line(&mut action)
            .expect("Error al leer la accion");

        match action.trim() {
            "1" => {
                add_book(&mut library);
            }
            "2" => {}
            "3" => {}
            "4" => {
                list_books(&library);
            }
            "5" => {
                list_available_books(&library);
            }
            "6" => {
                list_borrowed_books(&library);
            }
            "7" => {
                borrow_book(String::new(), String::new(), &library, &mut users);
            }
            "8" => {}
            "9" => {}
            "10" => {
                add_user(&mut users);
            }
            "11" => {
                list_users(&users);
            }
            "12" => {}
            "13" => {
                break;
            }
            _ => {
                println!("Opcion no valida");
            }
        }
    }
}
