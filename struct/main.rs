#![allow(unused)]

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, Read, Write};


#[derive(Default, Debug)]
struct EmployeeDetails {
    first_name: String,
    last_name: String,
    dob: String,
    dept: String,
    mobile: String,
    email: String,
}

fn read_employee_details() -> io::Result<EmployeeDetails>
    {
        let file = OpenOptions::new()
                    .read(true)
                    .open("employee.csv");
        match file {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;


                let mut details = EmployeeDetails::default();

                let fields: Vec<&str> = contents.split_whitespace().collect();

                details.first_name = fields[0].to_string();
                details.last_name = fields[1].to_string();
                details.dob = fields[2].to_string();
                details.dept = fields[3].to_string();
                details.mobile = fields[4].to_string();
                details.email = fields[5].to_string();

                Ok(details)
            }
            Err(error) => Err(error),
        }            
    }                            

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

   fn write_employee_details(&self) {

        let emp_str = format!(
                     "{} {} {} {} {} {}\n",
                    self.first_name, self.last_name, self.dob, 
                    self.dept, self.mobile, self.email
        );

        let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .append(true)
                    .create(true)
                    .open("employee.csv");
        match file {
            Ok(mut file) => {
                file.write_all(emp_str.as_bytes()).expect("Failed to write");
            }
            _ => {
                println!("Failed to write in the file");
            }
        }            
    }       

    fn display_employee (&self) {
        let emp = read_employee_details();
       //println!("display emp {:#?}",emp);

       match emp {
        Ok(employeeeetails) => {                   
            println!("The employee details are:
                        Name: {}, {}
                        Date of Birth: {}
                        Department: {}
                        Mobile: {}
                        Email: {}", 
                        employeeeetails.first_name, 
                        employeeeetails.last_name,
                        employeeeetails.dob, 
                        employeeeetails.dept, 
                        employeeeetails.mobile,
                        employeeeetails.email)
        }
       
       Err(_) => {
        println!("Error Occured");
       }
       } 
    }                
}
fn main() {

    let emp_1 = Employee::new(&String::from("Samba"),
                               &String::from("Krish"),
                               &String::from("10-Jul-1973"),
                               &String::from("IT"),
                               &String::from("9999999"), 
                               &String::from("samba@gmail.com"));

    emp_1.write_employee_details();
    emp_1.display_employee();   


    let emp_2 = Employee::new("Raghu",
                               "Krishnamurthy",
                               "01-01-1985",
                               "Sales",
                               "111111", 
                               "raghu.k@gmail.com");

    emp_2.write_employee_details();
    emp_2.display_employee();   
                    

}
