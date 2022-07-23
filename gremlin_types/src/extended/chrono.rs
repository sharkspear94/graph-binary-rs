use std::{fmt::Display, io::Read};

use chrono::{
    DateTime, Datelike, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Timelike,
};

#[cfg(feature = "graph_son")]
use chrono::format::{parse, Fixed, Item, Numeric, Pad, Parsed};
#[cfg(feature = "graph_son")]
use serde_json::json;

#[cfg(feature = "graph_binary")]
use crate::graph_binary::{Decode, Encode};
#[cfg(feature = "graph_son")]
use crate::graphson::{validate_type_entry, DecodeGraphSON, EncodeGraphSON};
use crate::{
    conversion,
    error::{DecodeError, EncodeError, GraphSonError},
    graphson::validate_type,
    specs::CoreType,
};

#[cfg(feature = "graph_son")]
fn parse_java_duration(s: &str) -> Result<Duration, GraphSonError> {
    let mut iter = s.chars();
    iter.next()
        .filter(|c| c.eq(&'P'))
        .ok_or_else(|| GraphSonError::Parse("P not found".to_string()))?;
    let mut duration = Duration::zero();
    let mut date = true;

    for pairs in s[1..].split_inclusive(['M', 'D', 'T', 'H', 'S']) {
        let (numeric, qualifier) = pairs.split_at(pairs.len() - 1);

        if date {
            match qualifier {
                "D" => {
                    let days = numeric
                        .parse::<i64>()
                        .map_err(|err| GraphSonError::Parse(err.to_string()))?;
                    duration = duration + Duration::days(days);
                }
                "T" => date = false,

                rest => {
                    return Err(GraphSonError::Parse(format!(
                        "identifier {rest} not valid while parsing Date portion"
                    )))
                }
            }
        } else {
            match qualifier {
                "H" => {
                    let hours = numeric
                        .parse::<i64>()
                        .map_err(|err| GraphSonError::Parse(err.to_string()))?;
                    duration = duration + Duration::hours(hours);
                }
                "M" => {
                    let minutes = numeric
                        .parse::<i64>()
                        .map_err(|err| GraphSonError::Parse(err.to_string()))?;
                    duration = duration + Duration::minutes(minutes);
                }
                "S" => {
                    let seconds = numeric
                        .parse::<f64>()
                        .map_err(|err| GraphSonError::Parse(err.to_string()))?;
                    let nanos = (seconds.fract() * 1000. * 1000. * 1000.) as i64;
                    duration = duration
                        + Duration::seconds(seconds.floor() as i64)
                        + Duration::nanoseconds(nanos);
                }
                a => {
                    return Err(GraphSonError::Parse(format!(
                        "identifier {a} not valid while parsing Time portion"
                    )))
                }
            }
        }
    }

    Ok(duration)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    secs: i64,
    nanos: i32,
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
    month: u8,
    day: u8,
}
impl Display for MonthDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "--{}-{}", self.month, self.day)
    }
}
#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct Year(i32);

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YearMonth {
    year: i32,
    month: u8,
}

impl Display for YearMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.year, self.month)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Period {
    years: i32,
    months: i32,
    days: i32,
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
                    period.years += years
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
pub struct ZonedDateTime(DateTime<FixedOffset>);

impl Display for ZonedDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OffsetTime {
    time: NaiveTime,
    offset: FixedOffset,
}

impl Display for OffsetTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.time, self.offset)
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for Duration {
    fn type_code() -> u8 {
        CoreType::Duration.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let s = self
            .to_std()
            .map_err(|e| EncodeError::Serilization(format!("duration out of range: {e}")))?;
        let a = i64::try_from(s.as_secs())
            .map_err(|e| EncodeError::Serilization(format!("cannot convert u64 to i64: {e}")))?;
        a.partial_encode(writer)?;
        (s.subsec_nanos() as i32).partial_encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for Duration {
    fn expected_type_code() -> u8 {
        CoreType::Duration.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let secs = i64::partial_decode(reader)?;

        let nanos = i32::partial_decode(reader)?;

        Ok(Duration::seconds(secs) + Duration::nanoseconds(nanos as i64))
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for Duration {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:Duration",
          "@value" : self.to_string()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for Duration {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = validate_type(j_val, "gx:Duration")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;

        parse_java_duration(s)
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for MonthDay {
    fn type_code() -> u8 {
        CoreType::MonthDay.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.month.partial_encode(writer)?;
        self.day.partial_encode(writer)
    }
}
#[cfg(feature = "graph_binary")]
impl Decode for MonthDay {
    fn expected_type_code() -> u8 {
        CoreType::MonthDay.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let month = u8::partial_decode(reader)?;
        let day = u8::partial_decode(reader)?;

        Ok(MonthDay { month, day })
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for MonthDay {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:MonthDay",
          "@value" : format!("--{:02}-{:02}",self.month,self.day)
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for MonthDay {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = validate_type(j_val, "gx:MonthDay")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;
        let month = s[2..4]
            .parse()
            .map_err(|err| GraphSonError::Parse(format!("month parse error: {err}")))?;
        let day = s[5..]
            .parse()
            .map_err(|err| GraphSonError::Parse(format!("month parse error: {err}")))?;

        Ok(MonthDay { month, day })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}
#[cfg(feature = "graph_binary")]
impl Encode for Year {
    fn type_code() -> u8 {
        CoreType::Year.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.0.partial_encode(writer)
    }
}
#[cfg(feature = "graph_binary")]
impl Decode for Year {
    fn expected_type_code() -> u8 {
        CoreType::Year.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Ok(Year(i32::partial_decode(reader)?))
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for Year {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:Year",
          "@value" : self.0
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for Year {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = validate_type(j_val, "gx:Year")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;
        let year = s
            .parse()
            .map_err(|e| GraphSonError::Parse(format!("year parse error: {e}")))?;

        Ok(Year(year))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for YearMonth {
    fn type_code() -> u8 {
        CoreType::YearMonth.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.year.partial_encode(writer)?;
        self.month.partial_encode(writer)
    }
}
#[cfg(feature = "graph_binary")]
impl Decode for YearMonth {
    fn expected_type_code() -> u8 {
        CoreType::YearMonth.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let year = i32::partial_decode(reader)?;
        let month = u8::partial_decode(reader)?;
        Ok(YearMonth { year, month })
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for YearMonth {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:YearMonth",
          "@value" : format!("{:04}-{:02}",self.year,self.month)
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for YearMonth {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = validate_type(j_val, "gx:YearMonth")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;

        if let Some((year, month)) = s.split_once('-') {
            let year = year
                .parse()
                .map_err(|e| GraphSonError::Parse(format!("year parse error: {e}")))?;
            let month = month
                .parse()
                .map_err(|e| GraphSonError::Parse(format!("year parse error: {e}")))?;
            Ok(YearMonth { year, month })
        } else {
            Err(GraphSonError::Parse(
                "YearMonth has wrong format".to_string(),
            ))
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for NaiveTime {
    fn type_code() -> u8 {
        CoreType::LocalTime.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        (self.num_seconds_from_midnight() as i64 * 1000 * 1000 * 1000).partial_encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for NaiveTime {
    fn expected_type_code() -> u8 {
        CoreType::LocalTime.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let nanos = i64::partial_decode(reader)?;

        let seconds = nanos / 1000 / 1000 / 1000;
        let nanos = nanos - seconds * 1000 * 1000 * 1000;

        NaiveTime::from_num_seconds_from_midnight_opt(seconds as u32, nanos as u32)
            .ok_or_else(|| DecodeError::DecodeError("data for NaiveTime out of range".to_string()))
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for NaiveTime {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:LocalTime",
          "@value" : self.format("%H:%M:%S").to_string()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for NaiveTime {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = validate_type(j_val, "gx:LocalTime")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;

        NaiveTime::parse_from_str(s, "%H:%M:%S")
            .or_else(|_| NaiveTime::parse_from_str(s, "%H:%M:%S.f"))
            .or_else(|_| NaiveTime::parse_from_str(s, "%H:%M"))
            .map_err(|err| GraphSonError::Parse(format!("cannot parse NaiveDate {err}")))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for NaiveDate {
    fn type_code() -> u8 {
        CoreType::LocalDate.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let year = self.year().to_be_bytes();
        let month = (self.month() as u8).to_be_bytes();
        let day = (self.day() as u8).to_be_bytes();
        let buf = [year[0], year[1], year[2], year[3], month[0], day[0]];
        writer.write_all(&buf)?;
        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for NaiveDate {
    fn expected_type_code() -> u8 {
        CoreType::LocalDate.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let year = i32::partial_decode(reader)?;
        let month = u8::partial_decode(reader)?;
        let day = u8::partial_decode(reader)?;

        NaiveDate::from_ymd_opt(year, month as u32, day as u32)
            .ok_or_else(|| DecodeError::DecodeError("data for DateTime out of range".to_string()))
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for NaiveDate {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:LocalDate",
          "@value" : self.format("%Y-%m-%d").to_string()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for NaiveDate {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = validate_type(j_val, "gx:LocalDate")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;

        NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|err| GraphSonError::Parse(format!("cannot parse NaiveDate {err}")))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for NaiveDateTime {
    fn type_code() -> u8 {
        CoreType::LocalDateTime.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.date().partial_encode(writer)?;
        self.time().partial_encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for NaiveDateTime {
    fn expected_type_code() -> u8 {
        CoreType::LocalDateTime.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let date = NaiveDate::partial_decode(reader)?;
        let time = NaiveTime::partial_decode(reader)?;

        Ok(NaiveDateTime::new(date, time))
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for NaiveDateTime {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type" : "gx:LocalDateTime",
            "@value" : self.format("%Y-%m-%dT%H:%M").to_string()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for NaiveDateTime {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = validate_type(j_val, "gx:LocalDateTime")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;

        NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
            .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S"))
            .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S.f"))
            .map_err(|err| GraphSonError::Parse(format!("cannot parse NaiveDateTime {err}")))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for FixedOffset {
    fn type_code() -> u8 {
        CoreType::ZoneOffset.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.local_minus_utc().partial_encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for FixedOffset {
    fn expected_type_code() -> u8 {
        CoreType::ZoneOffset.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let secs = i32::partial_decode(reader)?;

        FixedOffset::east_opt(secs)
            .ok_or_else(|| DecodeError::DecodeError("could not decode ZonedOffset".to_string()))
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for FixedOffset {
    fn encode_v3(&self) -> serde_json::Value {
        self.to_string();
        json!({
          "@type" : "gx:ZoneOffset",
          "@value" : self.to_string()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for FixedOffset {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = validate_type(j_val, "gx:ZoneOffset")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;

        let mut parsed = Parsed::new();

        parse(&mut parsed, s, [Item::Fixed(Fixed::TimezoneOffset)].iter())
            .map(|_| FixedOffset::east(parsed.offset.unwrap()))
            .or_else(|_| {
                parse(
                    &mut parsed,
                    s,
                    [
                        Item::Fixed(Fixed::TimezoneOffsetColon),
                        Item::Literal(":"),
                        Item::Numeric(Numeric::Second, Pad::Zero),
                    ]
                    .iter(),
                )
                .map(|_| {
                    let offset = parsed.offset.unwrap() + parsed.second.unwrap() as i32;
                    FixedOffset::east(offset)
                })
            })
            .map_err(|err| GraphSonError::Parse(format!("cannot parse FixedOffset {err}")))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for OffsetTime {
    fn type_code() -> u8 {
        CoreType::OffsetTime.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.time.partial_encode(writer)?;
        self.offset.partial_encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for OffsetTime {
    fn expected_type_code() -> u8 {
        CoreType::OffsetTime.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let time = NaiveTime::partial_decode(reader)?;
        let offset = FixedOffset::partial_decode(reader)?;

        Ok(OffsetTime { time, offset })
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for OffsetTime {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:OffsetTime",
          "@value" : format!("{}{}",self.time.format("%H:%M:%S"),self.offset)
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for OffsetTime {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = validate_type(j_val, "gx:OffsetTime")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;

        let time = NaiveTime::parse_from_str(s, "%H:%M:%S%z")
            .or_else(|_| NaiveTime::parse_from_str(s, "%H:%M:%S%.f%z"))
            .map_err(|err| GraphSonError::Parse(format!("cannot parse Time {err}")))?;

        let mut parsed = Parsed::new();
        parse(
            &mut parsed,
            &s[s.len() - 6..], // TODO not safe
            [Item::Fixed(Fixed::TimezoneOffset)].iter(),
        )
        .map_err(|err| GraphSonError::Parse(format!("cannot parse ZoneOffset {err}")))?;
        let offset = parsed
            .to_fixed_offset()
            .map_err(|err| GraphSonError::Parse(format!("cannot parse ZoneOffset {err}")))?;
        Ok(OffsetTime { time, offset })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for DateTime<FixedOffset> {
    fn type_code() -> u8 {
        CoreType::OffsetDateTime.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.naive_local().partial_encode(writer)?;
        self.offset().partial_encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for DateTime<FixedOffset> {
    fn expected_type_code() -> u8 {
        CoreType::OffsetDateTime.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let mut naive = NaiveDateTime::partial_decode(reader)?;
        let offset = FixedOffset::partial_decode(reader)?;
        // let x = offset.from_local_datetime(&naive).unwrap();
        // map_err(|err| {
        //     DecodeError::DecodeError(format!("cannot decode Datetime, err: {}", err))
        // });
        naive = naive - offset;
        Ok(DateTime::from_utc(naive, offset))
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for DateTime<FixedOffset> {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:OffsetDateTime",
          "@value" : self.format("%Y-%m-%dT%H:%M:%S%:z").to_string()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for DateTime<FixedOffset> {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = validate_type(j_val, "gx:OffsetDateTime")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;

        DateTime::parse_from_rfc3339(s)
            .or_else(|_| DateTime::parse_from_rfc2822(s))
            .or_else(|_| DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f%:z"))
            .map_err(|err| GraphSonError::Parse(format!("cannot parse ZoneOffset {err}")))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for ZonedDateTime {
    fn type_code() -> u8 {
        CoreType::ZonedDateTime.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.0.partial_encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for ZonedDateTime {
    fn expected_type_code() -> u8 {
        CoreType::ZonedDateTime.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let dt = DateTime::partial_decode(reader)?;
        Ok(ZonedDateTime(dt))
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for ZonedDateTime {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:ZonedDateTime",
          "@value" : self.0.format("%Y-%m-%dT%H:%M:%S%.f%:z[GMT%:z]").to_string()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for ZonedDateTime {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = validate_type(j_val, "gx:ZonedDateTime")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;

        let dt = DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f%:z[%Z%:z]")
            .or_else(|_| DateTime::parse_from_rfc3339(s))
            .or_else(|_| DateTime::parse_from_rfc2822(s))
            .map_err(|err| GraphSonError::Parse(format!("cannot parse ZonedDateTime {err}")))?;
        Ok(ZonedDateTime(dt))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for Period {
    fn type_code() -> u8 {
        CoreType::Period.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        self.years.partial_encode(writer)?;
        self.months.partial_encode(writer)?;
        self.days.partial_encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for Period {
    fn expected_type_code() -> u8 {
        CoreType::Period.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let years = i32::partial_decode(reader)?;
        let months = i32::partial_decode(reader)?;
        let days = i32::partial_decode(reader)?;

        Ok(Period {
            years,
            months,
            days,
        })
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

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for Period {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:Period",
          "@value" : self.to_string()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for Period {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = validate_type(j_val, "gx:Period")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;

        Period::parse(s)
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for Instant {
    fn type_code() -> u8 {
        CoreType::Instant.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        self.secs.partial_encode(writer)?;
        self.nanos.partial_encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for Instant {
    fn expected_type_code() -> u8 {
        CoreType::Instant.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let secs = i64::partial_decode(reader)?;
        let nanos = i32::partial_decode(reader)?;

        Ok(Instant { secs, nanos })
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for Instant {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:Instant",
          "@value" : NaiveDateTime::from_timestamp(self.secs, self.nanos as u32).format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for Instant {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = validate_type(j_val, "gx:Instant")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;

        let naive = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ")
            .map_err(|e| GraphSonError::Parse(format!("cannot parse Instant: {e}")))?;
        Ok(Instant {
            secs: naive.timestamp(),
            nanos: naive.nanosecond() as i32,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
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

#[test]
fn local_date_encode() {
    let expected = [0x84, 0x0, 0x0, 0x0, 0x7, 0xE6, 6, 13];

    let mut buf = vec![];
    NaiveDate::from_ymd(2022, 6, 13).encode(&mut buf).unwrap();

    assert_eq!(buf, expected)
}

#[test]
fn local_date_decode() {
    let buf = vec![0x84, 0x0, 0x0, 0x0, 0x7, 0xE6, 6, 13];

    let expected = NaiveDate::from_ymd(2022, 6, 13);
    let res = NaiveDate::decode(&mut &buf[..]).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn local_date_encode_v3() {
    let expected = r#"{"@type":"gx:LocalDate","@value":"2022-06-13"}"#;

    let v = NaiveDate::from_ymd(2022, 6, 13).encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn local_date_decode_v3() {
    let s = r#"{
        "@type" : "gx:LocalDate",
        "@value" : "2022-06-13"
      }"#;

    let v = serde_json::from_str(s).unwrap();
    let res = NaiveDate::decode_v3(&v).unwrap();
    assert_eq!(res, NaiveDate::from_ymd(2022, 6, 13))
}

#[test]
fn local_time_encode() {
    let expected = [0x86, 0x0, 0, 0, 0x3, 0x46, 0x30, 0xb8, 0xa0, 0];

    let mut buf = vec![];
    NaiveTime::from_num_seconds_from_midnight(3600, 0)
        .encode(&mut buf)
        .unwrap();

    assert_eq!(buf, expected)
}

#[test]
fn local_time_decode() {
    let buf = vec![0x86, 0x0, 0, 0, 0x3, 0x46, 0x30, 0xb8, 0xa0, 0];

    let expected = NaiveTime::from_num_seconds_from_midnight(3600, 0);
    let res = NaiveTime::decode(&mut &buf[..]).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn local_time_encode_v3() {
    let expected = r#"{"@type":"gx:LocalTime","@value":"12:30:45"}"#;

    let v = NaiveTime::from_hms_nano(12, 30, 45, 123123).encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn local_time_decode_v3() {
    let s = r#"{
        "@type" : "gx:LocalTime",
        "@value" : "12:30:45"
      }"#;

    let v = serde_json::from_str(s).unwrap();
    let res = NaiveTime::decode_v3(&v).unwrap();
    assert_eq!(res, NaiveTime::from_hms(12, 30, 45))
}

#[test]
fn local_date_time_encode_v3() {
    let expected = r#"{"@type":"gx:LocalDateTime","@value":"2016-01-01T12:30"}"#;

    let v = NaiveDateTime::new(
        NaiveDate::from_ymd(2016, 1, 1),
        NaiveTime::from_hms(12, 30, 2),
    )
    .encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn local_date_time_decode_v3() {
    let s = r#"{
        "@type" : "gx:LocalDateTime",
        "@value" : "2016-01-01T12:30"
      }"#;

    let v = serde_json::from_str(s).unwrap();
    let res = NaiveDateTime::decode_v3(&v).unwrap();
    assert_eq!(
        res,
        NaiveDateTime::new(
            NaiveDate::from_ymd(2016, 1, 1),
            NaiveTime::from_hms(12, 30, 0),
        )
    )
}

#[test]
fn offset_encode_v3() {
    let expected = r#"{"@type":"gx:ZoneOffset","@value":"+03:06:09"}"#;

    let v = FixedOffset::east(3600 * 3 + 60 * 6 + 9).encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn offset_decode_v3() {
    let s = r#"{
        "@type" : "gx:ZoneOffset",
        "@value" : "+03:06:09"
      }"#;

    let v = serde_json::from_str(s).unwrap();
    let res = FixedOffset::decode_v3(&v).unwrap();
    assert_eq!(res, FixedOffset::east(3600 * 3 + 60 * 6 + 9))
}

#[test]
fn time_offset_encode_v3() {
    let expected = r#"{"@type":"gx:OffsetTime","@value":"10:15:30+01:00"}"#;

    let v = OffsetTime {
        time: NaiveTime::from_hms(10, 15, 30),
        offset: FixedOffset::west(-3600),
    }
    .encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn time_offset_decode_v3() {
    let s = r#"{"@type":"gx:OffsetTime","@value":"10:15:30+01:00"}"#;

    let v = serde_json::from_str(s).unwrap();
    let res = OffsetTime::decode_v3(&v).unwrap();
    assert_eq!(
        res,
        OffsetTime {
            time: NaiveTime::from_hms(10, 15, 30),
            offset: FixedOffset::west(-3600)
        }
    )
}

#[test]
fn date_time_offset_encode() {
    let expected = [
        0x88, 0x0, 0x0, 0x0, 0x7, 0xE6, 6, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1c, 0x20,
    ];

    let mut buf = vec![];
    DateTime::parse_from_rfc3339("2022-06-13T00:00:00+02:00")
        .unwrap()
        .encode(&mut buf)
        .unwrap();

    assert_eq!(buf, expected)
}

#[test]
fn date_time_offset_decode() {
    let buf = vec![
        0x88, 0x0, 0x0, 0x0, 0x7, 0xE6, 6, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xe3, 0xe0,
    ];

    let expected = DateTime::parse_from_rfc3339("2022-06-13T00:00:00-02:00").unwrap();
    let res = DateTime::decode(&mut &buf[..]).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn date_time_offset_encode_v3() {
    let expected = r#"{"@type":"gx:OffsetDateTime","@value":"2007-12-03T10:15:30+01:00"}"#;

    let v = DateTime::parse_from_rfc3339("2007-12-03T10:15:30+01:00")
        .unwrap()
        .encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn date_time_offset_decode_v3() {
    let s = r#"{"@type":"gx:OffsetDateTime","@value":"2007-12-03T10:15:30+01:00"}"#;

    let v = serde_json::from_str(s).unwrap();
    let res = DateTime::decode_v3(&v).unwrap();
    assert_eq!(
        res,
        DateTime::parse_from_rfc3339("2007-12-03T10:15:30+01:00").unwrap()
    )
}

#[test]
fn zoned_date_time_offset_encode_v3() {
    let expected =
        r#"{"@type":"gx:ZonedDateTime","@value":"2016-12-23T12:12:24.000000034+02:00[GMT+02:00]"}"#;

    let v =
        ZonedDateTime(DateTime::parse_from_rfc3339("2016-12-23T12:12:24.000000034+02:00").unwrap())
            .encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn zoned_date_time_offset_decode_v3() {
    let s =
        r#"{"@type":"gx:ZonedDateTime","@value":"2016-12-23T12:12:24.000000000+02:00GMT+02:00"}"#;

    let v = serde_json::from_str(s).unwrap();
    let res = ZonedDateTime::decode_v3(&v).unwrap();
    assert_eq!(
        res,
        ZonedDateTime(DateTime::parse_from_rfc3339("2016-12-23T12:12:24.000000000+02:00").unwrap())
    )
}

#[test]
fn month_day_encode_v3() {
    let expected = r#"{"@type":"gx:MonthDay","@value":"--01-01"}"#;

    let v = MonthDay { month: 1, day: 1 }.encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn year_month_decode_v3() {
    let s = r#"{"@type":"gx:MonthDay","@value":"--01-21"}"#;

    let v = serde_json::from_str(s).unwrap();
    let res = MonthDay::decode_v3(&v).unwrap();
    assert_eq!(res, MonthDay { month: 1, day: 21 })
}

#[test]
fn month_day_decode_v3() {
    let s = r#"{"@type":"gx:YearMonth","@value":"2016-12"}"#;

    let v = serde_json::from_str(s).unwrap();
    let res = YearMonth::decode_v3(&v).unwrap();
    assert_eq!(
        res,
        YearMonth {
            year: 2016,
            month: 12,
        }
    )
}

#[test]
fn year_month_encode_v3() {
    let expected = r#"{"@type":"gx:YearMonth","@value":"2016-01"}"#;

    let v = YearMonth {
        year: 2016,
        month: 1,
    }
    .encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn duration_encode_v3() {
    let expected = r#"{"@type":"gx:Duration","@value":"P5DT2.001S"}"#;
    let v = (Duration::seconds(3600 * 24 * 5 + 2) + Duration::nanoseconds(1000 * 1000)).encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn duration_decode_v3() {
    let s = r#"{"@type":"gx:Duration","@value":"P5DT2.1S"}"#;
    let expected = Duration::seconds(3600 * 24 * 5 + 2) + Duration::nanoseconds(1000 * 1000 * 100);
    let v = serde_json::from_str(s).unwrap();
    let res = Duration::decode_v3(&v).unwrap();

    assert_eq!(res, expected);
}

#[test]
fn period_encode_v3() {
    let expected = r#"{"@type":"gx:Period","@value":"P2Y5M-1D"}"#;
    let v = Period::new(2, 5, -1).encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn period_zero_encode_v3() {
    let expected = r#"{"@type":"gx:Period","@value":"P0D"}"#;
    let v = Period::zero().encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn period_decode_v3() {
    let s = r#"{"@type":"gx:Period","@value":"P5Y2M-1D"}"#;
    let expected = Period::new(5, 2, -1);
    let v = serde_json::from_str(s).unwrap();
    let res = Period::decode_v3(&v).unwrap();

    assert_eq!(res, expected);
}

#[test]
fn instant_encode_v3() {
    let expected = r#"{"@type":"gx:Instant","@value":"2022-07-22T13:14:08.770323Z"}"#;
    let v = Instant {
        secs: 1658495648,
        nanos: 770323000,
    }
    .encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn instant_encode() {
    let expected = [
        0x83, 0x0, 0x0, 0x0, 0x0, 0x0, 0x62, 0xda, 0xa2, 0xa0, 0, 0, 0x0, 0x0,
    ];

    let mut buf = vec![];
    Instant::new(1658495648, 0).encode(&mut buf).unwrap();

    assert_eq!(buf, expected)
}

#[test]
fn instant_decode() {
    let buf = [
        0x83, 0x0, 0x0, 0x0, 0x0, 0x0, 0x62, 0xda, 0xa2, 0xa0, 0, 0, 0x0, 0x0,
    ];

    let res = Instant::decode(&mut &buf[..]).unwrap();
    let expected = Instant::new(1658495648, 0);

    assert_eq!(res, expected)
}

#[test]
fn instant_no_nanos_encode_v3() {
    let expected = r#"{"@type":"gx:Instant","@value":"2022-07-22T13:14:08Z"}"#;
    let v = Instant {
        secs: 1658495648,
        nanos: 0,
    }
    .encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn instant_decode_v3() {
    let s = r#"{"@type":"gx:Instant","@value":"2022-07-22T13:14:08.770323Z"}"#;
    let expected = Instant {
        secs: 1658495648,
        nanos: 770323000,
    };
    let v = serde_json::from_str(s).unwrap();
    let res = Instant::decode_v3(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn instant_no_nanos_decode_v3() {
    let s = r#"{"@type":"gx:Instant","@value":"2022-07-22T13:14:08Z"}"#;
    let expected = Instant {
        secs: 1658495648,
        nanos: 0,
    };
    let v = serde_json::from_str(s).unwrap();
    let res = Instant::decode_v3(&v).unwrap();
    assert_eq!(res, expected);
}
