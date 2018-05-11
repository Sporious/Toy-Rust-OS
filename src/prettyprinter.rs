use core::fmt::Write;
use uart::Uart;

pub const ESC: char = 27 as char;
pub struct FgColour<'a>(&'a str);
pub struct BgColour<'a>(&'a str);

pub const FG_CLEAR: FgColour = FgColour("0");
pub const FG_BLACK: FgColour = FgColour("30");
pub const FG_RED: FgColour = FgColour("31");
pub const FG_GREEN: FgColour = FgColour("32");
pub const FG_YELLOW: FgColour = FgColour("33");
pub const FG_BLUE: FgColour = FgColour("34");
pub const FG_WHITE: FgColour = FgColour("37");

pub const BG_CLEAR: BgColour = BgColour("0");
pub const BG_BLACK: BgColour = BgColour("40");
pub const BG_RED: BgColour = BgColour("41");
pub const BG_GREEN: BgColour = BgColour("42");
pub const BG_YELLOW: BgColour = BgColour("43");
pub const BG_BLUE: BgColour = BgColour("44");

impl AnsiPrettyPrinter for Uart {}
pub trait AnsiPrettyPrinter: Write {
    fn clr(&mut self) {
        self.write_char(ESC);
        self.write_char('c');
    }

    fn set_fg_colour(&mut self, fg: FgColour) {
        self.write_char(ESC);
        self.write_char('[');
        self.write_str(fg.0);
        self.write_char('m');
    }
    fn set_bg_colour(&mut self, bg: BgColour) {
        self.write_char(ESC);
        self.write_char('[');
        self.write_str(bg.0);
        self.write_char('m');
    }
    fn move_cursor_right(&mut self, n: &str) {
        self.write_char(ESC);
        self.write_char('[');
        self.write_str(n);
        self.write_char('C');
    }
    fn move_cursor_left(&mut self, n: &str) {
        self.write_char(ESC);
        self.write_char('[');
        self.write_str(n);
        self.write_char('D');
    }
    fn move_cursor_up(&mut self, n: &str) {
        self.write_char(ESC);
        self.write_char('[');
        self.write_str(n);
        self.write_char('A');
    }
    fn move_cursor_down(&mut self, n: &str) {
        self.write_char(ESC);
        self.write_char('[');
        self.write_str(n);
        self.write_char('B');
    }
}
