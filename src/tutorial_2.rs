fn main() {
    println!("----------     ----------     ----------     ----------     ----------");
    println!("VARIABLE AND DATATYPE ASSIGNMENT:");
    let x      = 4; // Implicit type assignment {assigned as an i64}
    let y: u32 = 5; // Explicit type assignment {assigned as an u32, 
                    // an unsigned 32-bit integer}
    println!("`let x = 4;`:");
    println!("x = {} is an implicit i64, a 64-bit integer.", x);
    println!();
    println!("`let y: u32 = 5;`:");
    println!("y = {} is an explicit u32, a 32-bit unsigned integer.",y);
    println!();
    println!("----------     ----------     ----------     ----------     ----------");
    println!("MUTABLE VS IMUTABLE:");
    let     a = 5;
    let mut b = 6;
    println!("b = {}",b);
    b = 7;
    println!("b = {},",b);
}