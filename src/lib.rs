use nih_plug::prelude::*;
use std::sync::Arc;

struct NihPlugExample {
    params: Arc<NihPlugExampleParams>,
}

#[derive(Params)]
struct NihPlugExampleParams {
    // todo
}

impl Default for NihPlugExample {
    fn default() -> Self {
        Self {
            params: Arc::new(NihPlugExampleParams::default()),
        }
    }
}

impl Default for NihPlugExampleParams {
    fn default() -> Self {
        Self {
            // todo
        }
    }
}

impl Plugin for NihPlugExample {
    const NAME: &'static str = "Nih Plug Example";
    const VENDOR: &'static str = "Saisana299";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "your@email.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];


    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn reset(&mut self) {
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {

            for _sample in channel_samples {
                // todo
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for NihPlugExample {
    const CLAP_ID: &'static str = "com.your-domain.nih-plug-example";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A short description of your plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] =
        &[ClapFeature::AudioEffect, ClapFeature::Filter, ClapFeature::Instrument];
}

impl Vst3Plugin for NihPlugExample {
    const VST3_CLASS_ID: [u8; 16] = *b"Exactly16Chars!!";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Filter, Vst3SubCategory::Instrument];
}

nih_export_clap!(NihPlugExample);
nih_export_vst3!(NihPlugExample);
