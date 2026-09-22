#[cfg(feature = "manager")]
use abi_stable::std_types::Tuple2;
use abi_stable::std_types::{
    RCowStr,
    ROption::{self, RNone, RSome},
};
use egui::Id;
use emath::Rect;
use mint::{Point2, Vector2};

use crate::{
    Align2, Order, Resize, UiKind, Vec2b,
    containers::{Area, Frame, ScrollArea, ScrollBarVisibility, ScrollSource},
    response::BunnyInnerResponse,
    ui::BunnyUi,
};
#[cfg(feature = "manager")]
use crate::{closure::PluginClosure, response::BunnyResponse};

#[repr(C)]
pub struct TitleBar<'a> {
    title: RCowStr<'a>,
    open: ROption<&'a mut bool>,
}

impl<'a> TitleBar<'a> {
    #[inline]
    pub fn new(title: impl Into<RCowStr<'a>>) -> Self {
        Self {
            title: title.into(),
            open: RNone,
        }
    }

    /// Add a close button. The window is invisible when open is false and visible when open is true.
    #[inline]
    pub fn open(mut self, open: &'a mut bool) -> Self {
        self.open = RSome(open);
        self
    }
}

#[repr(C)]
pub struct Window<'a> {
    area: Area,
    scroll: ScrollArea,
    title_bar: ROption<TitleBar<'a>>,
    frame: ROption<Frame>,
    resize: Resize,
    default_open: bool,
}

impl<'a> Window<'a> {
    pub fn new(id: impl Into<Id>) -> Self {
        let id = id.into();
        let area = Area::new(id).kind(UiKind::Window);
        Self {
            title_bar: RNone,
            area,
            frame: RNone,
            resize: Resize::default()
                .min_size([96.0, 32.0])
                .default_size([340.0, 420.0]),
            scroll: ScrollArea::neither().auto_shrink(false),
            default_open: true,
        }
    }

    #[inline]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.area = self.area.id(id);
        self
    }

    #[inline]
    pub fn title_bar(mut self, title_bar: TitleBar<'a>) -> Self {
        self.title_bar = RSome(title_bar);
        self
    }

    #[inline]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.area = self.area.enabled(enabled);
        self
    }

    #[inline]
    pub fn interactable(mut self, interactable: bool) -> Self {
        self.area = self.area.interactable(interactable);
        self
    }

    #[inline]
    pub fn movable(mut self, movable: bool) -> Self {
        self.area = self.area.movable(movable);
        self
    }

    #[inline]
    pub fn order(mut self, order: Order) -> Self {
        self.area = self.area.order(order);
        self
    }

    #[inline]
    pub fn fade_in(mut self, fade_in: bool) -> Self {
        self.area = self.area.fade_in(fade_in);
        self
    }

    #[inline]
    pub fn resize(mut self, mutate: impl Fn(Resize) -> Resize) -> Self {
        self.resize = mutate(self.resize);
        self
    }

    #[inline]
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = RSome(frame);
        self
    }

    #[inline]
    pub fn min_width(mut self, min_width: f32) -> Self {
        self.resize = self.resize.min_width(min_width);
        self
    }

    #[inline]
    pub fn min_height(mut self, min_height: f32) -> Self {
        self.resize = self.resize.min_height(min_height);
        self
    }

    #[inline]
    pub fn min_size(mut self, min_size: impl Into<Vector2<f32>>) -> Self {
        self.resize = self.resize.min_size(min_size);
        self
    }

    #[inline]
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.resize = self.resize.max_width(max_width);
        self
    }

    #[inline]
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.resize = self.resize.max_height(max_height);
        self
    }

    #[inline]
    pub fn max_size(mut self, max_size: impl Into<Vector2<f32>>) -> Self {
        self.resize = self.resize.max_size(max_size);
        self
    }

    #[inline]
    pub fn current_pos(mut self, current_pos: impl Into<Point2<f32>>) -> Self {
        self.area = self.area.current_pos(current_pos);
        self
    }

    #[inline]
    pub fn default_pos(mut self, default_pos: impl Into<Point2<f32>>) -> Self {
        self.area = self.area.default_pos(default_pos);
        self
    }

    #[inline]
    pub fn fixed_pos(mut self, pos: impl Into<Point2<f32>>) -> Self {
        self.area = self.area.fixed_pos(pos);
        self
    }

    #[inline]
    pub fn constrain(mut self, constrain: bool) -> Self {
        self.area = self.area.constrain(constrain);
        self
    }

    #[inline]
    pub fn constrain_to(mut self, constrain_rect: Rect) -> Self {
        self.area = self.area.constrain_to(constrain_rect);
        self
    }

    #[inline]
    pub fn pivot(mut self, pivot: Align2) -> Self {
        self.area = self.area.pivot(pivot);
        self
    }

    #[inline]
    pub fn anchor(mut self, align: Align2, offset: impl Into<Vector2<f32>>) -> Self {
        self.area = self.area.anchor(align, offset);
        self
    }

    #[inline]
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    #[inline]
    pub fn default_size(mut self, default_size: impl Into<Vector2<f32>> + Copy) -> Self {
        self.resize = self.resize.default_size(default_size);
        self.area = self.area.default_size(default_size);
        self
    }

    #[inline]
    pub fn default_width(mut self, default_width: f32) -> Self {
        self.resize = self.resize.default_width(default_width);
        self.area = self.area.default_width(default_width);
        self
    }

    #[inline]
    pub fn default_height(mut self, default_height: f32) -> Self {
        self.resize = self.resize.default_height(default_height);
        self.area = self.area.default_height(default_height);
        self
    }

    #[inline]
    pub fn fixed_size(mut self, size: impl Into<Vector2<f32>>) -> Self {
        self.resize = self.resize.fixed_size(size);
        self
    }

    #[inline]
    pub fn default_rect(self, rect: Rect) -> Self {
        self.default_pos(rect.min).default_size(rect.size())
    }

    #[inline]
    pub fn fixed_rect(self, rect: Rect) -> Self {
        self.fixed_pos(rect.min).fixed_size(rect.size())
    }

    #[inline]
    pub fn resizable(mut self, resizable: impl Into<Vec2b>) -> Self {
        self.resize = self.resize.resizable(resizable.into());
        self
    }

    #[inline]
    pub fn auto_sized(mut self) -> Self {
        self.resize = self.resize.auto_sized();
        self.scroll = ScrollArea::neither();
        self
    }

    #[inline]
    pub fn scroll(mut self, scroll: impl Into<Vec2b>) -> Self {
        self.scroll = self.scroll.scroll(scroll);
        self
    }

    #[inline]
    pub fn hscroll(mut self, hscroll: bool) -> Self {
        self.scroll = self.scroll.hscroll(hscroll);
        self
    }

    #[inline]
    pub fn vscroll(mut self, vscroll: bool) -> Self {
        self.scroll = self.scroll.vscroll(vscroll);
        self
    }

    #[inline]
    pub fn drag_to_scroll(mut self, drag_to_scroll: bool) -> Self {
        self.scroll = self.scroll.scroll_source(ScrollSource {
            drag: drag_to_scroll,
            ..Default::default()
        });
        self
    }

    #[inline]
    pub fn scroll_bar_visibility(mut self, visibility: ScrollBarVisibility) -> Self {
        self.scroll = self.scroll.scroll_bar_visibility(visibility);
        self
    }

    #[inline]
    pub fn show<R>(
        self,
        ui: &mut BunnyUi,
        add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> Option<BunnyInnerResponse<Option<R>>> {
        ui.window_show(self, add_contents)
    }
}

#[cfg(feature = "manager")]
impl TitleBar<'_> {
    fn show_impl(
        self,
        ui: &mut egui::Ui,
        window_id: Id,
        window_stroke: egui::Stroke,
        window_margin: egui::Margin,
    ) {
        // egui windows normally use global style, but since plugins can't
        // manipulate global style we should leave it to the user to set their
        // desired style before a window show call
        let visuals = ui.visuals();
        let window_rect = ui.max_rect();
        let title_bar_height = 24.0;
        let title_rect = {
            let mut rect = window_rect + window_margin;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        let mut painter = ui.painter().clone();
        painter.set_clip_rect(title_rect.expand(window_stroke.width));
        if let RSome(open) = self.open {
            let close_button_id = window_id.with("close button");
            let widget_state = ui
                .read_response(close_button_id)
                .map(|r| r.widget_state())
                .unwrap_or_default();
            let close_color = visuals.widgets.state(widget_state).fg_stroke.color;
            let close_rect = painter.text(
                title_rect.right_center() - egui::vec2(4.0, 0.0),
                egui::Align2::RIGHT_CENTER,
                "❌",
                egui::FontId::proportional(14.0),
                close_color,
            );
            if ui
                .interact(close_rect, close_button_id, egui::Sense::click())
                .clicked()
            {
                *open = false;
            }
        }

        painter.text(
            title_rect.center(),
            egui::Align2::CENTER_CENTER,
            &self.title,
            egui::FontId::proportional(16.0),
            visuals.text_color(),
        );
        painter.line_segment(
            [title_rect.left_bottom(), title_rect.right_bottom()],
            window_stroke,
        );

        ui.add_space(title_bar_height);
    }
}

#[cfg(feature = "manager")]
impl Window<'_> {
    pub(crate) fn show_impl(
        self,
        ui: &mut egui::Ui,
        contents: PluginClosure,
    ) -> ROption<Tuple2<BunnyResponse, bool>> {
        let Window {
            area,
            title_bar,
            frame,
            scroll,
            resize,
            default_open,
        } = self;
        if title_bar
            .as_ref()
            .and_then(|t| t.open.as_deref())
            .into_option()
            .is_some_and(|o| !*o)
        {
            return RNone;
        }
        let mut window = egui::Window::new("")
            .id(area.id)
            .title_bar(false)
            .enabled(area.enabled)
            .interactable(area.interactable)
            .movable(area.movable)
            .constrain(area.constrain)
            .pivot(area.pivot.into())
            .default_size(area.default_size)
            .default_open(default_open)
            .scroll(scroll.direction_enabled)
            .scroll_bar_visibility(scroll.scroll_bar_visibility.into())
            .drag_to_scroll(scroll.scroll_source.drag);
        if let RSome(frame) = frame {
            window = window.frame(frame.into());
        }
        if let RSome(constrain_rect) = area.constrain_rect {
            window = window.constrain_to(constrain_rect);
        }
        if let RSome(default_pos) = area.default_pos {
            window = window.default_pos(default_pos);
        }
        if let RSome(Tuple2(align, offset)) = area.anchor {
            window = window.anchor(align.into(), offset);
        }
        if let RSome(new_pos) = area.new_pos {
            window = window.current_pos(new_pos);
        }
        window = window.resize(|r| {
            r.min_size(resize.min_size)
                .max_size(resize.max_size)
                .resizable(resize.resizable)
        });
        let inner = window.show(ui, |ui| {
            if let RSome(title_bar) = title_bar {
                let (stroke, margin) = frame
                    .map(|f| (f.stroke.into(), f.inner_margin.into()))
                    .unwrap_or_else(|| (ui.visuals().window_stroke, ui.spacing().window_margin));
                title_bar.show_impl(ui, area.id, stroke, margin);
                ui.scope(|ui| {
                    let mut b = BunnyUi::new(ui);
                    contents.call(&mut b);
                });
            } else {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            }
        });
        inner
            .map(|inner| Tuple2(BunnyResponse::new(inner.response), inner.inner.is_some()))
            .into()
    }
}
