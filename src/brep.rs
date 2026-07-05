#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FaceId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoopId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoEdgeId(usize);

#[derive(Debug, Clone)]
pub struct BRepGraph {
    vertices: Vec<Point3>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
    loops: Vec<Loop>,
    co_edges: Vec<CoEdge>,
}

impl BRepGraph {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            loops: Vec::new(),
            co_edges: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, point: Point3) -> VertexId {
        self.vertices.push(point);
        VertexId(self.vertices.len() - 1)
    }

    pub fn add_edge(&mut self, from: VertexId, to: VertexId) -> EdgeId {
        let edge_id = EdgeId(self.edges.len());
        self.edges.push(Edge {
            id: edge_id,
            from,
            to,
            co_edge: None,
        });
        edge_id
    }

    pub fn add_face(&mut self) -> FaceId {
        let face_id = FaceId(self.faces.len());
        self.faces.push(Face {
            id: face_id,
            loops: Vec::new(),
        });
        face_id
    }

    pub fn add_loop(&mut self, face: FaceId, edges: Vec<EdgeId>) -> LoopId {
        let loop_id = LoopId(self.loops.len());
        self.loops.push(Loop {
            id: loop_id,
            face,
            edges: edges.clone(),
        });

        if let Some(face_state) = self.faces.get_mut(face.0) {
            face_state.loops.push(loop_id);
        }

        for edge_id in edges {
            if let Some(edge_state) = self.edges.get_mut(edge_id.0) {
                let co_edge_id = CoEdgeId(self.co_edges.len());
                edge_state.co_edge = Some(co_edge_id);
                self.co_edges.push(CoEdge {
                    id: co_edge_id,
                    edge: edge_id,
                    orientation: 1,
                });
            }
        }

        loop_id
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn loop_count(&self) -> usize {
        self.loops.len()
    }

    pub fn loop_contains(&self, loop_id: LoopId, edge_id: EdgeId) -> bool {
        self.loops
            .get(loop_id.0)
            .map(|loop_| loop_.edges.contains(&edge_id))
            .unwrap_or(false)
    }

    pub fn vertex_position(&self, id: VertexId) -> Point3 {
        self.vertices[id.0]
    }

    pub fn vertices(&self) -> impl Iterator<Item = Point3> + '_ {
        self.vertices.iter().copied()
    }

    pub fn edges(&self) -> impl Iterator<Item = EdgeRef> + '_ {
        self.edges.iter().enumerate().map(|(idx, edge)| EdgeRef {
            id: EdgeId(idx),
            from: edge.from,
            to: edge.to,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeRef {
    pub id: EdgeId,
    pub from: VertexId,
    pub to: VertexId,
}

impl EdgeRef {
    pub fn endpoints(self) -> Option<(VertexId, VertexId)> {
        Some((self.from, self.to))
    }
}

#[derive(Debug, Clone)]
struct Edge {
    id: EdgeId,
    from: VertexId,
    to: VertexId,
    co_edge: Option<CoEdgeId>,
}

#[derive(Debug, Clone)]
struct Face {
    id: FaceId,
    loops: Vec<LoopId>,
}

#[derive(Debug, Clone)]
struct Loop {
    id: LoopId,
    face: FaceId,
    edges: Vec<EdgeId>,
}

#[derive(Debug, Clone)]
struct CoEdge {
    id: CoEdgeId,
    edge: EdgeId,
    orientation: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_can_hold_vertices_edges_and_faces() {
        let mut graph = super::BRepGraph::new();
        let a = graph.add_vertex(Point3::new(0.0, 0.0, 0.0));
        let b = graph.add_vertex(Point3::new(1.0, 0.0, 0.0));
        let c = graph.add_vertex(Point3::new(1.0, 1.0, 0.0));
        let d = graph.add_vertex(Point3::new(0.0, 1.0, 0.0));

        let edge_ab = graph.add_edge(a, b);
        let edge_bc = graph.add_edge(b, c);
        let edge_cd = graph.add_edge(c, d);
        let edge_da = graph.add_edge(d, a);

        let face = graph.add_face();
        let loop_id = graph.add_loop(face, vec![edge_ab, edge_bc, edge_cd, edge_da]);

        assert_eq!(graph.vertex_count(), 4);
        assert_eq!(graph.edge_count(), 4);
        assert_eq!(graph.face_count(), 1);
        assert_eq!(graph.loop_count(), 1);
        assert!(graph.loop_contains(loop_id, edge_ab));
    }
}
