enum Cell {
    Int(i32),
    Float(f64),
    Text(String)
}


// #![allow(unused_variables)]
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

}
