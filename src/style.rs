use abi_stable::std_types::ROption;
use egui::{Color32, Rangef, Vec2};

use crate::{
    align::Align,
    margin::Margin,
    paint::{
        corner_radius::CornerRadius,
        stroke::Stroke,
        text::{TextOptions, fonts::FontId, text_layout_types::TextWrapMode},
    },
    shadow::Shadow,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextStyle {
    Small,
    Body,
    Monospace,
    Button,
    Heading,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Style {
    pub(crate) changed: bool,
    pub override_text_style: ROption<TextStyle>,
    pub override_font_id: ROption<FontId>,
    pub override_text_valign: ROption<Align>,
    pub drag_value_text_style: TextStyle,
    pub wrap_mode: ROption<TextWrapMode>,
    spacing: Spacing,
    interaction: Interaction,
    visuals: Visuals,
    pub animation_time: f32,
    pub explanation_tooltips: bool,
    pub always_scroll_the_only_direction: bool,
    pub scroll_animation: ScrollAnimation,
    pub compact_menu_style: bool,
}

impl Style {
    pub fn to_egui(&self, egui_style: &mut egui::Style) {
        let s = egui_style;
        if self.changed {
            s.override_text_style = self.override_text_style.map(|t| t.into()).into();
            s.override_font_id = self.override_font_id.as_ref().map(|f| f.into()).into();
            s.override_text_valign = self.override_text_valign.map(|a| a.into()).into();
            s.drag_value_text_style = self.drag_value_text_style.into();
            s.wrap_mode = self.wrap_mode.map(|t| t.into()).into();
            s.animation_time = self.animation_time;
            s.explanation_tooltips = self.explanation_tooltips;
            s.always_scroll_the_only_direction = self.always_scroll_the_only_direction;
            s.scroll_animation = self.scroll_animation.into();
            s.compact_menu_style = self.compact_menu_style;
        }
        if self.spacing.changed {
            let spacing = &self.spacing;
            s.spacing = spacing.into();
        }
        if self.interaction.changed {
            let interaction = &self.interaction;
            s.interaction = interaction.into();
        }
        if self.visuals.changed {
            let visuals = &self.visuals;
            s.visuals = visuals.into();
        }
    }

    pub fn from_egui(egui_style: &egui::Style) -> Self {
        Self {
            changed: false,
            override_text_style: egui_style
                .override_text_style
                .clone()
                .map(|t| t.into())
                .into(),
            override_font_id: egui_style
                .override_font_id
                .as_ref()
                .map(|f| f.into())
                .into(),
            override_text_valign: egui_style.override_text_valign.map(|a| a.into()).into(),
            drag_value_text_style: egui_style.drag_value_text_style.clone().into(),
            wrap_mode: egui_style.wrap_mode.map(|t| t.into()).into(),
            spacing: (&egui_style.spacing).into(),
            interaction: (&egui_style.interaction).into(),
            visuals: (&egui_style.visuals).into(),
            animation_time: egui_style.animation_time,
            explanation_tooltips: egui_style.explanation_tooltips,
            always_scroll_the_only_direction: egui_style.always_scroll_the_only_direction,
            scroll_animation: egui_style.scroll_animation.into(),
            compact_menu_style: egui_style.compact_menu_style,
        }
    }
}

impl Style {
    #[inline]
    pub fn spacing(&self) -> &Spacing {
        &self.spacing
    }

    #[inline]
    pub fn spacing_mut(&mut self) -> &mut Spacing {
        let spacing = &mut self.spacing;
        spacing.changed = true;
        spacing
    }

    #[inline]
    pub fn interaction(&self) -> &Interaction {
        &self.interaction
    }

    #[inline]
    pub fn interaction_mut(&mut self) -> &mut Interaction {
        let interaction = &mut self.interaction;
        interaction.changed = true;
        interaction
    }

    #[inline]
    pub fn visuals(&self) -> &Visuals {
        &self.visuals
    }

    #[inline]
    pub fn visuals_mut(&mut self) -> &mut Visuals {
        let visuals = &mut self.visuals;
        visuals.changed = true;
        visuals
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Spacing {
    changed: bool,
    pub item_spacing: Vec2,
    pub window_margin: Margin,
    pub button_padding: Vec2,
    pub menu_margin: Margin,
    pub indent: f32,
    pub interact_size: Vec2,
    pub slider_width: f32,
    pub slider_rail_height: f32,
    pub combo_width: f32,
    pub text_edit_width: f32,
    pub icon_width: f32,
    pub icon_width_inner: f32,
    pub icon_spacing: f32,
    pub default_area_size: Vec2,
    pub tooltip_width: f32,
    pub menu_width: f32,
    pub menu_spacing: f32,
    pub indent_ends_with_horizontal_line: bool,
    pub combo_height: f32,
    pub scroll: ScrollStyle,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollStyle {
    pub floating: bool,
    pub content_margin: Margin,
    pub bar_width: f32,
    pub handle_min_length: f32,
    pub bar_inner_margin: f32,
    pub bar_outer_margin: f32,
    pub floating_width: f32,
    pub floating_allocated_width: f32,
    pub foreground_color: bool,
    pub dormant_background_opacity: f32,
    pub active_background_opacity: f32,
    pub interact_background_opacity: f32,
    pub dormant_handle_opacity: f32,
    pub active_handle_opacity: f32,
    pub interact_handle_opacity: f32,
    pub fade: ScrollFadeStyle,
}

impl ScrollStyle {
    pub fn solid() -> Self {
        Self {
            floating: false,
            content_margin: Margin::ZERO,
            bar_width: 6.0,
            handle_min_length: 12.0,
            bar_inner_margin: 4.0,
            bar_outer_margin: 0.0,
            floating_width: 2.0,
            floating_allocated_width: 0.0,
            foreground_color: false,
            dormant_background_opacity: 0.0,
            active_background_opacity: 0.4,
            interact_background_opacity: 0.7,
            dormant_handle_opacity: 0.0,
            active_handle_opacity: 0.6,
            interact_handle_opacity: 1.0,
            fade: Default::default(),
        }
    }

    pub fn thin() -> Self {
        Self {
            floating: true,
            bar_width: 10.0,
            floating_allocated_width: 6.0,
            foreground_color: false,
            dormant_background_opacity: 1.0,
            dormant_handle_opacity: 1.0,
            active_background_opacity: 1.0,
            active_handle_opacity: 1.0,
            interact_background_opacity: 0.6,
            interact_handle_opacity: 0.6,

            ..Self::solid()
        }
    }

    pub fn floating() -> Self {
        Self {
            floating: true,
            bar_width: 10.0,
            foreground_color: true,
            floating_allocated_width: 0.0,
            dormant_background_opacity: 0.0,
            dormant_handle_opacity: 0.0,
            ..Self::solid()
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollFadeStyle {
    pub strength: f32,
    pub size: f32,
}

impl Default for ScrollFadeStyle {
    fn default() -> Self {
        Self {
            strength: 0.5,
            size: 20.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollAnimation {
    pub points_per_second: f32,
    pub duration: Rangef,
}

impl Default for ScrollAnimation {
    fn default() -> Self {
        Self {
            points_per_second: 1000.0,
            duration: Rangef::new(0.1, 0.3),
        }
    }
}

impl ScrollAnimation {
    pub fn new(points_per_second: f32, duration: Rangef) -> Self {
        Self {
            points_per_second,
            duration,
        }
    }

    pub fn none() -> Self {
        Self {
            points_per_second: f32::INFINITY,
            duration: Rangef::new(0.0, 0.0),
        }
    }

    pub fn duration(t: f32) -> Self {
        Self {
            points_per_second: f32::INFINITY,
            duration: Rangef::new(t, t),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Interaction {
    changed: bool,
    pub interact_radius: f32,
    pub resize_grab_radius_side: f32,
    pub resize_grab_radius_corner: f32,
    pub show_tooltips_only_when_still: bool,
    pub tooltip_delay: f32,
    pub tooltip_grace_time: f32,
    pub selectable_labels: bool,
    pub multi_widget_text_select: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextCursorStyle {
    pub stroke: Stroke,
    pub preview: bool,
    pub blink: bool,
    pub on_duration: f32,
    pub off_duration: f32,
}

impl Default for TextCursorStyle {
    fn default() -> Self {
        Self {
            stroke: Stroke::new(2.0, Color32::from_rgb(192, 222, 255)),
            preview: false,
            blink: true,
            on_duration: 0.5,
            off_duration: 0.5,
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Visuals {
    changed: bool,
    pub dark_mode: bool,
    pub text_options: TextOptions,
    pub override_text_color: ROption<Color32>,
    pub weak_text_alpha: f32,
    pub weak_text_color: ROption<Color32>,
    pub widgets: Widgets,
    pub selection: Selection,
    pub hyperlink_color: Color32,
    pub faint_bg_color: Color32,
    pub extreme_bg_color: Color32,
    pub text_edit_bg_color: ROption<Color32>,
    pub code_bg_color: Color32,
    pub warn_fg_color: Color32,
    pub error_fg_color: Color32,
    pub window_corner_radius: CornerRadius,
    pub window_shadow: Shadow,
    pub window_fill: Color32,
    pub window_stroke: Stroke,
    pub window_highlight_topmost: bool,
    pub menu_corner_radius: CornerRadius,
    pub panel_fill: Color32,
    pub popup_shadow: Shadow,
    pub resize_corner_size: f32,
    pub text_cursor: TextCursorStyle,
    pub clip_rect_margin: f32,
    pub button_frame: bool,
    pub collapsing_header_frame: bool,
    pub indent_has_left_vline: bool,
    pub striped: bool,
    pub slider_trailing_fill: bool,
    pub handle_shape: HandleShape,
    pub image_loading_spinners: bool,
    pub numeric_color_space: NumericColorSpace,
    pub disabled_alpha: f32,
}

impl Visuals {
    #[inline(always)]
    pub fn noninteractive(&self) -> &WidgetVisuals {
        &self.widgets.noninteractive
    }

    pub fn text_color(&self) -> Color32 {
        self.override_text_color
            .unwrap_or_else(|| self.widgets.noninteractive.text_color())
    }

    pub fn weak_text_color(&self) -> Color32 {
        self.weak_text_color
            .unwrap_or_else(|| self.text_color().gamma_multiply(self.weak_text_alpha))
    }

    #[inline(always)]
    pub fn strong_text_color(&self) -> Color32 {
        self.widgets.active.text_color()
    }

    pub fn text_edit_bg_color(&self) -> Color32 {
        self.text_edit_bg_color.unwrap_or(self.extreme_bg_color)
    }

    #[inline(always)]
    pub fn disable(&self, color: Color32) -> Color32 {
        color.gamma_multiply(self.disabled_alpha)
    }

    #[inline(always)]
    pub fn gray_out(&self, color: Color32) -> Color32 {
        egui::ecolor::tint_color_towards(color, self.widgets.noninteractive.weak_bg_fill)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Selection {
    pub bg_fill: Color32,
    pub stroke: Stroke,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HandleShape {
    Circle,
    Rect { aspect_ratio: f32 },
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Widgets {
    pub noninteractive: WidgetVisuals,
    pub inactive: WidgetVisuals,
    pub hovered: WidgetVisuals,
    pub active: WidgetVisuals,
    pub open: WidgetVisuals,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WidgetVisuals {
    pub bg_fill: Color32,
    pub weak_bg_fill: Color32,
    pub bg_stroke: Stroke,
    pub corner_radius: CornerRadius,
    pub fg_stroke: Stroke,
    pub expansion: f32,
}

impl WidgetVisuals {
    #[inline(always)]
    pub fn text_color(&self) -> Color32 {
        self.fg_stroke.color
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NumericColorSpace {
    GammaByte,
    Linear,
}

impl From<TextStyle> for egui::TextStyle {
    fn from(value: TextStyle) -> Self {
        match value {
            TextStyle::Small => Self::Small,
            TextStyle::Body => Self::Body,
            TextStyle::Monospace => Self::Monospace,
            TextStyle::Button => Self::Button,
            TextStyle::Heading => Self::Heading,
        }
    }
}

impl From<egui::TextStyle> for TextStyle {
    fn from(value: egui::TextStyle) -> Self {
        match value {
            egui::TextStyle::Small => Self::Small,
            egui::TextStyle::Body => Self::Body,
            egui::TextStyle::Monospace => Self::Monospace,
            egui::TextStyle::Button => Self::Button,
            egui::TextStyle::Heading => Self::Heading,
            egui::TextStyle::Name(_) => panic!("Unsupported TextStyle: Name"),
        }
    }
}

impl From<&Spacing> for egui::Spacing {
    fn from(value: &Spacing) -> Self {
        Self {
            item_spacing: value.item_spacing,
            window_margin: value.window_margin.into(),
            button_padding: value.button_padding,
            menu_margin: value.menu_margin.into(),
            indent: value.indent,
            interact_size: value.interact_size,
            slider_width: value.slider_width,
            slider_rail_height: value.slider_rail_height,
            combo_width: value.combo_width,
            text_edit_width: value.text_edit_width,
            icon_width: value.icon_width,
            icon_width_inner: value.icon_width_inner,
            icon_spacing: value.icon_spacing,
            default_area_size: value.default_area_size,
            tooltip_width: value.tooltip_width,
            menu_width: value.menu_width,
            menu_spacing: value.menu_spacing,
            indent_ends_with_horizontal_line: value.indent_ends_with_horizontal_line,
            combo_height: value.combo_height,
            scroll: value.scroll.clone().into(),
        }
    }
}

impl From<&egui::Spacing> for Spacing {
    fn from(value: &egui::Spacing) -> Self {
        Self {
            changed: false,
            item_spacing: value.item_spacing,
            window_margin: value.window_margin.into(),
            button_padding: value.button_padding,
            menu_margin: value.menu_margin.into(),
            indent: value.indent,
            interact_size: value.interact_size,
            slider_width: value.slider_width,
            slider_rail_height: value.slider_rail_height,
            combo_width: value.combo_width,
            text_edit_width: value.text_edit_width,
            icon_width: value.icon_width,
            icon_width_inner: value.icon_width_inner,
            icon_spacing: value.icon_spacing,
            default_area_size: value.default_area_size,
            tooltip_width: value.tooltip_width,
            menu_width: value.menu_width,
            menu_spacing: value.menu_spacing,
            indent_ends_with_horizontal_line: value.indent_ends_with_horizontal_line,
            combo_height: value.combo_height,
            scroll: value.scroll.into(),
        }
    }
}

impl From<ScrollStyle> for egui::style::ScrollStyle {
    fn from(value: ScrollStyle) -> Self {
        Self {
            floating: value.floating,
            content_margin: value.content_margin.into(),
            bar_width: value.bar_width,
            handle_min_length: value.handle_min_length,
            bar_inner_margin: value.bar_inner_margin,
            bar_outer_margin: value.bar_outer_margin,
            floating_width: value.floating_width,
            floating_allocated_width: value.floating_allocated_width,
            foreground_color: value.foreground_color,
            dormant_background_opacity: value.dormant_background_opacity,
            active_background_opacity: value.active_background_opacity,
            interact_background_opacity: value.interact_background_opacity,
            dormant_handle_opacity: value.dormant_handle_opacity,
            active_handle_opacity: value.active_handle_opacity,
            interact_handle_opacity: value.interact_handle_opacity,
            fade: value.fade.into(),
        }
    }
}

impl From<egui::style::ScrollStyle> for ScrollStyle {
    fn from(value: egui::style::ScrollStyle) -> Self {
        Self {
            floating: value.floating,
            content_margin: value.content_margin.into(),
            bar_width: value.bar_width,
            handle_min_length: value.handle_min_length,
            bar_inner_margin: value.bar_inner_margin,
            bar_outer_margin: value.bar_outer_margin,
            floating_width: value.floating_width,
            floating_allocated_width: value.floating_allocated_width,
            foreground_color: value.foreground_color,
            dormant_background_opacity: value.dormant_background_opacity,
            active_background_opacity: value.active_background_opacity,
            interact_background_opacity: value.interact_background_opacity,
            dormant_handle_opacity: value.dormant_handle_opacity,
            active_handle_opacity: value.active_handle_opacity,
            interact_handle_opacity: value.interact_handle_opacity,
            fade: value.fade.into(),
        }
    }
}

impl From<ScrollFadeStyle> for egui::style::ScrollFadeStyle {
    fn from(value: ScrollFadeStyle) -> Self {
        Self {
            strength: value.strength,
            size: value.size,
        }
    }
}

impl From<egui::style::ScrollFadeStyle> for ScrollFadeStyle {
    fn from(value: egui::style::ScrollFadeStyle) -> Self {
        Self {
            strength: value.strength,
            size: value.size,
        }
    }
}

impl From<ScrollAnimation> for egui::style::ScrollAnimation {
    fn from(value: ScrollAnimation) -> Self {
        Self {
            points_per_second: value.points_per_second,
            duration: value.duration,
        }
    }
}

impl From<egui::style::ScrollAnimation> for ScrollAnimation {
    fn from(value: egui::style::ScrollAnimation) -> Self {
        Self {
            points_per_second: value.points_per_second,
            duration: value.duration,
        }
    }
}

impl From<&Interaction> for egui::style::Interaction {
    fn from(value: &Interaction) -> Self {
        Self {
            interact_radius: value.interact_radius,
            resize_grab_radius_side: value.resize_grab_radius_side,
            resize_grab_radius_corner: value.resize_grab_radius_corner,
            show_tooltips_only_when_still: value.show_tooltips_only_when_still,
            tooltip_delay: value.tooltip_delay,
            tooltip_grace_time: value.tooltip_grace_time,
            selectable_labels: value.selectable_labels,
            multi_widget_text_select: value.multi_widget_text_select,
        }
    }
}

impl From<&egui::style::Interaction> for Interaction {
    fn from(value: &egui::style::Interaction) -> Self {
        Self {
            changed: false,
            interact_radius: value.interact_radius,
            resize_grab_radius_side: value.resize_grab_radius_side,
            resize_grab_radius_corner: value.resize_grab_radius_corner,
            show_tooltips_only_when_still: value.show_tooltips_only_when_still,
            tooltip_delay: value.tooltip_delay,
            tooltip_grace_time: value.tooltip_grace_time,
            selectable_labels: value.selectable_labels,
            multi_widget_text_select: value.multi_widget_text_select,
        }
    }
}

impl From<TextCursorStyle> for egui::style::TextCursorStyle {
    fn from(value: TextCursorStyle) -> Self {
        Self {
            stroke: value.stroke.into(),
            preview: value.preview,
            blink: value.blink,
            on_duration: value.on_duration,
            off_duration: value.off_duration,
        }
    }
}

impl From<egui::style::TextCursorStyle> for TextCursorStyle {
    fn from(value: egui::style::TextCursorStyle) -> Self {
        Self {
            stroke: value.stroke.into(),
            preview: value.preview,
            blink: value.blink,
            on_duration: value.on_duration,
            off_duration: value.off_duration,
        }
    }
}

impl From<&Visuals> for egui::style::Visuals {
    fn from(value: &Visuals) -> Self {
        Self {
            dark_mode: value.dark_mode,
            text_options: value.text_options.into(),
            override_text_color: value.override_text_color.into(),
            weak_text_alpha: value.weak_text_alpha,
            weak_text_color: value.weak_text_color.into(),
            widgets: value.widgets.clone().into(),
            selection: value.selection.into(),
            hyperlink_color: value.hyperlink_color,
            faint_bg_color: value.faint_bg_color,
            extreme_bg_color: value.extreme_bg_color,
            text_edit_bg_color: value.text_edit_bg_color.into(),
            code_bg_color: value.code_bg_color,
            warn_fg_color: value.warn_fg_color,
            error_fg_color: value.error_fg_color,
            window_corner_radius: value.window_corner_radius.into(),
            window_shadow: value.window_shadow.into(),
            window_fill: value.window_fill,
            window_stroke: value.window_stroke.into(),
            window_highlight_topmost: value.window_highlight_topmost,
            menu_corner_radius: value.menu_corner_radius.into(),
            panel_fill: value.panel_fill,
            popup_shadow: value.popup_shadow.into(),
            resize_corner_size: value.resize_corner_size,
            text_cursor: value.text_cursor.into(),
            clip_rect_margin: value.clip_rect_margin,
            button_frame: value.button_frame,
            collapsing_header_frame: value.collapsing_header_frame,
            indent_has_left_vline: value.indent_has_left_vline,
            striped: value.striped,
            slider_trailing_fill: value.slider_trailing_fill,
            handle_shape: value.handle_shape.into(),
            interact_cursor: None,
            image_loading_spinners: value.image_loading_spinners,
            numeric_color_space: value.numeric_color_space.into(),
            disabled_alpha: value.disabled_alpha,
        }
    }
}

impl From<&egui::style::Visuals> for Visuals {
    fn from(value: &egui::style::Visuals) -> Self {
        Self {
            changed: false,
            dark_mode: value.dark_mode,
            text_options: value.text_options.into(),
            override_text_color: value.override_text_color.into(),
            weak_text_alpha: value.weak_text_alpha,
            weak_text_color: value.weak_text_color.into(),
            widgets: value.widgets.clone().into(),
            selection: value.selection.into(),
            hyperlink_color: value.hyperlink_color,
            faint_bg_color: value.faint_bg_color,
            extreme_bg_color: value.extreme_bg_color,
            text_edit_bg_color: value.text_edit_bg_color.into(),
            code_bg_color: value.code_bg_color,
            warn_fg_color: value.warn_fg_color,
            error_fg_color: value.error_fg_color,
            window_corner_radius: value.window_corner_radius.into(),
            window_shadow: value.window_shadow.into(),
            window_fill: value.window_fill,
            window_stroke: value.window_stroke.into(),
            window_highlight_topmost: value.window_highlight_topmost,
            menu_corner_radius: value.menu_corner_radius.into(),
            panel_fill: value.panel_fill,
            popup_shadow: value.popup_shadow.into(),
            resize_corner_size: value.resize_corner_size,
            text_cursor: value.text_cursor.clone().into(),
            clip_rect_margin: value.clip_rect_margin,
            button_frame: value.button_frame,
            collapsing_header_frame: value.collapsing_header_frame,
            indent_has_left_vline: value.indent_has_left_vline,
            striped: value.striped,
            slider_trailing_fill: value.slider_trailing_fill,
            handle_shape: value.handle_shape.into(),
            image_loading_spinners: value.image_loading_spinners,
            numeric_color_space: value.numeric_color_space.into(),
            disabled_alpha: value.disabled_alpha,
        }
    }
}

impl From<Selection> for egui::style::Selection {
    fn from(value: Selection) -> Self {
        Self {
            bg_fill: value.bg_fill,
            stroke: value.stroke.into(),
        }
    }
}

impl From<egui::style::Selection> for Selection {
    fn from(value: egui::style::Selection) -> Self {
        Selection {
            bg_fill: value.bg_fill,
            stroke: value.stroke.into(),
        }
    }
}

impl From<HandleShape> for egui::style::HandleShape {
    fn from(value: HandleShape) -> Self {
        match value {
            HandleShape::Circle => Self::Circle,
            HandleShape::Rect { aspect_ratio } => Self::Rect { aspect_ratio },
        }
    }
}

impl From<egui::style::HandleShape> for HandleShape {
    fn from(value: egui::style::HandleShape) -> Self {
        match value {
            egui::style::HandleShape::Circle => Self::Circle,
            egui::style::HandleShape::Rect { aspect_ratio } => Self::Rect { aspect_ratio },
        }
    }
}

impl From<Widgets> for egui::style::Widgets {
    fn from(value: Widgets) -> Self {
        Self {
            noninteractive: value.noninteractive.into(),
            inactive: value.inactive.into(),
            hovered: value.hovered.into(),
            active: value.active.into(),
            open: value.open.into(),
        }
    }
}

impl From<egui::style::Widgets> for Widgets {
    fn from(value: egui::style::Widgets) -> Self {
        Self {
            noninteractive: value.noninteractive.into(),
            inactive: value.inactive.into(),
            hovered: value.hovered.into(),
            active: value.active.into(),
            open: value.open.into(),
        }
    }
}

impl From<WidgetVisuals> for egui::style::WidgetVisuals {
    fn from(value: WidgetVisuals) -> Self {
        Self {
            bg_fill: value.bg_fill,
            weak_bg_fill: value.weak_bg_fill,
            bg_stroke: value.bg_stroke.into(),
            corner_radius: value.corner_radius.into(),
            fg_stroke: value.fg_stroke.into(),
            expansion: value.expansion,
        }
    }
}

impl From<egui::style::WidgetVisuals> for WidgetVisuals {
    fn from(value: egui::style::WidgetVisuals) -> Self {
        Self {
            bg_fill: value.bg_fill,
            weak_bg_fill: value.weak_bg_fill,
            bg_stroke: value.bg_stroke.into(),
            corner_radius: value.corner_radius.into(),
            fg_stroke: value.fg_stroke.into(),
            expansion: value.expansion,
        }
    }
}

impl From<NumericColorSpace> for egui::style::NumericColorSpace {
    fn from(value: NumericColorSpace) -> Self {
        match value {
            NumericColorSpace::GammaByte => Self::GammaByte,
            NumericColorSpace::Linear => Self::Linear,
        }
    }
}

impl From<egui::style::NumericColorSpace> for NumericColorSpace {
    fn from(value: egui::style::NumericColorSpace) -> Self {
        match value {
            egui::style::NumericColorSpace::GammaByte => Self::GammaByte,
            egui::style::NumericColorSpace::Linear => Self::Linear,
        }
    }
}
