use std::io;

//Student struct
struct Student {
    name: String,
    age: u32,
    grade: u8,
}

//GradeLevel enum
enum GradeLevel {
    Freshman,
    Sophomore,
    Junior,
    Senior,
}

//add_student function
fn add_student(students: &mut Vec<Student>) {
    println!("Enter student name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();

    println!("Enter student age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: u32 = age_input.trim().parse().expect("Please enter a valid number");

    println!("Enter student grade (0-100):");
    let mut grade_input = String::new();
    io::stdin().read_line(&mut grade_input).expect("Failed to read line");
    let grade: u8 = grade_input.trim().parse().expect("Please enter a valid number between 0-100");

    let student = Student { name, age, grade };
    students.push(student);
    println!("Student added successfully!");
}

//display_students function
fn display_students(students: &Vec<Student>) {
    for student in students {
        println!("Name: {}\nAge: {}\nGrade: {}\n", student.name, student.age, student.grade);
    }
}

//get_average_grade function
fn get_average_grade(students: &Vec<Student>) -> f64 {
    let total_grades: u64 = students.iter().map(|student| student.grade as u64).sum();
    let average = total_grades as f64 / students.len() as f64;
    average
}

//get_highest_grade function
fn get_highest_grade(students: &Vec<Student>) -> Option<&Student> {
    students.iter().max_by_key(|student| student.grade)
}

//search_student function
fn search_student<'a>(students: &'a Vec<Student>, name: &'a str) -> Option<&'a Student> {
    students.iter().find(|student| student.name == name)
}

fn main() {
    let mut students: Vec<Student> = Vec::new();
    
    loop {
        println!("\nMenu:");
        println!("1. Add a new student");
        println!("2. Display all students");
        println!("3. Calculate the average grade");
        println!("4. Find the student with the highest grade");
        println!("5. Search for a specific student");
        println!("6. Exit");

        let mut choice_input = String::new();
        io::stdin().read_line(&mut choice_input).expect("Failed to read line");
        let choice: u8 = match choice_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => add_student(&mut students),
            2 => display_students(&students),
            3 => println!("Average Grade: {:.2}", get_average_grade(&students)),
            4 => match get_highest_grade(&students) {
                Some(student) => println!("Student with the highest grade:\nName: {}\nAge: {}\nGrade: {}\n", student.name, student.age, student.grade),
                None => println!("No students found."),
            },
            5 => {
                println!("Enter the name of the student you want to search:");
                let mut search_name = String::new();
                io::stdin().read_line(&mut search_name).expect("Failed to read line");
                let search_name = search_name.trim();
                match search_student(&students, &search_name) {
                    Some(student) => println!("Student Found:\nName: {}\nAge: {}\nGrade: {}\n", student.name, student.age, student.grade),
                    None => println!("Student not found."),
                }
            }
            6 => break,
            _ => println!("Invalid choice. Please choose a valid option."),
        }
    }
}
