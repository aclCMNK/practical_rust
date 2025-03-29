fn main() {
    let _args: Vec<String> = std::env::args().collect();
    let length: usize = _args.len();
    if length < 2 {
        println!("No name provided!");
        return;
    }
    let name: &String = &_args[1];
    println!("Hello, world!, {}", name);
}
