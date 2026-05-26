fn main()
{
    // For the string below owner is s1 and we will just pass the reference of s1
    let s1: String = "Rust coding with Rudra".to_string();

    println!("First string is :{} and length of string is: {} \n", s1, get_len(&s1));

    //now all the variable will have one owner

    let s2 = s1;

    println!("if i print s1 we will get an error \n");

    println!("Now s2: {};and length of s2:{}", s2, get_len(&s2));

} //The Scope of s1 and s2 finishes after this to the string(variable) will also be dropped after this and both s1 and s2 does not exist after this.


fn get_len(s1 : &String) -> i64
{
    return s1.len() as i64;
}
