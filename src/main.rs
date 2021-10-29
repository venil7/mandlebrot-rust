use mandelbrot::convert::*;

fn main() -> anyhow::Result<()> {
    write_image("./mandlebrot.png")?;

    Ok(())
}
