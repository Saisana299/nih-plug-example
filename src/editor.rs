use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::NihPlugExampleParams;

#[derive(Lens)]
struct Data {
    params: Arc<NihPlugExampleParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (200, 150))
}

pub(crate) fn create(
    params: Arc<NihPlugExampleParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
            //peak_meter: peak_meter.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Gain GUI")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Thin)
                .font_size(30.0)
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            Label::new(cx, "Gain");
            ParamSlider::new(cx, Data::params, |params| &params.gain);

            // PeakMeter::new(
            //     cx,
            //     Data::peak_meter
            //         .map(|peak_meter| util::gain_to_db(peak_meter.load(Ordering::Relaxed))),
            //     Some(Duration::from_millis(600)),
            // )
            // This is how adding padding works in vizia
            //.top(Pixels(10.0));
        })
        .background_color(Color::rgb(100, 100, 100))
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}