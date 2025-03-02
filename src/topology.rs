use crate::geom::{AxisKind, CornerKind, EdgeKind, FaceKind};
use crate::morton::MortonKey;
use iter_seq::{AsSequence, ConstLen, Sequence};

/// An octree node/cell.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct OctreeCell(MortonKey);

impl OctreeCell {
    /// Creates a new `Cell` from its morton code.
    ///
    /// If `key` doesn't represent a valid octree node, returns None.
    pub fn new(key: MortonKey) -> Option<Self> {
        if !key.is_none() {
            Some(OctreeCell(key))
        } else {
            None
        }
    }

    /// Retrieves the MortonKey code ("key") corresponding this cell.
    pub fn key(&self) -> MortonKey {
        self.0
    }

    /// Retrieves the sub-cell of this cell.
    ///
    /// This method does not distinguish between interior and leaf cells,
    /// so the caller must ensure that `self` is not a leaf to preserve
    /// the expected behavior.
    fn sub_cell(&self, corner: CornerKind) -> OctreeCell {
        OctreeCell(self.0.child(corner.0))
    }

    /// Returns the children of this octree cell.
    ///
    /// This method does not distinguish between interior and leaf cells, so
    /// the returned array will always contain 8 elements.
    #[inline]
    pub fn sub_cells(&self) -> impl Sequence<Item = OctreeCell> + ConstLen<8> + use<'_> {
        CornerKind::ALL
            .as_sequence()
            .map(|corner| self.sub_cell(*corner))
    }

    /// Returns this cell's interior faces, i.e., those between the
    /// face-adjacent pairs of its sub-cells, in any order.
    ///
    /// This method does not distinguish between interior and leaf cells, so
    /// the returned iterator will always yield 12 elements.
    pub fn interior_faces(&self) -> impl Sequence<Item = OctreeFace> + ConstLen<12> + use<'_> {
        EdgeKind::ALL
            .as_sequence()
            .map(|edge| OctreeFace::from_edge(*self, *edge))
    }

    /// Returns this cell's interior edges, i.e., those adjacent to 4 sub-cells
    /// of this cell at a time, in any order.
    ///
    /// This method does not distinguish between interior and leaf cells, so
    /// the returned iterator will always yield 6 elements.
    pub fn interior_edges(&self) -> impl Sequence<Item = Edge> + ConstLen<6> + use<'_> {
        FaceKind::ALL
            .as_sequence()
            .map(|face| Edge::from_face(self, *face))
    }

    /// Retrieves the sub-cells of this cell adjacent to the given face.
    fn face_sub_cells(&self, face: FaceKind) -> [OctreeCell; 4] {
        face.corners().map(|corner| self.sub_cell(corner))
    }

    /// Retrieves the sub-cells of this cell adjacent to the given edge.
    fn edge_sub_cells(&self, edge: EdgeKind) -> [OctreeCell; 2] {
        edge.endpoints().map(|corner| self.sub_cell(corner))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OctreeFace {
    normal: AxisKind,
    neighbors: [OctreeCell; 2],
}

impl OctreeFace {
    fn from_edge(cell: OctreeCell, edge: EdgeKind) -> OctreeFace {
        let normal = edge.axis();
        let neighbors = cell.edge_sub_cells(edge);
        Self { normal, neighbors }
    }

    pub fn sub_faces<L>(&self, _is_leaf: L) -> Option<[OctreeFace; 4]>
    where
        L: FnMut(&OctreeCell) -> bool,
    {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Edge {
    axis: AxisKind,
    neighbors: [OctreeCell; 4],
}

impl Edge {
    fn new(axis: AxisKind, neighbors: [OctreeCell; 4]) -> Edge {
        Self { axis, neighbors }
    }

    fn from_face(cell: &OctreeCell, face: FaceKind) -> Edge {
        let axis = face.normal_axis();
        let neighbors = cell.face_sub_cells(face);
        Self { axis, neighbors }
    }

    pub fn sub_edges<L>(&self, _is_leaf: L) -> Option<[Edge; 2]>
    where
        L: FnMut(&OctreeCell) -> bool,
    {
        todo!()
    }
}
