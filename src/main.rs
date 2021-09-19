use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    let mut x = 5;

    println!("{}", x);

    x = 6;

    println!("{}", x);

    const MAX_POINTS: u32 = 100_000;

    println!("{}", MAX_POINTS + 23213);

    let x = 198;

    println!("{}", x);

    let x = x + 3;

    println!("{}", x);

    let spaces = "       ";

    let spaces = spaces.len();

    println!("{}", spaces);

    let u: u32 = 342121;
    let i: i32 = 0x2880;

    let u_8: u8 = 32;
    let i_size: isize = 0o73;
    let u_b: u16 = 0b1111_0101_1011_0111;

    let byte: u8 = b'A';

    println!("{}", byte);

    let xxf: f32 = 3.43;
    let xxff: f64 = 231432.234424;

    println!("{}", xxf);
    println!("{}", xxff);

    let iif: bool = true;

    println!("{}", iif);

    let ch: char = 'd';

    println!("{}", ch);

    let tup: (i32, f64, u8) = (5000, 2.33, b'd');

    let (x, y, z) = tup;

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let arr2 = [3; 5];

    println!("{:?}", a);
    println!("{:?}", arr2);

    let x1 = a[0];

    println!("{}", x1);

    // a[1290];

    let x = 5;

    let y = {
        let x = 55;
        x + 1
    };

    let you: ();

    // repeat(3, "hii");

    // let arr: [i32; rangeLength] = rangeTo(10);
    //
    // println!("{:?}", arr);

    branches();

    loops();

    while_loop();
    fire(10);
    while_arr([10, 20, 30, 40, 50]);
    loop_arr([10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
    for_arr([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    mordern_arr();

    scopes();
}

fn scopes() {
    {
        let a: &str = "hello";

        a;
    };

    {
        let mut s = String::from("hello");

        s.push_str(", world!");

        println!("{}", s);
    }

    let x = 5;
    let y = x;

    let s1 = String::from("hello world");
    let s2 = s1;

    // println!("{}", s1);
    println!("{}", s2);

    let s3 = s2.clone();

    println!("{}", s2);
    println!("{}", s3);

    some_function(s3);

    // println!("{}", s3);

    some_function(s2.clone());

    println!("{}", s2);

    let r1 = give_ownership();

    let r2 = String::from("asdf");

    let r3 = takes_and_gives_back(r2);

    let (len_, mut r4) = get_len(r3);

    let len_refed: usize = get_len_by_reference(&r4);

    println!("{}.len(): {}", r4, len_refed);

    change(&mut r4);

    let r4_m1: &mut String = &mut r4;

    {
        let r4_m2: &mut String = &mut r4;

        // let r4_ref: &String = &r4;
        //
        // println!("{}, {}", r4_m2, r4_ref);

        // println!("{}, {}", r4_m1, r4_m2);
    }

    // println!("{}, {}", r4_m1, r4_m2);

    let amidead = dangle();
}

fn dangle() -> String {
    let dead_ = String::from("DEAD?");

    dead_
}

fn change(s: &mut String) -> () {
    s.push_str(", hello this is additional string!");
}

fn get_len_by_reference(str: &String) -> usize {
    str.len()
}

fn get_len(str: String) -> (usize, String) {
    (str.len(), str)
}

fn takes_and_gives_back(p0: String) -> String {
    p0
}

fn give_ownership() -> String {
    let some_string = String::from("hi! Im some string");

    some_string
}

fn some_function(param: String) {
    println!("{}", param);
}

fn mordern_arr() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    print!("FIRE!!!");
}

fn for_arr(arr: [i32; 12]) {
    for element in arr.iter() {
        println!("ELE: {}", element)
    }
}

const len: usize = 10;

fn loop_arr(arr: [i32; len]) {
    let mut index: usize = 0;

    loop {
        if index < len {
            println!("element22: {}", arr[index]);

            index += 1;
        } else {
            break;
        }
    }
}

fn while_arr(arr: [i32; 5]) {
    let mut index = 0;

    while index < 5 {
        println!("element: {}", arr[index]);

        index += 1;
    }
}

fn fire(number: i32) {
    let mut number = number;

    loop {
        if number != 0 {
            println!("{}", number);

            number -= 1;
        } else {
            break;
        }
    }

    println!("fire!!");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("Fire!");
}

fn loops() {
    // todo!()

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}

fn five() -> i32 {
    5
}

fn plus_one(inp: i32) -> i32 {
    inp + 1
}

fn for_loop() {}

fn repeat(count: i32, msg: &str) {
    let mut c = 0;

    loop {
        match c.cmp(&count) {
            Ordering::Greater => break,
            Ordering::Less => {
                c += 1;
                println!("{}", msg);
            }
            _ => {}
        }
    }
}

const rangeLength: usize = 5;

// fn rangeTo(start: i32) -> [i32; rangeLength] {
//     let arr: [i32; rangeLength] = [0; rangeLength];
//
//     let mut i: i32 = 0;
//
//     loop {
//         match i.cmp(rangeLength.into_i32()) {
//             Ordering::Less => arr[i] = start + i,
//             _ => {
//                 break;
//             }
//         }
//
//         i = i + 1
//     }
//
//     arr
// }

fn branches() {
    let number = 3;

    if number < 3 {
        println!("condition matched!");
    } else {
        println!("condition not matched!");
    }

    // let condition: bool = true;
    let condition: bool = false;

    let number = if condition { 100 } else { 200 };

    println!("{}", number);
}
