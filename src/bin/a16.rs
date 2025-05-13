#[derive(Debug)]
struct Locker {
    id: i32,
    assigned_to: Option<Students>,
}
#[derive(Debug)]
enum Students {
    Mark,
    Antony,
    Alex,
}


fn main() {
    let lockers = vec! [
        Locker{id: 1, assigned_to: Some(Students::Mark)},
        Locker{id: 3,assigned_to: Some(Students::Antony)},
        Locker{id: 2,assigned_to: Some(Students::Alex)},
        Locker{id: 4,assigned_to: None},
    ];

    for item in lockers {
        match item.assigned_to {
            Some(who) => println!("locker {:?} is assigned to {:?}", item.id, who),
            None => println!("locker {:?} is free", item.id),
        }
    }

}
