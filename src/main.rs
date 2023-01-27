mod complex {
    mod square_loop;
    pub use square_loop::*;
}

fn main() {
    println!("Hello, world!");
    complex::complex_square_add_loop();
}
