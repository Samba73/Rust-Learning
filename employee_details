#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_mut)]
//use std::{io,collections}::HashMap;
use std::io;
use std::collections::HashMap;
fn main() {

let mut company: HashMap<String,Vec<String>> = HashMap::new();

//company.insert(String::from("Sales"),vec![String::from("Pam"),String::from("Tom")]);
//company.insert(String::from("IT"),vec![String::from("Samba"),String::from("Parta")]);

println!("The Company details are {:#?}",company);

println!("Enter the choice (e.g Add Sally to Sales, All Tom to IT, List all in Sales, List all in company etc)");

let mut  input = String::new();

io::stdin().read_line(&mut input).expect("Unable to read input");

//println!("The input is {}", input);

let input_split: Vec<&str> = input.trim().split_whitespace().collect();

//println!("The vector of string split is {:#?}", input_split);

let action      = input_split[0];
let employee    = input_split[1];
let dept        = input_split[3];

//println!("The vector split are {action}, {employee} & {dept}");

if action == "Add" {
    if !company.contains_key(dept){
        company.insert(String::from(dept),vec![String::from(employee)]);
    } else {
        let dept_emp = company.entry(dept.to_string()).or_insert(vec![]);
        if !dept_emp.contains(&employee.to_string()) {
            dept_emp.push(employee.to_string());
        }
        
    }  
    println!("The Company details are {:#?}",company);  
} else if action == "List"  {
    if dept != "Company" {
        if !company.contains_key(dept) {
            println!("The input is incorrect. No such department exists");
            //break;
        } else{
            for (k,v) in &company {
                let mut employee_list = Vec::new();
                if &k == &dept {
                    
                    employee_list.push(&v);
                }
            println!("The list of employees in the department {k} are {:#?}", employee_list);    
            break;
            }
        }
    } else {
        let mut all_employee_list = Vec::new();
        for (_,v) in &company {
                all_employee_list.push(v);
        }
        println!("The list of all employees are {:#?}", all_employee_list); 
    }   
}

//println!("The Company details are {:#?}",company);

}
