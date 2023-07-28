use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum GradeLevel {
    FirstGrade,
    SecondGrade,
    ThirdGrade,
    FinalGrade,
}

struct Student {
    name: String,
    age: u32,
    grade: GradeLevel,
    score: u8,
}

fn add_student(students: &mut Vec<Student>) {
    println!("Enter student name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Read error");
    let name = name.trim().to_string();

    println!("Enter student age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Read error");
    let age: u32 = age_input.trim().parse().expect("Invalid age");

    println!("Enter student grade level (1-4):");
    let mut grade_input = String::new();
    io::stdin().read_line(&mut grade_input).expect("Read error");
    let grade: u8 = grade_input.trim().parse().expect("Invalid grade level");

    println!("Enter student score:");
    let mut score_input = String::new();
    io::stdin().read_line(&mut score_input).expect("Read error");
    let score: u8 = score_input.trim().parse().expect("Invalid score");

    let grade = match grade {
        1 => GradeLevel::FirstGrade,
        2 => GradeLevel::SecondGrade,
        3 => GradeLevel::ThirdGrade,
        4 => GradeLevel::FinalGrade,
        _ => panic!("Invalid grade level"),
    };

    let student = Student {
        name,
        age,
        grade,
        score,
    };

    students.push(student);
}

fn display_students(students: &[Student]) {
    for student in students {
        println!(
            "Name: {}, Age: {}, Grade Level: {:?}, Score: {}",
            student.name, student.age, student.grade, student.score
        );
    }
}

fn get_average_grade(students: &[Student]) -> f64 {
    let total_scores: u32 = students.iter().map(|s| s.score as u32).sum();
    let total_students = students.len() as u32;
    f64::from(total_scores) / f64::from(total_students)
}

fn get_highest_grade(students: &[Student]) -> Option<&Student> {
    students.iter().max_by_key(|s| s.score)
}

fn search_student<'a>(students: &'a [Student], name: &str) -> Option<&'a Student> {
    students.iter().find(|s| s.name == name)
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    loop {
        println!("------ MENU ------");
        println!("1. Add a new student");
        println!("2. Display all students");
        println!("3. Calculate average grade");
        println!("4. Find student with the highest grade");
        println!("5. Search for a student");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Read error");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice, please try again.");
                continue;
            }
        };

        match choice {
            1 => add_student(&mut students),
            2 => display_students(&students),
            3 => println!("Average Grade: {:.2}", get_average_grade(&students)),
            4 => match get_highest_grade(&students) {
                Some(student) => println!(
                    "Student with the highest grade: {} (Grade Level: {:?}, Score: {})",
                    student.name, student.grade, student.score
                ),
                None => println!("No students recorded."),
            },
            5 => {
                println!("Enter student name to search:");
                let mut search_name = String::new();
                io::stdin().read_line(&mut search_name).expect("Read error");
                let search_name = search_name.trim();
                match search_student(&students, search_name) {
                    Some(student) => println!(
                        "Name: {}, Age: {}, Grade Level: {:?}, Score: {}",
                        student.name, student.age, student.grade, student.score
                    ),
                    None => println!("Student not found in records."),
                }
            }
            6 => {
                println!("Exiting the program...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}
