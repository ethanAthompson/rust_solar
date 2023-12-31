//! The Chinese is a Lunisolar Calendar
//! These Systems are used in unity
//!

use crate::format::Text;
use strum::*;

#[doc(alias = "Celestial Stemm")]
#[doc = "The chinese calender is repeated per 60 years"]
#[doc = ""]
#[doc = "Used with Terrestrial Branch"]
pub enum CelestialStem {
    Jia = 1,
    Yi = 2,
    Bing = 3,
    Ding = 4,
    Wu = 5,
    Ji = 6,
    Geng = 7,
    Xin = 8,
    Ren = 9,
    Gui = 10,
}

#[doc(alias = "Terrestrial Branch")]
#[doc = "The chinese calendar zodiac animals as months"]
#[doc = ""]
#[doc = "Used with Celestial Stemm"]
pub enum TerrestrialBranch {
    /// Zi
    Rat = 1,
    /// Chou
    Ox = 2,
    /// Yin
    Tiger = 3,
    /// Mao & Hare
    Rabbit = 4,
    /// Chen
    Dragon = 5,
    /// Si
    Snake = 6,
    /// Wu
    Horse = 7,
    /// Wei
    Sheep = 8,
    /// Shen
    Monkey = 9,
    /// You
    Rooster = 10,
    /// Xu
    Dog = 11,
    /// Hai & Boar
    Pig = 12,
}

#[doc(alias = "Chinese Calendar")]
#[doc = "The Chinese calendar used in picking dates"]
pub enum ChineseCalendar {
    /// Jan 21 - Feb 20
    YinYue = 1,
    /// Feb 20 - Mar 21
    MaoYue = 2,
    /// Mar 21 - Apr 20
    ChenYue = 3,
    /// Apr 20 - May 21
    SiYue = 4,
    /// May 21 - Jun 21
    WuYue = 5,
    /// Jun 21 - Jul 23
    WeiYue = 6,
    /// Jul 23 - Aug 23
    ShenYue = 7,
    /// Aug 23 - Sept-23
    YouYue = 8,
    /// Sept 23 - Oct 23
    XuYue = 9,
    /// Oct 23 - Nov 22
    HaiYue = 10,
    /// Nov 22 - Dec 22
    ZiYue = 11,
    /// Dec 22 - Jan 21
    ChouYue = 12,
}

#[derive(PartialEq, Eq, Debug, EnumProperty)]
#[doc = "Represents the seasons in a year"]
pub enum Season {
    #[strum(props(Name = "Spring"))]
    Spring,
    #[strum(props(Name = "Summer"))]
    Summer,
    #[strum(props(Name = "Autumn"))]
    Autumn,
    #[strum(props(Name = "Winter"))]
    Winter,
}

#[doc = "Methods for the Chinese Calender, includes calendar related code"]
impl ChineseCalendar {
    #[doc(alias = "Has Season")]
    #[doc = "Returns a Text containg a season variant Name"]
    pub fn has_season(month: ChineseCalendar) -> Text {
        match month {
            Self::YinYue | Self::MaoYue | Self::ChenYue => Season::Spring.get_str("Name").unwrap(),
            Self::SiYue | Self::WuYue | Self::WeiYue => Season::Summer.get_str("Name").unwrap(),
            Self::ShenYue | Self::YouYue | Self::XuYue => Season::Autumn.get_str("Name").unwrap(),
            Self::HaiYue | Self::ZiYue | Self::ChouYue => Season::Winter.get_str("Name").unwrap(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::ChineseCalendar;

    #[test]
    /// Test to see if strum properly deserializes
    /// Yinyue is spring within the chinese calendar
    fn yinyue_is_spring() {
        let month = ChineseCalendar::has_season(ChineseCalendar::YinYue);

        assert_eq!(month, "Spring", "Tried {} is not {}", month, "Spring");
    }
}
