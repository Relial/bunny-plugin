use glam::{Vec2, Vec3};

use crate::{
    draw_list::PrimitiveTopology,
    mesh::{Extrudable, Inset, Mesh, MeshBuilder, PerimeterSegment, Primitive2d},
};

pub struct RingBuilder<P>
where
    P: Primitive2d + MeshBuilder,
{
    pub outer_shape: P,
    pub inner_shape: P,
}

impl<P> RingBuilder<P>
where
    P: Primitive2d + MeshBuilder,
{
    pub const fn new(outer_shape: P, inner_shape: P) -> Self {
        Self {
            outer_shape,
            inner_shape,
        }
    }

    pub fn with_inner(mut self, func: impl Fn(P) -> P) -> Self {
        self.outer_shape = func(self.outer_shape);
        self.inner_shape = func(self.inner_shape);
        self
    }
}

impl<P: Primitive2d + MeshBuilder> Primitive2d for RingBuilder<P> {}

impl<P: Primitive2d + MeshBuilder + Clone + Inset> RingBuilder<P> {
    pub fn from_primitive_and_thickness(primitive: P, thickness: f32) -> Self {
        let hollow = primitive.clone().inset(thickness);
        RingBuilder::new(primitive, hollow)
    }
}

impl<P> MeshBuilder for RingBuilder<P>
where
    P: Primitive2d + MeshBuilder,
{
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.1/src/bevy_mesh/primitives/dim2.rs.html#1391

        let inner_mesh = self.inner_shape.build();
        let inner_positions = inner_mesh.positions;
        let inner_uvs = inner_mesh.uvs;
        let outer_mesh = self.outer_shape.build();
        let outer_positions = outer_mesh.positions;
        let outer_uvs = outer_mesh.uvs;

        assert_eq!(inner_positions.len(), outer_positions.len());
        assert_eq!(inner_uvs.len(), outer_uvs.len());

        let mut uvs = outer_uvs;
        let inner_uvs = inner_positions
            .iter()
            .zip(&outer_positions)
            .zip(&inner_uvs)
            .map(|((inner_position, outer_position), inner_uv)| -> [f32; 2] {
                const UV_CENTER: Vec2 = Vec2::splat(0.5);
                let ip = Vec3::from(*inner_position).truncate();
                let op = Vec3::from(*outer_position).truncate();
                let uv = Vec2::from(*inner_uv) - UV_CENTER;
                (uv * ip.length() / op.length() + UV_CENTER).into()
            });
        uvs.extend(inner_uvs);

        let points = outer_positions.len() as u32;
        let mut indices = Vec::with_capacity(outer_positions.len() * 6);
        for i in 0..points {
            //                               for five points:
            indices.push(i); //              0  1  2  3  4
            indices.push(i + 1); //          1  2  3  4  0  <-
            indices.push(points + i); //     0' 1' 2' 3' 4'
            indices.push(points + i); //     0' 1' 2' 3' 4'
            indices.push(i + 1); //          1  2  3  4  0  <-
            indices.push(points + i + 1); // 1' 2' 3' 4' 0' <-
        }
        let indices_length = indices.len();
        // Fix up the last pair of triangles (return to start)
        if let (_, [_, b, _, _, e, f]) = indices.split_at_mut(indices_length.saturating_sub(6)) {
            *b = 0;
            *e = 0;
            *f = points;
        }

        let mut positions = outer_positions;
        positions.extend_from_slice(&inner_positions);

        Mesh::new(positions, uvs, vec![], indices, PrimitiveTopology::TriangleList)
    }
}

impl<P> Extrudable for RingBuilder<P>
where
    P: Primitive2d + Extrudable,
{
    fn perimeter(&self) -> Vec<PerimeterSegment> {
        let outer_shape = self.outer_shape.build();
        let inner_shape = self.inner_shape.build();

        assert_eq!(outer_shape.positions.len(), inner_shape.positions.len());
        assert_eq!(outer_shape.uvs.len(), inner_shape.uvs.len());

        let outer_vertex_count = outer_shape.positions.len();

        let mut outer_perimeter = self.outer_shape.perimeter();
        let inner_perimeter = self
            .inner_shape
            .perimeter()
            .into_iter()
            .rev()
            .map(|segment| match segment {
                PerimeterSegment::Smooth { mut indices } => PerimeterSegment::Smooth {
                    indices: {
                        let outer_perimeter_vertex_count = outer_vertex_count as u32;
                        indices.reverse();
                        for i in &mut indices {
                            *i += outer_perimeter_vertex_count;
                        }
                        indices
                    },
                },
                PerimeterSegment::Flat { mut indices } => PerimeterSegment::Flat {
                    indices: {
                        let outer_perimeter_vertex_count = outer_vertex_count as u32;
                        indices.reverse();
                        for i in &mut indices {
                            *i += outer_perimeter_vertex_count;
                        }
                        indices
                    },
                },
            });

        outer_perimeter.extend(inner_perimeter);
        outer_perimeter
    }
}
