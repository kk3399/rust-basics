struct Student {
    id: i32,
    name: String,
}

impl Student {
    fn new(id: i32, name: &str) -> Student {
        Student {
            id: id,
            name: name.to_string(),
        }
    }

    fn get_name(&self) -> String {
        format!("{}", self.name)
    }

    fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

//tupe struct
struct Employee<'emp>(i32, &'emp str);

impl std::fmt::Display for Employee<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Employee {{ id: {}, name: {} }}", self.0, self.1)
    }
}

pub fn run() {
    let student = Student {
        id: 1,
        name: String::from("rust"),
    };
    println!("id = {}, name = {}", student.id, student.name);

    let emp = Employee(1, "rust");
    println!("{}", emp);

    let mut student2 = Student::new(2, "joe");
    println!("{}", student2.get_name());
    student2.set_name("jim");
    println!("{}", student2.get_name());
}
