fn main()
{
    println!(" Can Rudra drive? \n");
    println!("{}", age_check(27));

    println!(" Can __ drive? \n");
    println!("{}", age_check(101));

    println!(" Can XXX drive? \n");
    println!("{}", age_check(17));
    
 
    println!(" Can baby drive? \n");
    println!("{}", age_check(1));
        
}


fn age_check(age: u64) -> bool
{ 
    if(age >= 18 && age < 99)
    {
        println!("Big enough to drive");
        return true;
    }
    else if(age >= 16 && age < 99
    )
    {
        println!("You can only drive Mopid");
        return true;
    } 
    else if(age > 99)
    {
        println!("You are probably dead if not please dont drive");
        return false;
    }
    else
    {
        println!("You cannot drive go buy a cycle");
    }
    return false;
}