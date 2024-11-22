use nih_plug::prelude::*;
use std::sync::Arc;

// [ENG]This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

// [JP] これはコメントの多くが削除されたゲイン例の短縮版です。開始するには
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs をチェックしてください

struct NihPlugExample {
    params: Arc<NihPlugExampleParams>,
}

#[derive(Params)]
struct NihPlugExampleParams {
    /// [ENG] The parameter's ID is used to identify the parameter in the wrappred plugin API. As long as
    /// these IDs remain constant, you can rename and reorder these fields as you wish. The
    /// parameters are exposed to the host in the same order they were defined. In this case, this
    /// gain parameter is stored as linear gain while the values are displayed in decibels.

    /// [JP] このパラメーターのIDは、ラップされたプラグインAPIでパラメーターを識別するために使用されます。
    /// これらのIDが一定である限り、これらのフィールドの名前を変更したり順序を変更したりすることができます。
    /// パラメーターは定義された順序でホストに公開されます。この場合、このゲインパラメーターは線形ゲインとして保存されていますが、
    /// 値はデシベルで表示されます。
    #[id = "gain"]
    pub gain: FloatParam,
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
            // [ENG] This gain is stored as linear gain. NIH-plug comes with useful conversion functions
            // to treat these kinds of parameters as if we were dealing with decibels. Storing this
            // as decibels is easier to work with, but requires a conversion for every sample.

            // [JP] このゲインは線形ゲインとして保存されています。NIH-plugには、
            // これらの種類のパラメータをデシベルを扱うようにする便利な変換関数が備わっています。
            // デシベルとして保存する方が扱いやすいですが、各サンプルごとに変換が必要です。
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    // [ENG] This makes the range appear as if it was linear when displaying the values as decibels
                    // [JP] デシベルとして値を表示するときに範囲が線形のように見えるようにします
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            // [ENG] Because the gain parameter is stored as linear gain instead of storing the value as
            // decibels, we need logarithmic smoothing

            // [JP] ゲインパラメーターは線形ゲインとして保存されているため、値をデシベルとして保存する代わりに、
            // 対数的なスムージングが必要です
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            // [ENG] There are many predefined formatters we can use here. If the gain was stored as
            // decibels instead of as a linear gain value, we could have also used the
            // `.with_step_size(0.1)` function to get internal rounding.

            // [JP] ここでは多くの事前定義されたフォーマッターを使用できます。
            // ゲインが線形ゲイン値として保存されている場合、内部の丸め処理を得るために
            // `.with_step_size(0.1)` 関数も使用できます。
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for NihPlugExample {
    const NAME: &'static str = "Nih Plug Example";
    const VENDOR: &'static str = "Saisana299";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "your@email.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // [ENG] The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.

    // [JP] 最初のオーディオIOレイアウトがデフォルトとして使用されます。他のレイアウトはプラグインAPI/バックエンドによって
    // 明示的にまたは自動的にホストまたはユーザーによって選択される場合があります。
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // [ENG] Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.

        // [JP] 個々のポートおよびレイアウト全体に名前を付けることができます。デフォルトでは、これらの名前は必要に応じて生成されます。
        // このレイアウトは「Stereo」と呼ばれ、入力チャネルと出力チャネルが1つだけのレイアウトは「Mono」と呼ばれます。
        names: PortNames::const_default(),
    }];


    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // [ENG] If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.

    // [JP] プラグインがSysExメッセージを送受信できる場合、ここでそれらのメッセージをラップするタイプを定義できます。
    // このタイプは、プレーンなバイトバッファへの変換を可能にする `SysExMessage` トレイトを実装します。
    type SysExMessage = ();
    // [ENG] More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.

    // [JP] より高度なプラグインは、これを使用して高価なバックグラウンドタスクを実行できます。フィールドの
    // ドキュメントを参照してください。 `()` は、プラグインにバックグラウンドタスクがないことを意味します。
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
        // [ENG] Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.

        // [JP] バッファのサイズ変更やその他の潜在的に高価な初期化操作をここで実行します。
        // この関数の直後に `reset()` 関数が常に呼び出されます。この関数が不要な場合は削除できます。
        true
    }

    fn reset(&mut self) {
        // [ENG] Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.

        // [JP] バッファとエンベロープをここでリセットします。これはオーディオスレッドから呼び出されることがあり、
        // メモリを割り当てないようにする必要があります。この関数が不要な場合は削除できます。
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            // [ENG] Smoothing is optionally built into the parameters themselves
            // [JP] スムージングはオプションでパラメーター自体に組み込まれています
            let gain = self.params.gain.smoothed.next();

            for sample in channel_samples {
                *sample *= gain;
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

    // [ENG] Don't forget to change these features
    // [JP] 機能を変更するのを忘れないでください
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for NihPlugExample {
    const VST3_CLASS_ID: [u8; 16] = *b"Exactly16Chars!!";

    // [ENG] And also don't forget to change these categories
    // [JP] これらのカテゴリを変更するのを忘れないでください
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(NihPlugExample);
nih_export_vst3!(NihPlugExample);
