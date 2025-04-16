fn return_tuple() -> (f32, f32) {
    return (23.43, 3.12);
}



fn main() {
    let (_x, _y) = return_tuple();

    if _y > 5.0 {
        println!("{}", "> 5")
    } else if _y < 5.0 {
        println!("{}", "< 5")
    } else {
        println!("{}", "= 5")
    }


}