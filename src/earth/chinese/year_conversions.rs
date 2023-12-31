//! Contains G -> C & C -> G
//!
//! Conversions are approximate from the Yellow Emperor in 2698 B.C.E
//! 
//!
#[doc = "The discovery date of the chinese year in gregorian terms"]
pub const DISCOVERY_YEAR: i32 = 2000;
#[doc = "The discovery date of the chinese year in chinese year terms"]
pub const CHINESE_DISCOVERY_YEAR: i32 = 4698;

#[doc(alias = "Gregorian Year to Chinese Year")]
#[doc = "if greg year 2000 is 4698 in china year, all we have to do is scale it"]
pub fn gregorian_year_to_chinese_year(gregorian_year: i32) -> i32 {
    if gregorian_year > DISCOVERY_YEAR {
        // 2023 - 2000 = 23 gregorian years
        let sync = gregorian_year - DISCOVERY_YEAR;

        // 4698 + 23 = 4721 chinese year
        return (CHINESE_DISCOVERY_YEAR + sync).abs();
    } else {
        // 2000 - 1809 = 191 gregorian years
        let sync = DISCOVERY_YEAR - gregorian_year;

        // 4698 - 191  = 4507 chinese years
        return (CHINESE_DISCOVERY_YEAR - sync).abs();
    }
}

#[doc(alias = "Chinese Year to Gregorian Year")]
#[doc = "The year 4700 in chinese calendar is the year 2002 in gregorian calendar"]
pub fn chinese_year_to_gregorian_year(chinese_year: i32) -> i32 {
    if chinese_year > 0 {
        // 4702, 540
        if chinese_year > CHINESE_DISCOVERY_YEAR {
            // 9800 - 4698 = 5102 chinese years
            let sync = chinese_year - CHINESE_DISCOVERY_YEAR;

            // 5102 - 2000 = 3102 gregorian years
            return (sync - DISCOVERY_YEAR).abs();
        } else {
            // 4698 - 540 = 4158 chinese year
            let sync = CHINESE_DISCOVERY_YEAR - chinese_year;

            // 4158 - 2000 = 2158 gregorian years
            return (sync - DISCOVERY_YEAR).abs();
        }
    } else {
        // this would not be good
        -1
    }
}

/// Successful conversions between the gregorian year to chinese year
#[cfg(test)]
mod tests {

    use super::chinese_year_to_gregorian_year as chin_to_greg;
    use super::gregorian_year_to_chinese_year as greg_to_chin;

    #[test]
    fn abraham_lincoln_birthday_in_chinese_year() {
        // gregorian year
        let gyear = 1809;

        // chinese year
        let cyear = 4507;

        // Abraham was born 4507 in the Chinese Calendar System
        // Abraham was born 1809 in the Gregorian Calendar System
        assert_eq!(greg_to_chin(gyear), cyear);
    }

    #[test]
    fn lebron_james_birthday_in_chinese_year() {
        // gregorian year
        let gyear = 1984;

        // chinese year
        let cyear = 4682;

        // Lebron James was born 4682 in the Chinese Calendar System
        // Lebron James was born 1984 in the Gregorian Calendar System
        assert_eq!(greg_to_chin(gyear), cyear);
    }

    #[test]
    fn deng_xiaoping_birthday_in_gregorian_year() {
        // gregorian year
        let gyear = 1904;

        // chinese year
        let cyear = 4794;

        // Deng Xiaoping was born 4682 in the Chinese Calendar System
        // Deng Xiaoping was born 1904 in the Gregorian Calendar System
        assert_eq!(chin_to_greg(cyear), gyear);
    }

    #[test]
    fn future_man_birthday_in_gregorian_year() {
        // gregorian year
        let gyear = 3102;

        // chinese year
        let cyear = 9800;

        // Future Man was born in 3102 in the Gregorian Calendar System
        // Future Man was born in 9800 in the Chinese Calendar System
        assert_eq!(
            chin_to_greg(cyear),
            gyear,
            "
                Found CYear {}, Gyear {}
            ",
            chin_to_greg(cyear),
            gyear
        );
    }
}
