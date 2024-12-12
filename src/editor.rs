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
    ViziaState::new(|| (320, 290))
}

pub(crate) fn create(
    params: Arc<NihPlugExampleParams>,
    peak_meter: Arc<AtomicF32>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_regular(cx);

        Data {
            params: params.clone(),
            peak_meter: peak_meter.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "LPF GUI")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Regular)
                .font_size(30.0)
                .color(Color::black())
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            Label::new(cx, "Cutoff").color(Color::black()).child_top(Pixels(10.0));
            ParamSlider::new(cx, Data::params, |params| &params.cutoff)
                .color(Color::black())
                .background_color(Color::rgb(225, 225, 225));

            Label::new(cx, "Resonance").color(Color::black()).child_top(Pixels(10.0));
            ParamSlider::new(cx, Data::params, |params| &params.resonance)
                .color(Color::black())
                .background_color(Color::rgb(225, 225, 225));

            Label::new(cx, "Gain").color(Color::black()).child_top(Pixels(10.0));
            ParamSlider::new(cx, Data::params, |params| &params.gain)
                .color(Color::black())
                .background_color(Color::rgb(225, 225, 225));

            PeakMeter::new(
                cx,
                Data::peak_meter
                    .map(|peak_meter| util::gain_to_db(peak_meter.load(Ordering::Relaxed))),
                Some(Duration::from_millis(600)),
            )
            .top(Pixels(10.0));
        })
        .background_color(Color::rgb(236, 236, 236))
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));

        // ResizeHandle::new(cx);
    })
}