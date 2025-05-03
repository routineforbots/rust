#[derive(Debug, Clone, Copy)] // derive must be put right before the element - enum here
enum Position {
    Manager,
    Supervisor,
    Worker
}

#[derive(Debug, Clone, Copy)] // derive must be put right before the element - struct here
struct Employee {
    position: Position,
    work_hours: i64
}

fn print_employee(emp: Employee) { // notice that we don't Borrow here (no & sign)
    println!("{:?}", emp);
}


fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40

    };

    print_employee(me);
    print_employee(me); // we didn't borrow Employee but since we used Clone, Copy there will be no error - two copies of Employee were created

}