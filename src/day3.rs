use core::str;

pub fn main(input: &str) -> anyhow::Result<(usize, usize)> {
    let mut part1: usize = 0;

    for chunk in input.as_bytes().windows(12) {
        match chunk {
            [b'm', b'u', b'l', b'(', tail1 @ ..] => {
                if let Some((a, b)) = match_digits(tail1) {
                    part1 += a * b;
                }
            }
            _ => continue,
        }
    }

    let mut part2 = 0;
    let mut enabled = true;

    for chunk in input.as_bytes().windows(12) {
        match chunk {
            [b'm', b'u', b'l', b'(', tail1 @ ..] if enabled => {
                if let Some((a, b)) = match_digits(tail1) {
                    part2 += a * b;
                }
            }
            [b'd', b'o', b'(', b')', ..] => {
                enabled = true;
            }
            [b'd', b'o', b'n', b'\'', b't', b'(', b')', ..] => {
                enabled = false;
            }
            _ => continue,
        }
    }
    Ok((part1, part2))
}

fn match_digits(slice: &[u8]) -> Option<(usize, usize)> {
    match slice {
        [a @ b'0'..=b'9', b',', tail2 @ ..] => Some((a.to_decimal(), match_right(tail2)?)),
        [a @ b'0'..=b'9', b @ b'0'..=b'9', b',', tail2 @ ..] => {
            Some(((a, b).to_decimal(), match_right(tail2)?))
        }
        [a @ b'0'..=b'9', b @ b'0'..=b'9', c @ b'0'..=b'9', b',', tail2 @ ..] => {
            Some(((a, b, c).to_decimal(), match_right(tail2)?))
        }
        _ => None,
    }
}

fn match_right(slice: &[u8]) -> Option<usize> {
    match slice {
        [a @ b'0'..=b'9', b')', ..] => Some(a.to_decimal()),
        [a @ b'0'..=b'9', b @ b'0'..=b'9', b')', ..] => Some((a, b).to_decimal()),
        [a @ b'0'..=b'9', b @ b'0'..=b'9', c @ b'0'..=b'9', b')', ..] => {
            Some((a, b, c).to_decimal())
        }
        _ => None,
    }
}

trait ToDecimal {
    fn to_decimal(&self) -> usize;
}

impl ToDecimal for &u8 {
    fn to_decimal(&self) -> usize {
        (*self - b'0') as usize
    }
}
impl<T: ToDecimal> ToDecimal for (T, T) {
    fn to_decimal(&self) -> usize {
        self.0.to_decimal() * 10 + self.1.to_decimal()
    }
}
impl<T: ToDecimal> ToDecimal for (T, T, T) {
    fn to_decimal(&self) -> usize {
        self.0.to_decimal() * 100 + self.1.to_decimal() * 10 + self.2.to_decimal()
    }
}
