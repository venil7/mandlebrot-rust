use image::jpeg::JPEGDecoder;
use image::png::PNGEncoder;
use image::ColorType;
use image::ImageDecoder;
use image::ImageResult;
use num::Complex;
use std::fs::File;

pub type Pair<T> = (T, T);
pub type Triplet<T> = (T, T, T);
pub type RGB = Triplet<u8>;
pub type Coord = Pair<usize>;
pub type Pixel = (Coord, RGB);
pub type ComplexCoord = Pair<Complex<f64>>;
pub type ComplexPixel = (Complex<f64>, RGB);

pub struct Image {
  pub bounds: Coord,
  pub pixels: Vec<Pixel>,
}

pub fn pixel_to_complex(
  (x, y): &Coord,
  (width, height): &Coord,
  (complex_top_left, complex_bottom_right): &ComplexCoord,
) -> Complex<f64> {
  let re_width = complex_bottom_right.re - complex_top_left.re;
  let im_height = complex_top_left.im - complex_bottom_right.im;
  Complex {
    re: complex_top_left.re + re_width * (*x as f64 / *width as f64),
    im: complex_top_left.im - im_height * (*y as f64 / *height as f64),
  }
}

pub fn complex_to_pixel(
  Complex { re, im }: &Complex<f64>,
  (width, height): &Coord,
  (complex_top_left, complex_bottom_right): &ComplexCoord,
) -> Coord {
  let re_width = complex_bottom_right.re - complex_top_left.re;
  let im_height = complex_top_left.im - complex_bottom_right.im;
  (
    (*width as f64 * (re - complex_top_left.re) / re_width) as usize,
    (*height as f64 * (im - complex_bottom_right.im) / im_height) as usize,
  )
}

pub fn complex_bounds((width, height): &Coord) -> ComplexCoord {
  (
    Complex {
      re: *width as f64 / -2.0,
      im: *height as f64 / -2.0,
    },
    Complex {
      re: *width as f64 / 2.0,
      im: *height as f64 / 2.0,
    },
  )
}

pub fn read_image(filename: &str) -> ImageResult<Image> {
  let input = File::open(filename)?;
  let decoder = JPEGDecoder::new(input)?;
  let (width, height) = decoder.dimensions();
  let (width, height) = (width as usize, height as usize);

  Ok(Image {
    bounds: (width, height),
    pixels: decoder
      .read_image()?
      .as_slice()
      .chunks(3)
      .enumerate()
      .map(|(i, rgb)| ((i % width, i / width), (rgb[0], rgb[1], rgb[2])))
      .collect(),
  })
}

pub fn write_image(
  filename: &str,
  Image {
    pixels,
    bounds: (width, height),
    ..
  }: &Image,
) -> Result<(), std::io::Error> {
  let output = File::create(filename)?;
  let encoder = PNGEncoder::new(output);
  let pixels: Vec<u8> = pixels
    .iter()
    .flat_map(|(_, (r, g, b))| vec![*r, *g, *b])
    .collect();
  encoder.encode(&pixels, *width as u32, *height as u32, ColorType::RGB(8))
}

pub fn min(a: f64, b: f64) -> f64 {
  if a < b {
    a
  } else {
    b
  }
}

pub fn max(a: f64, b: f64) -> f64 {
  if a > b {
    a
  } else {
    b
  }
}

pub fn calc_complex_bounds(complex_pixels: &[ComplexPixel]) -> ComplexCoord {
  let init: ComplexCoord = (Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 });
  complex_pixels
    .iter()
    .map(|(c, _)| c)
    .fold(init, |(top_left, bottom_right), item| {
      (
        Complex {
          re: min(top_left.re, item.re),
          im: max(top_left.im, item.im),
        },
        Complex {
          re: max(bottom_right.re, item.re),
          im: min(bottom_right.im, item.im),
        },
      )
    })
}

pub fn calc_bounds(pixels: &[Pixel]) -> Coord {
  let init = (0, 0);
  pixels
    .iter()
    .map(|(c, _)| c)
    .fold(init, |(width, height), item| {
      (std::cmp::max(item.0, width), std::cmp::max(item.1, height))
    })
}

pub fn multiply(complex_pixels: &[ComplexPixel]) -> Vec<ComplexPixel> {
  complex_pixels
    .iter()
    .map(|(c, rgb)| (c * Complex { re: 1.0, im: 0.1 }, *rgb))
    .collect()
}

pub fn compare((p1, _): &Pixel, (p2, _): &Pixel, width: usize) -> std::cmp::Ordering {
  (p1.1 * width + p1.0).cmp(&(p2.1 * width + p2.0))
}

pub fn transform(Image { bounds, pixels }: &Image) -> Image {
  let complex_pixels: Vec<ComplexPixel> = multiply(&to_complex_pixels(pixels, bounds));
  let complex_bounds: ComplexCoord = calc_complex_bounds(&complex_pixels);
  let mut pixels: Vec<Pixel> = complex_pixels
    .iter()
    .map(|(c, rgb)| (complex_to_pixel(c, &bounds, &complex_bounds), *rgb))
    .collect();
  pixels.sort_by(|p1, p2| compare(p1, p2, bounds.0));
  let bounds = calc_bounds(&pixels);
  Image { bounds, pixels }
}

pub fn to_complex_pixels(pixels: &[Pixel], bounds: &Coord) -> Vec<ComplexPixel> {
  pixels
    .iter()
    .map(|(coord, rgb)| {
      (
        pixel_to_complex(coord, bounds, &complex_bounds(bounds)),
        *rgb,
      )
    })
    .collect()
}

#[test]
fn test_1() {
  let bounds: Coord = (4000, 3000);
  let cbounds = complex_bounds(&bounds);
  let pixel = (0, 0);
  let result = pixel_to_complex(&pixel, &bounds, &cbounds);
  assert_eq!(
    result,
    Complex {
      re: -2000.0,
      im: -1500.0
    }
  );
}
#[test]
fn test_2() {
  let pixel = (0, 0);
  let bounds: Coord = (4000, 3000);
  let cbounds = complex_bounds(&bounds);
  let complex = pixel_to_complex(&pixel, &bounds, &cbounds);
  let result = complex_to_pixel(&complex, &bounds, &cbounds);
  assert_eq!(result, pixel);
}
