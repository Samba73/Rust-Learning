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
// this function need to be rewritten to read all the lines and construct struct for each line of record.
// currently it only read / fetches first line of record


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
                // following ideas can be implemented but this also involve expensive creation of collection. need better solution
               ////********* option 1
                //let mut lines = contents.lines();
                //while Some(line) = lines.next() {
                    //let fields: Vec<&str> = line.split_whitespace().collect();
                        //details.first_name = fields[0].to_string();
                        //details.last_name = fields[1].to_string();
                        //details.dob = fields[2].to_string();
                        //details.dept = fields[3].to_string();
                        //details.mobile = fields[4].to_string();
                        //details.email = fields[5].to_string();
                        // send back the details struct so it can take the next line, else the struct will be overwritten for last line?
                //}
                // in the above commented code, all the lines are read into emp_vec
                /////******** end option 1
                ////********* option 2

                        // let mut details = EmployeeDetails::default();
                        // let mut all_details: Vec<EmployeeDetails> = Vec::new();
                        // let mut field_count:u32 = 0;
                        // let mut field = String::new();
                        
                        // for c in contents.chars() {
                        //     match c {
                        //         '\t' => {
                        //             match field_count {
                        //                 0 => details.first_name = field.trim().to_string(),
                        //                 1 => details.last_name = field.trim().to_string(),
                        //                 2 => details.dob = field.trim().to_string(),
                        //                 3 => details.dept = field.trim().to_string(),
                        //                 4 => details.mobile = field.trim().to_string(),
                        //                 _ => details.email = field.trim().to_string(),
                        //             }
                        //             field.clear();
                        //             field_count+=1;
                        //         }
                        //         '\n' => {
                        //             details.email = field.trim().to_string();
                        //             all_details.push(details);
                        //             details = EmployeeDetails::default();
                        //             field_count = 0;
                        //             field.clear();
                        //         }
                        //         _ => {
                        //             field.push(c);
                        //         }
                        //     }
                        // }
                        // send all_details vector (this vector will be of type EmployeeDetails
                        // in this case the fn return will be a vector and no Result
                ////******* end option 2
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
