// Generics used to use to give parameters possibility to have multiple data types rather than a specific data type

pub fn takes_anything<T>(x: T) { // x has type T, T is a generic type
}

pub fn takes_two_of_the_same_things<T>(x: T, y: T) { // Both x and y has the same type
}

pub fn takes_two_things<T, U>(x: T, y: U) { // Multiple types
}

// x, y co-ordinates can be int or float
#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}
