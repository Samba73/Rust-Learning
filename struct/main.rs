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
fn read_employee_details() 
    {
        let file = OpenOptions::new()
                    .read(true)
                    .open("employee.csv");
        match file {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents);
                let mut details = EmployeeDetails::default();
                let mut all_details: Vec<EmployeeDetails> = Vec::new();
                let mut field_count:u32 = 0;
                let mut field = String::new();
                //println!("The content is {:#?}", contents);
                for c in contents.chars() {
                    match c {
                        '\t' => {
                            match field_count {
                                0 => details.first_name = field.trim().to_string(),
                                1 => details.last_name = field.trim().to_string(),
                                2 => details.dob = field.trim().to_string(),
                                3 => details.dept = field.trim().to_string(),
                                4 => details.mobile = field.trim().to_string(),
                                _ => details.email = field.trim().to_string(),
                            }
                            field.clear();
                            field_count+=1;
                        }
                        '\n' => {
                            details.email = field.trim().to_string();
                            all_details.push(details);
                            details = EmployeeDetails::default();
                            field_count = 0;
                            field.clear();
                        }
                        _ => {
                            field.push(c);
                        }
                    }
                }
                    //println!("The vector is {:#?}", all_details);
                    for (index, employee) in all_details.iter().enumerate() {
                            println!("Employee {}:", index + 1);
                            println!("First Name: {}", employee.first_name);
                            println!("Last Name: {}", employee.last_name);
                            println!("Date of Birth: {}", employee.dob);
                            println!("Department: {}", employee.dept);
                            println!("Mobile: {}", employee.mobile);
                            println!("Email: {}", employee.email);
                            println!();
                        }

            }   
            Err(error) => (),         
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
                     "{}\t{}\t{}\t{}\t{}\t{}\n",
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

    fn display_employee () {
        read_employee_details();
       //println!("display emp {:#?}",emp);

    //    match emp {
    //     Ok(employeeeetails) => {                   
    //         println!("The employee details are:
    //                     Name: {}, {}
    //                     Date of Birth: {}
    //                     Department: {}
    //                     Mobile: {}
    //                     Email: {}", 
    //                     employeeeetails.first_name, 
    //                     employeeeetails.last_name,
    //                     employeeeetails.dob, 
    //                     employeeeetails.dept, 
    //                     employeeeetails.mobile,
    //                     employeeeetails.email)
    //     }
       
    //    Err(_) => {
    //     println!("Error Occured");
    //    }
       } 
    //}                
}
fn main() {

    let emp_1 = Employee::new(&String::from("Sambasivam"),
                               &String::from("Krishnamurthy"),
                               &String::from("10-Jul-1973"),
                               &String::from("IT"),
                               &String::from("9999999"), 
                               &String::from("samba@gmail.com"));

    emp_1.write_employee_details();
    //emp_1.display_employee();   


    let emp_2 = Employee::new("Raghu",
                               "Krishnamurthy",
                               "01-01-1985",
                               "Sales",
                               "111111", 
                               "raghu.k@gmail.com");

    emp_2.write_employee_details();
    // emp_2.display_employee();   

    Employee::display_employee();


    let f = OpenOptions::new().write(true).truncate(true).open("employee.csv");

    match f {
            Ok(mut file) => {
                println!("The file is truncated successfully {:#?}", file);
            }
            _ => {
                println!("Failed to write in the file");
            }
        }                          

}
