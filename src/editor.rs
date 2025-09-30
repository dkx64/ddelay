use atomic_float::AtomicF32;
use nih_plug::prelude::{util, Editor};
use nih_plug::wrapper::vst3::vst3_sys::vst::kStringStereoBFS;
use vizia_plug::vizia::{prelude::*, vg};
use vizia_plug::vizia::vg::paint;
use vizia_plug::widgets::*;
use vizia_plug::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use crate::AudioPluginParams;

pub const NOTO_SANS: &str = "Noto Sans";

#[derive(Lens)]
struct Data {
    params: Arc<AudioPluginParams>
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (400,400))
}

pub(crate) fn create(
    params: Arc<AudioPluginParams>,
    editor_state: Arc<ViziaState>
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _|  {
        Data {
            params: params.clone(),
        }
        .build(cx);
        cx.add_font_mem(include_bytes!("../assets/SwanseaBoldItalic-p3Dv.ttf"));
        cx.add_font_mem(include_bytes!("../assets/SwanseaBold-D0ox.ttf"));
        cx.add_stylesheet(include_style!("assets/style.css"))
            .expect("Failed to load stylesheet");

        VStack::new(cx, |cx| {
            Label::new(cx, "ddelay")
                .font_size(30.0)
                .height(Pixels(50.0))
                .font_slant(FontSlant::Italic);
            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    ParamSlider::new(cx, Data::params, |params| &params.feedback).class("knob");
                    Label::new(cx, "Feedback");
                })
                .class("knob-widget")
                .alignment(Alignment::Center);
                VStack::new(cx, |cx| {
                    ParamSlider::new(cx, Data::params, |params| &params.time)
                        .class("knob");
                    Label::new(cx, "Delay Time");
                })
                .class("knob-widget")
                .alignment(Alignment::Center);
            })
            .padding_top(Pixels(50.0))
            .alignment(Alignment::Center);
            ParamButton::new(cx, Data::params, |params| &params.istime)
                .class("button-widget");
        })
        .font_weight(FontWeightKeyword::SemiBold)
        .alignment(Alignment::Center)
        .class("body");


    })
}
