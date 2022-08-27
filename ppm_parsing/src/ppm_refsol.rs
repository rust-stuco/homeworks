pub struct PPM {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<(u8, u8, u8)>,
}

pub fn parse(buf: &[u8]) -> Result<PPM, &'static str> {
    // information from http://ailab.eecs.wsu.edu/wise/P1/PPM.html
    enum ParseItem {
        MagicP,
        Magic6,
        Width,
        Height,
        Pixels,
    }
    enum ParseState {
        Parsing(ParseItem),
        WhitespaceToNext(ParseItem),
        CommentToNext(ParseItem),
    }
    // You can `use` on enums to get all their variants! wow very cool
    use ParseItem::*;
    use ParseState::*;

    let mut width = 0u32;
    let mut height = 0u32;
    let mut state = Parsing(MagicP);
    let mut index = 0usize;

    loop {
        if index >= buf.len() {
            return Err("Reached end of PPM before parsing could finish");
        }
        
        let c = buf[index];

        match state {
            Parsing(MagicP) => {
                if c != b'P' {
                    return Err("Magic value malformed");
                }
                state = Parsing(Magic6);
                index += 1;
            }
            Parsing(Magic6) => {
                if c != b'6' {
                    return Err("Magic value malformed; only P6 PPMs are supported");
                }
                state = WhitespaceToNext(Width);
                index += 1;
            }
            Parsing(Width) => {
                if c.is_ascii_digit() {
                    width *= 10;
                    width += (c - b'0') as u32;
                } else if c.is_ascii_whitespace() {
                    state = WhitespaceToNext(Height);
                } else {
                    return Err("Width contains a non-digit character");
                }
                index += 1;
            }
            Parsing(Height) => {
                if c.is_ascii_digit() {
                    height *= 10;
                    height += (c - b'0') as u32;
                } else if c.is_ascii_whitespace() {
                    state = WhitespaceToNext(Pixels);
                } else {
                    return Err("Height contains a non-digit character");
                }
                index += 1;
            }
            Parsing(Pixels) => break,
            WhitespaceToNext(next) => {
                if c.is_ascii_whitespace() {
                    index += 1;
                    state = WhitespaceToNext(next);
                } else if c == b'#' {
                    state = CommentToNext(next);
                    index += 1;
                } else {
                    state = Parsing(next);
                }
            }
            CommentToNext(next) => {
                if c == b'\n' {
                    state = WhitespaceToNext(next);
                } else {
                    state = CommentToNext(next);
                }
                index += 1;
            }
        }
    }

    let pixels = buf[index..]
        .chunks_exact(3)
        .skip(1)
        .map(|p| (p[1], p[2], p[0]))
        .collect::<Vec<_>>();

    assert!(pixels.len() == (width * height) as usize);

    Ok(PPM {
        width,
        height,
        pixels,
    })
}
