use std::io::{Read, Write};

use chrono::{
    DateTime, Datelike, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Timelike,
};

use crate::{
    error::{DecodeError, EncodeError},
    extended::chrono::{Instant, MonthDay, OffsetTime, Period, Year, YearMonth, ZonedDateTime},
    specs::CoreType,
};

use super::{Decode, Encode};

impl Encode for Duration {
    fn type_code() -> u8 {
        CoreType::Duration.into()
    }

    fn partial_encode<W: Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        let s = self
            .to_std()
            .map_err(|e| EncodeError::Serilization(format!("duration out of range: {e}")))?;
        let a = i64::try_from(s.as_secs())
            .map_err(|e| EncodeError::Serilization(format!("cannot convert u64 to i64: {e}")))?;
        a.partial_encode(writer)?;
        (s.subsec_nanos() as i32).partial_encode(writer)
    }
}

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

impl Encode for Instant {
    fn type_code() -> u8 {
        CoreType::Instant.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        self.secs.partial_encode(writer)?;
        self.nanos.partial_encode(writer)
    }
}

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
fn local_date_encode() {
    let expected = [0x84, 0x0, 0x0, 0x0, 0x7, 0xE6, 6, 13];

    let mut buf = vec![];
    NaiveDate::from_ymd_opt(2022, 6, 13)
        .expect("invalid or out-of-range date")
        .encode(&mut buf)
        .unwrap();

    assert_eq!(buf, expected)
}

#[test]
fn local_date_decode() {
    let buf = vec![0x84, 0x0, 0x0, 0x0, 0x7, 0xE6, 6, 13];

    let expected = NaiveDate::from_ymd_opt(2022, 6, 13).expect("invalid or out-of-range date");
    let res = NaiveDate::decode(&mut &buf[..]).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn local_time_encode() {
    let expected = [0x86, 0x0, 0, 0, 0x3, 0x46, 0x30, 0xb8, 0xa0, 0];

    let mut buf = vec![];
    NaiveTime::from_num_seconds_from_midnight_opt(3600, 0)
        .expect("invalid time")
        .encode(&mut buf)
        .unwrap();

    assert_eq!(buf, expected)
}

#[test]
fn local_time_decode() {
    let buf = vec![0x86, 0x0, 0, 0, 0x3, 0x46, 0x30, 0xb8, 0xa0, 0];

    let expected = NaiveTime::from_num_seconds_from_midnight_opt(3600, 0).expect("invalid time");
    let res = NaiveTime::decode(&mut &buf[..]).unwrap();
    assert_eq!(res, expected)
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
