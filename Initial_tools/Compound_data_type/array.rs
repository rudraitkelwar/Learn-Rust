fn main()
{
    let mut numbers : [i32; 5] = [1,2,3,4,5];
    println!("the array: {:?}", numbers);

    numbers[2] = 6;
    println!("the array: {:?}", numbers);

    println!("First element is: {}", numbers[0]);

 
}