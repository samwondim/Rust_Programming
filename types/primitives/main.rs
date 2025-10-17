use std::fmt;

//Tuples
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn transpose(matrix: &Matrix) -> Matrix {
    Matrix(matrix.2, matrix.0,matrix.3, matrix.1)
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", &self.0, &self.1, &self.2, &self.3)
    }
}


fn analyze_slice(slice: &[i32]) {
    println!("First Element of the array is {}.", slice[0]);
    println!("Second Element of the array is {}.", slice[1]);
}

fn main() {
    let other_number = 19i32;

    println!("{}", other_number);

    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", nums);

    // Tuples
    let matrix = Matrix(1.2, 2.3, 4.5, 10.0);

    println!("Debug: {:#?}", matrix);
    println!("Display: \n{}", matrix);

    println!("Transposed:\n{}", transpose(&matrix));

    // Arrays and Slices
    let a_1: [i32; 4] = [4, 5, 1, 2];
    let sliced = &a_1[0..3];
    analyze_slice(&sliced);

    for i in 0..a_1.len() + 1 {
        match a_1.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is to far", i),
        }
    }

}

