use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list:&[T]) -> T {
    let mut biggest = list[0];

    for &item in list {
        if item > biggest {
            biggest = item;
        }
    }

    biggest
}

fn main() {
    
}
