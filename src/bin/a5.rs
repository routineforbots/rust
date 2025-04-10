fn main() {

    let mut my_var = 1;

    loop {
        println!("{:?}", my_var.to_string());

        if my_var == 4 {
            break;
        }

        my_var += 1;

    }


}