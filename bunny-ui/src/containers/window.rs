use abi_stable::std_types::{
    RBox,
    ROption::{self, RNone, RSome},
    RString,
};
use ecolor::Color32;
use emath::{Pos2, Rect, Vec2};

use crate::{
    Id,
    align::Align2,
    area::Area,
    containers::{
        frame::Frame,
        scroll_area::{ScrollArea, ScrollBarVisibility, ScrollSource},
    },
    elements::Container,
    layout::Layout,
    paint::{corner_radius::CornerRadius, stroke::Stroke},
    resize::Resize,
    response::InnerResponse,
    ui_old::BunnyUi,
    vec2b::Vec2b,
};

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct TitleBar {
    title: ROption<RString>,
    close_button: bool,
}

impl TitleBar {
    #[inline]
    pub fn title(mut self, title: impl Into<RString>) -> Self {
        self.title = RSome(title.into());
        self
    }

    #[inline]
    pub fn close_button(mut self, close_button: bool) -> Self {
        self.close_button = close_button;
        self
    }
}

#[repr(C)]
pub struct Window {
    area: Area,
    title: ROption<RString>,
    frame: ROption<Frame>,
    scroll: ScrollArea,
    resize: Resize,
    open: ROption<*mut bool>,
    id: Id,
    default_open: bool,
    title_bar: bool,
}

impl Window {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            open: RNone,
            title: RNone,
            title_bar: false,
            area: Area::default(),
            frame: RSome(
                Frame::new()
                    .corner_radius(CornerRadius::ZERO)
                    .stroke(Stroke::new(1.0_f32, Color32::from_gray(60)))
                    .fill(Color32::from_gray(27)),
            ),
            resize: Resize::default()
                .min_size([96.0, 32.0])
                .default_size([340.0, 420.0]),
            scroll: ScrollArea::neither().auto_shrink(false),
            default_open: true,
        }
    }

    #[inline]
    pub fn open(mut self, open: &mut bool) -> Self {
        self.open = RSome(open);
        self
    }

    #[inline]
    pub fn title(mut self, title: impl Into<RString>) -> Self {
        self.title = RSome(title.into());
        self
    }

    #[inline]
    pub fn title_bar(mut self, title_bar: bool) -> Self {
        self.title_bar = title_bar;
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
    pub fn min_size(mut self, min_size: impl Into<Vec2>) -> Self {
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
    pub fn max_size(mut self, max_size: impl Into<Vec2>) -> Self {
        self.resize = self.resize.max_size(max_size);
        self
    }

    #[inline]
    pub fn current_pos(mut self, current_pos: impl Into<Pos2>) -> Self {
        self.area = self.area.current_pos(current_pos);
        self
    }

    #[inline]
    pub fn default_pos(mut self, default_pos: impl Into<Pos2>) -> Self {
        self.area = self.area.default_pos(default_pos);
        self
    }

    #[inline]
    pub fn fixed_pos(mut self, pos: impl Into<Pos2>) -> Self {
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
    pub fn anchor(mut self, align: Align2, offset: impl Into<Vec2>) -> Self {
        self.area = self.area.anchor(align, offset);
        self
    }

    #[inline]
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    #[inline]
    pub fn default_size(mut self, default_size: impl Into<Vec2>) -> Self {
        let default_size = default_size.into();
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
    pub fn fixed_size(mut self, size: impl Into<Vec2>) -> Self {
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
    pub fn show<'a, R>(
        self,
        ui: &mut BunnyUi<'a>,
        add_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        let mut new = ui.new_child(Some(Layout::default()));
        let ret = add_contents(&mut new);
        let response = ui.add_component_auto_id(Container::Window(RBox::new(WindowComponent {
            contents: new,
            window: self,
        })));
        InnerResponse::new(ret, response)
    }
}

#[repr(C)]
pub struct WindowComponent<'a> {
    window: Window,
    contents: BunnyUi<'a>,
}

#[cfg(feature = "manager")]
impl WindowComponent<'_> {
    fn ui_title_bar(&mut self, ui: &mut egui::Ui) {
        let title_bar_height = 24.0;
        let rect = {
            let mut rect = ui.max_rect();
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        let painter = ui.painter();
        if let RSome(open) = self.window.open {
            let id: egui::Id = self.window.id.with("close button").into();
            let widget_state = ui
                .read_response(id)
                .map(|r| r.widget_state())
                .unwrap_or_default();
            let close_color = ui.visuals().widgets.state(widget_state).fg_stroke.color;
            let close_rect = painter.text(
                rect.right_center() - egui::vec2(4.0, 0.0),
                egui::Align2::RIGHT_CENTER,
                "❌",
                egui::FontId::proportional(14.0),
                close_color,
            );
            if ui.interact(close_rect, id, egui::Sense::click()).clicked() {
                unsafe { *open = false };
            }
        }

        if let RSome(title) = &self.window.title {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                title,
                egui::FontId::proportional(16.0),
                ui.visuals().text_color(),
            );
        }

        painter.line_segment(
            [rect.left_bottom(), rect.right_bottom()],
            ui.visuals().widgets.noninteractive.bg_stroke,
        );

        ui.allocate_rect(rect, egui::Sense::empty());
    }
}

#[cfg(feature = "manager")]
impl crate::elements::UiContainer for WindowComponent<'_> {
    fn ui(
        mut self,
        ui: &mut egui::Ui,
        responses: &mut abi_stable::std_types::RHashMap<
            crate::Id,
            crate::response::Response,
            rapidhash::fast::RandomState,
        >,
        pointer_state: abi_stable::std_types::RArc<crate::input_state::PointerState>,
        id: crate::Id,
    ) -> crate::response::Response {
        let mut window = egui::Window::new("")
            .id(self.window.id.into())
            .title_bar(false)
            .enabled(self.window.area.enabled)
            .interactable(self.window.area.interactable)
            .movable(self.window.area.movable)
            .constrain(self.window.area.constrain)
            .pivot(self.window.area.pivot.into())
            .default_size(self.window.area.default_size)
            .default_open(self.window.default_open)
            .scroll(self.window.scroll.direction_enabled)
            .scroll_bar_visibility(self.window.scroll.scroll_bar_visibility.into())
            .drag_to_scroll(self.window.scroll.scroll_source.drag);
        if let RSome(frame) = self.window.frame {
            window = window.frame(frame.into());
        }
        if let RSome(constrain_rect) = self.window.area.constrain_rect {
            window = window.constrain_to(constrain_rect);
        }
        if let RSome(default_pos) = self.window.area.default_pos {
            window = window.default_pos(default_pos);
        }
        if let RSome(anchor) = self.window.area.anchor {
            window = window.anchor(anchor.0.into(), anchor.1);
        }
        if let RSome(new_pos) = self.window.area.new_pos {
            window = window.current_pos(new_pos);
        }
        window = window.resize(|r| {
            r.min_size(self.window.resize.min_size)
                .max_size(self.window.resize.max_size)
                .resizable(self.window.resize.resizable)
        });
        let inner = window.show(ui, |ui| {
            if self.window.title_bar {
                self.ui_title_bar(ui);
            }
            ui.take_available_space();
            self.contents.ui(ui, responses, pointer_state.clone());
        });
        if let Some(inner) = inner {
            crate::response::Response::new(id, inner.response, pointer_state)
        } else {
            crate::response::Response::empty(id, pointer_state)
        }
    }
}
