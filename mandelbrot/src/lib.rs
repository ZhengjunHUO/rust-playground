use image::{png::PNGEncoder, ColorType};
use num::Complex;
use std::{fs::File, io::Error, str::FromStr};

/// Parse a string slice `s` containing two values, seperated by a delimiter `d` to a tuple
pub fn get_pair<T: FromStr>(s: &str, d: char) -> Option<(T, T)> {
    match s.find(d) {
        None => None,
        Some(idx) => match (T::from_str(&s[..idx]), T::from_str(&s[idx + 1..])) {
            (Ok(w), Ok(h)) => Some((w, h)),
            _ => None,
        },
    }
}

/// Turn a string slice `s` containing two values, seperated by a delimiter `#` to a Complex
pub fn get_complex(s: &str) -> Option<Complex<f64>> {
    match get_pair(s, '#') {
        Some((re, im)) => Some(Complex { re, im }),
        _ => None,
    }
}

/// 曼德博集合是使序列不延伸至无限大的所有复数c的集合
/// return None if c is a member
fn is_mandelbrot_set_member(c: Complex<f64>, max_iter: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..max_iter {
        // During the iteration, z escape the border of 2 => not a member
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    // iteration exhausted, z still inside radius 2 => could be a member
    None
}

/// Map a pixel's coordiantes to a Complex
/// `pixel_range`: size of canvas in pixel
/// `complex_range`: the size is designated by
///    a upper left complex and a lower_right complex
/// `pixel`: the pixel to be converted
pub fn pixel2complex(
    pixel_range: (usize, usize),
    complex_range: (Complex<f64>, Complex<f64>),
    pixel: (usize, usize),
) -> Complex<f64> {
    let (w, h) = (
        complex_range.1.re - complex_range.0.re,
        complex_range.0.im - complex_range.1.im,
    );

    Complex {
        re: complex_range.0.re + pixel.0 as f64 * w / pixel_range.0 as f64,
        im: complex_range.0.im - pixel.1 as f64 * h / pixel_range.1 as f64,
    }
}

/// Map a rectangle range of pixels to Complex
/// then test if the Complex is a member of set
/// and parse the result to a grayscale value
/// eg. is a member => return None => 0 => black
pub fn render(
    pixel_range: (usize, usize),
    complex_range: (Complex<f64>, Complex<f64>),
    pixels: &mut [u8],
) {
    assert!(pixels.len() == pixel_range.0 * pixel_range.1);

    for row in 0..pixel_range.1 {
        for col in 0..pixel_range.0 {
            let cp = pixel2complex(pixel_range, complex_range, (col, row));

            pixels[row * pixel_range.0 + col] = match is_mandelbrot_set_member(cp, 255) {
                None => 0,
                Some(iter) => 255 - iter as u8,
            };
        }
    }
}

/// Write pixels (with grayscale value) to a png image
pub fn save_to_img(name: &str, pixel_range: (usize, usize), pixels: &[u8]) -> Result<(), Error> {
    let f = File::create(name)?;
    let enc = PNGEncoder::new(f);
    enc.encode(
        pixels,
        pixel_range.0 as u32,
        pixel_range.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pair() {
        assert_eq!(get_pair::<u32>("600,400", ','), Some((600, 400)));
        assert_eq!(get_pair::<u32>("600#", '#'), None);
        assert_eq!(get_pair::<u32>("600#abc", '#'), None);
        assert_eq!(get_pair::<f64>("1024.0;800.0", ';'), Some((1024.0, 800.0)));
        assert_eq!(get_pair::<f64>(";800.0;", ';'), None);
        assert_eq!(get_pair::<u32>("", ' '), None);
    }

    #[test]
    fn test_get_complex() {
        assert_eq!(
            get_complex("0.5#-0.75"),
            Some(Complex { re: 0.5, im: -0.75 })
        );
        assert_eq!(get_complex("0.25,0.25"), None);
        assert_eq!(get_complex("#1.0"), None);
    }

    #[test]
    fn test_pixel2complex() {
        assert_eq!(
            pixel2complex(
                (100, 200),
                (Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 }),
                (25, 175)
            ),
            Complex {
                re: -0.5,
                im: -0.75
            }
        );
    }
}
