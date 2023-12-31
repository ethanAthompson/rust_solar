
#[doc = "Shortcut for str"]
pub type Text = &'static str;


#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Year {
    Gregorian,
    Chinese,
    Islamic,
    Martian
}


impl Year {
    pub fn new(year: usize) -> usize {
        year
    }
}
