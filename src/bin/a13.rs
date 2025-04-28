fn main() {
    
    let my_vector = vec![ 10, 20, 30, 40 ];
    
    // for loop gets the ownership of my_vector so we need to borrow it
    for num in &my_vector {
        /*
        // one way to do that:
        
        if num == 30 {
            println!("thirty");
        } else {
            println!("{:?}", num);
        }
        
        */
        
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }
        
    }
    println!("\n");
    
    println!("Total number of elements in a vector: {}", my_vector.len());
    
}

