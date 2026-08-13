use core::f32::consts::PI;

use crate::{
    draw_list::PrimitiveTopology,
    mesh::{Mesh, MeshBuilder},
};

#[derive(Clone, Copy, Debug)]
pub struct SphereMesh {
    pub radius: f32,
    pub sectors: u32,
    pub stacks: u32,
}

impl Default for SphereMesh {
    fn default() -> Self {
        Self {
            radius: 1.0,
            sectors: 20,
            stacks: 12,
        }
    }
}

impl SphereMesh {
    #[inline]
    pub const fn new(radius: f32) -> Self {
        Self {
            radius,
            sectors: 20,
            stacks: 12,
        }
    }

    #[inline]
    pub const fn sectors(mut self, sectors: u32) -> Self {
        self.sectors = sectors;
        self
    }

    #[inline]
    pub const fn stacks(mut self, stacks: u32) -> Self {
        self.stacks = stacks;
        self
    }
}

impl MeshBuilder for SphereMesh {
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.0/src/bevy_mesh/primitives/dim3/sphere.rs.html#172
        let SphereMesh {
            radius,
            sectors,
            stacks,
        } = *self;

        let sectors_f32 = sectors as f32;
        let stacks_f32 = stacks as f32;
        let sector_step = 2. * PI / sectors_f32;
        let stack_step = PI / stacks_f32;

        let n_vertices = (stacks * sectors) as usize;
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(n_vertices);
        let mut indices: Vec<u32> = Vec::with_capacity(n_vertices * 2 * 3);

        for i in 0..stacks + 1 {
            let stack_angle = PI / 2. - (i as f32) * stack_step;
            let xy = radius * f32::cos(stack_angle);
            let z = radius * f32::sin(stack_angle);

            for j in 0..sectors + 1 {
                let sector_angle = (j as f32) * sector_step;
                let x = xy * f32::cos(sector_angle);
                let y = xy * f32::sin(sector_angle);

                vertices.push([x, y, z]);
                uvs.push([(j as f32) / sectors_f32, (i as f32) / stacks_f32]);
            }
        }

        // indices
        //  k1--k1+1
        //  |  / |
        //  | /  |
        //  k2--k2+1
        for i in 0..stacks {
            let mut k1 = i * (sectors + 1);
            let mut k2 = k1 + sectors + 1;
            #[allow(clippy::explicit_counter_loop)]
            for _j in 0..sectors {
                if i != 0 {
                    indices.push(k1);
                    indices.push(k2);
                    indices.push(k1 + 1);
                }
                if i != stacks - 1 {
                    indices.push(k1 + 1);
                    indices.push(k2);
                    indices.push(k2 + 1);
                }
                k1 += 1;
                k2 += 1;
            }
        }

        Mesh::new(vertices, uvs, indices, PrimitiveTopology::TriangleList)
    }
}
