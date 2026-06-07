

fn main() {
    println!("Looping in rust");

    let result: i32 = 100;
    //println!("loop give the answe ==> {}",looping(result));
    //println!("while looping: \n {}", while_loopingn(result));

    let mut numbers : [i32; 5] = [1,2,3,4,5];
    
    for_mech(&numbers);
}

fn looping(mut num: i32) -> String
{
    let answer: String = loop{
        num -= 1;

        println!("current marks {}", num);

        if(num == 95)
        {
            break "we got 95 marks".to_string();
        }
    };

    return answer;
}
fn while_loopingn(mut num: i32) -> String
{
    while(num > 80)
    {
        if(num == 82)
        {
            let com = format!("we got {num} marks");
            println!("\n {com} \n ");
        }
        println!("curr --> {}", num);
        num -= 1;
    }
    return "Done".to_string();
}


fn for_mech(mut num: &[i32])
{
    for i in 0..num.len()
    {
        println!("On {i}th index element is {}", num[i]);
    }
}