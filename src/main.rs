mod main_test;

use std::io;

#[derive(PartialEq)]
#[derive(Debug)]
enum Status {
    Studying,
    Expelled,
}

struct Student {
    status: Status,
    name: String,
    group: String,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();
    loop {
        let choice = read_user_input_number("Выберите действие (1 - добавить студента, 2 - изменить ФИО студента, 3 - перевести студента в другую группу, 4 - отчислить студента, 5 - показать список студентов, 0 - завершить):");
        println!("-----");
        match choice {
            0 => {
                println!("Программа завершена.");
                break;
            }
            1 => {
                let name = read_user_input("Введите ФИО студента:");
                let group = read_user_input("Ввеите номер группы стулента:");
                add_student(&mut students, name, group, Status::Studying);
            }
            2 => {
                let id = read_user_input_number_with_show_students("Выберите студента:", &students);
                if let Some(_student) = choose_student(&students, id) {
                    let new_name = read_user_input("Введите новое имя студента:");
                    modify_student_name(&mut students, id, new_name);
                } else {
                    println!("Ошибка! Студент не найден.");
                }
            }
            3 => {
                let id = read_user_input_number_with_show_students("Выберите студента:", &students);
                if let Some(_student) = choose_student(&students, id) {
                    let new_group = read_user_input("Введите новую группу студента:");
                    modify_student_group(&mut students, id, new_group);
                } else {
                    println!("Ошибка! Студент не найден.");
                }
            }
            4 => {
                let id = read_user_input_number_with_show_students("Выберите студента:", &students);
                if let Some(_student) = choose_student(&students, id) {
                    kick_student(&mut students, id);
                } else {
                    println!("Ошибка! Студент не найден.");
                }
            }
            5 => {
                show_student(&students);
            }
            _ => println!("Некорректный ввод. Повторите попытку."),
        }
    }
}

fn read_user_input_number_with_show_students(prompt: &str, students: &[Student]) -> usize {
    loop {
        println!("{}", prompt);
        show_student(students);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения строки");
        input.trim().to_string();
        match input.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("Ошибка! Введите корректное значение."),
        }
    }
}

fn read_user_input_number(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения строки");
        match input.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("Ошибка! Введите корректное значение."),
        }
    }
}

fn read_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения строки");
    input.trim().to_string()
}

fn kick_student(students: &mut Vec<Student>, selected_student: usize) {
    if let Some(student) = students.get_mut(selected_student - 1) {
        student.status = Status::Expelled;
        println!("Студент отчислен");
    } else {
        println!("Ошибка! Студент не найден.");
    }
}

fn modify_student_group(students: &mut Vec<Student>, selected_student: usize, new_group: String) {
    if let Some(student) = students.get_mut(selected_student - 1) {
        student.group = new_group.clone();
        println!("Группа изменена: {}", new_group);
    } else {
        println!("Ошибка! Студент не найден.");
    }
}

fn modify_student_name(students: &mut Vec<Student>, selected_student: usize, new_name: String) {
    if let Some(student) = students.get_mut(selected_student - 1) {
        student.name = new_name.clone();
        println!("Имя изменено: {}", new_name);
    } else {
        println!("Ошибка! Студент не найден.");
    }
}

fn add_student(students: &mut Vec<Student>, name: String, group: String, status: Status) {
    let new_student = Student { status, name: name.clone(), group: group.clone() };
    students.push(new_student);
    println!("Добавлен \"{}, {}\"", name, group);
}

fn choose_student(students: &[Student], id: usize) -> Option<&Student> {
    if id > 0 && id <= students.len() {
        Some(&students[id - 1])
    } else {
        None
    }
}

fn show_student(students: &[Student]) {
    println!("Список студентов:");
    for (i, student) in students.iter().enumerate() {
        let status_str = match student.status {
            Status::Studying => "учится",
            Status::Expelled => "отчислен",
        };
        println!("{}. {}, {}, {}", i + 1, student.name, student.group, status_str);
    }
}
