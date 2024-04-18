#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_add_student() {
        let mut students: Vec<Student> = Vec::new();
        add_student(&mut students, String::from("Иванов Иван"), String::from("Группа 1"), Status::Studying);
        assert_eq!(students.len(), 1);
        assert_eq!(students[0].name, "Иванов Иван");
        assert_eq!(students[0].group, "Группа 1");
        assert_eq!(students[0].status, Status::Studying);
    }

    #[test]
    fn test_modify_student_name() {
        let mut students = vec![Student {
            status: Status::Studying,
            name: String::from("Петров Петр"),
            group: String::from("Группа 2"),
        }];
        modify_student_name(&mut students, 1, String::from("Новый Имя"));
        assert_eq!(students[0].name, "Новый Имя");
    }

    #[test]
    fn test_modify_student_group() {
        let mut students = vec![Student {
            status: Status::Studying,
            name: String::from("Петров Петр"),
            group: String::from("Группа 2"),
        }];
        modify_student_group(&mut students, 1, String::from("Новая Группа"));
        assert_eq!(students[0].group, "Новая Группа");
    }

    #[test]
    fn test_kick_student() {
        let mut students = vec![
            Student {
                status: Status::Studying,
                name: String::from("Петров Петр"),
                group: String::from("Группа 2"),
            },
            Student {
                status: Status::Studying,
                name: String::from("Иванов Иван"),
                group: String::from("Группа 1"),
            },
        ];
        kick_student(&mut students, 2);
        assert_eq!(students[1].status, Status::Expelled);
    }

    #[test]
    fn test_choose_student() {
        let students = vec![
            Student {
                status: Status::Studying,
                name: String::from("Петров Петр"),
                group: String::from("Группа 2"),
            },
            Student {
                status: Status::Studying,
                name: String::from("Иванов Иван"),
                group: String::from("Группа 1"),
            },
        ];
        assert_eq!(choose_student(&students, 1).unwrap().name, "Петров Петр");
        assert_eq!(choose_student(&students, 2).unwrap().name, "Иванов Иван");
    }
}
