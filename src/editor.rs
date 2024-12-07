use nih_plug::prelude::{util, AtomicF32, Editor};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use crate::NihPlugExampleParams;

#[derive(Lens)]
struct Data {
    params: Arc<NihPlugExampleParams>,
    peak_meter: Arc<AtomicF32>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (300, 250))
}

pub(crate) fn create(
    params: Arc<NihPlugExampleParams>,
    peak_meter: Arc<AtomicF32>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
            peak_meter: peak_meter.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "LPF GUI")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Thin)
                .font_size(30.0)
                .color(Color::white())
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            Label::new(cx, "Cutoff").color(Color::white());
            ParamSlider::new(cx, Data::params, |params| &params.cutoff)
                .color(Color::white())
                .border_color(Color::white());

            Label::new(cx, "Resonance").color(Color::white());
            ParamSlider::new(cx, Data::params, |params| &params.resonance)
                .color(Color::white())
                .border_color(Color::white());

            Label::new(cx, "Gain").color(Color::white());
            ParamSlider::new(cx, Data::params, |params| &params.gain)
                .color(Color::white())
                .border_color(Color::white());

            PeakMeter::new(
                cx,
                Data::peak_meter
                    .map(|peak_meter| util::gain_to_db(peak_meter.load(Ordering::Relaxed))),
                Some(Duration::from_millis(600)),
            )
            // This is how adding padding works in vizia
            .top(Pixels(10.0));
        })
        .background_color(Color::rgb(40, 43, 48))
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}