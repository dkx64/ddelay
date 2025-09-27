use nih_plug::prelude::*;
use std::sync::Arc;
use vizia_plug::ViziaState;

mod editor;

struct AudioPlugin {
    params: Arc<AudioPluginParams>,

}

#[derive(Params)]
struct AudioPluginParams {
    #[persist = "editor-state"]
    editor_state:  Arc<ViziaState>,
}

impl Default for AudioPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(AudioPluginParams::default()),
        }
    }
}

impl Default for AudioPluginParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
        }
    }
}

impl Plugin for AudioPlugin {
    const VENDOR: &'static str = env!("CARGO_PKG_AUTHORS");
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "info@example.com";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default()
        }
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
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

    fn process(
            &mut self,
            buffer: &mut Buffer,
            aux: &mut AuxiliaryBuffers,
            context: &mut impl ProcessContext<Self>,
        ) -> ProcessStatus {
            for channel_samples in buffer.iter_samples() {
                for sample in channel_samples {
                }
            }

            ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl ClapPlugin for AudioPlugin {
    const CLAP_ID: &'static str = "com.dkdsp.oj";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A plugin template.");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for AudioPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"orangejuiceffect";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(AudioPlugin);
nih_export_vst3!(AudioPlugin);
