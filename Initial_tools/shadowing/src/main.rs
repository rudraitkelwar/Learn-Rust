fn main() {
    println!("Shadowing in Rust");

    let x = 5;
    println!("Current value of x :{}", x);
    let x = x + 1;
    println!("The updated value of x: {}", x);


    let spaces = "    ";
    println!("current the variable is -->{}|||", spaces);
    let spaces = spaces.len();
    // THIS IS SHADOWING 
    println!("new variable is -->{}|||", spaces);

}
