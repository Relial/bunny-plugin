#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum Key {
    // ----------------------------------------------
    // Commands:
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,

    Escape,
    Tab,
    Backspace,
    Enter,
    Space,

    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,

    Copy,
    Cut,
    Paste,

    // ----------------------------------------------
    // Punctuation:
    /// `:`
    Colon,

    /// `,`
    Comma,

    /// `\`
    Backslash,

    /// `/`
    Slash,

    /// `|`, a vertical bar
    Pipe,

    /// `?`
    Questionmark,

    // '!'
    Exclamationmark,

    // `[`
    OpenBracket,

    // `]`
    CloseBracket,

    // `{`
    OpenCurlyBracket,

    // `}`
    CloseCurlyBracket,

    /// Also known as "backquote" or "grave"
    Backtick,

    /// `-`
    Minus,

    /// `.`
    Period,

    /// `+`
    Plus,

    /// `=`
    Equals,

    /// `;`
    Semicolon,

    /// `'`
    Quote,

    // ----------------------------------------------
    // Digits:
    /// `0` (from main row or numpad)
    Num0,

    /// `1` (from main row or numpad)
    Num1,

    /// `2` (from main row or numpad)
    Num2,

    /// `3` (from main row or numpad)
    Num3,

    /// `4` (from main row or numpad)
    Num4,

    /// `5` (from main row or numpad)
    Num5,

    /// `6` (from main row or numpad)
    Num6,

    /// `7` (from main row or numpad)
    Num7,

    /// `8` (from main row or numpad)
    Num8,

    /// `9` (from main row or numpad)
    Num9,

    // ----------------------------------------------
    // Letters:
    A, // Used for cmd+A (select All)
    B,
    C, // |CMD COPY|
    D, // |CMD BOOKMARK|
    E, // |CMD SEARCH|
    F, // |CMD FIND firefox & chrome|
    G, // |CMD FIND chrome|
    H, // |CMD History|
    I, // italics
    J, // |CMD SEARCH firefox/DOWNLOAD chrome|
    K, // Used for ctrl+K (delete text after cursor)
    L,
    M,
    N,
    O, // |CMD OPEN|
    P, // |CMD PRINT|
    Q,
    R, // |CMD REFRESH|
    S, // |CMD SAVE|
    T, // |CMD TAB|
    U, // Used for ctrl+U (delete text before cursor)
    V, // |CMD PASTE|
    W, // Used for ctrl+W (delete previous word)
    X, // |CMD CUT|
    Y,
    Z, // |CMD UNDO|

    // ----------------------------------------------
    // Function keys:
    F1,
    F2,
    F3,
    F4,
    F5, // |CMD REFRESH|
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,

    /// Back navigation key from multimedia keyboard.
    /// Android sends this key on Back button press.
    /// Does not work on Web.
    BrowserBack,
    // When adding keys, remember to also update:
    // * crates/egui-winit/src/lib.rs
    // * Key::ALL
    // * Key::from_name
    // You should test that it works using the "Input Event History" window in the egui demo app.
    // Make sure to test both natively and on web!
    // Also: don't add keys last; add them to the group they best belong to.
}

impl Key {
    #[inline]
    pub fn symbol_or_name(self) -> &'static str {
        match self {
            Self::ArrowDown => "⏷",
            Self::ArrowLeft => "⏴",
            Self::ArrowRight => "⏵",
            Self::ArrowUp => "⏶",

            Self::Colon => ":",
            Self::Comma => ",",
            Self::Minus => "−",
            Self::Period => ".",
            Self::Plus => "+",
            Self::Equals => "=",
            Self::Semicolon => ";",
            Self::Backslash => "\\",
            Self::Slash => "/",
            Self::Pipe => "|",
            Self::Questionmark => "?",
            Self::Exclamationmark => "!",
            Self::OpenBracket => "[",
            Self::CloseBracket => "]",
            Self::OpenCurlyBracket => "{",
            Self::CloseCurlyBracket => "}",
            Self::Backtick => "`",

            _ => self.name(),
        }
    }

    #[inline]
    pub fn name(self) -> &'static str {
        match self {
            Self::ArrowDown => "Down",
            Self::ArrowLeft => "Left",
            Self::ArrowRight => "Right",
            Self::ArrowUp => "Up",

            Self::Escape => "Escape",
            Self::Tab => "Tab",
            Self::Backspace => "Backspace",
            Self::Enter => "Enter",

            Self::Insert => "Insert",
            Self::Delete => "Delete",
            Self::Home => "Home",
            Self::End => "End",
            Self::PageUp => "PageUp",
            Self::PageDown => "PageDown",

            Self::Copy => "Copy",
            Self::Cut => "Cut",
            Self::Paste => "Paste",

            Self::Space => "Space",
            Self::Colon => "Colon",
            Self::Comma => "Comma",
            Self::Minus => "Minus",
            Self::Period => "Period",
            Self::Plus => "Plus",
            Self::Equals => "Equals",
            Self::Semicolon => "Semicolon",
            Self::Backslash => "Backslash",
            Self::Slash => "Slash",
            Self::Pipe => "Pipe",
            Self::Questionmark => "Questionmark",
            Self::Exclamationmark => "Exclamationmark",
            Self::OpenBracket => "OpenBracket",
            Self::CloseBracket => "CloseBracket",
            Self::OpenCurlyBracket => "OpenCurlyBracket",
            Self::CloseCurlyBracket => "CloseCurlyBracket",
            Self::Backtick => "Backtick",
            Self::Quote => "Quote",

            Self::Num0 => "0",
            Self::Num1 => "1",
            Self::Num2 => "2",
            Self::Num3 => "3",
            Self::Num4 => "4",
            Self::Num5 => "5",
            Self::Num6 => "6",
            Self::Num7 => "7",
            Self::Num8 => "8",
            Self::Num9 => "9",

            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
            Self::I => "I",
            Self::J => "J",
            Self::K => "K",
            Self::L => "L",
            Self::M => "M",
            Self::N => "N",
            Self::O => "O",
            Self::P => "P",
            Self::Q => "Q",
            Self::R => "R",
            Self::S => "S",
            Self::T => "T",
            Self::U => "U",
            Self::V => "V",
            Self::W => "W",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::F1 => "F1",
            Self::F2 => "F2",
            Self::F3 => "F3",
            Self::F4 => "F4",
            Self::F5 => "F5",
            Self::F6 => "F6",
            Self::F7 => "F7",
            Self::F8 => "F8",
            Self::F9 => "F9",
            Self::F10 => "F10",
            Self::F11 => "F11",
            Self::F12 => "F12",
            Self::F13 => "F13",
            Self::F14 => "F14",
            Self::F15 => "F15",
            Self::F16 => "F16",
            Self::F17 => "F17",
            Self::F18 => "F18",
            Self::F19 => "F19",
            Self::F20 => "F20",
            Self::F21 => "F21",
            Self::F22 => "F22",
            Self::F23 => "F23",
            Self::F24 => "F24",
            Self::F25 => "F25",
            Self::F26 => "F26",
            Self::F27 => "F27",
            Self::F28 => "F28",
            Self::F29 => "F29",
            Self::F30 => "F30",
            Self::F31 => "F31",
            Self::F32 => "F32",
            Self::F33 => "F33",
            Self::F34 => "F34",
            Self::F35 => "F35",

            Self::BrowserBack => "BrowserBack",
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::Key> for Key {
    #[inline]
    fn from(value: egui::Key) -> Self {
        match value {
            egui::Key::ArrowDown => Self::ArrowDown,
            egui::Key::ArrowLeft => Self::ArrowLeft,
            egui::Key::ArrowRight => Self::ArrowRight,
            egui::Key::ArrowUp => Self::ArrowUp,
            egui::Key::Escape => Self::Escape,
            egui::Key::Tab => Self::Tab,
            egui::Key::Backspace => Self::Backspace,
            egui::Key::Enter => Self::Enter,
            egui::Key::Space => Self::Space,
            egui::Key::Insert => Self::Insert,
            egui::Key::Delete => Self::Delete,
            egui::Key::Home => Self::Home,
            egui::Key::End => Self::End,
            egui::Key::PageUp => Self::PageUp,
            egui::Key::PageDown => Self::PageDown,
            egui::Key::Copy => Self::Copy,
            egui::Key::Cut => Self::Cut,
            egui::Key::Paste => Self::Paste,
            egui::Key::Colon => Self::Colon,
            egui::Key::Comma => Self::Comma,
            egui::Key::Backslash => Self::Backslash,
            egui::Key::Slash => Self::Slash,
            egui::Key::Pipe => Self::Pipe,
            egui::Key::Questionmark => Self::Questionmark,
            egui::Key::Exclamationmark => Self::Exclamationmark,
            egui::Key::OpenBracket => Self::OpenBracket,
            egui::Key::CloseBracket => Self::CloseBracket,
            egui::Key::OpenCurlyBracket => Self::OpenCurlyBracket,
            egui::Key::CloseCurlyBracket => Self::CloseCurlyBracket,
            egui::Key::Backtick => Self::Backtick,
            egui::Key::Minus => Self::Minus,
            egui::Key::Period => Self::Period,
            egui::Key::Plus => Self::Plus,
            egui::Key::Equals => Self::Equals,
            egui::Key::Semicolon => Self::Semicolon,
            egui::Key::Quote => Self::Quote,
            egui::Key::Num0 => Self::Num0,
            egui::Key::Num1 => Self::Num1,
            egui::Key::Num2 => Self::Num2,
            egui::Key::Num3 => Self::Num3,
            egui::Key::Num4 => Self::Num4,
            egui::Key::Num5 => Self::Num5,
            egui::Key::Num6 => Self::Num6,
            egui::Key::Num7 => Self::Num7,
            egui::Key::Num8 => Self::Num8,
            egui::Key::Num9 => Self::Num9,
            egui::Key::A => Self::A,
            egui::Key::B => Self::B,
            egui::Key::C => Self::C,
            egui::Key::D => Self::D,
            egui::Key::E => Self::E,
            egui::Key::F => Self::F,
            egui::Key::G => Self::G,
            egui::Key::H => Self::H,
            egui::Key::I => Self::I,
            egui::Key::J => Self::J,
            egui::Key::K => Self::K,
            egui::Key::L => Self::L,
            egui::Key::M => Self::M,
            egui::Key::N => Self::N,
            egui::Key::O => Self::O,
            egui::Key::P => Self::P,
            egui::Key::Q => Self::Q,
            egui::Key::R => Self::R,
            egui::Key::S => Self::S,
            egui::Key::T => Self::T,
            egui::Key::U => Self::U,
            egui::Key::V => Self::V,
            egui::Key::W => Self::W,
            egui::Key::X => Self::X,
            egui::Key::Y => Self::Y,
            egui::Key::Z => Self::Z,
            egui::Key::F1 => Self::F1,
            egui::Key::F2 => Self::F2,
            egui::Key::F3 => Self::F3,
            egui::Key::F4 => Self::F4,
            egui::Key::F5 => Self::F5,
            egui::Key::F6 => Self::F6,
            egui::Key::F7 => Self::F7,
            egui::Key::F8 => Self::F8,
            egui::Key::F9 => Self::F9,
            egui::Key::F10 => Self::F10,
            egui::Key::F11 => Self::F11,
            egui::Key::F12 => Self::F12,
            egui::Key::F13 => Self::F13,
            egui::Key::F14 => Self::F14,
            egui::Key::F15 => Self::F15,
            egui::Key::F16 => Self::F16,
            egui::Key::F17 => Self::F17,
            egui::Key::F18 => Self::F18,
            egui::Key::F19 => Self::F19,
            egui::Key::F20 => Self::F20,
            egui::Key::F21 => Self::F21,
            egui::Key::F22 => Self::F22,
            egui::Key::F23 => Self::F23,
            egui::Key::F24 => Self::F24,
            egui::Key::F25 => Self::F25,
            egui::Key::F26 => Self::F26,
            egui::Key::F27 => Self::F27,
            egui::Key::F28 => Self::F28,
            egui::Key::F29 => Self::F29,
            egui::Key::F30 => Self::F30,
            egui::Key::F31 => Self::F31,
            egui::Key::F32 => Self::F32,
            egui::Key::F33 => Self::F33,
            egui::Key::F34 => Self::F34,
            egui::Key::F35 => Self::F35,
            egui::Key::BrowserBack => Self::BrowserBack,
        }
    }
}
