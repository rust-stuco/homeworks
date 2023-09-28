pub struct PPM {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<(u8, u8, u8)>,
}

enum ParseItem {
    Magic,
    Width,
    Height,
    Maxval,
    Pixels,
}
enum ParseState {
    Parsing(ParseItem),
    WhitespaceToNext(ParseItem),
    CommentToNext(ParseItem),
}

pub fn parse(buf: &[u8]) -> Result<PPM, &'static str> {
    // Read up on the PPM format at http://ailab.eecs.wsu.edu/wise/P1/PPM.html
    // We will be using the P6 version, i.e. pixels stored as raw bytes, for conveniece

    let mut width = 0u32;
    let mut height = 0u32;
    let mut maxval = 0u8;
    let mut state = ParseState::Parsing(ParseItem::Magic);
    let mut index = 0usize;

    loop {
        if index >= buf.len() {
            return Err("Reached end of PPM before parsing could finish");
        }

        let c = buf[index];

        match state {
            ParseState::Parsing(ParseItem::Magic) => {
                // If the next 3 tokens are 'P6\n', go to WhitespaceToNext(Width). Otherwise, terminate with error
                todo!()
            }
            ParseState::Parsing(ParseItem::Width) => {
                // While in this state, parse the width as base-10 ASCII digits before going to
                // WhitespaceToNext(Height)
                todo!()
            }
            ParseState::Parsing(ParseItem::Height) => {
                // While in this state, parse the height as base-10 ASCII digits before going to
                // WhitespaceToNext(Maxval)
                todo!()
            }
            ParseState::Parsing(ParseItem::Maxval) => {
                // While in this state, parse the "maxval" (maximum intensity of any pixel channel) as
                // base-10 ASCII digits, and check that the next non-digit byte is a newline before
                // going to Parsing(Pixels)
                todo!()
            }
            ParseState::Parsing(ParseItem::Pixels) => {
                // The pixels are stored as raw bytes, so we don't need to parse them; this is a terminal
                // state
                break;
            }
            ParseState::WhitespaceToNext(_) => {
                // While in this state, skip all whitespace and comments. If the current token is
                // whitespace, stay in this state. If the current token is the beginning of a comment,
                // transition to CommentToNext(ParseItem). Otherwise, trasition to Parsing(ParseItem).
                todo!()
            }
            ParseState::CommentToNext(_) => {
                // While in this state, skip until the end of a comment, denoted by a newline. Once a
                // newline character is found, transition to WhitespaceToNext(ParseItem). Otherwise, stay
                // in this state.
                todo!()
            }
        }
    }

    let pixels = buf[index..]
        .chunks_exact(3)
        .map(|p| (p[0], p[1], p[2]))
        .collect::<Vec<_>>();

    if pixels.len() != (width * height) as usize {
        println!("{} != {}", pixels.len(), width * height);
        return Err("Number of pixels does not match dimensions");
    }

    Ok(PPM {
        width,
        height,
        pixels,
    })
}
