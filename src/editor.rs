use atomic_float::AtomicF32;
use nih_plug::prelude::{util, Editor};
use vizia_plug::vizia::prelude::*;
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
    ViziaState::new(|| (200,150))
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

        VStack::new(cx, |cx| {
            Label::new(cx, "AudioPlugin")
                .font_family(vec![FamilyOwned::Named(String::from(NOTO_SANS))])
                .font_weight(FontWeightKeyword::Bold)
                .font_size(30.0)
                .height(Pixels(50.0))
                .alignment(Alignment::TopCenter);
            Knob::new(cx, 0.0, Data::params.map(|params| params.feedback.value()), false);
        });
    })
}
