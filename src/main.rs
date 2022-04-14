
// NOTES:
// Rust Macros use a trailing ! to call and this differentiates them from functions.
// eg. println!() is a macro and the sample below has a function example(x: i32, y: i32)

fn main() {

    // Basic Types
    // Note: All variables in Rust are immutable unless the 'mut' keyword is used
    //      to make them mutable
    let _boolean_type = true;
    let integer_type = 10;
    let float_type = 107.445;
    let mut _character_type = 'j'; //Mutable variable
    let string_type = "Jason";

    // Call functions
    let _return_val_01 = example_01_basic_add_numbers(integer_type, 10);

    // Print to console
    println!("KB Rust Guide"); // Print a string
    println!("---------------------------------");
    println!("float: {:?}", float_type);   // {:?} = token to print in debug mode. Inserts a value from
                                    // the proceeding variables where the token is (eg. float_type)
    println!("float and string: {:?} {:?}", float_type, string_type); // ... multi-token example.
    println!();

    // 02 - Control Flow
    example_02_if_elseif_else(99);
    example_02_match(2);

    // 03 - Repetition - loop and while
    example_03_loop();
    example_03_while();
}

// Function - Basic example passing in parameters and returning a value
fn example_01_basic_add_numbers(x: i32, y: i32) -> i32 {
    // *** NOTE: If we do not put a semicolon at the end of the last line it is
    // presumed to be the "return" value. Otherwise use the 'return' keyword and include ';'
    // eg. return x + y;
    x + y
}

// ********** 02 - Control Flow ************
// example_02_if_elseif_else
// *****************************************

// 02 - Control Flow - if..else if..else
fn example_02_if_elseif_else(a: i32) {
    print!("example_02_control_flow: ");
    if a >= 200 {
        println!("Huge Number");
    }
    else if a > 99 {
        println!("Big Number");
    }else{
        println!("Small Number");
    }
}

// 02 - Control Flow - match
// NOTE: 'match' must allow for every possibility, use the '_' as a value for 'anything else'
fn example_02_match(a: i32) {

    match a {
        1 => println!("We matched on the number 1"),
        2 => println!("We matched on the number 2"),
        100 => println!("We matched on the number 100"),
        _ => println!("We didn't match anything important")
    }
}

// ********** 03 - Repetition ************
// example_03_loop
// example_03_while
// *****************************************

// 03 - Repetition - loop
fn example_03_loop() {
    let mut a = 0;

    // 'loop' in Rust will loop indefinitely until something causes it to 'break' out
    println!("*** LOOP ***");
    loop {
        if a == 5 {
            break;
        }
        println!("loop #: {:?}", a);
        a = a + 1;
    }
}

// 03 - Repetition - while
fn example_03_while() {
    let mut a = 0;

    // 'while' in Rust will loop until a condition is met eg. as per "a != 5" below
    println!("*** WHILE ***");
    while a != 5 {
        println!("while #: {:?}", a);
        a = a + 1;
    }
}