use std::{collections::HashMap, fmt::Display};

use crate::conversion;
use crate::GremlinValue;

#[derive(Debug, PartialEq, Clone)]
pub struct Metrics {
    pub id: String,
    pub name: String,
    pub duration: i64,
    pub counts: HashMap<String, i64>,
    pub annotations: HashMap<String, GremlinValue>,
    pub nested_metrics: Vec<Metrics>,
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", build_string(self, 0))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraversalMetrics {
    pub duration: i64,
    pub metrics: Vec<Metrics>,
}

impl TraversalMetrics {
    #[must_use]
    pub fn new(dur: i64, metrics: Vec<Metrics>) -> TraversalMetrics {
        TraversalMetrics {
            duration: dur,
            metrics,
        }
    }
    #[must_use]
    pub fn duration(&self) -> &i64 {
        &self.duration
    }
    #[must_use]
    pub fn metrics(&self) -> &Vec<Metrics> {
        &self.metrics
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
        let s = (0..offset)
            .map(|_| ' ')
            .chain(time_string.chars())
            .collect::<String>();
        writeln!(f, "{s}")
    }
}

fn build_string(metrics: &Metrics, start_offset: usize) -> String {
    let mut result_string = String::with_capacity(109);

    let mut name = metrics.name.clone();
    if name.len() > 50 - start_offset {
        name.replace_range((47 - start_offset).., "...");
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
        .annotations
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

conversion!(TraversalMetrics, TraversalMetrics);
conversion!(Metrics, Metrics);

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
        annotations: HashMap::from([("percentDur".to_string(), 42.12312_f64.into())]),
        nested_metrics: vec![Metrics {
            id: "4.0.0()".to_string(),
            name: "TinkerGraphStep(vertex,[1])".to_string(),
            duration: 10000000000,
            counts: HashMap::from([
                ("elementCount".to_string(), 1),
                ("traverserCount".to_string(), 1),
            ]),
            annotations: HashMap::new(),
            nested_metrics: vec![Metrics {
                id: "4.0.0()".to_string(),
                name: "TinkerGraphStep(vertex,[1]893749182739817239817aa)".to_string(),
                duration: 1238123,
                counts: HashMap::from([
                    ("elementCount".to_string(), 2),
                    ("traverserCount".to_string(), 3),
                ]),
                annotations: HashMap::new(),
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
        annotations: HashMap::from([("percentDur".to_string(), 0_f64.into())]),
        nested_metrics: vec![Metrics {
            id: "4.0.0()".to_string(),
            name: "TinkerGraphStep(vertex,[1])".to_string(),
            duration: 1238123,
            counts: HashMap::from([
                ("elementCount".to_string(), 1),
                ("traverserCount".to_string(), 1),
            ]),
            annotations: HashMap::new(),
            nested_metrics: Vec::new(),
        }],
    };

    let expected = TraversalMetrics {
        duration: 123823123,
        metrics: vec![metric, metric2],
    };
    println!("{expected}");
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
        annotations: HashMap::from([("percentDur".to_string(), 19.11682037524559_f64.into())]),
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
        annotations: HashMap::from([("percentDur".to_string(), 19.11682037524559_f64.into())]),
        nested_metrics: Vec::new(),
    };
    println!("{}", build_string(&metric, 0));
    println!("{}", build_string(&metric2, 0))
}
