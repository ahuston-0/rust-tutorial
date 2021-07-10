// Chapter 3 of the Rust book

pub fn main() {
    mutability();
    shadowing();
    datatypes();
    functions();
    loops();
    // type annotations
    let _afloat: f64 = 1.0; // regular annotation
    let _aint = 5i32; // suffix annotation
}

fn mutability() {
    let x = 5; // Non-mutable variable
    println!("The value of x is: {}", x);
    //x = 6; // Reassignment is illegal on non-mu   table types
    let mut y = 7; // Mutable variable
    println!("The value of y is: {}", y);
    y = 12; // Reassign mutable variable
    println!("The value of y is: {}", y);

    // declaring constant variables
    // const seems to be compile-time while let is runtime
    // Usually done in all caps, with type definition
    // underscores can be used wit numbers for readability
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS)
}

fn shadowing() {
    let x = 5; // instantiate x
    let x = x + 1; // instantiate new x that shadows old x
    let x = x + 2;
    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len(); // Shadowing allows changing types
    println!("There are {} spaces in this string", spaces);
}

fn datatypes() {
    // Compiler needs more information as it can't infer this type
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess is {}", guess);

    /* Scalar Types: point to single value
     * integers: i8, u8, ..., i128, u128, isize, usize
     *      isize and usize are architecture dependent
     *      i.e. 64 bit isize for 64 bit system
     *      Dec: 10_200, Hex: 0xff, Oct: 0o77, 0b0110_0101
     *      Byte(u8 only): b'A'
     *      Over/underflow has expected behavior
     *      use wrapping_* for wrapping over/underflow
     *      use checked_* to return None on over/underflow
     *      use overflowing_* to return val and bool on overflow
     *      use saturating_* to return max/min on over/underflow
     * floating points: f32 and f64
     *      Rust supports std operations (+,-,*,/,%)
     * Booleans:
     *      bool: true, false
     * characters: char is 4B and is Unicode by default
     */

    // Compound types: container types
    // tuples: can declare with types
    let tup: (usize, f64, u8) = (3, 3.14159, 0x80);
    let (tup_size, pi, byte) = tup; // unpack/destructure tuple
    println!("tup looks like this: ({}, {}, {})", tup_size, pi, byte);
    println!("The size of the tuple is: {}", tup.0); // index access

    // arrays: use square brackets
    // locked size, vector is suggested over array
    let a = [0, 0, 1, 2, 3, 5, 8];
    let b: [u8; 5] = [1, 1, 2, 6, 24]; // specify type and size
    let c = [0; 100]; // intialize array of 100 0's
    println!("First value of a({}), b({}), and c({})", a[0], b[0], c[0]);
    // trying to index out of bounds causes a panic
}

fn functions() {
    println!("Function returned: {}", another_function(27));

    // if statments do not require parens
    if another_function(7) == 8 {
        println!("This is a successor function");
    } else {
        println!("This is not a successor function");
    }

    // Ternaries are basically just setting a value equal to an if statement
    // If ternary is used, all returns must be the same type
    let succ_seven = if another_function(7) == 8 {
        another_function(7)
    } else {
        8
    };

    println!("The successor of 7 is {}", succ_seven);
}

// arrow notation for return
fn another_function(x: i32) -> i32 {
    println!("The value of x is {}", x);
    x + 1 // no comma means return, called ending expression
          // Statements perform an action, no return
          // Expressions evaluate and return a value
}

fn loops() {
    let x = 0;
    let mut y = 0;

    // loop keyword is functionally equivalent to while(true)
    loop {
        // Shadowing respects scope, so x will always evaluate to 1
        let x = x + 1;
        y = y + 1;
        println!("x is {}, was originally {}", x, 0);
        println!("y is {}, was originally {}\n", y, 0);
        if y == 10 {
            break; // break works as expected
        }

        /*
        Rust keeps suggesting switching if/break to a "guarded return"
        The below code shows what it would look like

        if y == 10 {
            continue;
        }
        break;

        At the very least, this is indicative that continue also works
        as expected
        */
    }
    let mut counter = 0;
    let mult = 2;

    // Can set loop return equal to variable, like with if statements
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * mult; // Can return value from break
        }
    };

    println!("{}*{}={}", counter, mult, result);

    let mut factorial = 3;
    let mut counter = factorial;
    let orig = factorial;

    while counter != 1 {
        counter -= 1; // -- and ++ do not seem to work
                      // *= works as expected, will have to check similar operators
        factorial *= counter;
    }

    println!("{}!={}", orig, factorial);

    let a = [1, 2, 3, 4, 5];

    // Iterators!!!
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Thanks language server for telling me how to do for_each
    a.iter().for_each(|element| {
        println!("the value is: {}", element);
    });

    // Generate array via Range syntax, also reversing
    println!("Count down from 4!");
    for number in (1..4).rev() {
        println!({}, number);
    }

    println!("\nDo it again, but with for_each!");
    // This syntax looks like it might be a closure?
    // Confirmed its a closure
    // brackets don't seem to be necessary when its a one-liner
    (1..4).rev().for_each(|x| println!("{}", x));
}
