use std::io;
use std::io::Write;
use std::collections::HashMap;

type Departments = HashMap<String, Vec<String>>;

const PROMPT_MSG: &'static str = "[ What do you want to do? (Add <Person> to <Department>, List all, List in <Department>) ]";
const READLINE_SYMBOL: &'static str = "$ ";

fn get_input() -> Vec<String> {
  println!("");
  println!("{}", PROMPT_MSG);

  let mut input = String::new();

  // Prints out the readline symbol ($)
  print!("{}", READLINE_SYMBOL);
  io::stdout().flush().unwrap();

  io::stdin().read_line(&mut input).expect("Cannot read line");

  println!("");

  // Split the input into lists of words
  input
    .trim()
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect()
}

// Adds the proper conjunction for a specific index
fn get_list_conjunction(name: &String, index: usize, count: usize) -> String {
  match (index, count) {
    (i, _) if i == 0 => format!("{}", name),
    (i, len) if i == len - 1 => format!(" and {}", name),
    _ => format!(", {}", name),
  }
}

// The syntax hint, using the person name for context.
fn syntax_hint_add(person: &String) {
  println!("> The command must be 'Add {} to <Department>'.", person);
}

// Give syntax tips if some arguments are't present.
fn safe_get_add_args(words: Vec<String>) -> Option<(String, String)> {
  let person = match words.get(1) {
    Some(word) => word,
    _ => {
      // If the user forgot the person's name
      // Use the <Person> placeholder for syntax hint.
      syntax_hint_add(&String::from("<Person>"));
      return None
    }
  };

  match words.get(2) {
    Some(word) if word == "to" => word,
    _ => {
      // If the user forgot to use "to"
      println!("> Please use 'to' between the Person and Department name.");
      syntax_hint_add(person);
      return None
    }
  };

  let dept_name = match words.get(3) {
    Some(word) => word,
    _ => {
      // If the user forgot the department name
      println!("> Where to add {}? Missing Department Name.", person);
      syntax_hint_add(person);
      return None
    }
  };

  return Some((person.clone(), dept_name.clone()))
}

fn list_employee(employee: &Vec<String>, name: &String) {
  let mut employee_names = String::new();
  let count = employee.len();

  // Alphabetically sort the employee names
  let mut employee = employee.clone();
  employee.sort();

  // Adds each person to the employee list
  for (index, name) in employee.iter().enumerate() {
    let text = get_list_conjunction(name, index, count);
    employee_names.push_str(&text);
  }

  // Prints out the list
  println!("  {} is in the Department of {}.", employee_names, name);
}

fn list_everyone(departments: &Departments) {
  println!("> Here are the list of people in each departments:");

  // List all the departments
  for (name, people) in departments {
    list_employee(people, name);
  }
}

fn list_by_dept(departments: &Departments, dept_name: &String) {
  match departments.get(dept_name) {
    Some(people) => list_employee(people, dept_name),
    None => {
      println!("Department of {} does not exist.", dept_name);
      return
    }
  };
}

fn add_person(departments: &mut Departments, person: String, dept_name: String) {
  // Adds the person to the department
  let department = departments.entry(dept_name.clone()).or_insert(vec![]);
  department.push(person.clone());

  println!("> Added {} to {}.", person, dept_name);
}

fn main() {
  println!("===== Employee List CLI =====");

  let mut departments = HashMap::new();

  loop {
    // Retrieve the user input
    let words = get_input();

    // If the user enters nothing and submit, jump to beginning
    let command = match words.get(0) {
      Some(t) => t.clone(),
      None => continue
    };

    // Matches the first word against the commands, add and list
    match command.to_lowercase().as_ref() {
      "add" => {
        // Retrieve the name of the person and the department
        let (person, dept_name) = match safe_get_add_args(words) {
          Some(t) => (t),
          _ => continue
        };

        add_person(&mut departments, person, dept_name)
      }
      "list" => {
        match words.get(1) {
          Some(t) if t == "in" => {
            let dept_name = match words.get(2) {
              Some(t) => t,
              None => {
                println!("Missing Department Name. Syntax: 'List in <Department>'.");
                continue;
              }
            };

            list_by_dept(&departments, dept_name);
          },
          Some(t) if t == "all" => list_everyone(&departments),
          None => list_everyone(&departments),
          _ => {
            println!("Use 'List all' to List Everyone, or 'List in <Department>' to List by Department.");
            continue;
          }
        }
      },
      _ => {
        println!("> Command not found!");
        continue;
      }
    }
  }
}
