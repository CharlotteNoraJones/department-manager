use regex::Regex;
use std::collections::HashMap;
use std::io;

fn main() {
    let prompt: &str = r#"
    Enter "Add [Employee Name] to [Department]" to add an employee,
    "Display" to list all employees, "Display [Department]" to display 
    all employees associated with a particular department,
    or "Exit" to exit.
    "#;

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    let add_employee_regex = Regex::new(r"^add (?<name>\w+) to (?<department>\w+)$").unwrap();

    // let add_employee_regex = Regex::new(r"^add (\w+) to (\w+)$").unwrap();

    let display_department_regex = Regex::new(r"display (?<department>\w+)").unwrap();

    loop {
        println!("{}", prompt);
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match input.to_lowercase().as_str().trim() {
            i if add_employee_regex.is_match(i) => {
                let captures = add_employee_regex.captures(i).unwrap();
                add_employee(&mut company, &captures["name"], &captures["department"]);
            }
            i if display_department_regex.is_match(i) => display_department(
                &company,
                &display_department_regex.captures(i).unwrap()["department"],
            ),
            "display" => display_all(&company),
            "exit" => break,
            _ => continue,
        }
    }
}

fn add_employee(company: &mut HashMap<String, Vec<String>>, name: &str, department: &str) {
    if company.contains_key(department) {
        company
            .get_mut(department)
            .unwrap()
            .push(String::from(name));
    } else {
        company.insert(String::from(department), vec![String::from(name)]);
    }
}

fn display_all(company: &HashMap<String, Vec<String>>) {
    for department in company.keys() {
        display_department(company, department.as_str());
    }
}

fn display_department(company: &HashMap<String, Vec<String>>, department: &str) {
    println!("{} Department", department);
    let mut names: Vec<String> = company.get(&String::from(department)).unwrap().clone();
    names.sort();
    for name in names {
        println!("{}", name);
    }
}
