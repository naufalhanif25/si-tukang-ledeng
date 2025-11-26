pub struct Border {
    pub name: &'static str,
    pub code: &'static str,
}
pub struct TableBorder;

impl TableBorder {
    pub const UCBORDER: [Border; 8] = [
        Border { name: "H", code: "\u{02500}" },
        Border { name: "V", code: "\u{02502}" },
        Border { name: "TL", code: "\u{0256D}" },
        Border { name: "TR", code: "\u{0256E}" },
        Border { name: "ML", code: "\u{0251C}" },
        Border { name: "MR", code: "\u{02524}" },
        Border { name: "BL", code: "\u{02570}" },
        Border { name: "BR", code: "\u{0256F}" },
    ];

    pub fn get(name: &str) -> &'static str {
        return Self::UCBORDER.iter().find(|border| border.name == name).map(|border| border.code).unwrap_or("")
    }
}
