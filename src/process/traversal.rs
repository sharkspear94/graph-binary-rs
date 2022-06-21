use std::{collections::HashMap, marker::PhantomData, ops::DerefMut, vec};

use serde::Deserialize;
use uuid::Uuid;

use params::dedup_params::DedupStepParams;

use crate::{
    de::from_slice,
    graph_binary::{GraphBinary, MapKeys},
    structure::{
        bytecode::ByteCode,
        edge::Edge,
        enums::{Direction, Merge, Scope, T},
        lambda::{self, Lambda},
        path::Path,
        vertex::Vertex,
    },
};

use super::{
    bytecode_traversal::BytecodeTraversal,
    params::{
        self,
        add_element_params::AddElementParams,
        by_params::ByParams,
        coalesce_params::CoalesceParams,
        emit_params::EmitParams,
        from_step_params::FromStepParams,
        has_id_params::HasIdParams,
        has_params::HasStepParams,
        has_strings_params::HasStringsParams,
        is_param::IsParam,
        merge_params::MergeParams,
        multi_strings::MultiStringParams,
        object_param::{MultiObjectParam, ObjectParam},
        option_params::OptionParams,
        project_params::ProjectParams,
        property_params::PropertyParam,
        repeat_param::RepeatParam,
        sack_params::SackParam,
        scope_params::ScopeParams,
        select_params::SelectParam,
        single_string::SingleStringParam,
        tail_params::TailParams,
        to_step_params::ToStepParams,
        until_params::UntilParams,
        where_params::WhereParams,
    },
};
use super::{graph_traversal_source::GraphTraversalSource, params::fold_params::FoldParams};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct GraphTraversal<E, T> {
    end: PhantomData<E>,
    pub bytecode: ByteCode,
    terminator: PhantomData<T>,
}

impl<'de, E, T: Deserialize<'de>> GraphTraversal<E, T> {
    pub fn new(bytecode: ByteCode) -> GraphTraversal<E, T> {
        GraphTraversal {
            end: PhantomData,
            bytecode,
            terminator: PhantomData,
        }
    }

    pub fn bytecode(&self) -> &ByteCode {
        &self.bytecode
    }

    pub fn id(mut self) -> GraphTraversal<GraphBinary, GraphBinary> {
        self.bytecode.add_step("id", vec![]);
        GraphTraversal {
            end: PhantomData,
            bytecode: self.bytecode,
            terminator: PhantomData,
        }
    }

    pub fn label(mut self) -> GraphTraversal<String, String> {
        self.bytecode.add_step("label", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn constant<C: Into<GraphBinary> + Deserialize<'de>>(
        mut self,
        constant: C,
    ) -> GraphTraversal<C, C> {
        self.bytecode.add_step("constant", vec![constant.into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn v<A: Into<GraphBinary>>(mut self, a: A) -> GraphTraversal<Vertex, Vertex> {
        self.bytecode.add_step("V", vec![a.into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn to(mut self, to_vertex: impl ToStepParams) -> GraphTraversal<Vertex, Vertex> {
        // TODO overload of to return different traversal
        to_vertex.bytecode("to", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn out(mut self, labels: impl MultiStringParams) -> GraphTraversal<Vertex, Vertex> {
        labels.bytecode("out", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn in_(mut self, labels: impl MultiStringParams) -> GraphTraversal<Vertex, Vertex> {
        labels.bytecode("in", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn both(mut self, labels: impl MultiStringParams) -> GraphTraversal<Vertex, Vertex> {
        labels.bytecode("both", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn to_e(
        mut self,
        direction: Direction,
        labels: impl MultiStringParams,
    ) -> GraphTraversal<Edge, Edge> {
        self.bytecode.add_step("in", vec![direction.into()]);
        labels.extend_step(&mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn out_e(mut self, labels: impl MultiStringParams) -> GraphTraversal<Edge, Edge> {
        labels.bytecode("outE", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn in_e(mut self, labels: impl MultiStringParams) -> GraphTraversal<Edge, Edge> {
        labels.bytecode("inE", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn both_e(mut self, labels: impl MultiStringParams) -> GraphTraversal<Edge, Edge> {
        labels.bytecode("bothE", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn to_v(mut self, direction: Direction) -> GraphTraversal<Vertex, Vertex> {
        self.bytecode.add_step("toV", vec![direction.into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn in_v(mut self) -> GraphTraversal<Vertex, Vertex> {
        self.bytecode.add_step("inV", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn out_v(mut self) -> GraphTraversal<Vertex, Vertex> {
        self.bytecode.add_step("outV", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn both_v(mut self) -> GraphTraversal<Vertex, Vertex> {
        self.bytecode.add_step("bothV", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn other_v(mut self) -> GraphTraversal<Vertex, Vertex> {
        self.bytecode.add_step("otherV", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn order(mut self, scope: impl ScopeParams) -> Self {
        scope.bytecode("order", &mut self.bytecode);
        self
    }

    pub fn properties(mut self, property_keys: impl MultiStringParams) {
        property_keys.bytecode("properties", &mut self.bytecode)
    }

    pub fn values(
        mut self,
        property_keys: impl MultiStringParams,
    ) -> GraphTraversal<GraphBinary, GraphBinary> {
        property_keys.bytecode("values", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn property_map(mut self, property_keys: impl MultiStringParams) {
        property_keys.bytecode("propertyMap", &mut self.bytecode)
    }

    pub fn element_map(
        mut self,
    ) -> GraphTraversal<HashMap<MapKeys, GraphBinary>, HashMap<MapKeys, GraphBinary>> {
        self.bytecode.add_step("elementMap", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn value_map(mut self, property_keys: impl MultiStringParams) {
        property_keys.bytecode("valueMap", &mut self.bytecode)
    }

    pub fn key(mut self) -> GraphTraversal<String, String> {
        self.bytecode.add_step("key", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn value(mut self) -> Self {
        self.bytecode.add_step("value", vec![]);
        self
    }

    pub fn path(mut self) -> GraphTraversal<Path, Path> {
        self.bytecode.add_step("path", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn match_(
        mut self,
        match_traversal: GraphTraversal<E, T>,
    ) -> GraphTraversal<HashMap<MapKeys, GraphBinary>, HashMap<MapKeys, GraphBinary>> {
        self.bytecode
            .add_step("match", vec![match_traversal.bytecode.into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn sack(mut self, sackparam: impl SackParam) -> GraphTraversal<GraphBinary, GraphBinary> {
        sackparam.bytecode("sack", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn loops(mut self, s: impl SingleStringParam) -> GraphTraversal<i32, i32> {
        s.bytecode("loops", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn project(
        mut self,
        property_keys: impl ProjectParams,
    ) -> GraphTraversal<HashMap<String, GraphBinary>, HashMap<String, GraphBinary>> {
        property_keys.bytecode("project", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    // pub fn select_map(
    //     mut self,
    //     select_params: impl SelectMapParam,
    // ) -> GraphTraversal<HashMap<String, GraphBinary>, HashMap<String, GraphBinary>> {
    //     select_params.bytecode("select", &mut self.bytecode);
    //     GraphTraversal::new(self.bytecode)
    // }

    pub fn select(
        mut self,
        select_params: impl SelectParam,
    ) -> GraphTraversal<GraphBinary, GraphBinary> {
        select_params.bytecode("select", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn unfold(mut self) -> GraphTraversal<GraphBinary, GraphBinary> {
        self.bytecode.add_step("unfold", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn fold(mut self) -> GraphTraversal<Vec<GraphBinary>, Vec<GraphBinary>> {
        self.bytecode.add_step("fold", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn fold_with_seed<Seed: Into<GraphBinary> + Deserialize<'de>, L: Into<Lambda>>(
        mut self,
        seed: Seed,
        lambda: L,
    ) -> GraphTraversal<Seed, Seed> {
        self.bytecode
            .add_step("fold", vec![seed.into(), lambda.into().into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn count(mut self, scope: impl ScopeParams) -> GraphTraversal<i64, i64> {
        scope.bytecode("count", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn sum(mut self, scope: impl ScopeParams) -> GraphTraversal<GraphBinary, GraphBinary> {
        //TODO // Num Trait
        scope.bytecode("sum", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn max(mut self, scope: impl ScopeParams) -> GraphTraversal<GraphBinary, GraphBinary> {
        //TODO
        scope.bytecode("max", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn min(mut self, scope: impl ScopeParams) -> GraphTraversal<GraphBinary, GraphBinary> {
        //TODO
        scope.bytecode("min", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn mean(mut self, scope: impl ScopeParams) -> GraphTraversal<GraphBinary, GraphBinary> {
        //TODO
        scope.bytecode("mean", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn group(
        mut self,
        side_effect_key: impl SingleStringParam,
    ) -> GraphTraversal<HashMap<MapKeys, GraphBinary>, HashMap<MapKeys, GraphBinary>> {
        side_effect_key.bytecode("group", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn group_count(
        mut self,
        side_effect_key: impl SingleStringParam,
    ) -> GraphTraversal<HashMap<MapKeys, GraphBinary>, HashMap<MapKeys, i64>> {
        side_effect_key.bytecode("groupMap", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn tree(mut self, side_effect_key: impl SingleStringParam) -> Self {
        //Tree TODO
        side_effect_key.bytecode("tree", &mut self.bytecode);
        self
    }

    pub fn add_v(mut self, vertex_label: impl AddElementParams) -> GraphTraversal<Vertex, Vertex> {
        vertex_label.bytecode("addE", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }
    pub fn merge_v(mut self, merge_params: impl MergeParams) -> GraphTraversal<Vertex, Vertex> {
        merge_params.bytecode("mergeV", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn merge_e(mut self, merge_params: impl MergeParams) -> GraphTraversal<Edge, Edge> {
        merge_params.bytecode("mergeE", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn add_e(mut self, edge_label: impl AddElementParams) -> GraphTraversal<Edge, Edge> {
        edge_label.bytecode("addE", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn from(mut self, from_vertex: impl FromStepParams) -> Self {
        from_vertex.bytecode("from", &mut self.bytecode);
        self
    }

    pub fn math(mut self, expression: &str) -> GraphTraversal<f64, f64> {
        self.bytecode.add_step("math", vec![expression.into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn element(mut self) -> Self {
        //TODO
        self.bytecode.add_step("element", vec![]);
        self
    }

    pub fn call() {}

    pub fn filter(mut self) -> Self {
        // TODO
        self
    }

    pub fn none(mut self) -> Self {
        self.bytecode.add_step("none", vec![]);
        self
    }

    pub fn or() {} // TODO

    pub fn and() {} // TODO

    pub fn inject<I: Into<GraphBinary> + Deserialize<'de>>(
        mut self,
        items: I,
    ) -> GraphTraversal<I, I> {
        self.bytecode.add_step("inject", vec![items.into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn dedup(mut self, scope_and_labels: impl DedupStepParams) -> Self {
        scope_and_labels.bytecode("dedup", &mut self.bytecode);
        self
    }

    pub fn where_(mut self, params: impl WhereParams) -> Self {
        params.bytecode("where", &mut self.bytecode);
        self
    }

    pub fn has(mut self, params: impl HasStepParams) -> Self {
        params.bytecode("has", &mut self.bytecode);
        self
    }

    pub fn has_not(mut self, property_key: &str) -> Self {
        self.bytecode.add_step("hasNot", vec![property_key.into()]);
        self
    }
    pub fn has_label(mut self, label: impl HasStringsParams) -> Self {
        label.bytecode("hasLabel", &mut self.bytecode);
        self
    }
    pub fn has_id(mut self, has_id_params: impl HasIdParams) -> Self {
        has_id_params.bytecode("hasId", &mut self.bytecode);
        self
    }

    pub fn has_key(mut self, label: impl HasStringsParams) -> Self {
        label.bytecode("hasKey", &mut self.bytecode);
        self
    }

    pub fn has_value(
        mut self,
        value: impl Into<GraphBinary>,
        values: impl MultiObjectParam,
    ) -> Self {
        self.bytecode.add_step("hasValue", vec![value.into()]);
        values.extend_step(&mut self.bytecode);
        self
    }

    pub fn is(mut self, p_or_objet: impl IsParam<E>) -> Self {
        p_or_objet.bytecode("is", &mut self.bytecode);
        self
    }

    pub fn not(mut self, not_traversal: BytecodeTraversal) -> Self {
        self.bytecode.add_step("not", vec![not_traversal.into()]);
        self
    }

    pub fn coin(mut self, propability: f64) -> Self {
        self.bytecode.add_step("coin", vec![propability.into()]);
        self
    }

    pub fn range(mut self, scope: impl ScopeParams, low: i64, high: i64) -> Self {
        scope.bytecode("range", &mut self.bytecode);
        self.bytecode.add_to_last_step(low);
        self.bytecode.add_to_last_step(high);
        self
    }

    pub fn limit(mut self, scope: impl ScopeParams, limit: i64) -> Self {
        scope.bytecode("limit", &mut self.bytecode);
        self.bytecode.add_to_last_step(limit);
        self
    }

    pub fn tail(mut self, tail_param: impl TailParams) -> Self {
        tail_param.bytecode("tail", &mut self.bytecode);
        self
    }

    pub fn skip(mut self, scope: impl ScopeParams, skip: i64) -> Self {
        scope.bytecode("skip", &mut self.bytecode);
        self.bytecode.add_to_last_step(skip);
        self
    }

    pub fn time_limit(mut self, time_limit: i64) {
        self.bytecode.add_step("timeLimit", vec![time_limit.into()])
    }

    pub fn simple_path(mut self) -> Self {
        self.bytecode.add_step("simplePath", vec![]);
        self
    }

    pub fn cyclic_path(mut self) -> Self {
        self.bytecode.add_step("cyclicPath", vec![]);
        self
    }

    pub fn sample(mut self, scope: impl ScopeParams, amount_to_sample: i32) -> Self {
        scope.bytecode("sample", &mut self.bytecode);
        self.bytecode.add_to_last_step(amount_to_sample);
        self
    }

    pub fn drop(mut self) -> Self {
        self.bytecode.add_step("drop", vec![]);
        self
    }

    pub fn side_effect(mut self, side_effect_traversal: BytecodeTraversal) -> Self {
        //TODO
        self.bytecode
            .add_step("sideEffect", vec![side_effect_traversal.into()]);
        self
    }

    pub fn cap(
        mut self,
        side_effect_key: &str,
        side_effect_keys: impl MultiStringParams,
    ) -> GraphTraversal<GraphBinary, GraphBinary> {
        self.bytecode.add_step("cap", vec![side_effect_key.into()]);
        side_effect_keys.extend_step(&mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn subgraph(mut self, side_effect_key: &str) -> Self {
        self.bytecode
            .add_step("subgraph", vec![side_effect_key.into()]);
        self
    }

    pub fn aggregate(mut self, scope: impl ScopeParams, side_effect_key: &str) -> Self {
        scope.bytecode("aggregate", &mut self.bytecode);
        self.bytecode.add_to_last_step(side_effect_key);
        self
    }

    pub fn fail(mut self, message: impl SingleStringParam) -> Self {
        message.bytecode("fail", &mut self.bytecode);
        self
    }

    pub fn profile(mut self, message: impl SingleStringParam) -> Self {
        message.bytecode("profile", &mut self.bytecode);
        self
    }

    pub fn property(mut self, proptery_params: impl PropertyParam) -> Self {
        proptery_params.bytecode("property", &mut self.bytecode);
        self
    }

    pub fn branch(mut self) {}

    pub fn choose(mut self) {}

    pub fn optional(mut self, optional_traversel: BytecodeTraversal) -> Self {
        self.bytecode
            .add_step("optional", vec![optional_traversel.into()]);
        self
    }

    pub fn union(mut self, union_traversal: BytecodeTraversal) -> Self {
        self.bytecode
            .add_step("union", vec![union_traversal.into()]);
        self
    }

    pub fn coalesce(
        mut self,
        coalesce_traversals: impl CoalesceParams,
    ) -> GraphTraversal<GraphBinary, GraphBinary> {
        coalesce_traversals.bytecode("coalesce", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn repeat(
        mut self,
        loop_name: impl SingleStringParam,
        loop_traversal: BytecodeTraversal,
    ) -> Self {
        loop_name.bytecode("repeat", &mut self.bytecode);
        self.bytecode.add_to_last_step(loop_traversal);
        self
    }

    pub fn emit(mut self, emit_params: impl EmitParams) -> Self {
        emit_params.bytecode("emit", &mut self.bytecode);
        self
    }

    pub fn until(mut self, params: impl UntilParams) -> Self {
        params.bytecode("until", &mut self.bytecode);
        self
    }

    pub fn times(mut self, max_loops: i32) -> Self {
        self.bytecode.add_step("times", vec![max_loops.into()]);
        self
    }

    pub fn local(
        mut self,
        local_traversal: BytecodeTraversal,
    ) -> GraphTraversal<GraphBinary, GraphBinary> {
        self.bytecode
            .add_step("local", vec![local_traversal.into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn page_rank(mut self) {}

    pub fn peer_pressure(mut self) -> Self {
        self.bytecode.add_step("peerPressure", vec![]);
        self
    }

    pub fn connected_component(mut self) -> Self {
        self.bytecode.add_step("connectedComponent", vec![]);
        self
    }

    pub fn shortest_path(mut self) -> Self {
        self.bytecode.add_step("shortestPath", vec![]);
        self
    }

    pub fn programm(mut self) {}

    pub fn as_(mut self, step_label: &str, step_labels: impl MultiStringParams) -> Self {
        self.bytecode.add_step("as", vec![step_label.into()]);
        step_labels.extend_step(&mut self.bytecode);
        self
    }

    pub fn barrier(mut self) {}

    pub fn index(mut self) -> Self {
        self.bytecode.add_step("index", vec![]);
        self
    }

    pub fn with(mut self, key: &str, object: impl ObjectParam) -> Self {
        self.bytecode.add_step("with", vec![key.into()]);
        object.extend_step(&mut self.bytecode);
        self
    }

    pub fn by(mut self, params: impl ByParams) -> Self {
        params.bytecode("by", &mut self.bytecode);
        self
    }

    pub fn option(mut self, option_params: impl OptionParams) -> Self {
        option_params.bytecode("option", &mut self.bytecode);
        self
    }

    pub fn read(mut self) -> Self {
        self.bytecode.add_step("read", vec![]);
        self
    }

    pub fn write(mut self) -> Self {
        self.bytecode.add_step("write", vec![]);
        self
    }

    pub fn iterate(mut self) -> Self {
        self.bytecode.add_step("iterate", vec![]);
        self
    }
}

impl<E, T> From<GraphTraversal<E, T>> for GraphBinary {
    fn from(g: GraphTraversal<E, T>) -> Self {
        GraphBinary::ByteCode(g.bytecode)
    }
}

enum Id {
    String(String),
    Int(i32),
    Long(i64),
    Uuid(Uuid),
}

impl From<()> for Ids {
    fn from(_: ()) -> Self {
        Ids(vec![])
    }
}

impl From<String> for Ids {
    fn from(s: String) -> Self {
        Ids(vec![s.into()])
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Id::String(s)
    }
}

impl From<Id> for GraphBinary {
    fn from(id: Id) -> Self {
        match id {
            Id::String(s) => GraphBinary::String(s),
            Id::Int(v) => GraphBinary::Int(v),
            Id::Long(v) => GraphBinary::Long(v),
            Id::Uuid(v) => GraphBinary::Uuid(v),
        }
    }
}

pub struct Ids(Vec<Id>);

impl From<Ids> for Vec<GraphBinary> {
    fn from(ids: Ids) -> Self {
        ids.0.into_iter().map(Into::into).collect()
    }
}

#[derive(Debug, Clone)]
pub struct AnonymousTraversal;

impl AnonymousTraversal {
    pub fn id(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("id", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn label(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("label", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn constant<C: Into<GraphBinary>>(&self, constant: C) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("constant", vec![constant.into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn v(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("V", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn e(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("E", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn to(&self, to_vertex: impl ToStepParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        to_vertex.bytecode("to", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn out(&self, labels: impl MultiStringParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        labels.bytecode("out", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn in_(&self, labels: impl MultiStringParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        labels.bytecode("in", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn both(&self, labels: impl MultiStringParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        labels.bytecode("both", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn to_e(&self, direction: Direction, labels: impl MultiStringParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("in", vec![direction.into()]);
        labels.extend_step(&mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn out_e(&self, labels: impl MultiStringParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        labels.bytecode("outE", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn in_e(&self, labels: impl MultiStringParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        labels.bytecode("inE", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn both_e(&self, labels: impl MultiStringParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        labels.bytecode("bothE", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn to_v(&self, direction: Direction) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("toV", vec![direction.into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn in_v(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("inV", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn out_v(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("outV", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn both_v(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("bothV", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn other_v(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("otherV", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn order(&self, scope: impl ScopeParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        scope.bytecode("order", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn properties(&self, property_keys: impl MultiStringParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        property_keys.bytecode("properties", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn values(&self, property_keys: impl MultiStringParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        property_keys.bytecode("values", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn property_map(&self, property_keys: impl MultiStringParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        property_keys.bytecode("propertyMap", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn element_map(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("elementMap", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn value_map(&self, property_keys: impl MultiStringParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        property_keys.bytecode("valueMap", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn key(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("key", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn value(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("value", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn path(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("path", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn match_(&self, match_traversal: BytecodeTraversal) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("match", vec![match_traversal.into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn sack() {}

    pub fn loops(&self, s: impl SingleStringParam) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        s.bytecode("loops", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn project(
        &self,
        property_key: &str,
        other_property_keys: impl MultiStringParams,
    ) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("project", vec![property_key.into()]);
        other_property_keys.extend_step(&mut bc);
        BytecodeTraversal::new(bc)
    }

    // pub fn select_map(
    //     mut self,
    //     select_params: impl SelectMapParam,
    // ) -> GraphTraversal<S, HashMap<String, GraphBinary>, HashMap<String, GraphBinary>> {
    //     select_params.bytecode("select", &mut bc);
    //     self
    // }

    pub fn select(&self, select_params: impl SelectParam) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        select_params.bytecode("select", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn unfold(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("unfold", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn fold(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("fold", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn fold_with_seed<E2: Into<GraphBinary>, L: Into<Lambda>>(
        &self,
        seed: E2,
        lambda: L,
    ) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("fold", vec![seed.into(), lambda.into().into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn count(&self, scope: impl ScopeParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        scope.bytecode("count", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn sum(&self, scope: impl ScopeParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        scope.bytecode("sum", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn max(&self, scope: impl ScopeParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        scope.bytecode("max", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn min(&self, scope: impl ScopeParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        scope.bytecode("min", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn mean(&self, scope: impl ScopeParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        scope.bytecode("mean", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn group(&self, side_effect_key: impl SingleStringParam) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        side_effect_key.bytecode("group", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn group_count(&self, side_effect_key: impl SingleStringParam) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        side_effect_key.bytecode("groupMap", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn tree(&self, side_effect_key: impl SingleStringParam) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        side_effect_key.bytecode("tree", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn add_v(&self, vertex_label: impl AddElementParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        vertex_label.bytecode("addE", &mut bc);
        BytecodeTraversal::new(bc)
    }
    pub fn merge_v(&self, merge_params: impl MergeParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        merge_params.bytecode("mergeV", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn merge_e(&self, merge_params: impl MergeParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        merge_params.bytecode("mergeE", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn add_e(&self, edge_label: impl AddElementParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        edge_label.bytecode("addE", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn from(&self, from_vertex: impl FromStepParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        from_vertex.bytecode("from", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn math(&self, expression: &str) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("math", vec![expression.into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn element(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        //TODO
        bc.add_step("element", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn call() {}

    pub fn filter(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        // TODO
        BytecodeTraversal::new(bc)
    }

    pub fn none(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("none", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn or() {} // TODO

    pub fn and() {} // TODO

    pub fn inject<I: Into<GraphBinary>>(&self, items: I) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("inject", vec![items.into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn dedup(&self, scope_and_labels: impl DedupStepParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        scope_and_labels.bytecode("dedup", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn where_(&self, params: impl WhereParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        params.bytecode("where", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn has(&self, params: impl HasStepParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        params.bytecode("has", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn has_not(&self, property_key: &str) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("hasNot", vec![property_key.into()]);
        BytecodeTraversal::new(bc)
    }
    pub fn has_label(&self, label: impl HasStringsParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        label.bytecode("hasLabel", &mut bc);
        BytecodeTraversal::new(bc)
    }
    pub fn has_id(&self, has_id_params: impl HasIdParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        has_id_params.bytecode("hasId", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn has_key(&self, label: impl HasStringsParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        label.bytecode("hasKey", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn has_value(
        &self,
        value: impl Into<GraphBinary>,
        values: impl MultiObjectParam,
    ) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("hasValue", vec![value.into()]);
        values.extend_step(&mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn is<E>(&self, p_or_objet: impl IsParam<E>) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        p_or_objet.bytecode("is", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn not(&self, not_traversal: BytecodeTraversal) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("not", vec![not_traversal.into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn coin(&self, propability: f64) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("coin", vec![propability.into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn range(&self, scope: impl ScopeParams, low: i64, high: i64) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        scope.bytecode("range", &mut bc);
        bc.add_to_last_step(low);
        bc.add_to_last_step(high);
        BytecodeTraversal::new(bc)
    }

    pub fn limit(&self, scope: impl ScopeParams, limit: i64) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        scope.bytecode("limit", &mut bc);
        bc.add_to_last_step(limit);
        BytecodeTraversal::new(bc)
    }

    pub fn tail(&self, tail_param: impl TailParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        tail_param.bytecode("tail", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn skip(&self, scope: impl ScopeParams, skip: i64) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        scope.bytecode("skip", &mut bc);
        bc.add_to_last_step(skip);
        BytecodeTraversal::new(bc)
    }

    pub fn time_limit(&self, time_limit: i64) {
        let mut bc = ByteCode::default();
        bc.add_step("timeLimit", vec![time_limit.into()])
    }

    pub fn simple_path(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("simplePath", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn cyclic_path(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("cyclicPath", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn sample(&self, scope: impl ScopeParams, amount_to_sample: i32) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        scope.bytecode("sample", &mut bc);
        bc.add_to_last_step(amount_to_sample);
        BytecodeTraversal::new(bc)
    }

    pub fn drop(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("drop", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn side_effect(&self, side_effect_traversal: BytecodeTraversal) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        //TODO
        bc.add_step("sideEffect", vec![side_effect_traversal.into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn cap(
        &self,
        side_effect_key: &str,
        side_effect_keys: impl MultiStringParams,
    ) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("cap", vec![side_effect_key.into()]);
        side_effect_keys.extend_step(&mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn subgraph(&self, side_effect_key: &str) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("subgraph", vec![side_effect_key.into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn aggregate(&self, scope: impl ScopeParams, side_effect_key: &str) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        scope.bytecode("aggregate", &mut bc);
        bc.add_to_last_step(side_effect_key);
        BytecodeTraversal::new(bc)
    }

    pub fn fail(&self, message: impl SingleStringParam) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        message.bytecode("fail", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn profile(&self, message: impl SingleStringParam) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        message.bytecode("profile", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn property(&self, proptery_params: impl PropertyParam) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        proptery_params.bytecode("property", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn branch(&self) {}

    pub fn choose(&self) {}

    pub fn optional<E2>(&self, optional_traversel: BytecodeTraversal) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("optional", vec![optional_traversel.into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn union(&self, union_traversal: BytecodeTraversal) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("union", vec![union_traversal.into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn coalesce(&self, coalesce_traversals: impl CoalesceParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        coalesce_traversals.bytecode("coalesce", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn repeat(
        &self,
        loop_name: impl SingleStringParam,
        loop_traversal: BytecodeTraversal,
    ) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        loop_name.bytecode("repeat", &mut bc);
        bc.add_to_last_step(loop_traversal);
        BytecodeTraversal::new(bc)
    }

    pub fn emit(&self, emit_params: impl EmitParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        emit_params.bytecode("emit", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn until(&self, params: impl UntilParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        params.bytecode("until", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn times(&self, max_loops: i32) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("times", vec![max_loops.into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn local<E2>(&self, local_traversal: BytecodeTraversal) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("local", vec![local_traversal.into()]);
        BytecodeTraversal::new(bc)
    }

    pub fn page_rank(&self) {}

    pub fn peer_pressure(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("peerPressure", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn connected_component(&self) {}

    pub fn shortest_path(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("shortestPath", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn programm(&self) {}

    pub fn as_(&self, step_label: &str, step_labels: impl MultiStringParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("as", vec![step_label.into()]);
        step_labels.extend_step(&mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn barrier(&self) {}

    pub fn index(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("index", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn with(&self, key: &str, object: impl ObjectParam) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("with", vec![key.into()]);
        object.extend_step(&mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn by(&self, params: impl ByParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        params.bytecode("by", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn option(&self, option_params: impl OptionParams) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        option_params.bytecode("option", &mut bc);
        BytecodeTraversal::new(bc)
    }

    pub fn read(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("read", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn write(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("write", vec![]);
        BytecodeTraversal::new(bc)
    }

    pub fn iterate(&self) -> BytecodeTraversal {
        let mut bc = ByteCode::default();
        bc.add_step("iterate", vec![]);
        BytecodeTraversal::new(bc)
    }
}

lazy_static! {
    pub static ref __: AnonymousTraversal = AnonymousTraversal;
}

#[test]
fn test() {
    let g = GraphTraversalSource::<()>::new();
    // g.v(()).has("label", "key", P::eq(2f32));
    // g.v(()).has("label", "key", P::gt(2f32));
}

#[test]
fn test1() {
    let mut g = GraphTraversalSource::<()>::new();
    let mut g1 = GraphTraversalSource::<()>::new();
    let t1 = g1.inject(vec![1, 123, 3, 4]);

    let t = g.with_computer().inject(vec![1, 123, 3, 4]);

    let v = vec!["asasdd".to_string()];
    let t = g.v(()).project(["id", "d"]);
    let t = g.v(()).has(("age", __.v()));
    let t = g.v(()).add_v(__.v().values("name"));
    let t = g.v(()).add_e("asd").from(__.v());
    let t = g
        .v(())
        .option((Merge::OnCreate, HashMap::from([("asd", 3)])));
    let t = g.v(()).has_id([1, 2, 3]);

    let t = g.v(()).coalesce([__.v(), __.v().values("age"), __.v()]);
    let t = g.v(()).is(Vertex::new(1, "person"));
    let t = g.v(()).not(__.v().values("as").is(1)); // TODO
    let t = g.v(()).not(__.v().values("as").is(1)); // TODO
    let t = g
        .v(())
        .project(["id", "s"])
        .by(__.v().id())
        .by(__.v().values("age")); // TODO project params are stupid
    let t = g.v(()).project(["id", "s"]).by(__.v().id()).by(__.v());

    // let t = g.v(()).as_("v", ()).select("v");
    println!("{:?}", t.bytecode)
}
