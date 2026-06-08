fn main()
{
    let employee1: Employee = Employee{
        detail: Details{
            id: 101,
            name: "Rudra".to_string(),
        },
        salary: 100000.5,
        manager: "XYZ".to_string(),
        
    };
    println!("Details of employee 1: name->{}, id->{}, salary->{}, manager->{}", employee1.detail.id, employee1.detail.name, employee1.salary, employee1.manager);


    let employee2 = Employee::new(102, "ABC".to_string(), 100000.0, "MMM".to_string());
    println!("Details of employee 1: name->{}, id->{}, salary->{}, manager->{}", employee2.detail.id, employee2.detail.name, employee2.salary, employee2.manager);
}



impl Employee {
    fn new(id: u32, name: String, salary: f64, manager: String) -> Employee
    {
        Employee
        {
            detail: Details { id, name },
            salary,
            manager,
        }
    }
  }




struct Employee
{
    detail: Details,
    salary: f64,
    manager: String,
}
struct Details
{
    id: u32,
    name: String,
}
