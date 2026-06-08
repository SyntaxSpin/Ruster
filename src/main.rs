use std::io; // for input console std crate io is the module

fn test_one() {
    println!("Done")
}
fn test_with_params(x: i32, y: i32) {
    print!("Product : {}", x * y);
}
fn express(x: i32, y: i32) -> i32 {
    x * y // or return of clang
}

fn main() {
    let mut x: u64 = 4;
    println!("x = {}", x);
    //test
    x = 5;
    print!("X now is {}", x);
    let mut x = 7;
    print!("\nThis scoped x is now {}", x);
    x = x + 1;
    print!("This scoped increased x is now {} ", x);
    //Constants
    const YCONST: u128 = 25;
    println!("This is a constant {}", YCONST);
    /*
     * NOW THIS IS DATA TYPES TUTORIAL FOR LEARINING (SCALAR AND COMPOUND)
     */
    //Scalar DataTypes
    let y: i32 = 2;
    // WE HAVE SIGNED INTEGERS FROM i8 TO i128
    let z: u32 = 9; //UNSIGNED INT
    let t: f32 = 15.0; //ALSO THERE'S f64 FOR FLOATS f64 IS DEFAULT
    let b: bool = true; // BOOLEAN 0 OR 1 OR FALSE OR TRUE
    let l: char = 'a'; // CHARACTERS LIKE ClANG
    // Compound DataType
    let s: &str = "Hello World"; // FOR STRING USE & LIKE CLANG
    let tup: (i32, bool, char) = (1, true, 'r'); //TUPLE YOU CANNOT ADD ELEMENTS
    println!("{}", tup.0); // Indexing different than other programming languages so (tupleName.index)

    let arr = [1, 2, 3, 4, 5]; //Like Clang even in Indexing arrayName[0]
    let mut array: [i64; 6] = [1, 2, 3, 4, 5, 6]; // [datatype; size]

    // Inputing in console (scanf() , input() like in rust)
    println!("Input something : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("ERROR");
    println!("My message is {}", input);

    //Type conversion
    let j: u8 = 9;
    let k: u8 = 10;
    let m = j + k;
    println!("{} + {} = {}", j, k, m);
    // you can do this or that
    let j = 9_i8;
    let k = 10_i8;
    let sum = j + k;
    println!("{}", sum);
    // or
    let j = 9i8;
    let j = 9 as i8;
    // CONVERT STRING TO INT
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).expect("error");
    let mut int_input: i64 = inputs.trim().parse().unwrap();
    int_input = int_input + 56;
    println!("{} , num {}", inputs, int_input);

    //CONDITIONS
    let cond: bool = 2 < 3;
    println!("condition is {}", cond);
    let cond: bool = (2 as f64) < 2.2;
    println!("condition is {}", cond);
    // compound conditions are like c language  != && ||
    let food = "Burger";
    if food == "Burger" || food == "burger" {
        println!("McDonalds!!");
    } else if food == "Orange" {
        println!("GO BUY THEM FROM WALMART !! ");
    } else {
        println!("DairyQueen!!");
    }
    test_one();
    test_with_params(5, 9);
    //Statements
    let number = {
        let x = 3;
        x + 1 // Expression no ;
    };
    println!("{}", express(5, 9));

    //LOOPS
    for i in 1..5 {
        println!("{}", i); // to include the five add =
    }
    let mut i = 0
    while i <= 20 {
        println!("You are in step number {} " , i)
        i+=1
    }
}
