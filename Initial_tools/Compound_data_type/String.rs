

fn main()
{
    let mut stone_cold: String = String::from("Hell, ");

    println!("Stone cold says: {}", stone_cold);

    stone_cold.push_str("Yeah");
    println!("Stone cold says: {}", stone_cold);

    stone_cold.push(' ');
    stone_cold.push('.');

    println!("Stone cold says: {}", stone_cold);

    stone_cold.pop();
    stone_cold.pop();
    println!("Stone cold says: {}", stone_cold);

    //inspecting

    println!("Length of the string is: {}", stone_cold.len());

    println!("Emptinesss check for stone cold: {}", stone_cold.is_empty());

    println!("Does the string contain Yeah: {}", stone_cold.contains("Yeah"));

    println!(" Using split() {:?}", stone_cold.split(","));
}