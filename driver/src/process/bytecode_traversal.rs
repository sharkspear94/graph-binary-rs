use tinkerpop_io::{
    structure::{bytecode::Bytecode, enums::Direction, lambda::Lambda},
    GremlinValue,
};

use super::params::{
    add_element_params::AddElementParams,
    by_params::ByParams,
    coalesce_params::CoalesceParams,
    dedup_params::DedupStepParams,
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
    property_params::PropertyParam,
    scope_params::ScopeParams,
    select_params::SelectParam,
    single_string::SingleStringParam,
    tail_params::TailParams,
    to_step_params::ToStepParams,
    until_params::UntilParams,
    where_params::WhereParams,
};

#[derive(Debug, Clone)]
pub struct BytecodeTraversal(Bytecode);

impl BytecodeTraversal {
    pub fn new(bc: Bytecode) -> Self {
        BytecodeTraversal(bc)
    }

    pub fn id(mut self) -> Self {
        self.0.push_new_step("id", vec![]);
        self
    }

    pub fn label(mut self) -> Self {
        self.0.push_new_step("label", vec![]);
        self
    }

    pub fn constant<C: Into<GremlinValue>>(mut self, constant: C) -> Self {
        self.0.push_new_step("constant", vec![constant.into()]);
        self
    }

    pub fn v(mut self) -> Self {
        self.0.push_new_step("V", vec![]);
        // self.0.push_new_step("V", vec![]);
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
        self.0.push_new_step("in", vec![direction.into()]);
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
        self.0.push_new_step("toV", vec![direction.into()]);
        self
    }

    pub fn in_v(mut self) -> Self {
        self.0.push_new_step("inV", vec![]);
        self
    }

    pub fn out_v(mut self) -> Self {
        self.0.push_new_step("outV", vec![]);
        self
    }

    pub fn both_v(mut self) -> Self {
        self.0.push_new_step("bothV", vec![]);
        self
    }

    pub fn other_v(mut self) -> Self {
        self.0.push_new_step("otherV", vec![]);
        self
    }

    pub fn order(mut self, scope: impl ScopeParams) -> Self {
        scope.bytecode("order", &mut self.0);
        self
    }

    pub fn properties(mut self, property_keys: impl MultiStringParams) {
        property_keys.bytecode("properties", &mut self.0)
    }

    pub fn values(mut self, property_keys: impl MultiStringParams) -> Self {
        property_keys.bytecode("values", &mut self.0);
        self
    }

    pub fn property_map(mut self, property_keys: impl MultiStringParams) {
        property_keys.bytecode("propertyMap", &mut self.0)
    }

    pub fn element_map(mut self) -> Self {
        self.0.push_new_step("elementMap", vec![]);
        self
    }

    pub fn value_map(mut self, property_keys: impl MultiStringParams) {
        property_keys.bytecode("valueMap", &mut self.0)
    }

    pub fn key(mut self) -> Self {
        self.0.push_new_step("key", vec![]);
        self
    }

    pub fn value(mut self) -> Self {
        self.0.push_new_step("value", vec![]);
        self
    }

    pub fn path(mut self) -> Self {
        self.0.push_new_step("path", vec![]);
        self
    }

    pub fn match_(mut self, match_traversal: BytecodeTraversal) -> Self {
        self.0
            .push_new_step("match", vec![match_traversal.0.into()]);
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
        self.0.push_new_step("project", vec![property_key.into()]);
        other_property_keys.extend_step(&mut self.0);
        self
    }

    // pub fn select_map(
    //     mut self,
    //     select_params: impl SelectMapParam,
    // ) -> GraphTraversal<S, HashMap<String, GremlinValue>, HashMap<String, GremlinValue>> {
    //     select_params.bytecode("select", &mut self.0);
    //     self
    // }

    pub fn select(mut self, select_params: impl SelectParam) -> Self {
        select_params.bytecode("select", &mut self.0);
        self
    }

    pub fn unfold(mut self) -> Self {
        self.0.push_new_step("unfold", vec![]);
        self
    }

    pub fn fold<E2>(mut self) -> Self {
        self.0.push_new_step("fold", vec![]);
        self
    }

    pub fn fold_with_seed<E2: Into<GremlinValue>, L: Into<Lambda>>(
        mut self,
        seed: E2,
        lambda: L,
    ) -> Self {
        self.0
            .push_new_step("fold", vec![seed.into(), lambda.into().into()]);
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
        self.0.push_new_step("math", vec![expression.into()]);
        self
    }

    pub fn element(mut self) -> Self {
        //TODO
        self.0.push_new_step("element", vec![]);
        self
    }

    pub fn call() {}

    pub fn filter(mut self) -> Self {
        // TODO
        self
    }

    pub fn none(mut self) -> Self {
        self.0.push_new_step("none", vec![]);
        self
    }

    pub fn or() {} // TODO

    pub fn and(mut self) -> Self {
        self
    } // TODO

    pub fn inject<I: Into<GremlinValue>>(mut self, items: I) -> Self {
        self.0.push_new_step("inject", vec![items.into()]);
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
        self.0.push_new_step("hasNot", vec![property_key.into()]);
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
        value: impl Into<GremlinValue>,
        values: impl MultiObjectParam,
    ) -> Self {
        self.0.push_new_step("hasValue", vec![value.into()]);
        values.extend_step(&mut self.0);
        self
    }

    pub fn is<E>(mut self, p_or_objet: impl IsParam<E>) -> Self {
        p_or_objet.bytecode("is", &mut self.0);
        self
    }

    pub fn not(mut self, not_traversal: BytecodeTraversal) -> Self {
        self.0.push_new_step("not", vec![not_traversal.0.into()]);
        self
    }

    pub fn coin(mut self, propability: f64) -> Self {
        self.0.push_new_step("coin", vec![propability.into()]);
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
        self.0.push_new_step("timeLimit", vec![time_limit.into()])
    }

    pub fn simple_path(mut self) -> Self {
        self.0.push_new_step("simplePath", vec![]);
        self
    }

    pub fn cyclic_path(mut self) -> Self {
        self.0.push_new_step("cyclicPath", vec![]);
        self
    }

    pub fn sample(mut self, scope: impl ScopeParams, amount_to_sample: i32) -> Self {
        scope.bytecode("sample", &mut self.0);
        self.0.add_to_last_step(amount_to_sample);
        self
    }

    pub fn drop(mut self) -> Self {
        self.0.push_new_step("drop", vec![]);
        self
    }

    pub fn side_effect(mut self, side_effect_traversal: BytecodeTraversal) -> Self {
        //TODO
        self.0
            .push_new_step("sideEffect", vec![side_effect_traversal.0.into()]);
        self
    }

    pub fn cap<V>(
        mut self,
        side_effect_key: &str,
        side_effect_keys: impl MultiStringParams,
    ) -> Self {
        self.0.push_new_step("cap", vec![side_effect_key.into()]);
        side_effect_keys.extend_step(&mut self.0);
        self
    }

    pub fn subgraph(mut self, side_effect_key: &str) -> Self {
        self.0
            .push_new_step("subgraph", vec![side_effect_key.into()]);
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

    pub fn optional<E2>(mut self, optional_traversel: BytecodeTraversal) -> Self {
        self.0
            .push_new_step("optional", vec![optional_traversel.0.into()]);
        self
    }

    pub fn union(mut self, union_traversal: BytecodeTraversal) -> Self {
        self.0
            .push_new_step("union", vec![union_traversal.0.into()]);
        self
    }

    pub fn coalesce(mut self, coalesce_traversals: impl CoalesceParams) -> Self {
        coalesce_traversals.bytecode("coalesce", &mut self.0);
        self
    }

    pub fn repeat(
        mut self,
        loop_name: impl SingleStringParam,
        loop_traversal: BytecodeTraversal,
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
        self.0.push_new_step("times", vec![max_loops.into()]);
        self
    }

    pub fn local<E2>(mut self, local_traversal: BytecodeTraversal) -> Self {
        self.0
            .push_new_step("local", vec![local_traversal.0.into()]);
        self
    }

    pub fn page_rank(mut self) {}

    pub fn peer_pressure(mut self) -> Self {
        self.0.push_new_step("peerPressure", vec![]);
        self
    }

    pub fn connected_component(mut self) {}

    pub fn shortest_path(mut self) -> Self {
        self.0.push_new_step("shortestPath", vec![]);
        self
    }

    pub fn programm(mut self) {}

    pub fn as_(mut self, step_label: &str, step_labels: impl MultiStringParams) -> Self {
        self.0.push_new_step("as", vec![step_label.into()]);
        step_labels.extend_step(&mut self.0);
        self
    }

    pub fn barrier(mut self) {}

    pub fn index(mut self) -> Self {
        self.0.push_new_step("index", vec![]);
        self
    }

    pub fn with(mut self, key: &str, object: impl ObjectParam) -> Self {
        self.0.push_new_step("with", vec![key.into()]);
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
        self.0.push_new_step("read", vec![]);
        self
    }

    pub fn write(mut self) -> Self {
        self.0.push_new_step("write", vec![]);
        self
    }

    pub fn iterate(mut self) -> Self {
        self.0.push_new_step("iterate", vec![]);
        self
    }
}

impl From<BytecodeTraversal> for GremlinValue {
    fn from(a: BytecodeTraversal) -> Self {
        GremlinValue::Bytecode(a.0)
    }
}
