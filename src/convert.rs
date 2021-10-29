use anyhow::*;
use image::png::PngEncoder;
use image::ColorType;
use num::abs;
use num::Complex;
use std::fs::File;

type Cpx = Complex<f64>;

pub fn is_local(Cpx { re, im }: Cpx) -> bool {
    // (x - center_x)² + (y - center_y)² < radius².
    (re * re) + (im * im) <= 1.
}

pub fn stability(c: &Cpx) -> u8 {
    let mut n = Cpx::default();
    for i in 0..=255 {
        n = (n * n) + c;
        if !is_local(n) {
            return i;
        };
    }
    255
}

pub fn generate(width: u32, height: u32) -> (u32, u32, impl Iterator<Item = u8>) {
    let RIGHT_MOST = width as i32 / 2;
    let LEFT_MOST = -RIGHT_MOST;
    let TOP_MOST = height as i32 / 2;
    let BOTTOM_MOST = -TOP_MOST;

    dbg!(LEFT_MOST, RIGHT_MOST);
    dbg!(TOP_MOST, BOTTOM_MOST);

    let xs = (LEFT_MOST..RIGHT_MOST).cycle();
    let ys = (BOTTOM_MOST..TOP_MOST)
        .rev()
        .flat_map(move |y| [y].into_iter().cycle().take(width as usize));
    let xys = xs.zip(ys);

    let iter = xys.map(move |(x, y)| {
        let re = (-1.5 * x as f64) / LEFT_MOST as f64;
        let im = (1.5 * y as f64) / TOP_MOST as f64;
        let cpx = Cpx { re, im };
        stability(&cpx)
    });

    (width, height, iter)
}

pub fn write_image(filename: &str) -> Result<()> {
    let (width, height, data) = generate(320, 240);
    let file = File::create(filename)?;

    let data: Vec<u8> = data.collect();
    PngEncoder::new(file).encode(&data, width, height, ColorType::L8)?;
    Ok(())
}
