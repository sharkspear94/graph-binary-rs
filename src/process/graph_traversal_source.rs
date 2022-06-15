use std::marker::PhantomData;

use crate::{
    graph_binary::GraphBinary,
    structure::{bytecode::ByteCode, edge::Edge, vertex::Vertex},
};

use super::{
    params::add_element_params::AddElementParams,
    traversal::{GraphTraversal, Ids},
};

pub struct GraphTraversalSource<S, E> {
    pub start: PhantomData<S>,
    pub bc: Option<ByteCode>,
    pub end: PhantomData<E>,
}

impl<S, E> GraphTraversalSource<S, E> {
    pub fn new() -> Self {
        GraphTraversalSource {
            start: PhantomData,
            bc: None,
            end: PhantomData,
        }
    }

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
    pub fn add_v<L>(
        &mut self,
        vertex_label: impl AddElementParams,
    ) -> GraphTraversal<S, Vertex, Vertex> {
        if let Some(mut bc) = self.bc.take() {
            vertex_label.bytecode("addV", &mut bc);
            GraphTraversal::new(bc)
        } else {
            let mut bc = ByteCode::new();
            vertex_label.bytecode("addV", &mut bc);
            GraphTraversal::new(bc)
        }
    }

    pub fn add_e<L>(
        &mut self,
        vertex_label: impl AddElementParams,
    ) -> GraphTraversal<S, Edge, Edge> {
        if let Some(mut bc) = self.bc.take() {
            vertex_label.bytecode("addE", &mut bc);
            GraphTraversal::new(bc)
        } else {
            let mut bc = ByteCode::new();
            vertex_label.bytecode("addE", &mut bc);
            GraphTraversal::new(bc)
        }
    }

    pub fn e<I: Into<Ids>>(&mut self, ids: I) -> GraphTraversal<S, Edge, Edge> {
        if let Some(mut bc) = self.bc.take() {
            bc.add_step("E", ids.into().into());
            GraphTraversal::new(bc)
        } else {
            let mut bc = ByteCode::new();
            bc.add_step("E", ids.into().into());
            GraphTraversal::new(bc)
        }
    }

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

    fn close(self) {}
}
