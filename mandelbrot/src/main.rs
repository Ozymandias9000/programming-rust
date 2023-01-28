use std::convert::TryFrom;
use std::env;

use utils::{parse_complex, parse_pair, render, write_image};

use crate::utils::pixel_to_point;

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

    const USE_CONCURRENT: bool = true;
    if USE_CONCURRENT {
        render_concurrently(&mut pixels, bounds, upper_left, lower_right);
    } else {
        render(&mut pixels, bounds, upper_left, lower_right);
    }

    write_image(&filename, &pixels, bounds).expect("error writing file");
}

fn render_concurrently(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: num::Complex<f64>,
    lower_right: num::Complex<f64>,
) {
    let thread_count = num_cpus::get();
    println!("{} logical cpus", thread_count);

    let rows_per_band = bounds.1 / thread_count + 1;

    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
    crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

            spawner.spawn(move |_| {
                render(band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    })
    .unwrap()
}
