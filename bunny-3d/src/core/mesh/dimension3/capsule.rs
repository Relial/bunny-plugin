use glam::{Vec2, Vec3};

use crate::{
    draw_list::PrimitiveTopology,
    mesh::{Mesh, MeshBuilder},
};

pub const CAPSULE_LONGITUDES: u32 = 24;
pub const CAPSULE_LATITUDES: u32 = 12;

#[derive(Clone, Copy, Debug)]
pub struct CapsuleMesh {
    pub radius: f32,
    pub half_length: f32,
    pub rings: u32,
    pub longitudes: u32,
    pub latitudes: u32,
}

impl Default for CapsuleMesh {
    fn default() -> Self {
        Self {
            radius: 0.5,
            half_length: 0.5,
            rings: 0,
            longitudes: 24,
            latitudes: 12,
        }
    }
}

impl CapsuleMesh {
    #[inline]
    pub fn new(radius: f32, length: f32) -> Self {
        Self {
            radius,
            half_length: length / 2.0,
            ..Default::default()
        }
    }

    #[inline]
    pub const fn rings(mut self, rings: u32) -> Self {
        self.rings = rings;
        self
    }

    #[inline]
    pub const fn longitudes(mut self, longitudes: u32) -> Self {
        self.longitudes = longitudes;
        self
    }

    #[inline]
    pub const fn latitudes(mut self, latitudes: u32) -> Self {
        self.latitudes = latitudes;
        self
    }
}

impl MeshBuilder for CapsuleMesh {
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.0/src/bevy_mesh/primitives/dim3/capsule.rs.html#96
        let CapsuleMesh {
            radius,
            half_length,
            rings,
            longitudes,
            latitudes,
        } = *self;

        let calc_middle = rings > 0;
        let half_lats = latitudes / 2;
        let half_latsn1 = half_lats - 1;
        let half_latsn2 = half_lats - 2;
        let ringsp1 = rings + 1;
        let lonsp1 = longitudes + 1;
        let summit = half_length + radius;

        // Vertex index offsets.
        let vert_offset_north_hemi = longitudes;
        let vert_offset_north_equator = vert_offset_north_hemi + lonsp1 * half_latsn1;
        let vert_offset_cylinder = vert_offset_north_equator + lonsp1;
        let vert_offset_south_equator = if calc_middle {
            vert_offset_cylinder + lonsp1 * rings
        } else {
            vert_offset_cylinder
        };
        let vert_offset_south_hemi = vert_offset_south_equator + lonsp1;
        let vert_offset_south_polar = vert_offset_south_hemi + lonsp1 * half_latsn2;
        let vert_offset_south_cap = vert_offset_south_polar + lonsp1;

        // Initialize arrays.
        let vert_len = (vert_offset_south_cap + longitudes) as usize;

        let mut vs: Vec<Vec3> = vec![Vec3::ZERO; vert_len];
        let mut vts: Vec<Vec2> = vec![Vec2::ZERO; vert_len];

        let to_theta = 2.0 * core::f32::consts::PI / longitudes as f32;
        let to_phi = core::f32::consts::PI / latitudes as f32;
        let to_tex_horizontal = 1.0 / longitudes as f32;
        let to_tex_vertical = 1.0 / half_lats as f32;

        let vt_aspect_ratio = radius / (2.0 * half_length + radius + radius);
        let vt_aspect_north = 1.0 - vt_aspect_ratio;
        let vt_aspect_south = vt_aspect_ratio;

        let mut theta_cartesian: Vec<Vec2> = vec![Vec2::ZERO; longitudes as usize];
        let mut rho_theta_cartesian: Vec<Vec2> = vec![Vec2::ZERO; longitudes as usize];
        let mut s_texture_cache: Vec<f32> = vec![0.0; lonsp1 as usize];

        for j in 0..longitudes as usize {
            let jf = j as f32;
            let s_texture_polar = 1.0 - ((jf + 0.5) * to_tex_horizontal);
            let theta = jf * to_theta;

            theta_cartesian[j] = Vec2::from_angle(theta);
            rho_theta_cartesian[j] = radius * theta_cartesian[j];

            // North.
            vs[j] = Vec3::new(0.0, summit, 0.0);
            vts[j] = Vec2::new(s_texture_polar, 1.0);

            // South.
            let idx = vert_offset_south_cap as usize + j;
            vs[idx] = Vec3::new(0.0, -summit, 0.0);
            vts[idx] = Vec2::new(s_texture_polar, 0.0);
        }

        // Equatorial vertices.
        for (j, s_texture_cache_j) in s_texture_cache.iter_mut().enumerate().take(lonsp1 as usize) {
            let s_texture = 1.0 - j as f32 * to_tex_horizontal;
            *s_texture_cache_j = s_texture;

            // Wrap to first element upon reaching last.
            let j_mod = j % longitudes as usize;
            let rtc = rho_theta_cartesian[j_mod];

            // North equator.
            let idxn = vert_offset_north_equator as usize + j;
            vs[idxn] = Vec3::new(rtc.x, half_length, -rtc.y);
            vts[idxn] = Vec2::new(s_texture, vt_aspect_north);

            // South equator.
            let idxs = vert_offset_south_equator as usize + j;
            vs[idxs] = Vec3::new(rtc.x, -half_length, -rtc.y);
            vts[idxs] = Vec2::new(s_texture, vt_aspect_south);
        }

        // Hemisphere vertices.
        for i in 0..half_latsn1 {
            let ip1f = i as f32 + 1.0;
            let phi = ip1f * to_phi;

            // For coordinates.
            let (sin_phi_south, cos_phi_south) = f32::sin_cos(phi);

            // Symmetrical hemispheres mean cosine and sine only needs
            // to be calculated once.
            let cos_phi_north = sin_phi_south;
            let sin_phi_north = -cos_phi_south;

            let rho_cos_phi_north = radius * cos_phi_north;
            let rho_sin_phi_north = radius * sin_phi_north;
            let z_offset_north = half_length - rho_sin_phi_north;

            let rho_cos_phi_south = radius * cos_phi_south;
            let rho_sin_phi_south = radius * sin_phi_south;
            let z_offset_sout = -half_length - rho_sin_phi_south;

            // For texture coordinates.
            let t_tex_fac = ip1f * to_tex_vertical;
            let cmpl_tex_fac = 1.0 - t_tex_fac;
            let t_tex_north = cmpl_tex_fac + vt_aspect_north * t_tex_fac;
            let t_tex_south = cmpl_tex_fac * vt_aspect_south;

            let i_lonsp1 = i * lonsp1;
            let vert_curr_lat_north = vert_offset_north_hemi + i_lonsp1;
            let vert_curr_lat_south = vert_offset_south_hemi + i_lonsp1;

            for (j, s_texture) in s_texture_cache.iter().enumerate().take(lonsp1 as usize) {
                let j_mod = j % longitudes as usize;

                let tc = theta_cartesian[j_mod];

                // North hemisphere.
                let idxn = vert_curr_lat_north as usize + j;
                vs[idxn] = Vec3::new(
                    rho_cos_phi_north * tc.x,
                    z_offset_north,
                    -rho_cos_phi_north * tc.y,
                );
                vts[idxn] = Vec2::new(*s_texture, t_tex_north);

                // South hemisphere.
                let idxs = vert_curr_lat_south as usize + j;
                vs[idxs] = Vec3::new(
                    rho_cos_phi_south * tc.x,
                    z_offset_sout,
                    -rho_cos_phi_south * tc.y,
                );
                vts[idxs] = Vec2::new(*s_texture, t_tex_south);
            }
        }

        // Cylinder vertices.
        if calc_middle {
            // Exclude both origin and destination edges
            // (North and South equators) from the interpolation.
            let to_fac = 1.0 / ringsp1 as f32;
            let mut idx_cyl_lat = vert_offset_cylinder as usize;

            for h in 1..ringsp1 {
                let fac = h as f32 * to_fac;
                let cmpl_fac = 1.0 - fac;
                let t_texture = cmpl_fac * vt_aspect_north + fac * vt_aspect_south;
                let z = half_length - 2.0 * half_length * fac;

                for (j, s_texture) in s_texture_cache.iter().enumerate().take(lonsp1 as usize) {
                    let j_mod = j % longitudes as usize;
                    let rtc = rho_theta_cartesian[j_mod];

                    vs[idx_cyl_lat] = Vec3::new(rtc.x, z, -rtc.y);
                    vts[idx_cyl_lat] = Vec2::new(*s_texture, t_texture);

                    idx_cyl_lat += 1;
                }
            }
        }

        // Triangle indices.

        // Stride is 3 for polar triangles;
        // stride is 6 for two triangles forming a quad.
        let lons3 = longitudes * 3;
        let lons6 = longitudes * 6;
        let hemi_lons = half_latsn1 * lons6;

        let tri_offset_north_hemi = lons3;
        let tri_offset_cylinder = tri_offset_north_hemi + hemi_lons;
        let tri_offset_south_hemi = tri_offset_cylinder + ringsp1 * lons6;
        let tri_offset_south_cap = tri_offset_south_hemi + hemi_lons;

        let fs_len = tri_offset_south_cap + lons3;
        let mut tris: Vec<u32> = vec![0; fs_len as usize];

        // Polar caps.
        let mut i = 0;
        let mut k = 0;
        let mut m = tri_offset_south_cap as usize;
        while i < longitudes {
            // North.
            tris[k] = i;
            tris[k + 1] = vert_offset_north_hemi + i;
            tris[k + 2] = vert_offset_north_hemi + i + 1;

            // South.
            tris[m] = vert_offset_south_cap + i;
            tris[m + 1] = vert_offset_south_polar + i + 1;
            tris[m + 2] = vert_offset_south_polar + i;

            i += 1;
            k += 3;
            m += 3;
        }

        // Hemispheres.

        let mut i = 0;
        let mut k = tri_offset_north_hemi as usize;
        let mut m = tri_offset_south_hemi as usize;

        while i < half_latsn1 {
            let i_lonsp1 = i * lonsp1;

            let vert_curr_lat_north = vert_offset_north_hemi + i_lonsp1;
            let vert_next_lat_north = vert_curr_lat_north + lonsp1;

            let vert_curr_lat_south = vert_offset_south_equator + i_lonsp1;
            let vert_next_lat_south = vert_curr_lat_south + lonsp1;

            let mut j = 0;
            while j < longitudes {
                // North.
                let north00 = vert_curr_lat_north + j;
                let north01 = vert_next_lat_north + j;
                let north11 = vert_next_lat_north + j + 1;
                let north10 = vert_curr_lat_north + j + 1;

                tris[k] = north00;
                tris[k + 1] = north11;
                tris[k + 2] = north10;

                tris[k + 3] = north00;
                tris[k + 4] = north01;
                tris[k + 5] = north11;

                // South.
                let south00 = vert_curr_lat_south + j;
                let south01 = vert_next_lat_south + j;
                let south11 = vert_next_lat_south + j + 1;
                let south10 = vert_curr_lat_south + j + 1;

                tris[m] = south00;
                tris[m + 1] = south11;
                tris[m + 2] = south10;

                tris[m + 3] = south00;
                tris[m + 4] = south01;
                tris[m + 5] = south11;

                j += 1;
                k += 6;
                m += 6;
            }

            i += 1;
        }

        // Cylinder.
        let mut i = 0;
        let mut k = tri_offset_cylinder as usize;

        while i < ringsp1 {
            let vert_curr_lat = vert_offset_north_equator + i * lonsp1;
            let vert_next_lat = vert_curr_lat + lonsp1;

            let mut j = 0;
            while j < longitudes {
                let cy00 = vert_curr_lat + j;
                let cy01 = vert_next_lat + j;
                let cy11 = vert_next_lat + j + 1;
                let cy10 = vert_curr_lat + j + 1;

                tris[k] = cy00;
                tris[k + 1] = cy11;
                tris[k + 2] = cy10;

                tris[k + 3] = cy00;
                tris[k + 4] = cy01;
                tris[k + 5] = cy11;

                j += 1;
                k += 6;
            }

            i += 1;
        }

        let vs: Vec<[f32; 3]> = vs.into_iter().map(Into::into).collect();
        let vts: Vec<[f32; 2]> = vts.into_iter().map(Into::into).collect();

        Mesh::new(vs, vts, tris, PrimitiveTopology::TriangleList)
    }
}

impl From<CapsuleMesh> for Mesh {
    fn from(value: CapsuleMesh) -> Self {
        value.build()
    }
}
