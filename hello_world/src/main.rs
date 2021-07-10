// Old code, not sure when I did this or
// if this correlates to a chapter of the Rust book

fn main() {
    // Reminder to somehow set the debug and test/run keybindings
    hello_world();
    vars_and_comments();
    string_formatting();
    intro_structs_debug();
    display();
    testcase_list();
    display_cities();
    display_colors();
}

fn hello_world() {
    println!("Hello, world!");

    /*
     * // Comments
     * /* Block Comments */
     * /// Library documentation notation
     * //! Block library documentation (confirm?)
     */
}

fn vars_and_comments() {
    let x = 5 * 5; // JS style variables
    println!("x = {}", x);
    /*
     * So this seems like python style
     * substitution of variables.
     * Or is it JS again? Not sure.
     * Also everything gets stringified
     */
}

fn string_formatting() {
    print!("{0} arguments are a {1}\n", "positional", "thing");
    // Print and positional arguments both exist
    // Also named objects
    // Also numerical formatting

    println!("{number:b} is {number} in binary", number = 2);
    // {var:format} can format a variable being substituted
    // {name} is used to add named arguments

    println!(
        "{number:>0width$} is {number} but padded {} \
    with {width} digits",
        number = 12,
        width = 5
    );

    let pi = 3.141592;
    println!("Pi is roughly {:.*}", 3, pi);
}

fn intro_structs_debug() {
    #[allow(dead_code)] // Because apparently structures are dead
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct {:?} won't print...", Structure(3));
    // Prints struct, note this is dead code and probably is bad
    // #[derive(Debug)] can be used on structs to enable debug printing
    // {:?} for debug print or {:#?} for debug pretty print
}

fn display() {
    use std::fmt; //format module

    struct Structure(i32);

    // Implement 'fmt::Display' in order to use {}
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            // Taken from Rust by Example

            write!(f, "{}", self.0)
            // Notice the lack of semicolon here
            // Not sure what that's about yet
        }
    }

    println!("This struct '{}' will print...", Structure(3));

    /* Note that supposedly implementing Display does not work
     * for generic cases (such as vectors)
     * I would like to challenge that but for now, Debug exists
     */

    #[derive(Debug)]
    struct MinMax(i64, i64); // Multiple arguments, tuple

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        // Named arguments, C-style struct
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    println!("Minimum and maximum of set: {}", MinMax(1, 65));
    println!("Point in 2-space: {}", Point2D { x: 0.0, y: 2.0 });
    println!(
        "Example of a complex number; {}",
        Complex {
            real: 2.3,
            imag: 1.5
        }
    );

    // Other formatting has to implemented manunally
    // Check std::fmt for more on that
}

fn testcase_list() {
    use std::fmt;
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0; //ref to vector

            write!(f, "[")?;

            //iterate over v in vec while enumerating in count
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, v)?;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1, 3, 2]);
    println!("{}", v);
}

fn display_cities() {
    use std::fmt::{self, Display, Formatter};

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            write!(
                f,
                "{}: {:.3} degrees {} {:.3} degrees {}",
                self.name,
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            )
        }
    }

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }
}

fn display_colors() {
    use std::fmt::{self, Display, Formatter};

    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            //let color_mask: u32 = self.red*65536 | self.green*256 | self.blue; //only works if colors are u32

            //write!(f, "RGB ({}, {}, {}) 0x{:X}", self.red, self.green, self.blue, color_mask)
            write!(
                f,
                "RGB ({red}, {green}, {blue}) 0x{red:>2X}{green:>2X}{blue:>2X}",
                red = self.red,
                green = self.green,
                blue = self.blue
            )
        }
    }

    let color = Color {
        red: 128,
        green: 255,
        blue: 90,
    };
    println!("{}", color);
}
