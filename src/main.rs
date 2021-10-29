use mandelbrot::convert::*;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  match args.as_slice() {
    [_, path_in, path_out] => {
      match read_image(path_in).map(|image| write_image(path_out, &transform(&image))) {
        Ok(_) => println!("done"),
        Err(err) => println!("err {}", err),
      }
    }
    _ => print!("not enough args"),
  }
}
