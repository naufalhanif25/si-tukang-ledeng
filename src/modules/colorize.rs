pub struct Color {
    pub name: &'static str,
    pub code: &'static str,
}
pub struct ColorStyle;

impl ColorStyle {
    pub const colors: [Color; 41] = [
        Color { name: "Black", code: "\x1b[30m" },
        Color { name: "Red", code: "\x1b[31m" },
        Color { name: "Green", code: "\x1b[32m" },
        Color { name: "Yellow", code: "\x1b[33m" },
        Color { name: "Blue", code: "\x1b[34m" },
        Color { name: "Magenta", code: "\x1b[35m" },
        Color { name: "Cyan", code: "\x1b[36m" },
        Color { name: "White", code: "\x1b[37m" },
        Color { name: "BrightBlack", code: "\x1b[90m" },
        Color { name: "BrightRed", code: "\x1b[91m" },
        Color { name: "BrightGreen", code: "\x1b[92m" },
        Color { name: "BrightYellow", code: "\x1b[93m" },
        Color { name: "BrightBlue", code: "\x1b[94m" },
        Color { name: "BrightMagenta", code: "\x1b[95m" },
        Color { name: "BrightCyan", code: "\x1b[96m" },
        Color { name: "BrightWhite", code: "\x1b[97m" },
        Color { name: "BgBlack", code: "\x1b[40m" },
        Color { name: "BgRed", code: "\x1b[41m" },
        Color { name: "BgGreen", code: "\x1b[42m" },
        Color { name: "BgYellow", code: "\x1b[43m" },
        Color { name: "BgBlue", code: "\x1b[44m" },
        Color { name: "BgMagenta", code: "\x1b[45m" },
        Color { name: "BgCyan", code: "\x1b[46m" },
        Color { name: "BgWhite", code: "\x1b[47m" },
        Color { name: "BgBrightBlack", code: "\x1b[100m" },
        Color { name: "BgBrightRed", code: "\x1b[101m" },
        Color { name: "BgBrightGreen", code: "\x1b[102m" },
        Color { name: "BgBrightYellow", code: "\x1b[103m" },
        Color { name: "BgBrightBlue", code: "\x1b[104m" },
        Color { name: "BgBrightMagenta", code: "\x1b[105m" },
        Color { name: "BgBrightCyan", code: "\x1b[106m" },
        Color { name: "BgBrightWhite", code: "\x1b[107m" },
        Color { name: "Bold", code: "\x1b[1m" },
        Color { name: "Dim", code: "\x1b[2m" },
        Color { name: "Italic", code: "\x1b[3m" },
        Color { name: "Underline", code: "\x1b[4m" },
        Color { name: "Blink", code: "\x1b[5m" },
        Color { name: "Reverse", code: "\x1b[7m" },
        Color { name: "Hidden", code: "\x1b[8m" },
        Color { name: "Strikethrough", code: "\x1b[9m" },
        Color { name: "Reset", code: "\x1b[0m" },
    ];

    pub fn get(name: &str) -> &'static str {
        return Self::colors.iter().find(|color| color.name == name).map(|color| color.code).unwrap_or("")
    }
}
