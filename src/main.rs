
#![allow(unused_variables)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String)
}


fn main() {
    // init an empty vector
    let v: Vec<i32> = Vec::new();
    // init a vector with values
    // and implicit typing using a built in
    // macro
    let v = vec![1, 2, 3];

    // create a vector then add elements to it 
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    {
        let v = vec![1, 2, 3, 4];
    } // v goes out of scope and the memory is freed


    let v = vec![1, 2, 3, 4, 5];
    // two ways to access values
    // this causes a panic if looks outside the range of the vector
    let third: &i32 = &v[2];
    // this will never cause a panic
    let third: Option<&i32> = v.get(2);


    // let doesnt_exist: &i32 = &v[100]; // will cause a panic
    // this is safe. reaching past the vector just returns a None value
    let doesnt_exist: Option<&i32> = v.get(100);

    // if using an immutable borrow
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // then a mutable borrow (v.push(6)) won't compile
    // v.push(6);

    let mut v = vec![1, 2, 3, 4, 5];
    // iterate over vector
    for int in  &v {
        println!("int in vec: {}", int);
    }

    // iterate mutably over a vector and alter it
    for int in &mut v {
        *int += 50;
    }

    for int in  &v {
        println!("int in vec: {}", int);
    }


    // if you need multiple data types within a vector use enums
    let v = vec![
        Cell::Int(3),
        Cell::Float(3.3),
        Cell::Text(String::from("three"))
    ];

    // explore Strings in Rust as a collection

    let s = String::new();

    // any type that implements the Display trait can be
    // converte to a String like so
    let string_lit = "this is a string";
    let mut s = string_lit.to_string();
    // strings are UTF-8 by default. all of these are valid

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // some string functions

    s.push_str(", I added this with push()");
    s.push('!');

    let s2: &str = "a &str.";
    s.push_str(&s2);



    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // this is a bit unwieldy
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{:?}", s);

    // redeclare s1 variable because it has been moved and is no longer valid
    let s1 = String::from("tic");
    // this is a easier to work with
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{:?}", s);


    let hello = "Здравствуйте";
    // rust cannot do standard indexing because it is UTF-8 encoded
    // which means 1 byte may not be equivalent to to char
    let s = &hello[0..4];
    // &hello[0..1] => this will crash the program because it will cut off
    // in the middle of the hindi letter З. it is not the same as the english
    // number 3. slicing must be done with care because it can crash a program


    // a safer way to access single chars instead of slicing
    for ch in "नमस्ते".chars() {
        println!("{}", ch);
    }
    for byte in "नमस्ते".bytes() {
        println!("{}", byte);
    }
}
