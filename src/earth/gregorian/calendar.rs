#[doc = "Data for the gregorian calendar"]
#[doc = "Each year length can be different for each calendar"]
#[doc = "Gregorian is a Solar Calendar"]
pub mod calendar {
    #[doc = "Gregorian Months"]
    #[derive(Debug, PartialEq, Eq)]
    pub enum Calendar {
        /// 1st month
        January = 1,
        /// 2nd month
        February = 2,
        /// 3rd month
        March = 3,
        /// 4th month
        April = 4,
        /// 5th month
        May = 5,
        /// 6th month
        June = 6,
        /// 7th month
        July = 7,
        /// 8th month
        August = 8,
        /// 9th month
        September = 9,
        /// 10th month
        October = 10,
        /// 11th month
        November = 11,
        /// 12th month
        December = 12,
    }
}

