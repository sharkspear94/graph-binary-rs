use std::{collections::HashMap, marker::PhantomData, vec};

use uuid::Uuid;

use params::dedup_params::DedupStepParams;

use crate::{
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

use super::params::{
    self,
    add_element_params::AddElementParams,
    by_params::ByParams,
    coalesce_params::CoalesceParams,
    emit_params::EmitParams,
    fold_params::{self, FoldParams},
    from_step_params::FromStepParams,
    has_id_params::HasIdParams,
    has_params::HasStepParams,
    has_strings_params::HasStringsParams,
    is_param::IsParam,
    merge_params::MergeParams,
    multi_strings::MultiStringParams,
    object_param::{MultiObjectParam, ObjectParam},
    option_params::OptionParams,
    property_params::PropertyParam,
    repeat_param::RepeatParam,
    scope_params::ScopeParams,
    select_params::SelectParam,
    single_string::SingleStringParam,
    tail_params::TailParams,
    to_step_params::ToStepParams,
    until_params::UntilParams,
    where_params::WhereParams,
};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct GraphTraversal<S, E, T> {
    start: PhantomData<S>,
    end: PhantomData<E>,
    pub bytecode: ByteCode,
    terminator: PhantomData<T>,
}

pub struct GraphTraversalSource<S, E> {
    start: PhantomData<S>,
    bc: Option<ByteCode>,
    end: PhantomData<E>,
}

impl<S, E> GraphTraversalSource<S, E> {
    pub fn v<I>(&mut self, ids: I) -> GraphTraversal<S, Vertex, Vertex>
    where
        I: Into<Ids>,
    {
        if let Some(mut bc) = self.bc.take() {
            bc.add_step("V", ids.into().into());
            GraphTraversal::new(bc)
        } else {
            let mut bc = ByteCode::new();
            bc.add_step("V", ids.into().into());
            GraphTraversal::new(bc)
        }
    }
    // pub fn add_v<L>(self, label: L) -> GraphTraverser<S, Vertex, Vertex> {
    //     let bc = ByteCode::new();
    //     bc.add_step("addV", ids.into().into());
    //     GraphTraverser::new(bc)
    // }
    pub fn e<I: Into<Ids>>(&self, ids: I) -> GraphTraversal<S, Edge, Edge> {
        let mut bc = ByteCode::new();
        bc.add_step("E", ids.into().into());
        GraphTraversal::new(bc)
    }

    // pub fn add_e<L>(&self, label: L) -> GraphTraversal<S, Edge, Edge> {
    //     let bc = ByteCode::new();
    //     bc.add_step("addE", ids.into().into());
    //     GraphTraverser::new(bc)
    // }

    pub fn with_computer(&mut self) -> &mut Self {
        if let Some(ref mut bc) = self.bc {
            bc.add_source("withComputer", vec![])
        } else {
            let mut bc = ByteCode::default();
            bc.add_source("withComputer", vec![]);
            self.bc = Some(bc)
        }
        self
    }

    pub fn inject<I: Into<GraphBinary>>(&mut self, items: I) -> GraphTraversal<S, I, I> {
        if let Some(mut bc) = self.bc.take() {
            bc.add_step("V", vec![items.into()]);
            GraphTraversal::new(bc)
        } else {
            let mut bc = ByteCode::new();
            bc.add_step("V", vec![items.into()]);
            GraphTraversal::new(bc)
        }
    }
}

impl<S, E, T> GraphTraversal<S, E, T> {
    fn new(bytecode: ByteCode) -> GraphTraversal<S, E, T> {
        GraphTraversal {
            start: PhantomData,
            end: PhantomData,
            bytecode,
            terminator: PhantomData,
        }
    }

    pub fn id(mut self) -> GraphTraversal<S, Ids, Ids> {
        self.bytecode.add_step("id", vec![]);
        GraphTraversal {
            start: PhantomData,
            end: PhantomData,
            bytecode: self.bytecode,
            terminator: PhantomData,
        }
    }

    pub fn label(mut self) -> GraphTraversal<S, String, String> {
        self.bytecode.add_step("label", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn constant<C: Into<GraphBinary>>(mut self, constant: C) -> GraphTraversal<S, C, C> {
        self.bytecode.add_step("constant", vec![constant.into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn v<A: Into<GraphBinary>>(mut self, a: A) -> GraphTraversal<S, Vertex, T> {
        self.bytecode.add_step("V", vec![a.into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn to(mut self, to_vertex: impl ToStepParams) -> GraphTraversal<S, Vertex, Vertex> {
        // TODO overload of to return different traversal
        to_vertex.bytecode("to", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn out(mut self, labels: impl MultiStringParams) -> GraphTraversal<S, Vertex, Vertex> {
        labels.bytecode("out", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn in_(mut self, labels: impl MultiStringParams) {
        labels.bytecode("in", &mut self.bytecode)
    }

    pub fn both(mut self, labels: impl MultiStringParams) {
        labels.bytecode("both", &mut self.bytecode)
    }

    pub fn to_e(
        mut self,
        direction: Direction,
        labels: impl MultiStringParams,
    ) -> GraphTraversal<S, Edge, Edge> {
        self.bytecode.add_step("in", vec![direction.into()]);
        labels.extend_step(&mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn out_e(mut self, labels: impl MultiStringParams) -> GraphTraversal<S, Edge, Edge> {
        labels.bytecode("outE", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn in_e(mut self, labels: impl MultiStringParams) -> GraphTraversal<S, Edge, Edge> {
        labels.bytecode("inE", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn both_e(mut self, labels: impl MultiStringParams) -> GraphTraversal<S, Edge, Edge> {
        labels.bytecode("bothE", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn to_v(mut self, direction: Direction) -> GraphTraversal<S, Vertex, Vertex> {
        self.bytecode.add_step("toV", vec![direction.into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn in_v(mut self) -> GraphTraversal<S, Vertex, Vertex> {
        self.bytecode.add_step("inV", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn out_v(mut self) -> GraphTraversal<S, Vertex, Vertex> {
        self.bytecode.add_step("outV", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn both_v(mut self) -> GraphTraversal<S, Vertex, Vertex> {
        self.bytecode.add_step("bothV", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn other_v(mut self) -> GraphTraversal<S, Vertex, Vertex> {
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

    pub fn values<E2>(
        mut self,
        property_keys: impl MultiStringParams,
    ) -> GraphTraversal<S, E2, E2> {
        property_keys.bytecode("values", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn property_map(mut self, property_keys: impl MultiStringParams) {
        property_keys.bytecode("propertyMap", &mut self.bytecode)
    }

    pub fn element_map(
        mut self,
    ) -> GraphTraversal<S, HashMap<MapKeys, GraphBinary>, HashMap<MapKeys, GraphBinary>> {
        self.bytecode.add_step("elementMap", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn value_map(mut self, property_keys: impl MultiStringParams) {
        property_keys.bytecode("valueMap", &mut self.bytecode)
    }

    pub fn key(mut self) -> GraphTraversal<S, String, String> {
        self.bytecode.add_step("key", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn value(mut self) -> Self {
        self.bytecode.add_step("value", vec![]);
        self
    }

    pub fn path(mut self) -> GraphTraversal<S, Path, Path> {
        self.bytecode.add_step("path", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn match_(
        mut self,
        match_traversal: GraphTraversal<S, E, T>,
    ) -> GraphTraversal<S, HashMap<MapKeys, GraphBinary>, HashMap<MapKeys, GraphBinary>> {
        self.bytecode
            .add_step("match", vec![match_traversal.bytecode.into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn sack() {}

    pub fn loops(mut self, s: impl SingleStringParam) -> GraphTraversal<S, i32, i32> {
        s.bytecode("loops", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn project(
        mut self,
        property_key: &str,
        other_property_keys: impl MultiStringParams,
    ) -> GraphTraversal<S, HashMap<String, GraphBinary>, HashMap<String, GraphBinary>> {
        self.bytecode.add_step("project", vec![property_key.into()]);
        other_property_keys.extend_step(&mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    // pub fn select_map(
    //     mut self,
    //     select_params: impl SelectMapParam,
    // ) -> GraphTraversal<S, HashMap<String, GraphBinary>, HashMap<String, GraphBinary>> {
    //     select_params.bytecode("select", &mut self.bytecode);
    //     GraphTraversal::new(self.bytecode)
    // }

    pub fn select(
        mut self,
        select_params: impl SelectParam,
    ) -> GraphTraversal<S, GraphBinary, GraphBinary> {
        select_params.bytecode("select", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn unfold(mut self) -> GraphTraversal<S, GraphBinary, GraphBinary> {
        self.bytecode.add_step("unfold", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn fold<E2>(mut self) -> GraphTraversal<S, Vec<E2>, Vec<E2>> {
        self.bytecode.add_step("fold", vec![]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn fold_with_seed<E2: Into<GraphBinary>, L: Into<Lambda>>(
        mut self,
        seed: E2,
        lambda: L,
    ) -> GraphTraversal<S, E2, E2> {
        self.bytecode
            .add_step("fold", vec![seed.into(), lambda.into().into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn count(mut self, scope: impl ScopeParams) -> GraphTraversal<S, i64, i64> {
        scope.bytecode("count", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn sum<V: PartialOrd>(mut self, scope: impl ScopeParams) -> GraphTraversal<S, V, V> {
        //TODO // Num Trait
        scope.bytecode("sum", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn max<V: PartialOrd>(mut self, scope: impl ScopeParams) -> GraphTraversal<S, V, V> {
        //TODO
        scope.bytecode("max", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn min<V: PartialOrd>(mut self, scope: impl ScopeParams) -> GraphTraversal<S, V, V> {
        //TODO
        scope.bytecode("min", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn mean<V: PartialOrd>(mut self, scope: impl ScopeParams) -> GraphTraversal<S, V, V> {
        //TODO
        scope.bytecode("mean", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn group(
        mut self,
        side_effect_key: impl SingleStringParam,
    ) -> GraphTraversal<S, HashMap<MapKeys, GraphBinary>, HashMap<MapKeys, GraphBinary>> {
        side_effect_key.bytecode("group", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn group_count(
        mut self,
        side_effect_key: impl SingleStringParam,
    ) -> GraphTraversal<S, HashMap<MapKeys, GraphBinary>, HashMap<MapKeys, i64>> {
        side_effect_key.bytecode("groupMap", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn tree(mut self, side_effect_key: impl SingleStringParam) -> Self {
        //Tree TODO
        side_effect_key.bytecode("tree", &mut self.bytecode);
        self
    }

    pub fn add_v(
        mut self,
        vertex_label: impl AddElementParams,
    ) -> GraphTraversal<S, Vertex, Vertex> {
        vertex_label.bytecode("addE", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }
    pub fn merge_v(mut self, merge_params: impl MergeParams) -> GraphTraversal<S, Vertex, Vertex> {
        GraphTraversal::new(self.bytecode)
    }

    pub fn merge_e(mut self, merge_params: impl MergeParams) -> GraphTraversal<S, Edge, Edge> {
        GraphTraversal::new(self.bytecode)
    }

    pub fn add_e(mut self, edge_label: impl AddElementParams) -> GraphTraversal<S, Edge, Edge> {
        edge_label.bytecode("addE", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn from(mut self, from_vertex: impl FromStepParams) -> Self {
        from_vertex.bytecode("from", &mut self.bytecode);
        self
    }

    pub fn math(mut self, expression: &str) -> GraphTraversal<S, f64, f64> {
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

    pub fn inject<I: Into<GraphBinary>>(mut self, items: I) -> GraphTraversal<S, I, I> {
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

    // pub fn to_list(self) -> T {
    // }

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

    pub fn not<S1: Into<GraphBinary>, S2: Into<GraphBinary>>(
        mut self,
        not_traversal: GraphTraversal<S1, S2, S2>,
    ) -> Self {
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

    pub fn side_effect(mut self, side_effect_traversal: GraphTraversal<S, E, T>) -> Self {
        //TODO
        self.bytecode
            .add_step("sideEffect", vec![side_effect_traversal.into()]);
        self
    }

    pub fn cap<V>(
        mut self,
        side_effect_key: &str,
        side_effect_keys: impl MultiStringParams,
    ) -> GraphTraversal<S, V, V> {
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

    pub fn optional<E2>(mut self, optional_traversel: GraphTraversal<S, E2, E2>) -> Self {
        self.bytecode
            .add_step("optional", vec![optional_traversel.into()]);
        self
    }

    pub fn union(mut self, union_traversal: GraphTraversal<S, E, T>) -> Self {
        self.bytecode
            .add_step("union", vec![union_traversal.bytecode.into()]);
        self
    }

    pub fn coalesce(
        mut self,
        coalesce_traversals: impl CoalesceParams,
    ) -> GraphTraversal<S, GraphBinary, GraphBinary> {
        coalesce_traversals.bytecode("coalesce", &mut self.bytecode);
        GraphTraversal::new(self.bytecode)
    }

    pub fn repeat(
        mut self,
        loop_name: impl SingleStringParam,
        loop_traversal: GraphTraversal<S, E, T>,
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

    pub fn local<E2>(
        mut self,
        local_traversal: GraphTraversal<S, E2, E2>,
    ) -> GraphTraversal<S, E2, E2> {
        self.bytecode
            .add_step("local", vec![local_traversal.into()]);
        GraphTraversal::new(self.bytecode)
    }

    pub fn page_rank(mut self) {}

    pub fn peer_pressure(mut self) -> Self {
        self.bytecode.add_step("peerPressure", vec![]);
        self
    }

    pub fn connected_component(mut self) {}

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

impl<S, E, T> From<GraphTraversal<S, E, T>> for GraphBinary {
    fn from(g: GraphTraversal<S, E, T>) -> Self {
        g.bytecode.into()
    }
}

impl<S, E, T> From<GraphTraversal<S, E, T>> for ByteCode {
    fn from(g: GraphTraversal<S, E, T>) -> Self {
        g.bytecode
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

pub struct AnonymousTraversal(ByteCode);

impl AnonymousTraversal {
    pub fn id(mut self) -> AnonymousTraversal {
        self.0.add_step("id", vec![]);
        self
    }

    pub fn label(mut self) -> AnonymousTraversal {
        self.0.add_step("label", vec![]);
        self
    }

    pub fn constant<C: Into<GraphBinary>>(mut self, constant: C) -> Self {
        self.0.add_step("constant", vec![constant.into()]);
        self
    }

    pub fn v<A: Into<GraphBinary>>(mut self, a: A) -> Self {
        self.0.add_step("V", vec![a.into()]);
        self
    }

    pub fn to(mut self, to_vertex: impl ToStepParams) -> Self {
        // TODO overload of to return different traversal
        to_vertex.bytecode("to", &mut self.0);
        self
    }

    pub fn out(mut self, labels: impl MultiStringParams) -> Self {
        labels.bytecode("out", &mut self.0);
        self
    }

    pub fn in_(mut self, labels: impl MultiStringParams) {
        labels.bytecode("in", &mut self.0)
    }

    pub fn both(mut self, labels: impl MultiStringParams) {
        labels.bytecode("both", &mut self.0)
    }

    pub fn to_e(mut self, direction: Direction, labels: impl MultiStringParams) -> Self {
        self.0.add_step("in", vec![direction.into()]);
        labels.extend_step(&mut self.0);
        self
    }

    pub fn out_e(mut self, labels: impl MultiStringParams) -> Self {
        labels.bytecode("outE", &mut self.0);
        self
    }

    pub fn in_e(mut self, labels: impl MultiStringParams) -> Self {
        labels.bytecode("inE", &mut self.0);
        self
    }

    pub fn both_e(mut self, labels: impl MultiStringParams) -> Self {
        labels.bytecode("bothE", &mut self.0);
        self
    }

    pub fn to_v(mut self, direction: Direction) -> Self {
        self.0.add_step("toV", vec![direction.into()]);
        self
    }

    pub fn in_v(mut self) -> Self {
        self.0.add_step("inV", vec![]);
        self
    }

    pub fn out_v(mut self) -> Self {
        self.0.add_step("outV", vec![]);
        self
    }

    pub fn both_v(mut self) -> Self {
        self.0.add_step("bothV", vec![]);
        self
    }

    pub fn other_v(mut self) -> Self {
        self.0.add_step("otherV", vec![]);
        self
    }

    pub fn order(mut self, scope: impl ScopeParams) -> Self {
        scope.bytecode("order", &mut self.0);
        self
    }

    pub fn properties(mut self, property_keys: impl MultiStringParams) {
        property_keys.bytecode("properties", &mut self.0)
    }

    pub fn values<E2>(mut self, property_keys: impl MultiStringParams) -> Self {
        property_keys.bytecode("values", &mut self.0);
        self
    }

    pub fn property_map(mut self, property_keys: impl MultiStringParams) {
        property_keys.bytecode("propertyMap", &mut self.0)
    }

    pub fn element_map(mut self) -> Self {
        self.0.add_step("elementMap", vec![]);
        self
    }

    pub fn value_map(mut self, property_keys: impl MultiStringParams) {
        property_keys.bytecode("valueMap", &mut self.0)
    }

    pub fn key(mut self) -> Self {
        self.0.add_step("key", vec![]);
        self
    }

    pub fn value(mut self) -> Self {
        self.0.add_step("value", vec![]);
        self
    }

    pub fn path(mut self) -> Self {
        self.0.add_step("path", vec![]);
        self
    }

    pub fn match_(mut self, match_traversal: AnonymousTraversal) -> Self {
        self.0.add_step("match", vec![match_traversal.0.into()]);
        self
    }

    pub fn sack() {}

    pub fn loops(mut self, s: impl SingleStringParam) -> Self {
        s.bytecode("loops", &mut self.0);
        self
    }

    pub fn project(
        mut self,
        property_key: &str,
        other_property_keys: impl MultiStringParams,
    ) -> Self {
        self.0.add_step("project", vec![property_key.into()]);
        other_property_keys.extend_step(&mut self.0);
        self
    }

    // pub fn select_map(
    //     mut self,
    //     select_params: impl SelectMapParam,
    // ) -> GraphTraversal<S, HashMap<String, GraphBinary>, HashMap<String, GraphBinary>> {
    //     select_params.bytecode("select", &mut self.0);
    //     self
    // }

    pub fn select(mut self, select_params: impl SelectParam) -> Self {
        select_params.bytecode("select", &mut self.0);
        self
    }

    pub fn unfold(mut self) -> Self {
        self.0.add_step("unfold", vec![]);
        self
    }

    pub fn fold<E2>(mut self) -> Self {
        self.0.add_step("fold", vec![]);
        self
    }

    pub fn fold_with_seed<E2: Into<GraphBinary>, L: Into<Lambda>>(
        mut self,
        seed: E2,
        lambda: L,
    ) -> Self {
        self.0
            .add_step("fold", vec![seed.into(), lambda.into().into()]);
        self
    }

    pub fn count(mut self, scope: impl ScopeParams) -> Self {
        scope.bytecode("count", &mut self.0);
        self
    }

    pub fn sum<V: PartialOrd>(mut self, scope: impl ScopeParams) -> Self {
        //TODO // Num Trait
        scope.bytecode("sum", &mut self.0);
        self
    }

    pub fn max<V: PartialOrd>(mut self, scope: impl ScopeParams) -> Self {
        //TODO
        scope.bytecode("max", &mut self.0);
        self
    }

    pub fn min<V: PartialOrd>(mut self, scope: impl ScopeParams) -> Self {
        //TODO
        scope.bytecode("min", &mut self.0);
        self
    }

    pub fn mean<V: PartialOrd>(mut self, scope: impl ScopeParams) -> Self {
        //TODO
        scope.bytecode("mean", &mut self.0);
        self
    }

    pub fn group(mut self, side_effect_key: impl SingleStringParam) -> Self {
        side_effect_key.bytecode("group", &mut self.0);
        self
    }

    pub fn group_count(mut self, side_effect_key: impl SingleStringParam) -> Self {
        side_effect_key.bytecode("groupMap", &mut self.0);
        self
    }

    pub fn tree(mut self, side_effect_key: impl SingleStringParam) -> Self {
        //Tree TODO
        side_effect_key.bytecode("tree", &mut self.0);
        self
    }

    pub fn add_v(mut self, vertex_label: impl AddElementParams) -> Self {
        vertex_label.bytecode("addE", &mut self.0);
        self
    }
    pub fn merge_v(mut self, merge_params: impl MergeParams) -> Self {
        self
    }

    pub fn merge_e(mut self, merge_params: impl MergeParams) -> Self {
        self
    }

    pub fn add_e(mut self, edge_label: impl AddElementParams) -> Self {
        edge_label.bytecode("addE", &mut self.0);
        self
    }

    pub fn from(mut self, from_vertex: impl FromStepParams) -> Self {
        from_vertex.bytecode("from", &mut self.0);
        self
    }

    pub fn math(mut self, expression: &str) -> Self {
        self.0.add_step("math", vec![expression.into()]);
        self
    }

    pub fn element(mut self) -> Self {
        //TODO
        self.0.add_step("element", vec![]);
        self
    }

    pub fn call() {}

    pub fn filter(mut self) -> Self {
        // TODO
        self
    }

    pub fn none(mut self) -> Self {
        self.0.add_step("none", vec![]);
        self
    }

    pub fn or() {} // TODO

    pub fn and() {} // TODO

    pub fn inject<I: Into<GraphBinary>>(mut self, items: I) -> Self {
        self.0.add_step("inject", vec![items.into()]);
        self
    }

    pub fn dedup(mut self, scope_and_labels: impl DedupStepParams) -> Self {
        scope_and_labels.bytecode("dedup", &mut self.0);
        self
    }

    pub fn where_(mut self, params: impl WhereParams) -> Self {
        params.bytecode("where", &mut self.0);
        self
    }

    pub fn has(mut self, params: impl HasStepParams) -> Self {
        params.bytecode("has", &mut self.0);
        self
    }

    pub fn has_not(mut self, property_key: &str) -> Self {
        self.0.add_step("hasNot", vec![property_key.into()]);
        self
    }
    pub fn has_label(mut self, label: impl HasStringsParams) -> Self {
        label.bytecode("hasLabel", &mut self.0);
        self
    }
    pub fn has_id(mut self, has_id_params: impl HasIdParams) -> Self {
        has_id_params.bytecode("hasId", &mut self.0);
        self
    }

    pub fn has_key(mut self, label: impl HasStringsParams) -> Self {
        label.bytecode("hasKey", &mut self.0);
        self
    }

    // pub fn to_list(self) -> T {
    // }

    pub fn has_value(
        mut self,
        value: impl Into<GraphBinary>,
        values: impl MultiObjectParam,
    ) -> Self {
        self.0.add_step("hasValue", vec![value.into()]);
        values.extend_step(&mut self.0);
        self
    }

    pub fn is<E>(mut self, p_or_objet: impl IsParam<E>) -> Self {
        p_or_objet.bytecode("is", &mut self.0);
        self
    }

    pub fn not(mut self, not_traversal: AnonymousTraversal) -> Self {
        self.0.add_step("not", vec![not_traversal.0.into()]);
        self
    }

    pub fn coin(mut self, propability: f64) -> Self {
        self.0.add_step("coin", vec![propability.into()]);
        self
    }

    pub fn range(mut self, scope: impl ScopeParams, low: i64, high: i64) -> Self {
        scope.bytecode("range", &mut self.0);
        self.0.add_to_last_step(low);
        self.0.add_to_last_step(high);
        self
    }

    pub fn limit(mut self, scope: impl ScopeParams, limit: i64) -> Self {
        scope.bytecode("limit", &mut self.0);
        self.0.add_to_last_step(limit);
        self
    }

    pub fn tail(mut self, tail_param: impl TailParams) -> Self {
        tail_param.bytecode("tail", &mut self.0);
        self
    }

    pub fn skip(mut self, scope: impl ScopeParams, skip: i64) -> Self {
        scope.bytecode("skip", &mut self.0);
        self.0.add_to_last_step(skip);
        self
    }

    pub fn time_limit(mut self, time_limit: i64) {
        self.0.add_step("timeLimit", vec![time_limit.into()])
    }

    pub fn simple_path(mut self) -> Self {
        self.0.add_step("simplePath", vec![]);
        self
    }

    pub fn cyclic_path(mut self) -> Self {
        self.0.add_step("cyclicPath", vec![]);
        self
    }

    pub fn sample(mut self, scope: impl ScopeParams, amount_to_sample: i32) -> Self {
        scope.bytecode("sample", &mut self.0);
        self.0.add_to_last_step(amount_to_sample);
        self
    }

    pub fn drop(mut self) -> Self {
        self.0.add_step("drop", vec![]);
        self
    }

    pub fn side_effect(mut self, side_effect_traversal: AnonymousTraversal) -> Self {
        //TODO
        self.0
            .add_step("sideEffect", vec![side_effect_traversal.0.into()]);
        self
    }

    pub fn cap<V>(
        mut self,
        side_effect_key: &str,
        side_effect_keys: impl MultiStringParams,
    ) -> Self {
        self.0.add_step("cap", vec![side_effect_key.into()]);
        side_effect_keys.extend_step(&mut self.0);
        self
    }

    pub fn subgraph(mut self, side_effect_key: &str) -> Self {
        self.0.add_step("subgraph", vec![side_effect_key.into()]);
        self
    }

    pub fn aggregate(mut self, scope: impl ScopeParams, side_effect_key: &str) -> Self {
        scope.bytecode("aggregate", &mut self.0);
        self.0.add_to_last_step(side_effect_key);
        self
    }

    pub fn fail(mut self, message: impl SingleStringParam) -> Self {
        message.bytecode("fail", &mut self.0);
        self
    }

    pub fn profile(mut self, message: impl SingleStringParam) -> Self {
        message.bytecode("profile", &mut self.0);
        self
    }

    pub fn property(mut self, proptery_params: impl PropertyParam) -> Self {
        proptery_params.bytecode("property", &mut self.0);
        self
    }

    pub fn branch(mut self) {}

    pub fn choose(mut self) {}

    pub fn optional<E2>(mut self, optional_traversel: AnonymousTraversal) -> Self {
        self.0
            .add_step("optional", vec![optional_traversel.0.into()]);
        self
    }

    pub fn union(mut self, union_traversal: AnonymousTraversal) -> Self {
        self.0.add_step("union", vec![union_traversal.0.into()]);
        self
    }

    pub fn coalesce(mut self, coalesce_traversals: impl CoalesceParams) -> Self {
        coalesce_traversals.bytecode("coalesce", &mut self.0);
        self
    }

    pub fn repeat(
        mut self,
        loop_name: impl SingleStringParam,
        loop_traversal: AnonymousTraversal,
    ) -> Self {
        loop_name.bytecode("repeat", &mut self.0);
        self.0.add_to_last_step(loop_traversal.0);
        self
    }

    pub fn emit(mut self, emit_params: impl EmitParams) -> Self {
        emit_params.bytecode("emit", &mut self.0);
        self
    }

    pub fn until(mut self, params: impl UntilParams) -> Self {
        params.bytecode("until", &mut self.0);
        self
    }

    pub fn times(mut self, max_loops: i32) -> Self {
        self.0.add_step("times", vec![max_loops.into()]);
        self
    }

    pub fn local<E2>(mut self, local_traversal: AnonymousTraversal) -> Self {
        self.0.add_step("local", vec![local_traversal.0.into()]);
        self
    }

    pub fn page_rank(mut self) {}

    pub fn peer_pressure(mut self) -> Self {
        self.0.add_step("peerPressure", vec![]);
        self
    }

    pub fn connected_component(mut self) {}

    pub fn shortest_path(mut self) -> Self {
        self.0.add_step("shortestPath", vec![]);
        self
    }

    pub fn programm(mut self) {}

    pub fn as_(mut self, step_label: &str, step_labels: impl MultiStringParams) -> Self {
        self.0.add_step("as", vec![step_label.into()]);
        step_labels.extend_step(&mut self.0);
        self
    }

    pub fn barrier(mut self) {}

    pub fn index(mut self) -> Self {
        self.0.add_step("index", vec![]);
        self
    }

    pub fn with(mut self, key: &str, object: impl ObjectParam) -> Self {
        self.0.add_step("with", vec![key.into()]);
        object.extend_step(&mut self.0);
        self
    }

    pub fn by(mut self, params: impl ByParams) -> Self {
        params.bytecode("by", &mut self.0);
        self
    }

    pub fn option(mut self, option_params: impl OptionParams) -> Self {
        option_params.bytecode("option", &mut self.0);
        self
    }

    pub fn read(mut self) -> Self {
        self.0.add_step("read", vec![]);
        self
    }

    pub fn write(mut self) -> Self {
        self.0.add_step("write", vec![]);
        self
    }

    pub fn iterate(mut self) -> Self {
        self.0.add_step("iterate", vec![]);
        self
    }
}

#[test]
fn test() {
    let g = GraphTraversalSource::<(), ()> {
        start: PhantomData,
        bc: Some(ByteCode::default()),
        end: PhantomData,
    };
    // g.v(()).has("label", "key", P::eq(2f32));

    // g.v(()).has("label", "key", P::gt(2f32));
}

#[test]
fn test1() {
    let mut g = GraphTraversalSource::<GraphBinary, GraphBinary> {
        start: PhantomData,
        bc: Some(ByteCode::default()),
        end: PhantomData,
    };
    let t = g.with_computer().inject(vec![1, 123, 3, 4]);

    let v = vec!["asasdd".to_string()];
    let t = g.v(()).project("id", ["d"]);
    let t = g.v(()).has(("age", g.v(())));
    let t = g.v(()).add_v(g.v(()).values("name"));
    let t = g.v(()).add_e("asd").from(g.v(()));
    let t = g
        .v(())
        .option((Merge::OnCreate, HashMap::from([("asd", 3)])));

    let t = g.v(()).coalesce([g.v(()).values("age"), g.e(())]);
    let t = g.v(()).is(Vertex {
        id: todo!(),
        label: todo!(),
        properties: todo!(),
    });
    let t = g.v(()).not(g.v(()).values("as").is(1)); // TODO
    let t = g.v(()).project("id", ["s"]).by(g.v(()).id()).by(g.v(())); // TODO project params are stupid
    let t = g.v(()).project("id", ["s"]).by(g.v(()).id()).by(g.v(()));

    // let t = g.v(()).as_("v", ()).select("v");
    println!("{:?}", t.bytecode)
}
