use abi_stable::std_types::ROption::RSome;
use ecolor::Color32;

use crate::paint::{
    CircleShape, ColorMode, CubicBezierShape, EllipseShape, Mesh, PathShape, QuadraticBezierShape,
    RectShape, Shape, TextShape,
};

pub fn adjust_colors(
    shape: &mut Shape,
    adjust_color: impl Fn(&mut Color32) + Send + Sync + Copy + 'static,
) {
    match shape {
        Shape::Noop => {}
        Shape::Vec(shapes) => {
            for shape in shapes {
                adjust_colors(shape, adjust_color);
            }
        }

        Shape::LineSegment { points: _, stroke } => adjust_color(&mut stroke.color),

        Shape::Path(PathShape {
            points: _,
            closed: _,
            fill,
            stroke,
        })
        | Shape::QuadraticBezier(QuadraticBezierShape {
            points: _,
            closed: _,
            fill,
            stroke,
        })
        | Shape::CubicBezier(CubicBezierShape {
            points: _,
            closed: _,
            fill,
            stroke,
        }) => {
            adjust_color(fill);
            adjust_color_mode(&mut stroke.color, adjust_color);
        }

        Shape::Circle(CircleShape {
            center: _,
            radius: _,
            fill,
            stroke,
        })
        | Shape::Ellipse(EllipseShape {
            center: _,
            radius: _,
            fill,
            stroke,
            angle: _,
        })
        | Shape::Rect(RectShape {
            rect: _,
            corner_radius: _,
            fill,
            stroke,
            stroke_kind: _,
            round_to_pixels: _,
            blur_width: _,
            brush: _,
            angle: _,
        }) => {
            adjust_color(fill);
            adjust_color(&mut stroke.color);
        }

        Shape::Text(TextShape {
            pos: _,
            galley: _,
            underline,
            fallback_color,
            override_text_color,
            opacity_factor: _,
            angle: _,
        }) => {
            adjust_color(&mut underline.color);
            adjust_color(fallback_color);
            if let RSome(override_text_color) = override_text_color {
                adjust_color(override_text_color);
            }
        }
        Shape::Mesh(mesh) => {
            let Mesh {
                indices: _,
                vertices,
                texture_id: _,
            } = mesh.as_mut();

            for v in vertices {
                adjust_color(&mut v.color)
            }
        }
    }
}

fn adjust_color_mode(
    color_mode: &mut ColorMode,
    adjust_color: impl Fn(&mut Color32) + Send + Sync + Copy + 'static,
) {
    match color_mode {
        ColorMode::Solid(color) => adjust_color(color),
    }
}
