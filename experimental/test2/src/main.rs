// CALLBACKS:
// Rust allows us to run callbacks let us to use a potent and powerly procedures.
// We can use two ways to create a callbacks:
// 1. Sending a created function to callback function
// 2. Sending a anonymous function to callback function


fn callback<F>(cback: F) where F: Fn(i32) -> i32 {
    let result = cback(1);
    println!("{}", result);
}

fn test(a: i32) -> i32 {
    println!("{}", a);
    a + 1
}

struct Test {
    a: i32
}

fn read_struct(data: &mut Test) {
    println!("{}", data.a);
    data.a = 20;
}

fn main() {
    //NORMAL CALLBACK
    callback(test);
    //ANONIMUS CALLBACK
    callback(| a: i32 | -> i32 { 
        println!("{}", a);
        a * 3
    } );
    //STRUCT
    let mut test_struct: Test = Test {
        a: 10
    };
    read_struct(&mut test_struct);
    println!("{}", test_struct.a);
}
