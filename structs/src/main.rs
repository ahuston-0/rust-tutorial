// Chapter 5 of The Rust Programming Language

fn main() {
    create_users();
    tuple_structs();
    rectangles();
}

// struct defining a User
struct User {
    username: String, // Originally was a String, but why not str?
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn create_users() {
    let mut user1 = User {
        // mut structs are entirely mutable
        username: String::from("username12345"),
        email: String::from("example@domain.net"),
        sign_in_count: 1,
        active: true,
    };

    user1.sign_in_count += 1;

    println!(
        "Look at the example user! [{}, {}, {}, {}]",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );

    let user2 = build_user(
        String::from("example@example.com"),
        String::from("ahuston-0"),
    );

    println!("User 2's username is {}", user2.username);

    let user3 = User {
        username: String::from("turtle"),
        email: String::from("turtle@turtlestore.com"),
        ..user2
    };
    println!("User 3's username is {}", user3.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        username, // shorthand allows fn params w/ same name as struct params to be added like this
        email,
        sign_in_count: 1,
        active: true,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct UnitStruct();

fn tuple_structs() {
    let black = Color(0, 0, 0); // Very similar styles of constructor
    let origin = Point(0, 0, 0); // Contain same types, different structs though

    // Tuple structs behave like normal tuples, down to the access methods
    if black.0 == origin.0 {
        println!("Color's red value is equal to Points x value");
    }

    // Unit structs may be useful for replicating
    // behavior similar to the unit type
    let _unit = UnitStruct();
}

fn rectangles() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels",
        area_nums(width1, height1)
    );

    let rect2 = (20, 40);
    println!(
        "The area of the rectangle is {} square pixels",
        area_tuple(rect2)
    );

    let rect3 = Rectangle {
        width: 60,
        height: 40,
    };
    let rect4 = Rectangle::square(10);

    println!(
        "The area of the rectangle is {} square pixels",
        area_struct(&rect3)
    );

    println!("The rectangle from before is {:?}", rect3);

    println!(
        "The area of the rectangle is {} square pixels",
        rect3.area()
    );

    println!(
        "True or False? rectangle 3 can hold rectangle 4? {}",
        rect3.can_hold(&rect4)
    );
}

fn area_nums(width: u32, height: u32) -> u64 {
    let w64: u64 = width.into(); // .into seems to autocast
    let h64: u64 = height.into();
    w64 * h64
}

fn area_tuple(dimensions: (u32, u32)) -> u64 {
    let w64: u64 = dimensions.0.into();
    let h64: u64 = dimensions.1.into();
    w64 * h64
}

#[derive(Debug)] // allows us to display structs with {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

// Implement functions for Rectangle struct
// More than one of these impl blocks are allowed
impl Rectangle {
    fn area(&self) -> u64 {
        let w64: u64 = self.width.into();
        let h64: u64 = self.height.into();
        w64 * h64
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width <= self.width && other.height <= self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area_struct(dimensions: &Rectangle) -> u64 {
    let w64: u64 = dimensions.width.into();
    let h64: u64 = dimensions.height.into();
    w64 * h64
}
