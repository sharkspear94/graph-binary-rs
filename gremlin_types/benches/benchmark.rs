use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gremlin_types::{
    binary::{Decode, Encode},
    graphson::DecodeGraphSON,
    structure::metrics::TraversalMetrics,
    GremlinValue,
};

pub fn criterion_benchmark_graphson(c: &mut Criterion) {
    let value = r#"{"@type":"g:TraversalMetrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":0.004},"metrics",{"@type":"g:List","@value":[{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":4},"elementCount",{"@type":"g:Int64","@value":4}]},"name","TinkerGraphStep(vertex,[~label.eq(person)])","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","7.0.0()"]}},{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":13},"elementCount",{"@type":"g:Int64","@value":13}]},"name","VertexStep(OUT,vertex)","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","2.0.0()"]}},{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":7},"elementCount",{"@type":"g:Int64","@value":7}]},"name","VertexStep(OUT,vertex)","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","3.0.0()"]}},{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":1},"elementCount",{"@type":"g:Int64","@value":1}]},"name","TreeStep","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","4.0.0()"]}}]}]}}"#;

    c.bench_function("from_slice metrics", |b| {
        b.iter(|| GremlinValue::decode_v3(black_box(&serde_json::from_str(value).unwrap())))
    });
}

pub fn criterion_benchmark_graphbinary(c: &mut Criterion) {
    let value = serde_json::from_str(r#"{"@type":"g:TraversalMetrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":0.004},"metrics",{"@type":"g:List","@value":[{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":4},"elementCount",{"@type":"g:Int64","@value":4}]},"name","TinkerGraphStep(vertex,[~label.eq(person)])","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","7.0.0()"]}},{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":13},"elementCount",{"@type":"g:Int64","@value":13}]},"name","VertexStep(OUT,vertex)","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","2.0.0()"]}},{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":7},"elementCount",{"@type":"g:Int64","@value":7}]},"name","VertexStep(OUT,vertex)","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","3.0.0()"]}},{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":1},"elementCount",{"@type":"g:Int64","@value":1}]},"name","TreeStep","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","4.0.0()"]}}]}]}}"#).expect("Error parsing json");
    let x = TraversalMetrics::decode_v3(&value).unwrap();
    let mut buf = vec![];
    x.encode(&mut buf).unwrap();
    c.bench_function("from_slice metrics", |b| {
        b.iter(|| GremlinValue::decode(black_box(&mut &buf[..])))
    });
}

criterion_group!(
    benches,
    criterion_benchmark_graphson,
    criterion_benchmark_graphbinary
);
criterion_main!(benches);
