

#[derive(Debug)]
enum Name{
    Car(String, u32),
    Human(String),
}

fn main() 
{

    let bmw: Name = Name::Car("X".to_string(), 1);
    let rudra: Name = Name::Human("Rudra Itkelwar".to_string());
    
    //println!("BMW - {bmw} -------- {rudra}");
    println!("BMW - {:?} -------- {:?}", bmw, rudra);

}
