fn main() {
let d = sum_numbers(5, 3);

display_sum_result(d);

}

fn sum_numbers(a: i32, b: i32) -> i32 {

    return a + b;
}

fn display_sum_result(c: i32) {

    println!("{:?}", c);
    
}