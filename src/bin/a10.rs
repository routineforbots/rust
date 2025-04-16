fn print_msg (gt_100: bool) {
    match gt_100 {
        true => println!("its big"),
        _ => println!("its small")
    }

}


fn main() {

    let input_variable = 1;

    let is_gt_100 = input_variable > 100;

    print_msg(is_gt_100);

}