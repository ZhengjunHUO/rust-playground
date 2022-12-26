use num::Complex;
use std::str::FromStr;

/// Parse a string slice `s` containing two values, seperated by a delimiter `d` to a tuple
fn get_pair<T: FromStr>(s: &str, d: char) -> Option<(T, T)> {
    match s.find(d) {
        None => None,
        Some(idx) => match (T::from_str(&s[..idx]), T::from_str(&s[idx + 1..])) {
            (Ok(w), Ok(h)) => Some((w, h)),
            _ => None,
        },
    }
}

/// Turn a string slice `s` containing two values, seperated by a delimiter `#` to a Complex
fn get_complex(s: &str) -> Option<Complex<f64>> {
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
/// `panel_in_pixel`: size of canvas in pixel
/// `panel_in_complex`: the size is designated by
///    a upper left complex and a lower_right complex
/// `pixel`: the pixel to be converted
fn pixel2point(
    panel_in_pixel: (usize, usize),
    panel_in_complex: (Complex<f64>, Complex<f64>),
    pixel: (usize, usize),
) -> Complex<f64> {
    let (w, h) = (
        panel_in_complex.1.re - panel_in_complex.0.re,
        panel_in_complex.0.im - panel_in_complex.1.im,
    );

    Complex {
        re: panel_in_complex.0.re + pixel.0 as f64 * w / panel_in_pixel.0 as f64,
        im: panel_in_complex.0.im - pixel.1 as f64 * h / panel_in_pixel.1 as f64,
    }
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
    fn test_pixel2point() {
        assert_eq!(
            pixel2point(
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
