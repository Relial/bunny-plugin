use abi_stable::std_types::ROption;
use egui::{Color32, Rangef, Vec2};

use crate::{
    Align, Margin, Shadow,
    paint::{
        corner_radius::CornerRadius,
        stroke::Stroke,
        text::{TextOptions, text_layout_types::TextWrapMode},
    },
};

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum TextStyle {
    Small,
    Body,
    Monospace,
    Button,
    Heading,
}

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Style {
    pub visuals: Visuals,
    pub spacing: Spacing,
    pub interaction: Interaction,
    pub scroll_animation: ScrollAnimation,
    pub override_text_style: ROption<TextStyle>,
    pub override_text_valign: ROption<Align>,
    pub wrap_mode: ROption<TextWrapMode>,
    pub drag_value_text_style: TextStyle,
    pub animation_time: f32,
    pub compact_menu_style: bool,
    pub explanation_tooltips: bool,
    pub always_scroll_the_only_direction: bool,
}

#[cfg(feature = "manager")]
impl From<Style> for egui::Style {
    fn from(value: Style) -> Self {
        let Style {
            visuals,
            spacing,
            interaction,
            scroll_animation,
            override_text_style,
            override_text_valign,
            wrap_mode,
            drag_value_text_style,
            animation_time,
            compact_menu_style,
            explanation_tooltips,
            always_scroll_the_only_direction,
        } = value;
        Self {
            override_text_style: override_text_style.map(|t| t.into()).into_option(),
            override_text_valign: override_text_valign.map(|a| a.into()).into_option(),
            drag_value_text_style: drag_value_text_style.into(),
            wrap_mode: wrap_mode.map(|t| t.into()).into_option(),
            spacing: spacing.into(),
            interaction: interaction.into(),
            visuals: visuals.into(),
            animation_time,
            explanation_tooltips,
            always_scroll_the_only_direction,
            scroll_animation: scroll_animation.into(),
            compact_menu_style,
            ..Default::default()
        }
    }
}

#[cfg(feature = "manager")]
impl From<&Style> for egui::Style {
    fn from(value: &Style) -> Self {
        let &Style {
            visuals,
            spacing,
            interaction,
            scroll_animation,
            override_text_style,
            override_text_valign,
            wrap_mode,
            drag_value_text_style,
            animation_time,
            compact_menu_style,
            explanation_tooltips,
            always_scroll_the_only_direction,
        } = &value;
        Self {
            override_text_style: override_text_style.map(|t| t.into()).into_option(),
            override_text_valign: override_text_valign.map(|a| a.into()).into_option(),
            drag_value_text_style: (*drag_value_text_style).into(),
            wrap_mode: wrap_mode.map(|t| t.into()).into_option(),
            spacing: spacing.into(),
            interaction: interaction.into(),
            visuals: visuals.into(),
            animation_time: *animation_time,
            explanation_tooltips: *explanation_tooltips,
            always_scroll_the_only_direction: *always_scroll_the_only_direction,
            scroll_animation: (*scroll_animation).into(),
            compact_menu_style: *compact_menu_style,
            ..Default::default()
        }
    }
}

#[allow(deprecated)]
#[cfg(feature = "manager")]
impl From<egui::Style> for Style {
    fn from(value: egui::Style) -> Self {
        #[cfg(debug_assertions)]
        let egui::Style {
            override_text_style,
            override_font_id: _,
            override_text_valign,
            text_styles: _,
            drag_value_text_style,
            number_formatter: _,
            wrap: _,
            wrap_mode,
            spacing,
            interaction,
            visuals,
            animation_time,
            debug: _,
            explanation_tooltips,
            url_in_tooltip: _,
            always_scroll_the_only_direction,
            scroll_animation,
            compact_menu_style,
        } = value;
        #[cfg(not(debug_assertions))]
        let egui::Style {
            override_text_style,
            override_font_id: _,
            override_text_valign,
            text_styles: _,
            drag_value_text_style,
            number_formatter: _,
            wrap: _,
            wrap_mode,
            spacing,
            interaction,
            visuals,
            animation_time,
            explanation_tooltips,
            url_in_tooltip: _,
            always_scroll_the_only_direction,
            scroll_animation,
            compact_menu_style,
        } = value;
        Self {
            visuals: visuals.into(),
            spacing: spacing.into(),
            interaction: interaction.into(),
            scroll_animation: scroll_animation.into(),
            override_text_style: override_text_style.map(|t| t.into()).into(),
            override_text_valign: override_text_valign.map(|a| a.into()).into(),
            wrap_mode: wrap_mode.map(|t| t.into()).into(),
            drag_value_text_style: drag_value_text_style.into(),
            animation_time,
            compact_menu_style,
            explanation_tooltips,
            always_scroll_the_only_direction,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Spacing {
    pub scroll: ScrollStyle,
    pub item_spacing: Vec2,
    pub button_padding: Vec2,
    pub interact_size: Vec2,
    pub default_area_size: Vec2,
    pub window_margin: Margin,
    pub menu_margin: Margin,
    pub indent: f32,
    pub slider_width: f32,
    pub slider_rail_height: f32,
    pub combo_width: f32,
    pub text_edit_width: f32,
    pub icon_width: f32,
    pub icon_width_inner: f32,
    pub icon_spacing: f32,
    pub tooltip_width: f32,
    pub menu_width: f32,
    pub menu_spacing: f32,
    pub combo_height: f32,
    pub indent_ends_with_horizontal_line: bool,
}

#[cfg(feature = "manager")]
impl From<Spacing> for egui::Spacing {
    fn from(value: Spacing) -> Self {
        let Spacing {
            scroll,
            item_spacing,
            button_padding,
            interact_size,
            default_area_size,
            window_margin,
            menu_margin,
            indent,
            slider_width,
            slider_rail_height,
            combo_width,
            text_edit_width,
            icon_width,
            icon_width_inner,
            icon_spacing,
            tooltip_width,
            menu_width,
            menu_spacing,
            combo_height,
            indent_ends_with_horizontal_line,
        } = value;
        Self {
            item_spacing,
            window_margin: window_margin.into(),
            button_padding,
            menu_margin: menu_margin.into(),
            indent,
            interact_size,
            slider_width,
            slider_rail_height,
            combo_width,
            text_edit_width,
            icon_width,
            icon_width_inner,
            icon_spacing,
            default_area_size,
            tooltip_width,
            menu_width,
            menu_spacing,
            indent_ends_with_horizontal_line,
            combo_height,
            scroll: scroll.into(),
        }
    }
}

#[cfg(feature = "manager")]
impl From<&Spacing> for egui::style::Spacing {
    fn from(value: &Spacing) -> Self {
        value.clone().into()
    }
}

#[cfg(feature = "manager")]
impl From<egui::Spacing> for Spacing {
    fn from(value: egui::Spacing) -> Self {
        let egui::Spacing {
            item_spacing,
            window_margin,
            button_padding,
            menu_margin,
            indent,
            interact_size,
            slider_width,
            slider_rail_height,
            combo_width,
            text_edit_width,
            icon_width,
            icon_width_inner,
            icon_spacing,
            default_area_size,
            tooltip_width,
            menu_width,
            menu_spacing,
            indent_ends_with_horizontal_line,
            combo_height,
            scroll,
        } = value;
        Self {
            scroll: scroll.into(),
            item_spacing,
            button_padding,
            interact_size,
            default_area_size,
            window_margin: window_margin.into(),
            menu_margin: menu_margin.into(),
            indent,
            slider_width,
            slider_rail_height,
            combo_width,
            text_edit_width,
            icon_width,
            icon_width_inner,
            icon_spacing,
            tooltip_width,
            menu_width,
            menu_spacing,
            combo_height,
            indent_ends_with_horizontal_line,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct ScrollStyle {
    pub fade: ScrollFadeStyle,
    pub content_margin: Margin,
    pub bar_width: f32,
    pub handle_min_length: f32,
    pub bar_inner_margin: f32,
    pub bar_outer_margin: f32,
    pub floating_width: f32,
    pub floating_allocated_width: f32,
    pub dormant_background_opacity: f32,
    pub active_background_opacity: f32,
    pub interact_background_opacity: f32,
    pub dormant_handle_opacity: f32,
    pub active_handle_opacity: f32,
    pub interact_handle_opacity: f32,
    pub floating: bool,
    pub foreground_color: bool,
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

#[cfg(feature = "manager")]
impl From<ScrollStyle> for egui::style::ScrollStyle {
    fn from(value: ScrollStyle) -> Self {
        let ScrollStyle {
            fade,
            content_margin,
            bar_width,
            handle_min_length,
            bar_inner_margin,
            bar_outer_margin,
            floating_width,
            floating_allocated_width,
            dormant_background_opacity,
            active_background_opacity,
            interact_background_opacity,
            dormant_handle_opacity,
            active_handle_opacity,
            interact_handle_opacity,
            floating,
            foreground_color,
        } = value;
        Self {
            floating,
            content_margin: content_margin.into(),
            bar_width,
            handle_min_length,
            bar_inner_margin,
            bar_outer_margin,
            floating_width,
            floating_allocated_width,
            foreground_color,
            dormant_background_opacity,
            active_background_opacity,
            interact_background_opacity,
            dormant_handle_opacity,
            active_handle_opacity,
            interact_handle_opacity,
            fade: fade.into(),
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::ScrollStyle> for ScrollStyle {
    fn from(value: egui::style::ScrollStyle) -> Self {
        let egui::style::ScrollStyle {
            floating,
            content_margin,
            bar_width,
            handle_min_length,
            bar_inner_margin,
            bar_outer_margin,
            floating_width,
            floating_allocated_width,
            foreground_color,
            dormant_background_opacity,
            active_background_opacity,
            interact_background_opacity,
            dormant_handle_opacity,
            active_handle_opacity,
            interact_handle_opacity,
            fade,
        } = value;
        Self {
            fade: fade.into(),
            content_margin: content_margin.into(),
            bar_width,
            handle_min_length,
            bar_inner_margin,
            bar_outer_margin,
            floating_width,
            floating_allocated_width,
            dormant_background_opacity,
            active_background_opacity,
            interact_background_opacity,
            dormant_handle_opacity,
            active_handle_opacity,
            interact_handle_opacity,
            floating,
            foreground_color,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct ScrollAnimation {
    pub duration: Rangef,
    pub points_per_second: f32,
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

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Interaction {
    pub interact_radius: f32,
    pub resize_grab_radius_side: f32,
    pub resize_grab_radius_corner: f32,
    pub tooltip_delay: f32,
    pub tooltip_grace_time: f32,
    pub selectable_labels: bool,
    pub multi_widget_text_select: bool,
    pub show_tooltips_only_when_still: bool,
}

#[cfg(feature = "manager")]
impl From<Interaction> for egui::style::Interaction {
    fn from(value: Interaction) -> Self {
        let Interaction {
            interact_radius,
            resize_grab_radius_side,
            resize_grab_radius_corner,
            tooltip_delay,
            tooltip_grace_time,
            selectable_labels,
            multi_widget_text_select,
            show_tooltips_only_when_still,
        } = value;
        Self {
            interact_radius,
            resize_grab_radius_side,
            resize_grab_radius_corner,
            show_tooltips_only_when_still,
            tooltip_delay,
            tooltip_grace_time,
            selectable_labels,
            multi_widget_text_select,
        }
    }
}

#[cfg(feature = "manager")]
impl From<&Interaction> for egui::style::Interaction {
    fn from(value: &Interaction) -> Self {
        value.clone().into()
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::Interaction> for Interaction {
    fn from(value: egui::style::Interaction) -> Self {
        let egui::style::Interaction {
            interact_radius,
            resize_grab_radius_side,
            resize_grab_radius_corner,
            show_tooltips_only_when_still,
            tooltip_delay,
            tooltip_grace_time,
            selectable_labels,
            multi_widget_text_select,
        } = value;
        Self {
            interact_radius,
            resize_grab_radius_side,
            resize_grab_radius_corner,
            tooltip_delay,
            tooltip_grace_time,
            selectable_labels,
            multi_widget_text_select,
            show_tooltips_only_when_still,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct TextCursorStyle {
    pub stroke: Stroke,
    pub on_duration: f32,
    pub off_duration: f32,
    pub preview: bool,
    pub blink: bool,
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

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Visuals {
    pub widgets: Widgets,
    pub text_cursor: TextCursorStyle,
    pub selection: Selection,
    pub text_options: TextOptions,
    pub override_text_color: ROption<Color32>,
    pub weak_text_color: ROption<Color32>,
    pub text_edit_bg_color: ROption<Color32>,
    pub window_shadow: Shadow,
    pub window_stroke: Stroke,
    pub popup_shadow: Shadow,
    pub handle_shape: HandleShape,
    pub weak_text_alpha: f32,
    pub hyperlink_color: Color32,
    pub faint_bg_color: Color32,
    pub extreme_bg_color: Color32,
    pub code_bg_color: Color32,
    pub warn_fg_color: Color32,
    pub error_fg_color: Color32,
    pub window_corner_radius: CornerRadius,
    pub window_fill: Color32,
    pub menu_corner_radius: CornerRadius,
    pub panel_fill: Color32,
    pub resize_corner_size: f32,
    pub clip_rect_margin: f32,
    pub numeric_color_space: NumericColorSpace,
    pub disabled_alpha: f32,
    pub dark_mode: bool,
    pub window_highlight_topmost: bool,
    pub button_frame: bool,
    pub collapsing_header_frame: bool,
    pub indent_has_left_vline: bool,
    pub striped: bool,
    pub slider_trailing_fill: bool,
    pub image_loading_spinners: bool,
}

#[cfg(feature = "manager")]
impl From<Visuals> for egui::style::Visuals {
    fn from(value: Visuals) -> Self {
        let Visuals {
            widgets,
            text_cursor,
            selection,
            text_options,
            override_text_color,
            weak_text_color,
            text_edit_bg_color,
            window_shadow,
            window_stroke,
            popup_shadow,
            handle_shape,
            weak_text_alpha,
            hyperlink_color,
            faint_bg_color,
            extreme_bg_color,
            code_bg_color,
            warn_fg_color,
            error_fg_color,
            window_corner_radius,
            window_fill,
            menu_corner_radius,
            panel_fill,
            resize_corner_size,
            clip_rect_margin,
            numeric_color_space,
            disabled_alpha,
            dark_mode,
            window_highlight_topmost,
            button_frame,
            collapsing_header_frame,
            indent_has_left_vline,
            striped,
            slider_trailing_fill,
            image_loading_spinners,
        } = value;
        Self {
            dark_mode,
            text_options: text_options.into(),
            override_text_color: override_text_color.into_option(),
            weak_text_alpha,
            weak_text_color: weak_text_color.into_option(),
            widgets: widgets.into(),
            selection: selection.into(),
            hyperlink_color,
            faint_bg_color,
            extreme_bg_color,
            text_edit_bg_color: text_edit_bg_color.into_option(),
            code_bg_color,
            warn_fg_color,
            error_fg_color,
            window_corner_radius: window_corner_radius.into(),
            window_shadow: window_shadow.into(),
            window_fill,
            window_stroke: window_stroke.into(),
            window_highlight_topmost,
            menu_corner_radius: menu_corner_radius.into(),
            panel_fill,
            popup_shadow: popup_shadow.into(),
            resize_corner_size,
            text_cursor: text_cursor.into(),
            clip_rect_margin,
            button_frame,
            collapsing_header_frame,
            indent_has_left_vline,
            striped,
            slider_trailing_fill,
            handle_shape: handle_shape.into(),
            image_loading_spinners,
            numeric_color_space: numeric_color_space.into(),
            disabled_alpha,
            ..Default::default()
        }
    }
}

#[cfg(feature = "manager")]
impl From<&Visuals> for egui::style::Visuals {
    fn from(value: &Visuals) -> Self {
        value.clone().into()
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::Visuals> for Visuals {
    fn from(value: egui::style::Visuals) -> Self {
        let egui::style::Visuals {
            dark_mode,
            text_options,
            override_text_color,
            weak_text_alpha,
            weak_text_color,
            widgets,
            selection,
            hyperlink_color,
            faint_bg_color,
            extreme_bg_color,
            text_edit_bg_color,
            code_bg_color,
            warn_fg_color,
            error_fg_color,
            window_corner_radius,
            window_shadow,
            window_fill,
            window_stroke,
            window_highlight_topmost,
            menu_corner_radius,
            panel_fill,
            popup_shadow,
            resize_corner_size,
            text_cursor,
            clip_rect_margin,
            button_frame,
            collapsing_header_frame,
            indent_has_left_vline,
            striped,
            slider_trailing_fill,
            handle_shape,
            interact_cursor: _,
            image_loading_spinners,
            numeric_color_space,
            disabled_alpha,
        } = value;
        Self {
            widgets: widgets.into(),
            text_cursor: text_cursor.into(),
            selection: selection.into(),
            text_options: text_options.into(),
            override_text_color: override_text_color.into(),
            weak_text_color: weak_text_color.into(),
            text_edit_bg_color: text_edit_bg_color.into(),
            window_shadow: window_shadow.into(),
            window_stroke: window_stroke.into(),
            popup_shadow: popup_shadow.into(),
            handle_shape: handle_shape.into(),
            weak_text_alpha,
            hyperlink_color,
            faint_bg_color,
            extreme_bg_color,
            code_bg_color,
            warn_fg_color,
            error_fg_color,
            window_corner_radius: window_corner_radius.into(),
            window_fill,
            menu_corner_radius: menu_corner_radius.into(),
            panel_fill,
            resize_corner_size,
            clip_rect_margin,
            numeric_color_space: numeric_color_space.into(),
            disabled_alpha,
            dark_mode,
            window_highlight_topmost,
            button_frame,
            collapsing_header_frame,
            indent_has_left_vline,
            striped,
            slider_trailing_fill,
            image_loading_spinners,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Selection {
    pub stroke: Stroke,
    pub bg_fill: Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum HandleShape {
    Rect { aspect_ratio: f32 },
    Circle,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Widgets {
    pub noninteractive: WidgetVisuals,
    pub inactive: WidgetVisuals,
    pub hovered: WidgetVisuals,
    pub active: WidgetVisuals,
    pub open: WidgetVisuals,
}

#[cfg(feature = "manager")]
impl From<Widgets> for egui::style::Widgets {
    fn from(value: Widgets) -> Self {
        let Widgets {
            noninteractive,
            inactive,
            hovered,
            active,
            open,
        } = value;
        Self {
            noninteractive: noninteractive.into(),
            inactive: inactive.into(),
            hovered: hovered.into(),
            active: active.into(),
            open: open.into(),
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::Widgets> for Widgets {
    fn from(value: egui::style::Widgets) -> Self {
        let egui::style::Widgets {
            noninteractive,
            inactive,
            hovered,
            active,
            open,
        } = value;
        Self {
            noninteractive: noninteractive.into(),
            inactive: inactive.into(),
            hovered: hovered.into(),
            active: active.into(),
            open: open.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct WidgetVisuals {
    pub bg_stroke: Stroke,
    pub fg_stroke: Stroke,
    pub bg_fill: Color32,
    pub weak_bg_fill: Color32,
    pub corner_radius: CornerRadius,
    pub expansion: f32,
}

impl WidgetVisuals {
    #[inline(always)]
    pub fn text_color(&self) -> Color32 {
        self.fg_stroke.color
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum NumericColorSpace {
    GammaByte,
    Linear,
}

#[cfg(feature = "manager")]
impl From<WidgetVisuals> for egui::style::WidgetVisuals {
    #[inline]
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

#[cfg(feature = "manager")]
impl From<egui::style::WidgetVisuals> for WidgetVisuals {
    #[inline]
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

#[cfg(feature = "manager")]
impl From<NumericColorSpace> for egui::style::NumericColorSpace {
    #[inline]
    fn from(value: NumericColorSpace) -> Self {
        match value {
            NumericColorSpace::GammaByte => Self::GammaByte,
            NumericColorSpace::Linear => Self::Linear,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::NumericColorSpace> for NumericColorSpace {
    #[inline]
    fn from(value: egui::style::NumericColorSpace) -> Self {
        match value {
            egui::style::NumericColorSpace::GammaByte => Self::GammaByte,
            egui::style::NumericColorSpace::Linear => Self::Linear,
        }
    }
}

#[cfg(feature = "manager")]
impl From<Selection> for egui::style::Selection {
    #[inline]
    fn from(value: Selection) -> Self {
        Self {
            bg_fill: value.bg_fill,
            stroke: value.stroke.into(),
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::Selection> for Selection {
    #[inline]
    fn from(value: egui::style::Selection) -> Self {
        Selection {
            bg_fill: value.bg_fill,
            stroke: value.stroke.into(),
        }
    }
}

#[cfg(feature = "manager")]
impl From<HandleShape> for egui::style::HandleShape {
    #[inline]
    fn from(value: HandleShape) -> Self {
        match value {
            HandleShape::Circle => Self::Circle,
            HandleShape::Rect { aspect_ratio } => Self::Rect { aspect_ratio },
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::HandleShape> for HandleShape {
    #[inline]
    fn from(value: egui::style::HandleShape) -> Self {
        match value {
            egui::style::HandleShape::Circle => Self::Circle,
            egui::style::HandleShape::Rect { aspect_ratio } => Self::Rect { aspect_ratio },
        }
    }
}

#[cfg(feature = "manager")]
impl From<TextCursorStyle> for egui::style::TextCursorStyle {
    #[inline]
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

#[cfg(feature = "manager")]
impl From<egui::style::TextCursorStyle> for TextCursorStyle {
    #[inline]
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

#[cfg(feature = "manager")]
impl From<ScrollFadeStyle> for egui::style::ScrollFadeStyle {
    #[inline]
    fn from(value: ScrollFadeStyle) -> Self {
        Self {
            strength: value.strength,
            size: value.size,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::ScrollFadeStyle> for ScrollFadeStyle {
    #[inline]
    fn from(value: egui::style::ScrollFadeStyle) -> Self {
        Self {
            strength: value.strength,
            size: value.size,
        }
    }
}

#[cfg(feature = "manager")]
impl From<ScrollAnimation> for egui::style::ScrollAnimation {
    #[inline]
    fn from(value: ScrollAnimation) -> Self {
        Self {
            points_per_second: value.points_per_second,
            duration: value.duration,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::style::ScrollAnimation> for ScrollAnimation {
    #[inline]
    fn from(value: egui::style::ScrollAnimation) -> Self {
        Self {
            points_per_second: value.points_per_second,
            duration: value.duration,
        }
    }
}

#[cfg(feature = "manager")]
impl From<TextStyle> for egui::TextStyle {
    #[inline]
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

#[cfg(feature = "manager")]
impl From<egui::TextStyle> for TextStyle {
    #[inline]
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
