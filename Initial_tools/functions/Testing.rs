

fn boys(name: &str, age: i32, score: u64)
{
    println!("Name: {} -- Age: {} -- Score: {}", name, age, score);
}


fn cost_cal(quantity : i64, cost: i64) -> i64
{
    return cost * quantity;
} 


fn main()
{
    boys("Rudra", 27, 100);
    boys("Messi", 38, 100);
    boys("Ronaldo", 40, 98);
    boys("Neymar", 35, 94);

    println!("Total cost: {}",cost_cal(96, 10265));

}