use std::collections::HashMap;

use crate::{
    de::from_slice,
    graph_binary::{Decode, Encode, GraphBinary},
    specs::CoreType,
    struct_deserialize,
};

#[derive(Debug, PartialEq)]
pub struct Metrics {
    id: String,
    name: String,
    duration: i64,
    counts: HashMap<String, i64>,
    annotation: HashMap<String, GraphBinary>,
    nested_metrics: Vec<Metrics>,
}

impl From<Metrics> for GraphBinary {
    fn from(metrics: Metrics) -> Self {
        GraphBinary::Metrics(metrics)
    }
}

impl Encode for Metrics {
    fn type_code() -> u8 {
        CoreType::Metrics.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.id.write_patial_bytes(writer)?;
        self.name.write_patial_bytes(writer)?;
        self.duration.write_patial_bytes(writer)?;
        self.counts.write_patial_bytes(writer)?;
        self.annotation.write_patial_bytes(writer)?;
        self.nested_metrics.write_patial_bytes(writer)
    }
}

impl Decode for Metrics {
    fn expected_type_code() -> u8 {
        CoreType::Metrics.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let id = String::partial_decode(reader)?;
        let name = String::partial_decode(reader)?;
        let duration = i64::partial_decode(reader)?;
        let counts = HashMap::<String, i64>::partial_decode(reader)?;
        let annotation = HashMap::<String, GraphBinary>::partial_decode(reader)?;
        let nested_metrics = Vec::<Metrics>::partial_decode(reader)?;

        Ok(Metrics {
            id,
            name,
            duration,
            counts,
            annotation,
            nested_metrics,
        })
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = String::partial_count_bytes(bytes)?;
        len += String::partial_count_bytes(&bytes[len..])?;
        len += i64::partial_count_bytes(&bytes[len..])?;
        len += HashMap::<String, i64>::partial_count_bytes(&bytes[len..])?;
        len += HashMap::<String, GraphBinary>::partial_count_bytes(&bytes[len..])?;
        len += Vec::<Metrics>::partial_count_bytes(&bytes[len..])?;
        Ok(len)
    }
}

#[derive(Debug, PartialEq)]
pub struct TraversalMetrics {
    duration: i64,
    metrics: Vec<Metrics>,
}

impl From<TraversalMetrics> for GraphBinary {
    fn from(metrics: TraversalMetrics) -> Self {
        GraphBinary::TraversalMetrics(metrics)
    }
}

impl Encode for TraversalMetrics {
    fn type_code() -> u8 {
        CoreType::TraversalMetrics.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.duration.write_patial_bytes(writer)?;
        self.metrics.write_patial_bytes(writer)
    }
}

impl Decode for TraversalMetrics {
    fn expected_type_code() -> u8 {
        CoreType::TraversalMetrics.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let duration = i64::partial_decode(reader)?;
        let metrics = Vec::<Metrics>::partial_decode(reader)?;

        Ok(TraversalMetrics { duration, metrics })
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = i64::partial_count_bytes(bytes)?;
        len += Vec::<Metrics>::partial_count_bytes(&bytes[len..])?;
        Ok(len)
    }
}

struct_deserialize!(
    (TraversalMetrics, TraversalMetricsVisitor),
    (Metrics, MetricsVisitor)
);

#[test]
fn metric_encode_test() {
    let metric = Metrics {
        id: "4.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[1])".to_string(),
        duration: 1,
        counts: HashMap::from([
            // ("traverserCount".to_string(), 1),
            ("elementCount".to_string(), 1),
        ]),
        annotation: HashMap::from([("percentDur".to_string(), 0_f64.into())]),
        nested_metrics: Vec::new(),
    };
    let mut buf = vec![];
    metric.write_full_qualified_bytes(&mut buf).unwrap();

    let msg = [
        0x2c, 0x0, 0x0, 0x0, 0x0, 0x7, 0x34, 0x2e, 0x30, 0x2e, 0x30, 0x28, 0x29, 0x0, 0x0, 0x0,
        0x1b, 0x54, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x47, 0x72, 0x61, 0x70, 0x68, 0x53, 0x74, 0x65,
        0x70, 0x28, 0x76, 0x65, 0x72, 0x74, 0x65, 0x78, 0x2c, 0x5b, 0x31, 0x5d, 0x29, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x00, 0x00, 0x01, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0xc, 0x65,
        0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x2, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0xa, 0x70, 0x65,
        0x72, 0x63, 0x65, 0x6e, 0x74, 0x44, 0x75, 0x72, 0x7, 0x0, 0x00, 0x00, 0x00, 0x00, 0x0,
        0x00, 0x00, 0x00, 0x0, 0x0, 0x0, 0x0,
    ];

    assert_eq!(&msg[..], &buf)
}

#[test]
fn metric_decode_test() {
    let expected = Metrics {
        id: "4.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[1])".to_string(),
        duration: 1,
        counts: HashMap::from([
            // ("traverserCount".to_string(), 1),
            ("elementCount".to_string(), 1),
        ]),
        annotation: HashMap::from([("percentDur".to_string(), 0_f64.into())]),
        nested_metrics: Vec::new(),
    };

    let msg = vec![
        0x2c, 0x0, 0x0, 0x0, 0x0, 0x7, 0x34, 0x2e, 0x30, 0x2e, 0x30, 0x28, 0x29, 0x0, 0x0, 0x0,
        0x1b, 0x54, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x47, 0x72, 0x61, 0x70, 0x68, 0x53, 0x74, 0x65,
        0x70, 0x28, 0x76, 0x65, 0x72, 0x74, 0x65, 0x78, 0x2c, 0x5b, 0x31, 0x5d, 0x29, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x00, 0x00, 0x01, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0xc, 0x65,
        0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x2, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0xa, 0x70, 0x65,
        0x72, 0x63, 0x65, 0x6e, 0x74, 0x44, 0x75, 0x72, 0x7, 0x0, 0x00, 0x00, 0x00, 0x00, 0x0,
        0x00, 0x00, 0x00, 0x0, 0x0, 0x0, 0x0,
    ];

    let p = Metrics::fully_self_decode(&mut &msg[..]);

    assert_eq!(expected, p.unwrap());
}

#[test]
fn traversal_metric_encode_test() {
    let metric = Metrics {
        id: "4.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[1])".to_string(),
        duration: 1,
        counts: HashMap::from([("elementCount".to_string(), 1)]),
        annotation: HashMap::from([("percentDur".to_string(), 0_f64.into())]),
        nested_metrics: Vec::new(),
    };

    let traversal_metric = TraversalMetrics {
        duration: 214692,
        metrics: vec![metric],
    };
    let mut buf = vec![];
    traversal_metric
        .write_full_qualified_bytes(&mut buf)
        .unwrap();

    let msg = [
        0x2d, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x46, 0xa4, 0x0, 0x0, 0x0, 0x1, 0x2c, 0x0, 0x0,
        0x0, 0x0, 0x7, 0x34, 0x2e, 0x30, 0x2e, 0x30, 0x28, 0x29, 0x0, 0x0, 0x0, 0x1b, 0x54, 0x69,
        0x6e, 0x6b, 0x65, 0x72, 0x47, 0x72, 0x61, 0x70, 0x68, 0x53, 0x74, 0x65, 0x70, 0x28, 0x76,
        0x65, 0x72, 0x74, 0x65, 0x78, 0x2c, 0x5b, 0x31, 0x5d, 0x29, 0x0, 0x0, 0x0, 0x0, 0x0, 0x00,
        0x00, 0x01, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0xc, 0x65, 0x6c, 0x65, 0x6d, 0x65,
        0x6e, 0x74, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1,
        0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0xa, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74,
        0x44, 0x75, 0x72, 0x7, 0x0, 0x00, 0x00, 0x00, 0x00, 0x0, 0x00, 0x00, 0x00, 0x0, 0x0, 0x0,
        0x0,
    ];

    assert_eq!(&msg[..], &buf)
}

#[test]
fn metric_deser_test() {
    let expected = Metrics {
        id: "4.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[1])".to_string(),
        duration: 1,
        counts: HashMap::from([
            // ("traverserCount".to_string(), 1),
            ("elementCount".to_string(), 1),
        ]),
        annotation: HashMap::from([("percentDur".to_string(), 0_f64.into())]),
        nested_metrics: Vec::new(),
    };

    let msg = vec![
        0x2c, 0x0, 0x0, 0x0, 0x0, 0x7, 0x34, 0x2e, 0x30, 0x2e, 0x30, 0x28, 0x29, 0x0, 0x0, 0x0,
        0x1b, 0x54, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x47, 0x72, 0x61, 0x70, 0x68, 0x53, 0x74, 0x65,
        0x70, 0x28, 0x76, 0x65, 0x72, 0x74, 0x65, 0x78, 0x2c, 0x5b, 0x31, 0x5d, 0x29, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x00, 0x00, 0x01, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0xc, 0x65,
        0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x2, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0xa, 0x70, 0x65,
        0x72, 0x63, 0x65, 0x6e, 0x74, 0x44, 0x75, 0x72, 0x7, 0x0, 0x00, 0x00, 0x00, 0x00, 0x0,
        0x00, 0x00, 0x00, 0x0, 0x0, 0x0, 0x0,
    ];

    let p = from_slice(&msg);

    assert_eq!(expected, p.unwrap());

    let p = from_slice(&msg);

    assert_eq!(GraphBinary::Metrics(expected), p.unwrap());
}

#[test]
fn traversal_metric_decode_test() {
    let metric = Metrics {
        id: "4.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[1])".to_string(),
        duration: 1,
        counts: HashMap::from([("elementCount".to_string(), 1)]),
        annotation: HashMap::from([("percentDur".to_string(), 0_f64.into())]),
        nested_metrics: Vec::new(),
    };

    let expected = TraversalMetrics {
        duration: 1,
        metrics: vec![metric],
    };

    let msg = [
        0x2d, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x2c, 0x0, 0x0, 0x0,
        0x0, 0x7, 0x34, 0x2e, 0x30, 0x2e, 0x30, 0x28, 0x29, 0x0, 0x0, 0x0, 0x1b, 0x54, 0x69, 0x6e,
        0x6b, 0x65, 0x72, 0x47, 0x72, 0x61, 0x70, 0x68, 0x53, 0x74, 0x65, 0x70, 0x28, 0x76, 0x65,
        0x72, 0x74, 0x65, 0x78, 0x2c, 0x5b, 0x31, 0x5d, 0x29, 0x0, 0x0, 0x0, 0x0, 0x0, 0x00, 0x00,
        0x01, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0xc, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e,
        0x74, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0,
        0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0xa, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74,
        0x44, 0x75, 0x72, 0x7, 0x0, 0x00, 0x00, 0x00, 0x00, 0x0, 0x00, 0x00, 0x00, 0x0, 0x0, 0x0,
        0x0,
    ];

    let p = TraversalMetrics::fully_self_decode(&mut &msg[..]);

    assert_eq!(expected, p.unwrap());
}

#[test]
fn traversal_metric_deser_test() {
    let metric = Metrics {
        id: "4.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[1])".to_string(),
        duration: 1,
        counts: HashMap::from([("elementCount".to_string(), 1)]),
        annotation: HashMap::from([("percentDur".to_string(), 0_f64.into())]),
        nested_metrics: Vec::new(),
    };

    let expected = TraversalMetrics {
        duration: 1,
        metrics: vec![metric],
    };

    let msg = [
        0x2d_u8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x2c, 0x0, 0x0,
        0x0, 0x0, 0x7, 0x34, 0x2e, 0x30, 0x2e, 0x30, 0x28, 0x29, 0x0, 0x0, 0x0, 0x1b, 0x54, 0x69,
        0x6e, 0x6b, 0x65, 0x72, 0x47, 0x72, 0x61, 0x70, 0x68, 0x53, 0x74, 0x65, 0x70, 0x28, 0x76,
        0x65, 0x72, 0x74, 0x65, 0x78, 0x2c, 0x5b, 0x31, 0x5d, 0x29, 0x0, 0x0, 0x0, 0x0, 0x0, 0x00,
        0x00, 0x01, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0xc, 0x65, 0x6c, 0x65, 0x6d, 0x65,
        0x6e, 0x74, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1,
        0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0xa, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74,
        0x44, 0x75, 0x72, 0x7, 0x0, 0x00, 0x00, 0x00, 0x00, 0x0, 0x00, 0x00, 0x00, 0x0, 0x0, 0x0,
        0x0,
    ];

    let p = from_slice(&msg);

    assert_eq!(expected, p.unwrap());

    let p = from_slice(&msg);

    assert_eq!(GraphBinary::TraversalMetrics(expected), p.unwrap());
}
