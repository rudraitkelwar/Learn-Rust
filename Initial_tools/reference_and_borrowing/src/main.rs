fn main()
{
    let mut x: i32 = 5;
    
    // value of x is borrowed by y here and x does not control the space anymore
    let  y: &mut i32 = &mut x;       
    
    println!("value of y={} \n", *y);
    *y += 10;

    // now here the scope of y finishes and then the value is passed back to x so we can see the changes in x

    println!("Value of x = {}", x);
    //println!("Value of y = {} this is the reference to x", y);

    println!(" \n -------------------************------------------\n");

    let mut a : BankAccount = BankAccount{owner: "Rudra".to_string(), balance : 1500.0};

    a.check_balance();

    a.withdraw(100.0);

    a.check_balance();
    
}


struct BankAccount
{
    owner: String,
    balance: f64,
}


impl BankAccount
{
    fn withdraw(&mut self, amount: f64)
    {
        println!("taking money from {} which currently has balance of {} and will take {} \n", self.owner, self.balance, amount);

        self.balance -= amount;

        println!("Current balance in acccount of {} is {} \n", self.owner, self.balance);
    }

    fn check_balance(&self)
    {
        println!("Current balance of {}'s account is: {} \n", self.owner, self.balance);
    }
}