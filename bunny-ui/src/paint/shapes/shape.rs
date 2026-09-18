use abi_stable::{
    rvec,
    std_types::{RBox, RString, RVec},
};
use ecolor::Color32;
use emath::{Pos2, Rangef, Rect, TSTransform, pos2};
use mint::{Point2, Vector2};
use tracing::error;

use crate::{
    Align2, Direction,
    galley::BunnyGalley,
    paint::{
        TextureId,
        corner_radius::CornerRadius,
        mesh::{Mesh, Vertex},
        shapes::{
            bezier_shape::{CubicBezierShape, QuadraticBezierShape},
            circle_shape::CircleShape,
            ellipse_shape::EllipseShape,
            path_shape::PathShape,
            rect_shape::RectShape,
            text_shape::TextShape,
        },
        stroke::{PathStroke, Stroke, StrokeKind},
        text::fonts::FontId,
    },
    ui::BunnyUi,
};

#[repr(C)]
pub enum Shape {
    Noop,
    Vec(RVec<Shape>),
    Circle(CircleShape),
    Ellipse(EllipseShape),
    LineSegment { points: [Pos2; 2], stroke: Stroke },
    Path(PathShape),
    Rect(RectShape),
    Text(TextShape),
    Mesh(RBox<Mesh>),
    QuadraticBezier(QuadraticBezierShape),
    CubicBezier(CubicBezierShape),
}

impl From<Vec<Self>> for Shape {
    #[inline(always)]
    fn from(value: Vec<Self>) -> Self {
        Self::Vec(value.into())
    }
}

impl From<RVec<Self>> for Shape {
    #[inline(always)]
    fn from(value: RVec<Self>) -> Self {
        Self::Vec(value)
    }
}

impl From<Mesh> for Shape {
    #[inline(always)]
    fn from(value: Mesh) -> Self {
        Self::Mesh(RBox::new(value))
    }
}

impl From<RBox<Mesh>> for Shape {
    #[inline(always)]
    fn from(value: RBox<Mesh>) -> Self {
        Self::Mesh(value)
    }
}

impl Shape {
    #[inline]
    pub fn line_segment(points: [Pos2; 2], stroke: impl Into<Stroke>) -> Self {
        Self::LineSegment {
            points,
            stroke: stroke.into(),
        }
    }

    pub fn hline(x: impl Into<Rangef>, y: f32, stroke: impl Into<Stroke>) -> Self {
        let x = x.into();
        Self::LineSegment {
            points: [pos2(x.min, y), pos2(x.max, y)],
            stroke: stroke.into(),
        }
    }

    pub fn vline(x: f32, y: impl Into<Rangef>, stroke: impl Into<Stroke>) -> Self {
        let y = y.into();
        Self::LineSegment {
            points: [pos2(x, y.min), pos2(x, y.max)],
            stroke: stroke.into(),
        }
    }

    #[inline]
    pub fn line<P: Into<Point2<f32>>>(
        points: impl IntoIterator<Item = P>,
        stroke: impl Into<PathStroke>,
    ) -> Self {
        Self::Path(PathShape::line(points, stroke))
    }

    #[inline]
    pub fn closed_line<P: Into<Point2<f32>>>(
        points: impl IntoIterator<Item = P>,
        stroke: impl Into<PathStroke>,
    ) -> Self {
        Self::Path(PathShape::closed_line(points, stroke))
    }

    pub fn dotted_line(
        path: &[Pos2],
        color: impl Into<Color32>,
        spacing: f32,
        radius: f32,
    ) -> Vec<Self> {
        let mut shapes = Vec::new();
        points_from_line(path, spacing, radius, color.into(), &mut shapes);
        shapes
    }

    pub fn dashed_line(
        path: &[Pos2],
        stroke: impl Into<Stroke>,
        dash_length: f32,
        gap_length: f32,
    ) -> Vec<Self> {
        let mut shapes = Vec::new();
        dashes_from_line(
            path,
            stroke.into(),
            &[dash_length],
            &[gap_length],
            &mut shapes,
            0.0,
        );
        shapes
    }

    pub fn dashed_line_with_offset(
        path: &[Pos2],
        stroke: impl Into<Stroke>,
        dash_lengths: &[f32],
        gap_lengths: &[f32],
        dash_offset: f32,
    ) -> Vec<Self> {
        let mut shapes = Vec::new();
        dashes_from_line(
            path,
            stroke.into(),
            dash_lengths,
            gap_lengths,
            &mut shapes,
            dash_offset,
        );
        shapes
    }

    pub fn dashed_line_many(
        points: &[Pos2],
        stroke: impl Into<Stroke>,
        dash_length: f32,
        gap_length: f32,
        shapes: &mut Vec<Shape>,
    ) {
        dashes_from_line(
            points,
            stroke.into(),
            &[dash_length],
            &[gap_length],
            shapes,
            0.0,
        );
    }

    pub fn dashed_line_many_with_offset(
        points: &[Pos2],
        stroke: impl Into<Stroke>,
        dash_lengths: &[f32],
        gap_lengths: &[f32],
        dash_offset: f32,
        shapes: &mut Vec<Shape>,
    ) {
        dashes_from_line(
            points,
            stroke.into(),
            dash_lengths,
            gap_lengths,
            shapes,
            dash_offset,
        );
    }

    #[inline]
    pub fn convex_polygon<P: Into<Point2<f32>>>(
        points: impl IntoIterator<Item = P>,
        fill: impl Into<Color32>,
        stroke: impl Into<PathStroke>,
    ) -> Self {
        Self::Path(PathShape::convex_polygon(points, fill, stroke))
    }

    #[inline]
    pub fn circle_filled(
        center: impl Into<Point2<f32>>,
        radius: f32,
        fill_color: impl Into<Color32>,
    ) -> Self {
        Self::Circle(CircleShape::filled(center, radius, fill_color))
    }

    #[inline]
    pub fn circle_stroke(
        center: impl Into<Point2<f32>>,
        radius: f32,
        stroke: impl Into<Stroke>,
    ) -> Self {
        Self::Circle(CircleShape::stroke(center, radius, stroke))
    }

    #[inline]
    pub fn ellipse_filled(
        center: impl Into<Point2<f32>>,
        radius: impl Into<Vector2<f32>>,
        fill_color: impl Into<Color32>,
    ) -> Self {
        Self::Ellipse(EllipseShape::filled(center, radius, fill_color))
    }

    #[inline]
    pub fn ellipse_stroke(
        center: impl Into<Point2<f32>>,
        radius: impl Into<Vector2<f32>>,
        stroke: impl Into<Stroke>,
    ) -> Self {
        Self::Ellipse(EllipseShape::stroke(center, radius, stroke))
    }

    #[inline]
    pub fn rect_filled(
        rect: Rect,
        corner_radius: impl Into<CornerRadius>,
        fill_color: impl Into<Color32>,
    ) -> Self {
        Self::Rect(RectShape::filled(rect, corner_radius, fill_color))
    }

    #[inline]
    pub fn rect_stroke(
        rect: Rect,
        corner_radius: impl Into<CornerRadius>,
        stroke: impl Into<Stroke>,
        stroke_kind: StrokeKind,
    ) -> Self {
        Self::Rect(RectShape::stroke(rect, corner_radius, stroke, stroke_kind))
    }

    #[inline]
    pub fn gradient_rect(rect: Rect, direction: Direction, [from, to]: [Color32; 2]) -> Self {
        let (left_top, right_top, left_bottom, right_bottom) = match direction {
            Direction::LeftToRight => (from, from, to, to),
            Direction::RightToLeft => (to, to, from, from),
            Direction::TopDown => (from, to, from, to),
            Direction::BottomUp => (to, from, to, from),
        };
        Self::from(Mesh {
            indices: rvec![0, 1, 2, 2, 1, 3],
            vertices: rvec![
                Vertex::untextured(rect.left_top(), left_top),
                Vertex::untextured(rect.right_top(), right_top),
                Vertex::untextured(rect.left_bottom(), left_bottom),
                Vertex::untextured(rect.right_bottom(), right_bottom),
            ],
            texture_id: Default::default(),
        })
    }

    #[inline]
    pub fn text(
        ui: &mut BunnyUi,
        pos: impl Into<Point2<f32>>,
        anchor: Align2,
        text: impl Into<RString>,
        font_id: FontId,
        color: Color32,
    ) -> Self {
        let galley = ui.fonts_layout_no_wrap(text, font_id, color);
        let rect = anchor.anchor_size(pos, galley.size());
        Self::galley(rect.min, galley, color)
    }

    #[inline]
    pub fn galley(
        pos: impl Into<Point2<f32>>,
        galley: BunnyGalley,
        fallback_color: Color32,
    ) -> Self {
        TextShape::new(pos, galley, fallback_color).into()
    }

    #[inline]
    pub fn galley_with_override_text_color(
        pos: impl Into<Point2<f32>>,
        galley: BunnyGalley,
        text_color: Color32,
    ) -> Self {
        TextShape::new(pos, galley, text_color)
            .with_override_text_color(text_color)
            .into()
    }

    #[inline]
    pub fn mesh(mesh: impl Into<RBox<Mesh>>) -> Self {
        let mesh = mesh.into();
        debug_assert!(mesh.is_valid(), "Invalid mesh: {mesh:#?}");
        Self::Mesh(mesh)
    }

    #[inline]
    pub fn image(texture_id: TextureId, rect: Rect, uv: Rect, tint: Color32) -> Self {
        let mut mesh = Mesh::with_texture(texture_id);
        mesh.add_rect_with_uv(rect, uv, tint);
        Self::mesh(RBox::new(mesh))
    }
}

impl Shape {
    #[inline(always)]
    pub fn scale(&mut self, factor: f32) {
        self.transform(TSTransform::from_scaling(factor));
    }

    #[inline(always)]
    pub fn translate(&mut self, delta: impl Into<Vector2<f32>>) {
        self.transform(TSTransform::from_translation(delta.into().into()));
    }

    pub fn transform(&mut self, transform: TSTransform) {
        match self {
            Shape::Noop => {}
            Shape::Vec(shapes) => {
                for shape in shapes {
                    shape.transform(transform);
                }
            }
            Shape::Circle(circle_shape) => {
                circle_shape.center = transform * circle_shape.center;
                circle_shape.radius *= transform.scaling;
                circle_shape.stroke.width *= transform.scaling;
            }
            Shape::Ellipse(ellipse_shape) => {
                ellipse_shape.center = transform * ellipse_shape.center;
                ellipse_shape.radius *= transform.scaling;
                ellipse_shape.stroke.width *= transform.scaling;
            }
            Shape::LineSegment { points, stroke } => {
                for p in points {
                    *p = transform * *p;
                }
                stroke.width *= transform.scaling;
            }
            Shape::Path(path_shape) => {
                for p in &mut path_shape.points {
                    *p = transform * *p;
                }
                path_shape.stroke.width *= transform.scaling;
            }
            Shape::Rect(rect_shape) => {
                rect_shape.rect = transform * rect_shape.rect;
                rect_shape.corner_radius *= transform.scaling;
                rect_shape.stroke.width *= transform.scaling;
                rect_shape.blur_width *= transform.scaling;
            }
            Shape::Text(_) => {
                error!("Transforming text is unsupported.");
            }
            Shape::Mesh(mesh) => mesh.transform(transform),
            Shape::QuadraticBezier(bezier) => {
                for p in &mut bezier.points {
                    *p = transform * *p;
                }
                bezier.stroke.width *= transform.scaling;
            }
            Shape::CubicBezier(bezier) => {
                for p in &mut bezier.points {
                    *p = transform * *p;
                }
                bezier.stroke.width *= transform.scaling;
            }
        }
    }
}

fn points_from_line(
    path: &[Pos2],
    spacing: f32,
    radius: f32,
    color: Color32,
    shapes: &mut Vec<Shape>,
) {
    let mut position_on_segment = 0.0;
    for window in path.windows(2) {
        let (start, end) = (window[0], window[1]);
        let vector = end - start;
        let segment_length = vector.length();
        while position_on_segment < segment_length {
            let new_point = start + vector * (position_on_segment / segment_length);
            shapes.push(Shape::circle_filled(new_point, radius, color));
            position_on_segment += spacing;
        }
        position_on_segment -= segment_length
    }
}

fn dashes_from_line(
    path: &[Pos2],
    stroke: Stroke,
    dash_lengths: &[f32],
    gap_lengths: &[f32],
    shapes: &mut Vec<Shape>,
    dash_offset: f32,
) {
    assert_eq!(
        dash_lengths.len(),
        gap_lengths.len(),
        "Mismatched dash and gap lengths, got dash_lengths: {}, gap_lengths: {}",
        dash_lengths.len(),
        gap_lengths.len()
    );
    let mut position_on_segment = dash_offset;
    let mut drawing_dash = false;
    let mut step = 0;
    let steps = dash_lengths.len();
    for window in path.windows(2) {
        let (start, end) = (window[0], window[1]);
        let vector = end - start;
        let segment_length = vector.length();

        let mut start_point = start;
        while position_on_segment < segment_length {
            let new_point = start + vector * (position_on_segment / segment_length);
            if drawing_dash {
                shapes.push(Shape::line_segment([start_point, new_point], stroke));
                position_on_segment += gap_lengths[step];
                step += 1;
                if step >= steps {
                    step = 0;
                }
            } else {
                start_point = new_point;
                position_on_segment += dash_lengths[step];
            }
            drawing_dash = !drawing_dash;
        }

        if drawing_dash {
            shapes.push(Shape::line_segment([start_point, end], stroke));
        }

        position_on_segment -= segment_length;
    }
}

#[cfg(feature = "manager")]
impl From<Shape> for egui::Shape {
    fn from(value: Shape) -> Self {
        match value {
            Shape::Noop => egui::Shape::Noop,
            Shape::Vec(shapes) => {
                egui::Shape::Vec(shapes.into_iter().map(egui::Shape::from).collect())
            }
            Shape::Circle(circle_shape) => egui::Shape::Circle(circle_shape.into()),
            Shape::Ellipse(ellipse_shape) => egui::Shape::Ellipse(ellipse_shape.into()),
            Shape::LineSegment { points, stroke } => egui::Shape::LineSegment {
                points,
                stroke: stroke.into(),
            },
            Shape::Path(path_shape) => egui::Shape::Path(path_shape.into()),
            Shape::Rect(rect_shape) => egui::Shape::Rect(rect_shape.into()),
            Shape::Text(text_shape) => egui::Shape::Text(text_shape.into()),
            Shape::Mesh(mesh) => {
                egui::Shape::Mesh(std::sync::Arc::new(RBox::into_inner(mesh).into()))
            }
            Shape::QuadraticBezier(quadratic_bezier_shape) => {
                egui::Shape::QuadraticBezier(quadratic_bezier_shape.into())
            }
            Shape::CubicBezier(cubic_bezier_shape) => {
                egui::Shape::CubicBezier(cubic_bezier_shape.into())
            }
        }
    }
}
