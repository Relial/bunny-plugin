use glam::Vec3;

use crate::{
    draw_list::PrimitiveTopology,
    mesh::{CircleBuilder, Mesh, MeshBuilder},
};

pub enum PerimeterSegment {
    Smooth { indices: Vec<u32> },
    Flat { indices: Vec<u32> },
}

impl PerimeterSegment {
    fn vertices_per_layer(&self) -> u32 {
        match self {
            PerimeterSegment::Smooth { indices } => indices.len() as u32,
            PerimeterSegment::Flat { indices } => 2 * (indices.len() as u32 - 1),
        }
    }

    fn indices_per_segment(&self) -> usize {
        match self {
            PerimeterSegment::Smooth { indices } | PerimeterSegment::Flat { indices } => {
                6 * (indices.len() - 1)
            }
        }
    }
}

pub trait Extrudable: MeshBuilder {
    fn perimeter(&self) -> Vec<PerimeterSegment>;
}

pub struct ExtrusionMesh<P>
where
    P: Extrudable,
{
    pub base_shape: P,
    pub half_depth: f32,
    pub segments: usize,
}

impl<P> ExtrusionMesh<P>
where
    P: Extrudable,
{
    pub fn new(base_shape: P, depth: f32) -> Self {
        Self {
            base_shape,
            half_depth: depth / 2.0,
            segments: 1,
        }
    }

    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }

    pub fn with_inner(mut self, func: impl Fn(P) -> P) -> Self {
        self.base_shape = func(self.base_shape);
        self
    }
}

impl ExtrusionMesh<CircleBuilder> {
    pub fn resolution(mut self, resolution: u32) -> Self {
        self.base_shape.resolution = resolution;
        self
    }
}

impl<P> MeshBuilder for ExtrusionMesh<P>
where
    P: Extrudable,
{
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.1/src/bevy_mesh/primitives/extrusion.rs.html#180

        // Create and move the base mesh to the front
        let mut front_face =
            self.base_shape
                .build()
                .translated_by(Vec3::new(0., 0., self.half_depth));

        assert_eq!(
            front_face.primitive_topology,
            PrimitiveTopology::TriangleList
        );

        // Move the uvs of the front face to be between (0., 0.) and (0.5, 0.5)
        front_face
            .uvs
            .iter_mut()
            .for_each(|uv| *uv = uv.map(|coord| coord * 0.5));

        let back_face = {
            // Flip the normals, etc. and move mesh to the back
            let mut back_face = front_face.clone().scaled_by(Vec3::new(1., 1., -1.));

            // Move the uvs of the back face to be between (0.5, 0.) and (1., 0.5)
            back_face
                .uvs
                .iter_mut()
                .for_each(|uv| *uv = [uv[0] + 0.5, uv[1]]);

            // By swapping the first and second indices of each triangle we invert the winding order thus making the mesh visible from the other side
            back_face
                .indices
                .as_chunks_mut()
                .0
                .iter_mut()
                .for_each(|[a, b, _]| core::mem::swap(a, b));
            back_face
        };

        // An extrusion of depth 0 does not need a mantel
        if self.half_depth == 0. {
            front_face.merge(&back_face).unwrap();
            return front_face;
        }

        let mantel = {
            let cap_verts = &front_face.positions;

            let layers = self.segments + 1;
            let layer_depth_delta = self.half_depth * 2.0 / self.segments as f32;

            let perimeter = self.base_shape.perimeter();
            let (vert_count, index_count) =
                perimeter
                    .iter()
                    .fold((0, 0), |(verts, indices), perimeter| {
                        (
                            verts + layers * perimeter.vertices_per_layer() as usize,
                            indices + self.segments * perimeter.indices_per_segment(),
                        )
                    });
            let mut positions = Vec::with_capacity(vert_count);
            let mut indices = Vec::with_capacity(index_count);
            let mut uvs = Vec::with_capacity(vert_count);

            // Compute the amount of horizontal space allocated to each segment of the perimeter.
            let uv_segment_delta = 1. / perimeter.len() as f32;
            for (i, segment) in perimeter.into_iter().enumerate() {
                // The start of the x range of the area of the current perimeter-segment.
                let uv_start = i as f32 * uv_segment_delta;

                match segment {
                    PerimeterSegment::Flat {
                        indices: segment_indices,
                    } => {
                        let uv_delta = uv_segment_delta / (segment_indices.len() - 1) as f32;
                        for i in 0..(segment_indices.len() - 1) {
                            let uv_x = uv_start + uv_delta * i as f32;
                            // Get the positions for the current and the next index.
                            let a = cap_verts[segment_indices[i] as usize];
                            let b = cap_verts[segment_indices[i + 1] as usize];

                            // Get the index of the next vertex added to the mantel.
                            let index = positions.len() as u32;

                            // Push the positions of the two indices and their equivalent points on each layer.
                            for i in 0..layers {
                                let i = i as f32;
                                let z = a[2] - layer_depth_delta * i;
                                positions.push([a[0], a[1], z]);
                                positions.push([b[0], b[1], z]);

                                // UVs for the mantel are between (0, 0.5) and (1, 1).
                                let uv_y = 0.5 + 0.5 * i / self.segments as f32;
                                uvs.push([uv_x, uv_y]);
                                uvs.push([uv_x + uv_delta, uv_y]);
                            }

                            // Add the indices for the vertices created above to the mesh.
                            for i in 0..self.segments as u32 {
                                let base_index = index + 2 * i;
                                indices.extend_from_slice(&[
                                    base_index,
                                    base_index + 2,
                                    base_index + 1,
                                    base_index + 1,
                                    base_index + 2,
                                    base_index + 3,
                                ]);
                            }
                        }
                    }
                    PerimeterSegment::Smooth {
                        indices: segment_indices,
                    } => {
                        let uv_delta = uv_segment_delta / (segment_indices.len() - 1) as f32;

                        // Since the indices for this segment will be added after its vertices have been added,
                        // we need to store the index of the first vertex that is part of this segment.
                        let base_index = positions.len() as u32;

                        // If there is a first vertex, we need to add it and its counterparts on each layer.
                        // The normal is provided by `segment.first_normal`.
                        if let Some(i) = segment_indices.first() {
                            let p = cap_verts[*i as usize];
                            for i in 0..layers {
                                let i = i as f32;
                                let z = p[2] - layer_depth_delta * i;
                                positions.push([p[0], p[1], z]);

                                let uv_y = 0.5 + 0.5 * i / self.segments as f32;
                                uvs.push([uv_start, uv_y]);
                            }
                        }

                        // For all points inbetween the first and last vertices, we can automatically compute the normals.
                        for i in 1..(segment_indices.len() - 1) {
                            let uv_x = uv_start + uv_delta * i as f32;

                            // Get the positions for the last, current and the next index.
                            let b = cap_verts[segment_indices[i] as usize];

                            // Add the current vertex and its counterparts on each layer.
                            for i in 0..layers {
                                let i = i as f32;
                                let z = b[2] - layer_depth_delta * i;
                                positions.push([b[0], b[1], z]);

                                let uv_y = 0.5 + 0.5 * i / self.segments as f32;
                                uvs.push([uv_x, uv_y]);
                            }
                        }

                        // If there is a last vertex, we need to add it and its counterparts on each layer.
                        // The normal is provided by `segment.last_normal`.
                        if let Some(i) = segment_indices.last() {
                            let p = cap_verts[*i as usize];
                            for i in 0..layers {
                                let i = i as f32;
                                let z = p[2] - layer_depth_delta * i;
                                positions.push([p[0], p[1], z]);

                                let uv_y = 0.5 + 0.5 * i / self.segments as f32;
                                uvs.push([uv_start + uv_segment_delta, uv_y]);
                            }
                        }

                        let columns = segment_indices.len() as u32;
                        let segments = self.segments as u32;
                        let layers = segments + 1;
                        for s in 0..segments {
                            for column in 0..(columns - 1) {
                                let index = base_index + s + column * layers;
                                indices.extend_from_slice(&[
                                    index,
                                    index + 1,
                                    index + layers,
                                    index + layers,
                                    index + 1,
                                    index + layers + 1,
                                ]);
                            }
                        }
                    }
                }
            }

            Mesh::new(positions, uvs, indices, PrimitiveTopology::TriangleList)
        };

        front_face.merge(&back_face).unwrap();
        front_face.merge(&mantel).unwrap();
        front_face
    }
}
