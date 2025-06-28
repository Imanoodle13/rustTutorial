/* Variables, Constants, and Shadowing */
fn main(){
    let mut x = 4; // Implicit type assignment. Assigned as an integer at compile time.
    println!("x is : {}",x);

    {
        let x = 2;
        println!("x is : {}",x);
    }

    x += 2;
    println!("x is : {}",x);
    println!();

    // Complete reassignment of datatype and value is possible
    let y = 4;
    println!("y is : {}",y);
    let y = "Hello!";
    println!("y is : {}",y);
    println!();

    const MULTIPLIER: u32 = 7; // const is a stricter version of immutable variables.
    print!("Multiplier is : {}",MULTIPLIER);
}