#[derive(clap::ArgEnum, Debug, Clone, Copy)]
pub enum Style {
    Thin,
    Thick,
    Double,
    Chest,
    Balloon,
    Balloon2,
}

pub struct BoxDrawings {
    pub up_and_left: char,
    pub up_and_right: char,
    pub down_and_left: char,
    pub down_and_right: char,
    pub vertical: char,
    pub horizontal: char,
    pub vertical_and_horizontal: char,
    pub down_and_horizontal: char,
    pub up_and_horizontal: char,

    pub top_connection: Option<char>,
    pub bottom_connection: Option<char>,
}

impl BoxDrawings {
    pub fn new(style: Style) -> BoxDrawings {
        match style {
            Style::Thin => BoxDrawings::THIN,
            Style::Thick => BoxDrawings::THICK,
            Style::Double => BoxDrawings::DOUBLE,
            Style::Chest => BoxDrawings::CHEST,
            Style::Balloon => BoxDrawings::BALLOON,
            Style::Balloon2 => BoxDrawings::BALLOON2,
        }
    }

    pub const THIN: BoxDrawings = BoxDrawings {
        up_and_left: '┌',
        up_and_right: '┐',
        down_and_left: '└',
        down_and_right: '┘',
        vertical: '│',
        horizontal: '─',
        vertical_and_horizontal: '┼',
        down_and_horizontal: '┬',
        up_and_horizontal: '┴',
        top_connection: None,
        bottom_connection: None,
    };

    pub const THICK: BoxDrawings = BoxDrawings {
        up_and_left: '┏',
        up_and_right: '┓',
        down_and_left: '┗',
        down_and_right: '┛',
        vertical: '┃',
        horizontal: '━',
        vertical_and_horizontal: '╋',
        down_and_horizontal: '┳',
        up_and_horizontal: '┻',
        top_connection: None,
        bottom_connection: None,
    };

    pub const DOUBLE: BoxDrawings = BoxDrawings {
        up_and_left: '╔',
        up_and_right: '╗',
        down_and_left: '╚',
        down_and_right: '╝',
        vertical: '║',
        horizontal: '═',
        vertical_and_horizontal: '╬',
        down_and_horizontal: '╦',
        up_and_horizontal: '╩',
        top_connection: None,
        bottom_connection: None,
    };

    pub const CHEST: BoxDrawings = BoxDrawings {
        up_and_left: '╔',
        up_and_right: '╗',
        down_and_left: '╚',
        down_and_right: '╝',
        vertical: '┃',
        horizontal: '━',
        vertical_and_horizontal: '╋',
        down_and_horizontal: '┳',
        up_and_horizontal: '┻',
        top_connection: None,
        bottom_connection: None,
    };

    pub const BALLOON: BoxDrawings = BoxDrawings {
        up_and_left: '╭',
        up_and_right: '╮',
        down_and_left: '╰',
        down_and_right: '╯',
        vertical: '│',
        horizontal: '─',
        vertical_and_horizontal: '┼',
        down_and_horizontal: '┬',
        up_and_horizontal: '┴',
        top_connection: None,
        bottom_connection: None,
    };

    //    ╭───────────╮
    //    │ Root Node │
    //    ╰─────¤─────╯
    //     ╭────┴─────╮
    // ╭───¤───╮  ╭───¤───╮
    // │ Child │  │ Child │
    // │  (1)  │  │  (2)  │
    // ╰───────╯  ╰───────╯
    pub const BALLOON2: BoxDrawings = BoxDrawings {
        up_and_left: '╭',
        up_and_right: '╮',
        down_and_left: '╰',
        down_and_right: '╯',
        vertical: '│',
        horizontal: '─',
        vertical_and_horizontal: '┼',
        down_and_horizontal: '┬',
        up_and_horizontal: '┴',
        top_connection: Some('¤'),
        bottom_connection: Some('¤'),
    };
}
