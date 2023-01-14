use std::fmt::Display;

use chrono::{DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};

use crate::conversion;
#[cfg(feature = "graph_son")]
use crate::error::GraphSonError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    pub secs: i64,
    pub nanos: i32,
}

impl Instant {
    pub fn new(secs: i64, nanos: i32) -> Instant {
        Instant { secs, nanos }
    }
}

impl Display for Instant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            NaiveDateTime::from_timestamp(self.secs, self.nanos as u32)
                .format("%Y-%m-%dT%H:%M:%S%.fZ")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonthDay {
    pub month: u8,
    pub day: u8,
}
impl Display for MonthDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "--{}-{}", self.month, self.day)
    }
}
#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct Year(pub i32);

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YearMonth {
    pub year: i32,
    pub month: u8,
}

impl Display for YearMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.year, self.month)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Period {
    pub years: i32,
    pub months: i32,
    pub days: i32,
}

impl Period {
    pub fn new(years: i32, months: i32, days: i32) -> Period {
        Period {
            years,
            months,
            days,
        }
    }

    pub fn zero() -> Period {
        Period::new(0, 0, 0)
    }

    #[cfg(feature = "graph_son")]
    pub fn parse(s: &str) -> Result<Period, GraphSonError> {
        let mut iter = s.chars();
        iter.next().filter(|c| c.eq(&'P')).ok_or_else(|| {
            GraphSonError::Parse("parsing error of Duration/Period literal P not found".to_string())
        })?;
        let mut period = Period::zero();
        for pairs in s[1..].split_inclusive(['Y', 'M', 'W', 'D']) {
            let (numeric, qualifier) = pairs.split_at(pairs.len() - 1);

            match qualifier {
                "Y" => {
                    let years = numeric.parse::<i32>().map_err(|err| {
                        GraphSonError::Parse(format!("cannot parse years: {err}"))
                    })?;
                    period.years += years;
                }
                "M" => {
                    let months = numeric.parse::<i32>().map_err(|err| {
                        GraphSonError::Parse(format!("cannot parse months: {err}"))
                    })?;
                    period.months += months;
                }
                "W" => {
                    let weeks = numeric.parse::<i32>().map_err(|err| {
                        GraphSonError::Parse(format!("cannot parse weeks: {err}"))
                    })?;
                    period.days += weeks * 7;
                }
                "D" => {
                    let days = numeric
                        .parse::<i32>()
                        .map_err(|err| GraphSonError::Parse(format!("cannot parse days: {err}")))?;
                    period.days += days;
                }
                a => {
                    return Err(GraphSonError::Parse(format!(
                        "identifier {a} not valid while parsing Period "
                    )))
                }
            }
        }
        Ok(period)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZonedDateTime(pub DateTime<FixedOffset>);

impl Display for ZonedDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OffsetTime {
    pub time: NaiveTime,
    pub offset: FixedOffset,
}

impl Display for OffsetTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.time, self.offset)
    }
}

impl Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P")?;
        let mut some = false;
        if self.years != 0 {
            write!(f, "{}Y", self.years)?;
            some = true;
        }
        if self.months != 0 {
            write!(f, "{}M", self.months)?;
            some = true;
        }
        match (some, self.days) {
            (true, 0) => Ok(()),
            (true, not_zero) => write!(f, "{not_zero}D"),
            (false, either) => write!(f, "{either}D"),
        }
    }
}

conversion!(Instant, Instant);
conversion!(Duration, Duration);
conversion!(NaiveDate, LocalDate);
conversion!(NaiveDateTime, LocalDateTime);
conversion!(NaiveTime, LocalTime);
conversion!(MonthDay, MonthDay);
conversion!(DateTime<FixedOffset>, OffsetDateTime);
conversion!(OffsetTime, OffsetTime);
conversion!(Period, Period);
conversion!(Year, Year);
conversion!(YearMonth, YearMonth);
conversion!(ZonedDateTime, ZonedDateTime);
conversion!(FixedOffset, ZoneOffset);
