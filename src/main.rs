use std::cmp::Ordering;

fn main() {
    repeat(5, "hello, wolrd!");

    let arrrr = rangeTo(5);

    println!("{:?}", arrrr);
}

fn repeat(count: i32, msg: &str) {
    for i in 0..count {
        println!("{}", msg);
    }
}

const rangeLength: usize = 10;

fn rangeTo(start: i32) -> [i32; rangeLength] {
    let mut arr = [0; rangeLength];

    let mut i: usize = 0;

    while i < rangeLength {
        arr[i] = start + i as i32;

        i = i + 1;
    }

    arr
}
