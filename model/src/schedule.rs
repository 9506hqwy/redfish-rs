pub type Schedule = crate::schedule::v1_2_5::Schedule;
pub mod v1_2_5 {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum DayOfWeek {
        #[default]
        #[serde(rename = "Every")]
        Every,
        #[serde(rename = "Friday")]
        Friday,
        #[serde(rename = "Monday")]
        Monday,
        #[serde(rename = "Saturday")]
        Saturday,
        #[serde(rename = "Sunday")]
        Sunday,
        #[serde(rename = "Thursday")]
        Thursday,
        #[serde(rename = "Tuesday")]
        Tuesday,
        #[serde(rename = "Wednesday")]
        Wednesday,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum MonthOfYear {
        #[default]
        #[serde(rename = "April")]
        April,
        #[serde(rename = "August")]
        August,
        #[serde(rename = "December")]
        December,
        #[serde(rename = "Every")]
        Every,
        #[serde(rename = "February")]
        February,
        #[serde(rename = "January")]
        January,
        #[serde(rename = "July")]
        July,
        #[serde(rename = "June")]
        June,
        #[serde(rename = "March")]
        March,
        #[serde(rename = "May")]
        May,
        #[serde(rename = "November")]
        November,
        #[serde(rename = "October")]
        October,
        #[serde(rename = "September")]
        September,
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct Schedule {
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnabledDaysOfMonth")]
        pub enabled_days_of_month: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnabledDaysOfWeek")]
        pub enabled_days_of_week: Option<Vec<crate::schedule::v1_2_5::ScheduleEnabledDaysOfWeek>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "EnabledIntervals")]
        pub enabled_intervals: Option<Vec<String>>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            rename = "EnabledMonthsOfYear"
        )]
        pub enabled_months_of_year:
            Option<Vec<crate::schedule::v1_2_5::ScheduleEnabledMonthsOfYear>>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "InitialStartTime")]
        pub initial_start_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Lifetime")]
        pub lifetime: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "MaxOccurrences")]
        pub max_occurrences: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", rename = "RecurrenceInterval")]
        pub recurrence_interval: Option<String>,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ScheduleEnabledDaysOfWeek {
        V010205(crate::schedule::v1_2_5::DayOfWeek),
        V000001(crate::schedule::v1_2_5::ScheduleEnabledDaysOfWeekN1),
    }
    impl Default for ScheduleEnabledDaysOfWeek {
        fn default() -> Self {
            Self::V010205(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ScheduleEnabledDaysOfWeekN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum ScheduleEnabledMonthsOfYear {
        V010205(crate::schedule::v1_2_5::MonthOfYear),
        V000001(crate::schedule::v1_2_5::ScheduleEnabledMonthsOfYearN1),
    }
    impl Default for ScheduleEnabledMonthsOfYear {
        fn default() -> Self {
            Self::V010205(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub enum ScheduleEnabledMonthsOfYearN1 {
        #[default]
        #[serde(rename = "null")]
        Null,
    }
}
