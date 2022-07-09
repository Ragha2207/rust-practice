pub fn vector_game() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("Reference of v {}", i);
    }

    for i in &mut v {
        println!("Mutable reference of v {}", i);
    }

    for i in v {
        println!("Take ownership of vector and its element {}", i);
    }

    // println!("Vector v {:?}", v); // this won't work the previous block has taken ownership of the vector
}
