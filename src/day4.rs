
pub fn main(input: &str) -> anyhow::Result<(usize, usize)> {
    let mut part1 = 0;
    let mut part2 = 0;

    let width = input.find('\n').expect("New line not found") + 1;
    let buf = input.as_bytes();

    for token in horizontal(buf)
        .chain(vertical(buf, width))
        .chain(diagonal_tl_br(buf, width))
        .chain(diagonal_tr_bl(buf, width))
    {
        if matches!(&token[..], b"XMAS" | b"SAMX") {
            part1 += 1;
        }
    }

    // Part 2
    for i in 0..(input.len() - 2 * width - 3) {
        let slice = [
            buf[i], buf[i + 2],
            buf[i + width + 1],
            buf[i + 2 * width], buf[i + 2 * width + 2],
        ];
        match &slice[..] {
            b"MMASS" => part2 += 1,
            b"SMASM" => part2 += 1,
            b"MSAMS" => part2 += 1,
            b"SSAMM" => part2 += 1,
            _ => (),
        }
    }


    Ok((part1, part2))
}


fn horizontal(buf: &[u8]) -> impl Iterator<Item=[u8; 4]> + '_ {
    buf.windows(4).map(|s| [s[0], s[1], s[2], s[3]])
}

fn vertical(buf: &[u8], w: usize) -> impl Iterator<Item=[u8; 4]> + '_ {
    let x = buf.len() - 3 * w;
    (0..x).into_iter().map(move |i| {
        let slice = [buf[i], buf[i + w], buf[i + 2 * w], buf[i + 3 * w]];
        slice
    })
}

fn diagonal_tl_br(buf: &[u8], w: usize) -> impl Iterator<Item=[u8; 4]> + '_ {
    let x = buf.len() - 3 * w - 4;

    (0..x).into_iter().map(move |i| {
        let slice = [buf[i], buf[i + w + 1], buf[i + 2 * w + 2], buf[i + 3 * w + 3]];
        slice
    })
}

fn diagonal_tr_bl(buf: &[u8], w: usize) -> impl Iterator<Item=[u8; 4]> + '_ {
    let x = buf.len() - 3 * w - 4;
    (0..x).into_iter().map(move |i| {
        let slice = [buf[i + 3], buf[i + w + 2], buf[i + 2 * w + 1], buf[i + 3 * w]];
        slice
    })
}
