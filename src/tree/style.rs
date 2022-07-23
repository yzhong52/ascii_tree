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
}
impl BoxDrawings {
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
    };
}