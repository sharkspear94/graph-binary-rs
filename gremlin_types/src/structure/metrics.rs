use std::{collections::HashMap, fmt::Display};

use crate::{
    conversions,
    graph_binary::{Decode, Encode, GremlinTypes},
    specs::CoreType,
    struct_de_serialize,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Metrics {
    id: String,
    name: String,
    duration: i64,
    counts: HashMap<String, i64>,
    annotation: HashMap<String, GremlinTypes>,
    nested_metrics: Vec<Metrics>,
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", build_string(self, 0))
    }
}

impl Encode for Metrics {
    fn type_code() -> u8 {
        CoreType::Metrics.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.id.partial_encode(writer)?;
        self.name.partial_encode(writer)?;
        self.duration.partial_encode(writer)?;
        self.counts.partial_encode(writer)?;
        self.annotation.partial_encode(writer)?;
        self.nested_metrics.partial_encode(writer)
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
        let annotation = HashMap::<String, GremlinTypes>::partial_decode(reader)?;
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

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = String::get_partial_len(bytes)?;
        len += String::get_partial_len(&bytes[len..])?;
        len += i64::get_partial_len(&bytes[len..])?;
        len += HashMap::<String, i64>::get_partial_len(&bytes[len..])?;
        len += HashMap::<String, GremlinTypes>::get_partial_len(&bytes[len..])?;
        len += Vec::<Metrics>::get_partial_len(&bytes[len..])?;
        Ok(len)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraversalMetrics {
    pub duration: i64,
    pub metrics: Vec<Metrics>,
}

#[test]
fn test_build_string() {
    let metric = Metrics {
        id: "4.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[1])".to_string(),
        duration: 206082,
        counts: HashMap::from([
            ("traverserCount".to_string(), 1),
            ("elementCount".to_string(), 1),
        ]),
        annotation: HashMap::from([("percentDur".to_string(), 19.11682037524559_f64.into())]),
        nested_metrics: Vec::new(),
    };
    let metric2 = Metrics {
        id: "4.0.0()".to_string(),
        name: "TraversalFilterStep([VertexStep(OUT,vertex), HasLabel(asdasd)".to_string(),
        duration: 3206082,
        counts: HashMap::from([
            ("traverserCount".to_string(), 11),
            ("elementCount".to_string(), 1123),
        ]),
        annotation: HashMap::from([("percentDur".to_string(), 19.11682037524559_f64.into())]),
        nested_metrics: Vec::new(),
    };
    println!("{}", build_string(&metric, 0));
    println!("{}", build_string(&metric2, 0))
}

impl Encode for TraversalMetrics {
    fn type_code() -> u8 {
        CoreType::TraversalMetrics.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.duration.partial_encode(writer)?;
        self.metrics.partial_encode(writer)
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

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = i64::get_partial_len(bytes)?;
        len += Vec::<Metrics>::get_partial_len(&bytes[len..])?;
        Ok(len)
    }
}

impl Display for TraversalMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"Step                                                               Count  Traversers       Time (ms)    % Dur")?;
        writeln!(f,"=============================================================================================================")?;
        for metrics in &self.metrics {
            writeln!(f, "{metrics}",)?;
        }
        write!(
            f,
            "                                            >TOTAL                     -           -"
        )?;
        let time_string = format!("{:.3}        -", self.duration as f64 / 1000. / 1000.); // from ns to ms
        let offset = 25 - time_string.len();
        let s = String::from_iter((0..offset).map(|_| ' ').chain(time_string.chars()));
        writeln!(f, "{s}")
    }
}

fn build_string(metrics: &Metrics, start_offset: usize) -> String {
    let mut result_string = String::with_capacity(109);

    let mut name = metrics.name.clone();
    if name.len() > 50 - start_offset {
        name.replace_range((47 - start_offset).., "...")
    }
    result_string.extend((0..start_offset).map(|_| ' ').chain(name.chars()));
    let element_count = metrics
        .counts
        .get("elementCount")
        .map(ToString::to_string)
        .unwrap_or_default();
    let offset = 72 - name.len() - start_offset - element_count.len(); // offset to Count
    result_string.extend((0..offset).map(|_| ' ').chain(element_count.chars()));

    let traveser_count = metrics
        .counts
        .get("traverserCount")
        .map(ToString::to_string)
        .unwrap_or_default();
    let offset = 12 - traveser_count.len(); // offset to Traversers
    result_string.extend((0..offset).map(|_| ' ').chain(traveser_count.chars()));

    let time_string = format!("{:.3}", metrics.duration as f64 / 1000. / 1000.);
    let offset = 16 - time_string.len(); // offset to Time
    result_string.extend((0..offset).map(|_| ' ').chain(time_string.chars()));

    let duration_string = metrics
        .annotation
        .get("percentDur")
        .and_then(|gb| gb.get_ref::<f64>())
        .map(|dur| format!("{dur:.2}"))
        .unwrap_or_default();
    let offset = 9 - duration_string.len();
    result_string.extend((0..offset).map(|_| ' ').chain(duration_string.chars()));

    for nested in &metrics.nested_metrics {
        result_string.push('\n');
        result_string += &build_string(nested, start_offset + 2);
    }
    result_string
}

struct_de_serialize!(
    (TraversalMetrics, TraversalMetricsVisitor, 128),
    (Metrics, MetricsVisitor, 64)
);

conversions!((TraversalMetrics, TraversalMetrics), (Metrics, Metrics));

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
    metric.encode(&mut buf).unwrap();

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
fn metric_json() {
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

    let json = serde_json::to_string_pretty(&expected).unwrap();

    print!("{json}")
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

    let p = Metrics::decode(&mut &msg[..]);

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
    traversal_metric.encode(&mut buf).unwrap();

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

    let p = TraversalMetrics::decode(&mut &msg[..]);

    assert_eq!(expected, p.unwrap());
}

#[test]
fn traversal_metric_display_test() {
    let metric = Metrics {
        id: "4.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[1]893749182739817239817aa)".to_string(),
        duration: 1234872,
        counts: HashMap::from([
            ("elementCount".to_string(), 111111),
            ("traverserCount".to_string(), 111111),
        ]),
        annotation: HashMap::from([("percentDur".to_string(), 42.12312_f64.into())]),
        nested_metrics: vec![Metrics {
            id: "4.0.0()".to_string(),
            name: "TinkerGraphStep(vertex,[1])".to_string(),
            duration: 10000000000,
            counts: HashMap::from([
                ("elementCount".to_string(), 1),
                ("traverserCount".to_string(), 1),
            ]),
            annotation: HashMap::new(),
            nested_metrics: vec![Metrics {
                id: "4.0.0()".to_string(),
                name: "TinkerGraphStep(vertex,[1]893749182739817239817aa)".to_string(),
                duration: 1238123,
                counts: HashMap::from([
                    ("elementCount".to_string(), 2),
                    ("traverserCount".to_string(), 3),
                ]),
                annotation: HashMap::new(),
                nested_metrics: Vec::new(),
            }],
        }],
    };

    let metric2 = Metrics {
        id: "4.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[1])".to_string(),
        duration: 100000,
        counts: HashMap::from([
            ("elementCount".to_string(), 1),
            ("traverserCount".to_string(), 1),
        ]),
        annotation: HashMap::from([("percentDur".to_string(), 0_f64.into())]),
        nested_metrics: vec![Metrics {
            id: "4.0.0()".to_string(),
            name: "TinkerGraphStep(vertex,[1])".to_string(),
            duration: 1238123,
            counts: HashMap::from([
                ("elementCount".to_string(), 1),
                ("traverserCount".to_string(), 1),
            ]),
            annotation: HashMap::new(),
            nested_metrics: Vec::new(),
        }],
    };

    let expected = TraversalMetrics {
        duration: 123823123,
        metrics: vec![metric, metric2],
    };
    println!("{expected}");
}
