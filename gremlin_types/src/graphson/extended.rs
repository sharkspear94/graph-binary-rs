use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use chrono::{
    format::{parse, Fixed, Item, Numeric, Pad, Parsed},
    DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Timelike,
};
use serde_json::json;

use crate::{
    error::GraphSonError,
    extended::chrono::{Instant, MonthDay, OffsetTime, Period, Year, YearMonth, ZonedDateTime},
};

use super::{validate_type, validate_type_entry, DecodeGraphSON, EncodeGraphSON};

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

impl EncodeGraphSON for Instant {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:Instant",
          "@value" : NaiveDateTime::from_timestamp_opt(self.secs, self.nanos as u32)
            .expect("NaiveDateTime out-of-range number of seconds and/or invalid nanosecond")
            .format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

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
            .ok()
            .and_then(|_| parsed.offset.and_then(|secs| FixedOffset::east_opt(secs)))
            .or_else(|| {
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
                .ok()
                .and_then(|_| {
                    parsed
                        .offset
                        .and_then(|secs| parsed.second.map(|s| s as i32 + secs))
                        .and_then(|secs| FixedOffset::east_opt(secs))
                })
            })
            .ok_or_else(|| GraphSonError::Parse(format!("cannot parse FixedOffset")))
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

impl DecodeGraphSON for ZonedDateTime {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let mut s = validate_type(j_val, "gx:ZonedDateTime")?
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;
        if let Some(len) = s.find('[') {
            s = &s[..len];
        } else {
            return Err(GraphSonError::Parse(format!(
                "cannot parse ZonedDateTime: [ not found"
            )));
        }

        let dt =
            DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f%:z") //FIXME
                // .or_else(|_| DateTime::parse_from_rfc3339(s))
                // .or_else(|_| DateTime::parse_from_rfc2822(s))
                .map_err(|err| {
                    GraphSonError::Parse(format!("cannot parse ZonedDateTime: {err}"))
                })?;
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

impl DecodeGraphSON for char {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "gx:Char"))
            .and_then(|map| map.get("@value"))
            .and_then(|value| value.as_str())
            .and_then(|s| s.chars().next()) //FIXME more than 1 char is not evaluated
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_str()
            .and_then(|s| s.chars().next())
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))
    }
}

impl EncodeGraphSON for char {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type": "gx:Char",
            "@value": self
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!({
            "@type": "gx:Char",
            "@value": self
        })
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl EncodeGraphSON for IpAddr {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type" : "gx:InetAddress",
            "@value" : self.to_string()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!({
            "@type" : "gx:InetAddress",
            "@value" : self.to_string()
        })
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for IpAddr {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "gx:InetAddress")?;

        match value_object
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?
        {
            "localhost" => Ok(IpAddr::V4(Ipv4Addr::LOCALHOST)),
            other => IpAddr::from_str(other).map_err(|e| GraphSonError::Parse(e.to_string())),
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

#[test]
fn local_date_encode_v3() {
    let expected = r#"{"@type":"gx:LocalDate","@value":"2022-06-13"}"#;

    let v = NaiveDate::from_ymd_opt(2022, 6, 13)
        .expect("invalid or out-of-range date")
        .encode_v3();
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
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(2022, 6, 13).expect("invalid or out-of-range date")
    )
}

#[test]
fn local_time_encode_v3() {
    let expected = r#"{"@type":"gx:LocalTime","@value":"12:30:45"}"#;

    let v = NaiveTime::from_hms_nano_opt(12, 30, 45, 123123)
        .expect("invalid time")
        .encode_v3();
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
    assert_eq!(
        res,
        NaiveTime::from_hms_opt(12, 30, 45).expect("invalid time")
    )
}

#[test]
fn local_date_time_encode_v3() {
    let expected = r#"{"@type":"gx:LocalDateTime","@value":"2016-01-01T12:30"}"#;

    let v = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2016, 1, 1).expect("invalid or out-of-range date"),
        NaiveTime::from_hms_opt(12, 30, 2).expect("invalid time"),
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
            NaiveDate::from_ymd_opt(2016, 1, 1).expect("invalid or out-of-range date"),
            NaiveTime::from_hms_opt(12, 30, 0).expect("invalid time"),
        )
    )
}

#[test]
fn offset_encode_v3() {
    let expected = r#"{"@type":"gx:ZoneOffset","@value":"+03:06:09"}"#;

    let v = {
        let secs = 3600 * 3 + 60 * 6 + 9;
        FixedOffset::east_opt(secs).expect("FixedOffset::east out of bounds")
    }
    .encode_v3();
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
    assert_eq!(
        res,
        FixedOffset::east_opt(3600 * 3 + 60 * 6 + 9).expect("FixedOffset::east out of bounds")
    )
}

#[test]
fn time_offset_encode_v3() {
    let expected = r#"{"@type":"gx:OffsetTime","@value":"10:15:30+01:00"}"#;

    let v = OffsetTime {
        time: NaiveTime::from_hms_opt(10, 15, 30).expect("invalid time"),
        offset: {
            let secs = -3600;
            FixedOffset::west_opt(secs).expect("FixedOffset::west out of bounds")
        },
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
            time: NaiveTime::from_hms_opt(10, 15, 30).expect("invalid time"),
            offset: FixedOffset::west_opt(-3600).expect("FixedOffset::west out of bounds")
        }
    )
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
        r#"{"@type":"gx:ZonedDateTime","@value":"2016-12-23T12:12:24.000000036+02:00[GMT+02:00]"}"#;

    let v = serde_json::from_str(s).unwrap();
    let res = ZonedDateTime::decode_v3(&v).unwrap();
    assert_eq!(
        res,
        ZonedDateTime(DateTime::parse_from_rfc3339("2016-12-23T12:12:24.000000036+02:00").unwrap())
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

#[test]
fn ip_encode_v3() {
    let expected = r#"{"@type":"gx:InetAddress","@value":"167.123.5.1"}"#;

    let v = IpAddr::from_str("167.123.5.1").unwrap().encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn ip_decode_v3() {
    let s = r#"{
        "@type" : "gx:InetAddress",
        "@value" : "localhost"
      }"#;

    let v = serde_json::from_str(s).unwrap();
    let res = IpAddr::decode_v3(&v).unwrap();
    assert_eq!(res, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
}

#[test]
fn ip_v6_decode_v3() {
    use std::net::Ipv6Addr;
    let s = r#"{
        "@type" : "gx:InetAddress",
        "@value" : "2001:0db8:85a3:08d3:1319:8a2e:0370:7347"
      }"#;

    let v = serde_json::from_str(s).unwrap();
    let res = IpAddr::decode_v3(&v).unwrap();
    assert_eq!(
        res,
        IpAddr::V6(Ipv6Addr::from_str("2001:0db8:85a3:08d3:1319:8a2e:0370:7347").unwrap())
    )
}
