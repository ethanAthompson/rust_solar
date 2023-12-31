//! The Chinese is a Lunisolar Calendar
//! These Systems are used in unity
//!

use crate::format::Text;
use strum::*;

#[doc(alias = "Chinese 60 Year Cycle")]
#[doc = "Repeats after GuiHai"]
#[derive(Debug, EnumProperty)]
pub enum ChineseCycle {
    #[strum(props(Name = "JiaZi", Value = "1"))]
    JiaZi,
    #[strum(props(Name = "YiChou", Value = "2"))]
    YiChou,
    #[strum(props(Name = "BingYin", Value = "3"))]
    BingYin,
    #[strum(props(Name = "DingMao", Value = "4"))]
    DingMao,
    #[strum(props(Name = "WuChen", Value = "5"))]
    WuChen,
    #[strum(props(Name = "JiSi", Value = "6"))]
    JiSi,
    #[strum(props(Name = "GengWu", Value = "7"))]
    GengWu,
    #[strum(props(Name = "XinWei", Value = "8"))]
    XinWei,
    #[strum(props(Name = "RenShen", Value = "9"))]
    RenShen,
    #[strum(props(Name = "GuiYou", Value = "10"))]
    GuiYou,
    #[strum(props(Name = "JiaXu", Value = "11"))]
    JiaXu,
    #[strum(props(Name = "YiHai", Value = "12"))]
    YiHai,
    #[strum(props(Name = "BingZi", Value = "13"))]
    BingZi,
    #[strum(props(Name = "DingChou", Value = "14"))]
    DingChou,
    #[strum(props(Name = "WuYin", Value = "15"))]
    WuYin,
    #[strum(props(Name = "JiMao", Value = "16"))]
    JiMao,
    #[strum(props(Name = "GengChen", Value = "17"))]
    GengChen,
    #[strum(props(Name = "XinSi", Value = "18"))]
    XinSi,
    #[strum(props(Name = "RenWu", Value = "19"))]
    RenWu,
    #[strum(props(Name = "GuiWei", Value = "20"))]
    GuiWei,
    #[strum(props(Name = "JiaShen", Value = "21"))]
    JiaShen,
    #[strum(props(Name = "YiYou", Value = "22"))]
    YiYou,
    #[strum(props(Name = "BingXu", Value = "23"))]
    BingXu,
    #[strum(props(Name = "DingHai", Value = "24"))]
    DingHai,
    #[strum(props(Name = "WuZi", Value = "25"))]
    WuZi,
    #[strum(props(Name = "JiChou", Value = "26"))]
    JiChou,
    #[strum(props(Name = "GengYin", Value = "27"))]
    GengYin,
    #[strum(props(Name = "XinMao", Value = "28"))]
    XinMao,
    #[strum(props(Name = "RenChen", Value = "29"))]
    RenChen,
    #[strum(props(Name = "GuiSi", Value = "30"))]
    GuiSi,
    #[strum(props(Name = "JiaWu", Value = "31"))]
    JiaWu,
    #[strum(props(Name = "YiWei", Value = "32"))]
    YiWei,
    #[strum(props(Name = "BingShen", Value = "33"))]
    BingShen,
    #[strum(props(Name = "DingYou", Value = "34"))]
    DingYou,
    #[strum(props(Name = "WuXu", Value = "35"))]
    WuXu,
    #[strum(props(Name = "JiHai", Value = "36"))]
    JiHai,
    #[strum(props(Name = "GengZi", Value = "37"))]
    GengZi,
    #[strum(props(Name = "XinChou", Value = "38"))]
    XinChou,
    #[strum(props(Name = "RenYin", Value = "39"))]
    RenYin,
    #[strum(props(Name = "GuiMao", Value = "40"))]
    GuiMao,
    #[strum(props(Name = "JiaChen", Value = "41"))]
    JiaChen,
    #[strum(props(Name = "YiWu", Value = "42"))]
    YiWu,
    #[strum(props(Name = "BingSi", Value = "43"))]
    BingSi,
    #[strum(props(Name = "DingWei", Value = "44"))]
    DingWei,
    #[strum(props(Name = "WuShen", Value = "45"))]
    WuShen,
    #[strum(props(Name = "JiYou", Value = "46"))]
    JiYou,
    #[strum(props(Name = "GengXu", Value = "47"))]
    GengXu,
    #[strum(props(Name = "XinHai", Value = "48"))]
    XinHai,
    #[strum(props(Name = "RenZi", Value = "49"))]
    RenZi,
    #[strum(props(Name = "GuiChou", Value = "50"))]
    GuiChou,
    #[strum(props(Name = "JiaYin", Value = "51"))]
    JiaYin,
    #[strum(props(Name = "YiMao", Value = "52"))]
    YiMao,
    #[strum(props(Name = "BingChen", Value = "53"))]
    BingChen,
    #[strum(props(Name = "DingWu", Value = "54"))]
    DingWu,
    #[strum(props(Name = "WuSi", Value = "55"))]
    WuSi,
    #[strum(props(Name = "JiWei", Value = "56"))]
    JiWei,
    #[strum(props(Name = "GengShen", Value = "57"))]
    GengShen,
    #[strum(props(Name = "XinYou", Value = "58"))]
    XinYou,
    #[strum(props(Name = "RenXu", Value = "59"))]
    RenXu,
    #[strum(props(Name = "GuiHai", Value = "60"))]
    GuiHai,
}

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

impl ChineseCycle {

    #[doc(alias = "Is Cycle Year")]
    #[doc = "Returns the Celestial Stem from the year given"]
    pub fn is_cycle_year(year: i32) -> Text {
        /// determine what makes a cycle year?
        ""
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
