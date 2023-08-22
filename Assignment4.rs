struct Student{
    name:String,
    email:String,
    phone:String,
    id:u32
}

fn main(){
    let mut students:Vec<Student>=Vec::new();

    students.push(Student{
        name:String::from("John"),
        email:String::from("john@example.com"),
        phone:String::from("111-111-1111"),
        id:1,
    });

    students.push(Student{
        name:String::from("Sam"),
        email:String::from("sam@example.com"),
        phone:String::from("222-222-2222"),
        id:2,
    });
    students.push(Student{
        name:String::from("Ram"),
        email:String::from("ram@example.com"),
        phone:String::from("333-333-3333"),
        id:3,
    });
    students.push(Student{
        name:String::from("Shyam"),
        email:String::from("shyam@example.com"),
        phone:String::from("444-444-4444"),
        id:4,
});
    students.push(Student{
        name:String::from("Karan"),
        email:String::from("karan@example.com"),
        phone:String::from("555-555-5555"),
        id:5,
});

let index_to_access=2;

match students.get(index_to_access){
    Some(student)=>{
        println!("Student Details:");
        println!("Name: {}", student.name);
        println!("Email: {}", student.email);
        println!("Phone: {}", student.phone);
        println!("ID: {}", student.id);
    }
    None => {
        eprintln!("Error: Student not found at index {}", index_to_access);
}

}

}
