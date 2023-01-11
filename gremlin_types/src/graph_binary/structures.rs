use std::collections::HashMap;

use bigdecimal::BigDecimal;
use num::BigInt;

use crate::{
    error::{DecodeError, EncodeError},
    specs::CoreType,
    structure::{
        bulkset::BulkSet,
        bytebuffer::ByteBuffer,
        bytecode::{Bytecode, Source, Step},
        edge::Edge,
        graph::{Graph, GraphEdge},
        lambda::Lambda,
        map::MapKeys,
        metrics::{Metrics, TraversalMetrics},
        path::Path,
        property::{EitherParent, Property},
        set::Set,
        traverser::{TraversalStrategy, Traverser},
        vertex::Vertex,
        vertex_property::VertexProperty,
    },
    Binding, GremlinValue,
};

use super::{Decode, Encode, ValueFlag};

impl Encode for BigInt {
    fn type_code() -> u8 {
        CoreType::BigInteger.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let bytes = self.to_signed_bytes_be();
        let len = bytes.len() as i32;
        len.partial_encode(writer)?;
        writer.write_all(&bytes)?;
        Ok(())
    }
}

impl Decode for BigInt {
    fn expected_type_code() -> u8 {
        CoreType::BigInteger.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)?;
        let mut buf = vec![0; len as usize];
        reader.read_exact(&mut buf)?;
        Ok(BigInt::from_signed_bytes_be(&buf))
    }
}

impl Encode for BigDecimal {
    fn type_code() -> u8 {
        CoreType::BigDecimal.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let (big_int, scale) = self.as_bigint_and_exponent();
        (scale as i32).partial_encode(writer)?;
        big_int.partial_encode(writer)
    }
}

impl Decode for BigDecimal {
    fn expected_type_code() -> u8 {
        CoreType::BigDecimal.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let scale = i32::partial_decode(reader)?;
        let big_int = BigInt::partial_decode(reader)?;

        Ok(BigDecimal::new(big_int, scale as i64))
    }
}

impl Encode for Binding {
    fn type_code() -> u8 {
        CoreType::Binding.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.key.partial_encode(writer)?;
        self.value.encode(writer)
    }
}

impl Decode for Binding {
    fn expected_type_code() -> u8 {
        CoreType::Binding.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let key = String::partial_decode(reader)?;
        let value = Box::new(GremlinValue::decode(reader)?);

        Ok(Binding { key, value })
    }
}

impl Encode for BulkSet {
    fn type_code() -> u8 {
        CoreType::BulkSet.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let vec_len = self.0.len() as i32;
        vec_len.partial_encode(writer)?;
        for (gb, bulk) in &self.0 {
            gb.encode(writer)?;
            bulk.partial_encode(writer)?;
        }
        Ok(())
    }
}

impl Decode for BulkSet {
    fn expected_type_code() -> u8 {
        CoreType::BulkSet.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)?;
        let len = usize::try_from(len)?;
        let mut items = Vec::with_capacity(len);
        for _ in 0..len {
            let gb = GremlinValue::decode(reader)?;
            let bulk = i64::partial_decode(reader)?;
            items.push((gb, bulk));
        }
        Ok(BulkSet(items))
    }
}

impl Encode for ByteBuffer {
    fn type_code() -> u8 {
        CoreType::ByteBuffer.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let len = self.0.len() as i32;
        len.partial_encode(writer)?;
        writer.write_all(&self.0)?;
        Ok(())
    }
}

impl Decode for ByteBuffer {
    fn expected_type_code() -> u8 {
        CoreType::ByteBuffer.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)? as usize;
        let mut buffer = vec![0; len];
        reader.read_exact(&mut buffer)?;
        Ok(ByteBuffer(buffer))
    }
}

impl Encode for Bytecode {
    fn type_code() -> u8 {
        CoreType::ByteCode.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let len = self.steps.len() as i32;
        len.partial_encode(writer)?;
        for step in &self.steps {
            step.name.partial_encode(writer)?;
            step.values.partial_encode(writer)?;
        }
        let len = self.sources.len() as i32;
        len.partial_encode(writer)?;
        for source in &self.sources {
            source.name.partial_encode(writer)?;
            source.values.partial_encode(writer)?;
        }
        Ok(())
    }
}
impl Decode for Bytecode {
    fn expected_type_code() -> u8 {
        CoreType::ByteCode.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)? as usize;
        let mut steps = Vec::with_capacity(len);
        for _ in 0..len {
            let name = String::partial_decode(reader)?;
            let values = Vec::<GremlinValue>::partial_decode(reader)?;
            steps.push(Step { name, values });
        }

        let len = i32::partial_decode(reader)? as usize;

        let mut sources = Vec::with_capacity(len);
        for _ in 0..len {
            let name = String::partial_decode(reader)?;
            let values = Vec::<GremlinValue>::partial_decode(reader)?;
            sources.push(Source { name, values });
        }

        Ok(Bytecode { steps, sources })
    }
}

impl Encode for Edge {
    fn type_code() -> u8 {
        CoreType::Edge.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.id.encode(writer)?;
        self.label.partial_encode(writer)?;
        self.in_v_id.encode(writer)?;
        self.in_v_label.partial_encode(writer)?;
        self.out_v_id.encode(writer)?;
        self.out_v_label.partial_encode(writer)?;
        self.parent.encode(writer)?;
        self.properties.encode(writer)
    }
}

impl Decode for Edge {
    fn expected_type_code() -> u8 {
        CoreType::Edge.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let id = GremlinValue::decode(reader)?;
        let label = String::partial_decode(reader)?;
        let in_v_id = GremlinValue::decode(reader)?;
        let in_v_label = String::partial_decode(reader)?;
        let out_v_id = GremlinValue::decode(reader)?;
        let out_v_label = String::partial_decode(reader)?;
        let parent = Option::<Vertex>::decode(reader)?;
        let properties = Option::<Vec<Property>>::decode(reader)?;

        Ok(Edge {
            id: Box::new(id),
            label,
            in_v_id: Box::new(in_v_id),
            in_v_label,
            out_v_id: Box::new(out_v_id),
            out_v_label,
            parent,
            properties,
        })
    }
}

impl Decode for GraphEdge {
    fn expected_type_code() -> u8 {
        unimplemented!("GraphEdge is not a valid GraphBinary Type")
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let id = GremlinValue::decode(reader)?;
        let label = String::partial_decode(reader)?;
        let in_v_id = GremlinValue::decode(reader)?;
        let in_v_label = Option::<String>::decode(reader)?;
        let out_v_id = GremlinValue::decode(reader)?;
        let out_v_label = Option::<String>::decode(reader)?;
        let parent = Option::<Vertex>::decode(reader)?;
        let properties = Vec::<Property>::partial_decode(reader)?;

        Ok(GraphEdge {
            id,
            label,
            in_v_id,
            in_v_label,
            out_v_id,
            out_v_label,
            parent,
            properties,
        })
    }
}

impl Encode for Graph {
    fn type_code() -> u8 {
        CoreType::Graph.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let v_len = self.vertices.len() as i32;
        let e_len = self.edges.len() as i32;

        v_len.partial_encode(writer)?;
        for vertex in &self.vertices {
            vertex.id.encode(writer)?;
            vertex.label.partial_encode(writer)?;
            if vertex.properties.is_some() {
                let p_len = vertex.properties.as_ref().unwrap().len() as i32;
                p_len.partial_encode(writer)?;
                for prop in vertex.properties.as_ref().unwrap() {
                    prop.id.encode(writer)?;
                    prop.label.partial_encode(writer)?;
                    prop.value.encode(writer)?;
                    prop.parent.encode(writer)?;
                    if prop.properties.is_some() {
                        prop.properties.as_ref().unwrap().partial_encode(writer)?;
                    } else {
                        prop.properties.encode(writer)?;
                    }
                }
            } else {
                None::<i32>.encode(writer)?;
            }
            // vertex.properties.write_patial_bytes(writer)?;
        }

        e_len.partial_encode(writer)?;
        for edge in &self.edges {
            edge.id.encode(writer)?;
            edge.label.partial_encode(writer)?;
            edge.in_v_id.encode(writer)?;
            edge.in_v_label.encode(writer)?;
            edge.out_v_id.encode(writer)?;
            edge.out_v_label.encode(writer)?;
            edge.parent.encode(writer)?;
            edge.properties.partial_encode(writer)?; // TODO not sure if prop identifier is needed
        }
        Ok(())
    }
}

impl Decode for Graph {
    fn expected_type_code() -> u8 {
        CoreType::Graph.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let v_len = i32::partial_decode(reader)? as usize;
        let mut v_vec = Vec::with_capacity(v_len);
        for _ in 0..v_len {
            let v_id = GremlinValue::decode(reader)?;
            let v_label = String::partial_decode(reader)?;
            let p_len = i32::partial_decode(reader)? as usize;
            let mut p_vec = Vec::with_capacity(p_len);
            for _ in 0..p_len {
                let p_id = GremlinValue::decode(reader)?;
                let p_label = String::partial_decode(reader)?;
                let p_value = GremlinValue::decode(reader)?;
                let p_parent = Option::<Vertex>::decode(reader)?;
                let p_properties = Option::<Vec<Property>>::partial_decode(reader)?;
                p_vec.push(VertexProperty {
                    id: Box::new(p_id),
                    label: p_label,
                    value: Box::new(p_value),
                    parent: p_parent,
                    properties: p_properties,
                });
            }
            v_vec.push(Vertex {
                id: Box::new(v_id),
                label: v_label,
                properties: Some(p_vec),
            });
        }
        let e_len = i32::partial_decode(reader)? as usize;
        let mut e_vec = Vec::with_capacity(v_len);
        for _ in 0..e_len {
            e_vec.push(GraphEdge::partial_decode(reader)?);
        }
        Ok(Graph {
            vertices: v_vec,
            edges: e_vec,
        })
    }
}

impl Encode for Lambda {
    fn type_code() -> u8 {
        CoreType::Lambda.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.language.partial_encode(writer)?;
        self.script.partial_encode(writer)?;
        self.arguments_length.partial_encode(writer)
    }
}

impl Decode for Lambda {
    fn expected_type_code() -> u8 {
        CoreType::Lambda.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let language = String::partial_decode(reader)?;
        let script = String::partial_decode(reader)?;
        let arguments_length = i32::partial_decode(reader)?;

        Ok(Lambda {
            language,
            script,
            arguments_length,
        })
    }
}

impl<T: Encode> Encode for Set<T> {
    fn type_code() -> u8 {
        CoreType::Set.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        self.set().partial_encode(writer)
    }
}

impl<T: Decode> Decode for Set<T> {
    fn expected_type_code() -> u8 {
        CoreType::Set.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Ok(Set::new(Vec::<T>::partial_decode(reader)?))
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
        self.annotations.partial_encode(writer)?;
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
        let annotation = HashMap::<String, GremlinValue>::partial_decode(reader)?;
        let nested_metrics = Vec::<Metrics>::partial_decode(reader)?;

        Ok(Metrics {
            id,
            name,
            duration,
            counts,
            annotations: annotation,
            nested_metrics,
        })
    }
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
}

impl Encode for Path {
    fn type_code() -> u8 {
        CoreType::Path.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        writer.write_all(&[CoreType::List.into(), 0x0])?;
        let len = i32::try_from(self.labels.len())?;
        len.partial_encode(writer)?;
        for set in &self.labels {
            writer.write_all(&[CoreType::Set.into(), 0x0])?;
            set.partial_encode(writer)?;
        }
        self.objects.encode(writer)
    }
}

impl Decode for Path {
    fn expected_type_code() -> u8 {
        CoreType::Path.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        reader.read_exact(&mut [0_u8, 0])?;
        let len = i32::partial_decode(reader)? as usize;
        let mut labels = Vec::with_capacity(len);
        for _ in 0..len {
            reader.read_exact(&mut [0_u8, 0])?;
            let set = Set::<String>::partial_decode(reader)?;
            labels.push(set);
        }
        let objects = Vec::<GremlinValue>::decode(reader)?;

        Ok(Path { labels, objects })
    }
}

impl Encode for Vertex {
    fn type_code() -> u8 {
        CoreType::Vertex.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.id.encode(writer)?;
        self.label.partial_encode(writer)?;
        self.properties.encode(writer)
    }
}

impl Decode for Vertex {
    fn expected_type_code() -> u8 {
        CoreType::Vertex.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let id = Box::new(GremlinValue::decode(reader)?);
        let label = String::partial_decode(reader)?;
        let properties = Option::<Vec<VertexProperty>>::decode(reader)?;

        Ok(Vertex {
            id,
            label,
            properties,
        })
    }
}

impl Encode for VertexProperty {
    fn type_code() -> u8 {
        CoreType::VertexProperty.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.id.encode(writer)?;
        self.label.partial_encode(writer)?;
        self.value.encode(writer)?;
        self.parent.encode(writer)?;
        self.properties.encode(writer)?;
        Ok(())
    }
}

impl Decode for VertexProperty {
    fn expected_type_code() -> u8 {
        CoreType::VertexProperty.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let id = GremlinValue::decode(reader)?;
        let label = String::partial_decode(reader)?;
        let value = GremlinValue::decode(reader)?;
        let parent = Option::<Vertex>::decode(reader)?;
        let properties = Option::<Vec<Property>>::decode(reader)?;

        Ok(VertexProperty {
            id: Box::new(id),
            label,
            value: Box::new(value),
            parent,
            properties,
        })
    }
}

impl Encode for EitherParent {
    fn type_code() -> u8 {
        unimplemented!()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        unimplemented!()
    }

    fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        match self {
            EitherParent::Edge(e) => e.encode(writer),
            EitherParent::VertexProperty(v) => v.encode(writer),
            EitherParent::None => GremlinValue::UnspecifiedNullObject.encode(writer),
        }
    }
}

impl Encode for Property {
    fn type_code() -> u8 {
        CoreType::Property.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.key.partial_encode(writer)?;
        self.value.encode(writer)?;
        self.parent.encode(writer)
    }
}

impl Decode for EitherParent {
    fn expected_type_code() -> u8 {
        unreachable!()
    }

    fn partial_decode<R: std::io::Read>(_reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        unreachable!()
    }

    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let mut buf = [255_u8; 2];
        reader.read_exact(&mut buf)?;

        let identifier = CoreType::try_from(buf[0])?;
        let value_flag = ValueFlag::try_from(buf[1])?;

        match (identifier, value_flag) {
            (CoreType::Edge, ValueFlag::Set) => {
                Ok(EitherParent::Edge(Edge::partial_decode(reader)?))
            }
            (CoreType::VertexProperty, ValueFlag::Set) => Ok(EitherParent::VertexProperty(
                VertexProperty::partial_decode(reader)?,
            )),
            (CoreType::UnspecifiedNullObject, ValueFlag::Null) => Ok(EitherParent::None),
            (c, v) => Err(crate::error::DecodeError::DecodeError(format!(
                "EitherParent decode with Coretype {c:?} and Valueflag {v:?}"
            ))),
        }
    }
}

impl Decode for Property {
    fn expected_type_code() -> u8 {
        CoreType::Property.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let key = String::partial_decode(reader)?;
        let value = Box::new(GremlinValue::decode(reader)?);
        let parent = EitherParent::decode(reader)?;

        Ok(Property { key, value, parent })
    }
}

impl Encode for Traverser {
    fn type_code() -> u8 {
        CoreType::Traverser.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.bulk.partial_encode(writer)?;
        self.value.encode(writer)
    }
}

impl Decode for Traverser {
    fn expected_type_code() -> u8 {
        CoreType::Traverser.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let bulk = i64::partial_decode(reader)?;
        let value = Box::new(GremlinValue::decode(reader)?);

        Ok(Traverser { bulk, value })
    }
}

impl Encode for TraversalStrategy {
    fn type_code() -> u8 {
        CoreType::TraversalStrategy.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.strategy_class.partial_encode(writer)?;
        self.configuration.partial_encode(writer)
    }
}

impl Decode for TraversalStrategy {
    fn expected_type_code() -> u8 {
        CoreType::TraversalStrategy.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let strategy_class = String::partial_decode(reader)?;
        let configuration = HashMap::<String, GremlinValue>::partial_decode(reader)?;

        Ok(TraversalStrategy {
            strategy_class,
            configuration,
        })
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for MapKeys {
    fn type_code() -> u8 {
        unimplemented!()
    }

    fn partial_encode<W: std::io::Write>(&self, _writer: &mut W) -> Result<(), EncodeError> {
        todo!()
    }

    fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            MapKeys::Int(val) => val.encode(writer),
            MapKeys::String(val) => val.encode(writer),
            MapKeys::Long(val) => val.encode(writer),
            MapKeys::Uuid(val) => val.encode(writer),
            MapKeys::T(val) => val.encode(writer),
            MapKeys::Direction(val) => val.encode(writer),
        }
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for MapKeys {
    fn expected_type_code() -> u8 {
        unimplemented!("MapKeys is a collection of different GrapBinary Keys")
    }

    fn partial_decode<R: std::io::Read>(_reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!()
    }

    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let key = GremlinValue::decode(reader)?;
        MapKeys::try_from(key)
    }
}

#[test]
fn big_dec_decode() {
    use std::str::FromStr;
    let reader = [
        0x22, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x2, 0xff, 0x7f,
    ];
    let expected = BigDecimal::from_str("-1.29").unwrap();
    let res = BigDecimal::decode(&mut &reader[..]).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn big_dec_encode() {
    use std::str::FromStr;
    let s = BigDecimal::from_str("-1.29").unwrap();
    let expected = [
        0x22, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x2, 0xff, 0x7f,
    ];
    let mut buf = vec![];
    s.encode(&mut buf).unwrap();
    assert_eq!(buf, expected)
}

#[test]
fn big_int_decode() {
    use std::str::FromStr;
    let expected = BigInt::from_str("-129").unwrap();
    let reader = [0x23, 0x0, 0x0, 0x0, 0x0, 0x2, 0xff, 0x7f];
    let res = BigInt::decode(&mut &reader[..]).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn big_int_encode() {
    use std::str::FromStr;
    let s = BigInt::from_str("-129").unwrap();
    let expected = [0x23, 0x0, 0x0, 0x0, 0x0, 0x2, 0xff, 0x7f];
    let mut buf = vec![];
    s.encode(&mut buf).unwrap();
    assert_eq!(buf, expected)
}

#[test]
fn binding_encode_gb() {
    let expected = [
        0x14_u8, 0x0, 0x0, 0x00, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x01, 0x00, 0x00, 0x0, 0x0,
        0x01,
    ];
    let mut buf: Vec<u8> = vec![];
    let b = Binding {
        key: "test".to_string(),
        value: Box::new(1_i32.into()),
    };
    b.encode(&mut buf).unwrap();
    assert_eq!(expected, &*buf)
}

#[test]
fn binding_decode_gb() {
    let buf = vec![
        0x14_u8, 0x0, 0x0, 0x00, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x01, 0x00, 0x00, 0x0, 0x0,
        0x01,
    ];
    let expected = Binding {
        key: "test".to_string(),
        value: Box::new(1_i32.into()),
    };
    let b = Binding::decode(&mut &buf[..]).unwrap();
    assert_eq!(expected, b)
}

#[test]
fn encode_bytecode() {
    let expected = [0x25, 0x0, 0x0, 0x0, 0x0, 0x4, b'a', b'b', b'c', b'd'];
    let byte_buffer = ByteBuffer(vec![b'a', b'b', b'c', b'd']);

    let mut writer = Vec::new();
    byte_buffer.encode(&mut writer).unwrap();
    assert_eq!(writer, expected)
}

#[test]
fn decode_bytecode() {
    let buf = vec![0x25, 0x0, 0x0, 0x0, 0x0, 0x4, b'a', b'b', b'c', b'd'];
    let expected = ByteBuffer(vec![b'a', b'b', b'c', b'd']);

    let res = ByteBuffer::decode(&mut &buf[..]).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn edge_none_encode_gb() {
    let expected = [
        0xd_u8, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x7, 0x63, 0x72, 0x65, 0x61,
        0x74, 0x65, 0x64, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x8,
        0x73, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1, 0xfe, 0x1,
    ];

    let e = Edge {
        id: Box::new(9_i32.into()),
        label: "created".to_string(),
        in_v_id: Box::new(3_i64.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1_i64.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: None,
    };

    print!("{e}");

    let mut buf = Vec::new();
    let e = e.encode(&mut buf);
    assert!(e.is_ok());
    assert_eq!(expected, buf[..])
}

#[test]
fn edge_decode_gb() {
    let reader = vec![
        0xd, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x7, 0x63, 0x72, 0x65, 0x61, 0x74,
        0x65, 0x64, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x8, 0x73,
        0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1,
        0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1, 0xfe, 0x1,
    ];

    let p = Edge::decode(&mut &reader[..]);

    // assert!(p.is_ok());
    let expected = Edge {
        id: Box::new(9_i32.into()),
        label: "created".to_string(),
        in_v_id: Box::new(3_i64.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1_i64.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: None,
    };

    assert_eq!(expected, p.unwrap());
}

#[test]
fn encode_gb() {
    use crate::structure::property::EitherParent;
    let expected = [
        0x10_u8, 0x0, 0x0, 0x0, 0x0, 0x2, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0,
        0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x0, 0x0, 0x0, 0x2, 0x2, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4, 0x6e, 0x61, 0x6d, 0x65, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x5, 0x6d, 0x61, 0x72, 0x6b, 0x6f, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x3, 0x61, 0x67, 0x65, 0x1, 0x0, 0x0,
        0x0, 0x0, 0x1d, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x2, 0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x0, 0x0, 0x0, 0x2, 0x2, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x4, 0x6e, 0x61, 0x6d, 0x65, 0x3,
        0x0, 0x0, 0x0, 0x0, 0x5, 0x76, 0x61, 0x64, 0x61, 0x73, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4, 0x0, 0x0, 0x0, 0x3, 0x61, 0x67, 0x65, 0x1,
        0x0, 0x0, 0x0, 0x0, 0x1b, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x2, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xd, 0x0, 0x0, 0x0, 0x4, 0x74, 0x65, 0x73, 0x74, 0x2, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0xfe, 0x1, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x1, 0xfe, 0x1, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x1, 0xf, 0x0, 0x0, 0x0, 0x0, 0x5, 0x73,
        0x69, 0x6e, 0x63, 0x65, 0x1, 0x0, 0x0, 0x0, 0x0, 0x7b, 0xfe, 0x1,
    ];

    let v_s = vec![
        Vertex {
            id: Box::new(1_i64.into()),
            label: "person".to_string(),
            properties: Some(vec![
                VertexProperty {
                    id: Box::new(0i64.into()),
                    label: "name".to_string(),
                    value: Box::new("marko".into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
                VertexProperty {
                    id: Box::new(2i64.into()),
                    label: "age".to_string(),
                    value: Box::new(29_i32.into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
            ]),
        },
        Vertex {
            id: Box::new(2_i64.into()),
            label: "person".to_string(),
            properties: Some(vec![
                VertexProperty {
                    id: Box::new(3i64.into()),
                    label: "name".to_string(),
                    value: Box::new("vadas".into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
                VertexProperty {
                    id: Box::new(4i64.into()),
                    label: "age".to_string(),
                    value: Box::new(27_i32.into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
            ]),
        },
    ];

    let edge = vec![GraphEdge {
        id: 13_i64.into(),
        label: "test".to_string(),
        in_v_id: 2_i64.into(),
        in_v_label: None,
        out_v_id: 1_i64.into(),
        out_v_label: None,
        parent: None,
        properties: vec![Property {
            key: "since".to_string(),
            value: Box::new(123_i32.into()),
            parent: EitherParent::None,
        }],
    }];

    let graph = Graph {
        vertices: v_s,
        edges: edge,
    };

    let mut buf = Vec::new();

    graph.encode(&mut buf).unwrap();

    assert_eq!(expected, *buf);
}

#[test]
fn decode_gb() {
    use crate::structure::property::EitherParent;
    let reader = vec![
        0x10_u8, 0x0, 0x0, 0x0, 0x0, 0x2, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0,
        0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x0, 0x0, 0x0, 0x2, 0x2, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4, 0x6e, 0x61, 0x6d, 0x65, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x5, 0x6d, 0x61, 0x72, 0x6b, 0x6f, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x3, 0x61, 0x67, 0x65, 0x1, 0x0, 0x0,
        0x0, 0x0, 0x1d, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x2, 0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x0, 0x0, 0x0, 0x2, 0x2, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x4, 0x6e, 0x61, 0x6d, 0x65, 0x3,
        0x0, 0x0, 0x0, 0x0, 0x5, 0x76, 0x61, 0x64, 0x61, 0x73, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4, 0x0, 0x0, 0x0, 0x3, 0x61, 0x67, 0x65, 0x1,
        0x0, 0x0, 0x0, 0x0, 0x1b, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x2, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xd, 0x0, 0x0, 0x0, 0x4, 0x74, 0x65, 0x73, 0x74, 0x2, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0xfe, 0x1, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x1, 0xfe, 0x1, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x1, 0xf, 0x0, 0x0, 0x0, 0x0, 0x5, 0x73,
        0x69, 0x6e, 0x63, 0x65, 0x1, 0x0, 0x0, 0x0, 0x0, 0x7b, 0xfe, 0x1,
    ];

    let v_s = vec![
        Vertex {
            id: Box::new(1_i64.into()),
            label: "person".to_string(),
            properties: Some(vec![
                VertexProperty {
                    id: Box::new(0i64.into()),
                    label: "name".to_string(),
                    value: Box::new("marko".into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
                VertexProperty {
                    id: Box::new(2i64.into()),
                    label: "age".to_string(),
                    value: Box::new(29_i32.into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
            ]),
        },
        Vertex {
            id: Box::new(2_i64.into()),
            label: "person".to_string(),
            properties: Some(vec![
                VertexProperty {
                    id: Box::new(3i64.into()),
                    label: "name".to_string(),
                    value: Box::new("vadas".into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
                VertexProperty {
                    id: Box::new(4i64.into()),
                    label: "age".to_string(),
                    value: Box::new(27_i32.into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
            ]),
        },
    ];

    let edge = vec![GraphEdge {
        id: 13_i64.into(),
        label: "test".to_string(),
        in_v_id: 2_i64.into(),
        in_v_label: None,
        out_v_id: 1_i64.into(),
        out_v_label: None,
        parent: None,
        properties: vec![Property {
            key: "since".to_string(),
            value: Box::new(123_i32.into()),
            parent: EitherParent::None,
        }],
    }];

    let expected = Graph {
        vertices: v_s,
        edges: edge,
    };

    let graph = Graph::decode(&mut &reader[..]).unwrap();

    assert_eq!(expected, graph);
}

#[test]
fn metric_encode() {
    let metric = Metrics {
        id: "4.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[1])".to_string(),
        duration: 1,
        counts: HashMap::from([
            // ("traverserCount".to_string(), 1),
            ("elementCount".to_string(), 1),
        ]),
        annotations: HashMap::from([("percentDur".to_string(), 0_f64.into())]),
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
fn metric_decode() {
    let expected = Metrics {
        id: "4.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[1])".to_string(),
        duration: 1,
        counts: HashMap::from([
            // ("traverserCount".to_string(), 1),
            ("elementCount".to_string(), 1),
        ]),
        annotations: HashMap::from([("percentDur".to_string(), 0_f64.into())]),
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
fn traversal_metric_encode() {
    let metric = Metrics {
        id: "4.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[1])".to_string(),
        duration: 1,
        counts: HashMap::from([("elementCount".to_string(), 1)]),
        annotations: HashMap::from([("percentDur".to_string(), 0_f64.into())]),
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
fn traversal_metric_decode() {
    let metric = Metrics {
        id: "4.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[1])".to_string(),
        duration: 1,
        counts: HashMap::from([("elementCount".to_string(), 1)]),
        annotations: HashMap::from([("percentDur".to_string(), 0_f64.into())]),
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
fn path_encode() {
    let expected = [
        0xe_u8, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0xb, 0x0, 0x0,
        0x0, 0x0, 0x0, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x5, 0x6d, 0x61, 0x72, 0x6b, 0x6f, 0x1, 0x0, 0x0, 0x0, 0x0, 0x20, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x6, 0x72, 0x69, 0x70, 0x70, 0x6c, 0x65,
    ];

    let path = Path {
        labels: vec![Set::new(vec![]), Set::new(vec![]), Set::new(vec![])],
        objects: vec!["marko".into(), 32_i32.into(), "ripple".into()],
    };
    let mut buf = Vec::new();
    path.encode(&mut buf).unwrap();

    assert_eq!(&expected[..], &buf)
}

#[test]
fn path_decode() {
    let expecetd = Path {
        labels: vec![Set::new(vec![]), Set::new(vec![]), Set::new(vec![])],
        objects: vec!["marko".into(), 32_i32.into(), "ripple".into()],
    };

    let buf = vec![
        0xe_u8, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0xb, 0x0, 0x0,
        0x0, 0x0, 0x0, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x5, 0x6d, 0x61, 0x72, 0x6b, 0x6f, 0x1, 0x0, 0x0, 0x0, 0x0, 0x20, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x6, 0x72, 0x69, 0x70, 0x70, 0x6c, 0x65,
    ];

    let path = Path::decode(&mut &buf[..]).unwrap();

    assert_eq!(expecetd, path)
}

#[test]
fn vertex_none_encode() {
    let expected = [
        0x11_u8, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70,
        0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1,
    ];
    let v = Vertex {
        id: Box::new(1_i64.into()),
        label: String::from("person"),
        properties: None,
    };
    let mut buf = Vec::new();
    let v = v.encode(&mut buf);
    assert!(v.is_ok());
    assert_eq!(expected, buf[..])
}

#[test]
fn vertex_decode_none() {
    let reader = vec![
        0x11_u8, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70,
        0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1,
    ];

    let v = Vertex::decode(&mut &reader[..]);
    assert!(v.is_ok());

    let expected = Vertex {
        id: Box::new(1_i64.into()),
        label: String::from("person"),
        properties: None,
    };

    assert_eq!(expected, v.unwrap())
}

#[test]
fn encode_traverser() {
    let expected = [
        0x21, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x03, 0x0, 0x0, 0x0, 0x0, 0x3, b'a',
        b'b', b'c',
    ];

    let t = Traverser {
        bulk: 3,
        value: Box::new("abc".into()),
    };
    let mut writer = Vec::<u8>::new();
    t.encode(&mut writer).unwrap();
    assert_eq!(expected, &writer[..])
}

#[test]
fn decode_traverser() {
    let reader = vec![
        0x21, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x03, 0x0, 0x0, 0x0, 0x0, 0x3, b'a',
        b'b', b'c',
    ];

    let expected = Traverser {
        bulk: 3,
        value: Box::new("abc".into()),
    };

    assert_eq!(expected, Traverser::decode(&mut &reader[..]).unwrap())
}
