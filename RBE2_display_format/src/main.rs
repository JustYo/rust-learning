use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct Person<'a> {
    // Debug display
    name: &'a str,
    age: u8,
}

/// Let's play to display a write formatter with two lines
impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "This is {}'s presentation:", self.name)?;
        write!(f, "He is : {} years old.", self.age)
    }
}

struct Structure(i32); // pretty display for a structure 1

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64); // pretty display for a structure 2

// implement a formatter for the structure MinMax so it displays coordonates nicely
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

//Activity :
#[derive(Debug)]
struct Compl {
    rea: f64,
    img: f64,
}

// Display for good print of a complex formula
impl fmt::Display for Compl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.rea, self.img)
    }
}

// List display
struct List(Vec<i32>);

// Implement a formatter to display a list nicely with brackets
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

// different formattage de chiffre
struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}
/// Display the temperature in a city
impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}: {:.3}°{} {:.3}° {}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
///Display RGB values and the hexadecimal conversion
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:0>2X?}{:0>2X?}{:0>2X?}",
            self.red, self.green, self.blue, self.red, self.green, self.blue
        )
    }
}

fn main() {
    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    // Debug display of a structure
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    //No pretty
    println!("{:?}", peter);

    // Pretty print
    println!("{:#?}", peter);

    println!("{}", peter);

    // Normal display for a structure
    println!("{}", Structure(12));

    let minmax = MinMax(0, 14);

    println!("Compare Structures :");
    println!("Display : {}", minmax);
    println!("Debug : {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let imaginary = Compl { rea: 3.3, img: 7.2 };

    println!("Compare compl:");
    println!("Display: {}", imaginary);
    println!("Debug: {:?}", imaginary);

    let vecteur = List(vec![1, 2, 3]);
    println!("{}", vecteur);

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
        println!("{}", city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{}", color)
    }
}
