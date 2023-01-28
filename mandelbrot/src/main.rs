use std::convert::TryFrom;
use std::env;

use utils::{parse_complex, parse_pair, render, write_image};

mod utils {
    mod escape_time;
    pub use escape_time::*;

    mod parsers;
    pub use parsers::*;

    mod pixel_to_point;
    pub use pixel_to_point::*;

    mod render;
    pub use render::*;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1.0,20",
            args[0]
        );
        std::process::exit(1);
    }

    let [_, filename, pixels, upper_left_string, lower_right_string] =
        <[String; 5]>::try_from(args).ok().unwrap();

    let bounds = parse_pair::<usize>(&pixels, 'x').expect("error parsing image dims");

    let upper_left = parse_complex(&upper_left_string).expect("error parsing upper left corner");
    let lower_right = parse_complex(&lower_right_string).expect("error parsing lower right corner");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&filename, &pixels, bounds).expect("error writing file");
}
