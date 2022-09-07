pub struct PPM {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<(u8, u8, u8)>,
}

enum ParseItem {
    /// If the next 3 tokens are 'P6\n', go to WhitespaceToNext(Width). Otherwise, terminate with error
    Magic,
    /// While in this state, parse the width as base-10 ASCII digits before going to
    /// WhitespaceToNext(Height)
    Width,
    /// While in this state, parse the height as base-10 ASCII digits before going to
    /// WhitespaceToNext(Maxval)
    Height,
    /// While in this state, parse the "maxval" (maximum intensity of any pixel channel) as
    /// base-10 ASCII digits before going to NewlineToPixels
    Maxval,
    /// The pixels are stored as raw bytes, so we don't need to parse them; this is a terminal
    /// state
    Pixels,
}
enum ParseState {
    /// While in this state, the current item is being parsed
    Parsing(ParseItem),
    /// While in this state, skip all whitespace and comments. If the current token is
    /// whitespace, stay in this state. If the current token is the beginning of a comment,
    /// transition to CommentToNext(ParseItem). Otherwise, trasition to Parsing(ParseItem).
    WhitespaceToNext(ParseItem),
    /// While in this state, skip until the end of a comment, denoted by a newline. Once a
    /// newline character is found, transition to WhitespaceToNext(ParseItem). Otherwise, stay
    /// in this state.
    CommentToNext(ParseItem),
}

pub fn parse(buf: &[u8]) -> Result<PPM, &'static str> {
    // Read up on the PPM format at http://ailab.eecs.wsu.edu/wise/P1/PPM.html
    // We will be using the P6 version, i.e. pixels stored as raw bytes, for conveniece (you just
    // need to parse the header)

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

        /* BEGIN CODE YOU SHOULD EDIT */
        match state {
            ParseState::Parsing(ParseItem::Pixels) => break,
            ParseState::Parsing(_) => todo!(),
            ParseState::WhitespaceToNext(_) => todo!(),
            ParseState::CommentToNext(_) => todo!(),
        }
        /* END CODE YOU SHOULD EDIT */
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
