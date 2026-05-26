fn main()
{
    let a: i32 = 15;

    const X: i64 = 10001;

    println!("a is a immutable variable: a = {}",a);

    let mut b: i32 = 15;

    println!("b is a mutable variable and added key word mut while defining it: b = {}", b);

    b += 10;

    println!("updated values of b = {} -- and we can do this because b is mutable", b);

    println!("Just printing const's here :- \nBEST_NUM = {}, \nX = {}", BEST_NUM, X);
}

const BEST_NUM: i64 = 69;
