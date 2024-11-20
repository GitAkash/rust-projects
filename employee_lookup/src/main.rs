use std::collections::HashMap;

// Using a hash map and vectors,
// create a text interface to allow a user to add employee names
// to a department in a company. For example,
// “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.
pub enum Department {
    Engineering,
    Sales,
    Programming,
    CustomerService,
}

fn main() {
    let mut register: HashMap<String, Department> = HashMap::new();
}

pub fn assign(input: Vec<(String, Department)>) -> HashMap<String, Department> {
    let mut register: HashMap<(String, Department)> = vec![];
        match input {
            Department::Engineering => register(name.tou_string),
            Department::Programming => output.push(name.to_string()),
            Department::CustomerService => output.push(name.to_string()),
            Department::Sales => output.push(name.to_string()),
        }
    }
    output
}
// fn add_user() {
//     let mut map: HashMap<String, Department> = HashMap::new();
//     pub fn assign(input: Vec<(String, Department)>) -> Vec<String> {
//         let mut output: Vec<String> = vec![];
//         for (name, departement) in input.iter() {
//             match departement {
//                 Department::Engineering => map.insert(name, Department::Engineering),
//                 Department::Sales => map.insert(name, Department::Sales),
//                 Department::Programming => map.insert(name, Department::Programming),
//                 Department::CustomerService => map.insert(name, Department::CustomerService),
//             }
//         }
//         output
//     }
// }

