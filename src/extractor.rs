use crate::source::HermiteSource;
use auto_impl::auto_impl;
use glam::Vec3;

#[auto_impl(&mut, Box)]
pub trait Extractor {
    fn extract_vertex(&mut self, position: Vec3);
    fn extract_face(&mut self, face: [u32; 3]);
}

#[derive(Debug, Default)]
pub struct SeparateNormals {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
}

#[derive(Debug, Default)]
pub struct IndexedSeparateNormals {
    pub vertices: SeparateNormals,
    pub faces: Vec<[u32; 3]>,
}

pub struct WithIndexedSeparateNormals<'a, S> {
    buf: &'a mut IndexedSeparateNormals,
    source: S,
}

impl<'a, S> WithIndexedSeparateNormals<'a, S> {
    pub fn new(buffer: &'a mut IndexedSeparateNormals, source: S) -> Self {
        Self {
            buf: buffer,
            source,
        }
    }
}

impl<S: HermiteSource> WithIndexedSeparateNormals<'_, S> {
    fn average_vertex_normal(&self, face: [u32; 3]) -> Vec3 {
        let normals = &self.buf.vertices.normals;
        face.map(|i| normals[i as usize]).iter().sum::<Vec3>() / 3.0
    }

    fn face_plane_normal(&self, face: [u32; 3]) -> Vec3 {
        plane_normal(&face.map(|i| self.buf.vertices.positions[i as usize]))
    }
}

impl<S: HermiteSource> Extractor for WithIndexedSeparateNormals<'_, S> {
    fn extract_vertex(&mut self, position: Vec3) {
        self.buf.vertices.positions.push(position);
        let normal = self.source.sample_normal(position);
        self.buf.vertices.normals.push(normal);
    }

    fn extract_face(&mut self, mut face: [u32; 3]) {
        let normal = self.average_vertex_normal(face);

        if normal.dot(self.face_plane_normal(face)) < 0.0 {
            face.reverse();
        }

        self.buf.faces.push(face);
    }
}

fn plane_normal(points: &[Vec3; 3]) -> Vec3 {
    (points[1] - points[0]).cross(points[2] - points[1])
}
