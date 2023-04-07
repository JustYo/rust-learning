use std::fmt::{self, Display, Formatter};

//Tuples
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Display: ")?;
        writeln!(f, "( {} {} )",self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

#[derive(Debug)]
struct Transpose(Matrix);

impl Display for Transpose {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Transpose: ")?;
        writeln!(f, "( {} {} )", self.0.0, self.0.2)?;
        write!(f, "( {} {} )", self.0.1, self.0.3)
    }
}

fn main() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1 - 2);
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    println!("true AND false is {}", true && false);
    println!("true OF false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);

    println!("One million is written as {}", 1_000_000u32);

    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);

    println!("Uhe reversed pair is {:?}", reverse(pair));
    let matrix1 = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("{}", matrix1);
    println!("Debug :{:?}", matrix1);

    println!("{}", Transpose(matrix1));
    // println!("Debug :{:?}", Transpose(matrix1))?;
}