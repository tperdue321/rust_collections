
#![allow(unused_variables)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String)
}

use std::collections::HashMap;

fn main() {
    // exploring Vectors in Rust



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



    // explore Hash maps in Rust



    // keys must all be of one type
    // and values must all be of one type
    // (keys can be different than values) 
    let mut scores = HashMap::new(); 

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    // A vector of tuples can be transformed into HashMaps

    // type must be explict when using collect to transform because collect
    // can go to many data types but
    // HashMap. <_,_> is used because Rust can infer key, val types
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10,50];
    // zip vectors together into vector of tuples and transform into hash map
    let scores_from_vectors: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();


    // HashMaps take ownership of types that don't implement the Copy trait
    let key = 10;
    let value = String::from("Purple");

    let mut map = HashMap::new();

    map.insert(key, value);
    // map now owns key, value. the program will fail if those
    // variables are now used.
    // println!("{:?}", value); => produces moved error
    // if a ref to a val is in a HashMap, it takes ownership of the reference
    // but the value itself is not moved into the HashMap

    let key = 10;
    // get returns an option, Some(&String) in this case
    let field_value = map.get(&key);
    match field_value {
        Some(string) => println!("{:?}", string),
        _ => (),
    }

    for (key, value) in &scores {
        println!("key: {}, value: {}", key, value);
    }

    // overwrite a value
    scores.insert(String::from("Blue"), 25) ;
    // it's that simple!
    println!("{:?}", scores);

    // insert a value if it currently doesn't exist in the HashMap
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Purple")).or_insert(50);
    println!("{:?}", scores);


    // iterate over a string and return a hash of words as keys
    // and their count in a string as a value
    let string_lit = "Hello Rust. I am studying Rust. I am counting words Rust.";
    let mut map = HashMap::new();
    for word in string_lit.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
