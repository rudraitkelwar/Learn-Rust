fn main()
{
    let mut v: Vec<i32> = vec![-2, -1, 0];

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);


    println!(" First iteration of vectors is {:?}", v);


    v.pop();

    v.remove(4);

    println!(" Second iteration of vectors is {:?}", v);
    v.push(6);
    println!(" Third iteration of vectors is {:?}", v);
    v.insert(4, 2);
    println!(" Fourth iteration of vectors is {:?}", v);

    println!("First element of v -> {:?}", v.first());
    println!("Last element of v -> {:?}", v.last());
    println!("5th element of v -> {}", v[5]);


    println!("the length of vector -> {}", v.len());
    
    

}
