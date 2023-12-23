#![allow(unused)]
#[derive(Debug)]
struct Employee  {
    first_name: String,
    last_name: String,
    dob: String,
    dept: String,
    mobile: String,
    email: String,
}

impl Employee {
    fn new(first_name: &str, last_name: &str,
            dob: &str, dept: &str, mobile: &str,
            email: &str) -> Employee 
            {
                Employee {
                    first_name: first_name.to_string(),
                    last_name: last_name.to_string(),
                    dob: dob.to_string(),
                    dept: dept.to_string(),
                    mobile: mobile.to_string(),
                    email: email.to_string(),
                }
            }

    fn add (&self) {
        println!
        ("The new employee {}, {} is added", 
        self.first_name,self.last_name);
    }        

    fn display_employee (&self) {
        println!("The newly added employee details are
                  Name: {}, {}
                  Date of Birth: {}
                  Department: {}
                  Mobile: {}
                  Email: {}",
                  self.first_name,self.last_name,
                  self.dob, self.dept,
                  self.mobile,self.email);
    }
}
fn main() {

    let emp_1 = Employee::new(&String::from("Samba"),
                               &String::from("Krish"),
                               &String::from("10-Jul-1973"),
                               &String::from("IT"),
                               &String::from("9999999"), 
                               &String::from("samba@gmail.com"));

    emp_1.add();
    emp_1.display_employee();   

    let emp_2 = Employee::new("Raghu",
                               "Krishnamurthy",
                               "01-01-1985",
                               "Sales",
                               "111111", 
                               "raghu.k@gmail.com");

    emp_2.add();
    emp_2.display_employee();                           

}
