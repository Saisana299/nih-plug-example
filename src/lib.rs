use nih_plug::prelude::*;
use std::{f32::consts::PI, sync::Arc};
use nih_plug_iced::IcedState;

mod editor;

struct NihPlugExample {
    params: Arc<NihPlugExampleParams>,
    sample_rate: f32,

    // フィルタ係数
    a0: f32,
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32,

    // 入出力キャッシュ
    in1: [f32; 2],
    in2: [f32; 2],
    out1: [f32; 2],
    out2: [f32; 2],
}

#[derive(Params)]
struct NihPlugExampleParams {
    #[persist = "editor_state"]
    editor_state: Arc<IcedState>,

    #[id = "gain"] // ゲイン
    pub gain: FloatParam,

    #[id = "cutoff"] // カットオフ周波数
    pub cutoff: FloatParam,

    #[id = "resonance"] // Q値
    pub resonance: FloatParam,

    #[id = "bypass"] // バイパス
    pub bypass: BoolParam,
}

impl Default for NihPlugExample {
    fn default() -> Self {
        Self {
            params: Arc::new(NihPlugExampleParams::default()),
            sample_rate: 1.0,

            a0: 1.0,
            a1: 0.0,
            a2: 0.0,
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,

            in1: [0.0; 2],
            in2: [0.0; 2],
            out1: [0.0; 2],
            out2: [0.0; 2],
        }
    }
}

impl Default for NihPlugExampleParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::SymmetricalSkewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(12.0),
                    factor: FloatRange::skew_factor(0.0),
                    center: util::db_to_gain(0.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            cutoff: FloatParam::new(
                "Cutoff",
                500.0,
                FloatRange::Skewed {
                    min: 10.0,
                    max: 20000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit("")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),

            resonance: FloatParam::new(
                "Resonance",
                1.0,
                FloatRange::SymmetricalSkewed {
                    min: 0.1,
                    max: 30.0,
                    factor: FloatRange::skew_factor(-1.0),
                    center: 1.0,
                },
            )
            .with_unit("")
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            bypass: BoolParam::new("Bypass", false),
        }
    }
}

impl Plugin for NihPlugExample {
    const NAME: &'static str = "NIH-plug Example";
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

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;

        true
    }

    fn reset(&mut self) {
        self.in1 = [0.0; 2];
        self.in2 = [0.0; 2];
        self.out1 = [0.0; 2];
        self.out2 = [0.0; 2];
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {

        if self.params.bypass.value() {
            return ProcessStatus::Normal;
        }

        // パラメータの値を取得
        let cutoff = self.params.cutoff.smoothed.next();
        let resonance = self.params.resonance.smoothed.next();

        // フィルタ係数を更新
        self.update_lowpass(cutoff, resonance);

        for channel_samples in buffer.iter_samples() {
            for (channel, sample) in channel_samples.into_iter().enumerate() {
                if channel == 0 || channel == 1 {
                    *sample = self.process_lowpass(*sample, channel);
                    *sample *= self.params.gain.smoothed.next();
                }
            }
        }

        ProcessStatus::Normal
    }
}

impl NihPlugExample {
    fn update_lowpass(&mut self, cutoff: f32, resonance: f32) {
        let omega = 2.0 * PI * cutoff / self.sample_rate;
        let alpha = (omega.sin()) / (2.0 * resonance);

        self.a0 = 1.0 + alpha;
        self.a1 = -2.0 * omega.cos();
        self.a2 = 1.0 - alpha;
        self.b0 = (1.0 - omega.cos()) / 2.0;
        self.b1 = 1.0 - omega.cos();
        self.b2 = (1.0 - omega.cos()) / 2.0;
    }

    fn process_lowpass(&mut self, sample: f32, channel: usize) -> f32 {
        let input = sample;
        let output = self.b0 / self.a0 * input + self.b1 / self.a0 * self.in1[channel] + self.b2 / self.a0 * self.in2[channel]
            - self.a1 / self.a0 * self.out1[channel] - self.a2 / self.a0 * self.out2[channel];

        self.in2[channel] = self.in1[channel];
        self.in1[channel] = input;
        self.out2[channel] = self.out1[channel];
        self.out1[channel] = output;

        return output;
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
