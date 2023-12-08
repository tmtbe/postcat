use eframe::emath::Align;
use eframe::epaint::text::LayoutJob;
use egui::{FontSelection, Id, InnerResponse, RichText, Style, Ui, Widget, WidgetText};

use crate::data::Request;
use crate::panels::HORIZONTAL_GAP;
use crate::widgets::selectable_value_with_close_button::{
    ExResponse, SelectableLabelWithCloseButton,
};

pub fn build_rest_ui_header(request: Request, ui: &Ui) -> LayoutJob {
    let mut lb = LayoutJob::default();
    let style = Style::default();
    if request.base_url != "" {
        RichText::new(request.method.to_string() + " ")
            .color(ui.visuals().warn_fg_color)
            .strong()
            .append_to(&mut lb, &style, FontSelection::Default, Align::Center);
        RichText::new(request.base_url.to_string())
            .color(ui.visuals().text_color())
            .append_to(&mut lb, &style, FontSelection::Default, Align::Center);
    } else {
        RichText::new("Untitled Request")
            .strong()
            .color(ui.visuals().text_color())
            .append_to(&mut lb, &style, FontSelection::Default, Align::Center);
    }
    lb
}

pub fn selectable_label_with_close_button<Value: PartialEq>(
    ui: &mut Ui,
    current_value: &mut Value,
    selected_value: Value,
    text: impl Into<WidgetText>,
) -> ExResponse {
    let mut ex_response =
        _selectable_label_with_close_button(ui, *current_value == selected_value, text);
    if ex_response.clicked() && *current_value != selected_value {
        *current_value = selected_value;
        ex_response.response().mark_changed();
    }
    ex_response
}

#[must_use = "You should check if the user clicked this with `if ui.selectable_label(…).clicked() { … } "]
fn _selectable_label_with_close_button(
    ui: &mut Ui,
    checked: bool,
    text: impl Into<WidgetText>,
) -> ExResponse {
    SelectableLabelWithCloseButton::new(checked, text).ui(ui)
}

pub fn build_with_count_ui_header(name: String, count: usize, ui: &Ui) -> LayoutJob {
    let mut lb = LayoutJob::default();
    let style = Style::default();
    RichText::new(name + " ")
        .color(ui.visuals().text_color())
        .strong()
        .append_to(&mut lb, &style, FontSelection::Default, Align::Center);
    if count > 0 {
        RichText::new("(".to_string() + count.to_string().as_str() + ")")
            .color(ui.visuals().warn_fg_color)
            .strong()
            .append_to(&mut lb, &style, FontSelection::Default, Align::Center);
    }
    lb
}

pub fn left_right_panel(
    ui: &mut Ui,
    left_id: impl Into<Id>,
    left: impl FnOnce(&mut Ui),
    right_id: impl Into<Id>,
    right: impl FnOnce(&mut Ui),
) -> InnerResponse<()> {
    ui.horizontal(|ui| {
        egui::SidePanel::right(right_id)
            .resizable(true)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                right(ui);
            });
        egui::SidePanel::left(left_id)
            .resizable(true)
            .min_width(ui.available_width() - HORIZONTAL_GAP * 2.0)
            .show_inside(ui, |ui| {
                left(ui);
            });
    })
}