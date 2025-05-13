// this program collects and prints results of the survey

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
    q4: Option<String>,
}


fn main() {

    let response = Survey {
        q1: Some(18),
        q2: Some(true),
        q3: Some("A".to_owned()),
        q4: None,

    };

    match response.q1 {
        Some(answer) => println!("q1 answer is: {:?}", answer),
        None => println!("no answer was provided for q1")

    }

    match response.q2 {
        Some(answer) => println!("q2 answer is: {:?}", answer),
        None => println!("no answer was provided for q2")

    }

    match response.q3 {
        Some(answer) => println!("q3 answer is: {:?}", answer),
        None => println!("no answer was provided for q3")

    }

    match response.q4 {
        Some(answer) => println!("q4 answer is: {:?}", answer),
        None => println!("no answer was provided for q4")

    }
}