use std::marker::PhantomData;

use serde::Deserialize;
use tinkerpop_io::{
    structure::{bytecode::Bytecode, edge::Edge, vertex::Vertex},
    GremlinValue,
};

use super::{
    params::{add_element_params::AddElementParams, sack_params::WithSackParam},
    traversal::{GraphTraversal, Ids},
};

pub struct GraphTraversalSource<E> {
    pub bc: Option<Bytecode>,
    pub end: PhantomData<E>,
}

impl<'de, E> GraphTraversalSource<E> {
    pub fn new() -> Self {
        GraphTraversalSource {
            bc: None,
            end: PhantomData,
        }
    }

    pub fn v<I>(&mut self, ids: I) -> GraphTraversal<Vertex, Vertex>
    where
        I: Into<Ids>,
    {
        if let Some(mut bc) = self.bc.take() {
            bc.push_new_step("V", ids.into().into());
            GraphTraversal::new(bc)
        } else {
            let mut bc = Bytecode::new();
            bc.push_new_step("V", ids.into().into());
            GraphTraversal::new(bc)
        }
    }
    pub fn add_v<L>(
        &mut self,
        vertex_label: impl AddElementParams,
    ) -> GraphTraversal<Vertex, Vertex> {
        if let Some(mut bc) = self.bc.take() {
            vertex_label.bytecode("addV", &mut bc);
            GraphTraversal::new(bc)
        } else {
            let mut bc = Bytecode::new();
            vertex_label.bytecode("addV", &mut bc);
            GraphTraversal::new(bc)
        }
    }

    pub fn add_e<L>(&mut self, vertex_label: impl AddElementParams) -> GraphTraversal<Edge, Edge> {
        if let Some(mut bc) = self.bc.take() {
            vertex_label.bytecode("addE", &mut bc);
            GraphTraversal::new(bc)
        } else {
            let mut bc = Bytecode::new();
            vertex_label.bytecode("addE", &mut bc);
            GraphTraversal::new(bc)
        }
    }

    pub fn e<I: Into<Ids>>(&mut self, ids: I) -> GraphTraversal<Edge, Edge> {
        if let Some(mut bc) = self.bc.take() {
            bc.push_new_step("E", ids.into().into());
            GraphTraversal::new(bc)
        } else {
            let mut bc = Bytecode::new();
            bc.push_new_step("E", ids.into().into());
            GraphTraversal::new(bc)
        }
    }

    pub fn with_sack(&mut self, params: impl WithSackParam) -> &mut Self {
        let bc = self.bc.get_or_insert(Bytecode::default());
        params.bytecode("withSack", bc);
        self
    }

    pub fn with_computer(&mut self) -> &mut Self {
        let bc = self.bc.get_or_insert(Bytecode::default());
        bc.push_new_source("withComputer", vec![]);
        self
    }

    pub fn with_path(&mut self) -> &mut Self {
        let bc = self.bc.get_or_insert(Bytecode::default());
        bc.push_new_source("withPath", vec![]);
        self
    }

    pub fn with_side_effect(&mut self) -> &mut Self {
        //TODO
        let bc = self.bc.get_or_insert(Bytecode::default());
        bc.push_new_source("withSideEffect", vec![]);
        self
    }

    pub fn with_strategies(&mut self) -> &mut Self {
        //TODO
        let bc = self.bc.get_or_insert(Bytecode::default());
        bc.push_new_source("withStrategies", vec![]);
        self
    }

    pub fn inject<I: Into<GremlinValue> + Deserialize<'de>>(
        &mut self,
        items: I,
    ) -> GraphTraversal<I, I> {
        if let Some(mut bc) = self.bc.take() {
            bc.push_new_step("V", vec![items.into()]);
            GraphTraversal::new(bc)
        } else {
            let mut bc = Bytecode::new();
            bc.push_new_step("V", vec![items.into()]);
            GraphTraversal::new(bc)
        }
    }

    fn close(self) {}
}
