use anyhow::*;
use image::png::PngEncoder;
use image::ColorType;
use num::Complex;
use rayon::prelude::*;
use std::fs::File;

type Cpx = Complex<f64>;

pub fn is_local(Cpx { re, im }: Cpx) -> bool {
    // (x - center_x)² + (y - center_y)² < radius².
    (re * re) + (im * im) <= 1.
}

pub fn stability(c: &Cpx) -> Vec<u8> {
    let mut n = Cpx::default();
    for i in 0..=255_u32 {
        n = (n * n) + c;
        if !is_local(n) {
            let r = i as u8;
            let g = (i * 2) as u8;
            let b = (i * 3) as u8;
            return vec![r, g, b];
        };
    }
    vec![255; 3]
}

pub fn generate(width: u32, height: u32) -> (u32, u32, Vec<u8>) {
    let right_most = width as i32 / 2;
    let left_most = -right_most;
    let top_most = height as i32 / 2;
    let bottom_most = -top_most;

    let xs = (left_most..right_most).cycle();
    let ys = (bottom_most..top_most)
        .rev()
        .flat_map(move |y| [y].into_iter().cycle().take(width as usize));
    let xys = xs.zip(ys).collect::<Vec<(i32, i32)>>();

    let iter = xys.par_iter().flat_map(move |&(x, y)| {
        let re = (-1.1 * x as f64) / left_most as f64;
        let im = (1.1 * y as f64) / top_most as f64;
        let cpx = Cpx { re, im };
        stability(&cpx)
    });

    (width, height, iter.collect())
}

pub fn write_image(filename: &str) -> Result<()> {
    let (width, height, data) = generate(640, 480);
    let file = File::create(filename)?;

    PngEncoder::new(file).encode(&data, width, height, ColorType::Rgb8)?;
    Ok(())
}
