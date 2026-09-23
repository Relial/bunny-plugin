use abi_stable::std_types::{
    ROption::{self, RNone, RSome},
    Tuple2,
};
use emath::{Pos2, Rect, Vec2};
use mint::{Point2, Vector2};

use crate::{
    Align2, Id, LayerId, Layout, Order, Sense, UiKind, UiStackInfo, response::BunnyInnerResponse,
    ui::BunnyUi,
};
#[cfg(feature = "manager")]
use crate::{BunnyResponse, closure::PluginClosure};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Area {
    info: UiStackInfo,
    pub(crate) constrain_rect: ROption<Rect>,
    pub(crate) anchor: ROption<Tuple2<Align2, Vec2>>,
    layout: Layout,
    pub(crate) default_pos: ROption<Pos2>,
    pub(crate) new_pos: ROption<Pos2>,
    pub(crate) id: Id,
    pub(crate) default_size: Vec2,
    pub(crate) pivot: Align2,
    order: Order,
    sense: ROption<Sense>,
    pub(crate) movable: bool,
    pub(crate) interactable: bool,
    pub(crate) enabled: bool,
    pub(crate) constrain: bool,
    fade_in: bool,
    sizing_pass: bool,
}

impl Area {
    #[inline]
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            info: UiStackInfo::new(UiKind::GenericArea),
            sense: RNone,
            movable: true,
            interactable: true,
            enabled: true,
            constrain: true,
            constrain_rect: RNone,
            order: Order::Middle,
            default_pos: RNone,
            new_pos: RNone,
            default_size: Vec2::NAN,
            pivot: Align2::LEFT_TOP,
            anchor: RNone,
            fade_in: true,
            layout: Default::default(),
            sizing_pass: false,
        }
    }

    #[inline]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = id.into();
        self
    }

    #[inline]
    pub fn kind(mut self, kind: UiKind) -> Self {
        self.info.kind = RSome(kind);
        self
    }

    #[inline]
    pub fn info(mut self, info: UiStackInfo) -> Self {
        self.info = info;
        self
    }

    #[inline]
    pub fn layer(&self) -> LayerId {
        LayerId::new(self.order, self.id)
    }

    #[inline]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    #[inline]
    pub fn movable(mut self, movable: bool) -> Self {
        self.movable = movable;
        self.interactable |= movable;
        self
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    #[inline]
    pub fn is_movable(&self) -> bool {
        self.movable && self.enabled
    }

    #[inline]
    pub fn interactable(mut self, interactable: bool) -> Self {
        self.interactable = interactable;
        self.movable &= interactable;
        self
    }

    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = RSome(sense);
        self
    }

    #[inline]
    pub fn order(mut self, order: Order) -> Self {
        self.order = order;
        self
    }

    #[inline]
    pub fn default_pos(mut self, default_pos: impl Into<Point2<f32>>) -> Self {
        self.default_pos = RSome(default_pos.into().into());
        self
    }

    #[inline]
    pub fn default_size(mut self, default_size: impl Into<Vector2<f32>>) -> Self {
        self.default_size = default_size.into().into();
        self
    }

    #[inline]
    pub fn default_width(mut self, default_width: f32) -> Self {
        self.default_size.x = default_width;
        self
    }

    #[inline]
    pub fn default_height(mut self, default_height: f32) -> Self {
        self.default_size.y = default_height;
        self
    }

    #[inline]
    pub fn fixed_pos(mut self, fixed_pos: impl Into<Point2<f32>>) -> Self {
        self.new_pos = RSome(fixed_pos.into().into());
        self.movable = false;
        self
    }

    #[inline]
    pub fn constrain(mut self, constrain: bool) -> Self {
        self.constrain = constrain;
        self
    }

    #[inline]
    pub fn constrain_to(mut self, constrain_rect: Rect) -> Self {
        self.constrain = true;
        self.constrain_rect = RSome(constrain_rect);
        self
    }

    #[inline]
    pub fn pivot(mut self, pivot: Align2) -> Self {
        self.pivot = pivot;
        self
    }

    #[inline]
    pub fn current_pos(mut self, current_pos: impl Into<Point2<f32>>) -> Self {
        self.new_pos = RSome(current_pos.into().into());
        self
    }

    #[inline]
    pub fn anchor(mut self, align: Align2, offset: impl Into<Vector2<f32>>) -> Self {
        self.anchor = RSome(Tuple2(align, offset.into().into()));
        self.movable(false)
    }

    #[inline]
    pub fn fade_in(mut self, fade_in: bool) -> Self {
        self.fade_in = fade_in;
        self
    }

    #[inline]
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    #[inline]
    pub fn sizing_pass(mut self, resize: bool) -> Self {
        self.sizing_pass = resize;
        self
    }

    #[inline]
    pub fn show<R>(
        self,
        ui: &mut BunnyUi,
        add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<R> {
        ui.area_show(self, add_contents)
    }
}

#[cfg(feature = "manager")]
impl From<Area> for egui::Area {
    fn from(value: Area) -> Self {
        let Area {
            id,
            info,
            sense,
            movable,
            interactable,
            enabled,
            constrain,
            constrain_rect: constraint_rect,
            order,
            default_pos,
            new_pos,
            default_size,
            pivot,
            anchor,
            fade_in,
            layout,
            sizing_pass,
        } = value;
        let mut area = egui::Area::new(id.into())
            .info(info.into())
            .enabled(enabled)
            .movable(movable)
            .interactable(interactable)
            .order(order.into())
            .default_size(default_size)
            .constrain(constrain)
            .pivot(pivot.into())
            .fade_in(fade_in)
            .layout(layout.into())
            .sizing_pass(sizing_pass);
        if let RSome(sense) = sense {
            area = area.sense(sense.into());
        }
        if let RSome(constrain_rect) = constraint_rect {
            area = area.constrain_to(constrain_rect);
        }
        if let RSome(Tuple2(align, offset)) = anchor {
            area = area.anchor(align.into(), offset);
        }
        if let RSome(default_pos) = default_pos {
            area = area.default_pos(default_pos);
        }
        if let RSome(new_pos) = new_pos {
            area = area.current_pos(new_pos);
        }
        area
    }
}

#[cfg(feature = "manager")]
impl Area {
    pub(crate) fn show_impl(self, ui: &mut egui::Ui, contents: PluginClosure) -> BunnyResponse {
        let area: egui::Area = self.into();
        let response = area
            .show(ui, |ui| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b);
            })
            .response;
        BunnyResponse::new(response)
    }
}
