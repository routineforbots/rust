struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Colors,
}

impl Box {

    fn new(weight: f64, color: Colors, dimensions: Dimensions) -> Self {
        
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
        println!("\n");

    }

}


enum Colors {
    Black,
    Red,
    Blue,
}

impl Colors {
    fn print(&self) {
        match self {
            Colors::Black => println!("color: black"),
            Colors::Red => println!("color: red"),
            Colors::Blue => println!("color: blue"),
            _ => println!("color was not defined"),
        }
    }

}


struct Dimensions {
    width: f64,
    height: f64,
    dep: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("dep: {:?}", self.dep);
    }
}


fn main() {
    let small_dimensions  = Dimensions {
        width: 1.0,
        height: 2.0,
        dep: 3.0,
    };

    let small_box = Box::new(5.0, Colors::Red, small_dimensions);
    small_box.print();


    let medium_dimensions  = Dimensions {
        width: 10.0,
        height: 20.0,
        dep: 30.0,
    };

    let medium_box = Box::new(23.3, Colors::Black, medium_dimensions);
    medium_box.print();


    let large_dimensions  = Dimensions {
        width: 100.0,
        height: 200.0,
        dep: 300.0,
    };

    let large_box = Box::new(199.3, Colors::Blue, large_dimensions);
    large_box.print();

}
