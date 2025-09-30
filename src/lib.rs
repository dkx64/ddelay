use nih_plug::prelude::*;
use std::sync::Arc;
use vizia_plug::ViziaState;
mod editor;
mod delay;
use circular_buffer::*;

use crate::delay::DELAY_SAMPLES;



pub struct AudioPlugin {
    params: Arc<AudioPluginParams>,
    circbuf: CircularBuffer<{delay::DELAY_SAMPLES}, f32>
}

#[derive(Params)]
struct AudioPluginParams {
    #[persist = "editor-state"]
    editor_state:  Arc<ViziaState>,

    #[id="bytime"]
    pub istime: BoolParam,

    #[id= "time"]
    pub time: IntParam,

    #[id="feedback"]
    pub feedback: FloatParam,
}

impl Default for AudioPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(AudioPluginParams::default()),
            circbuf: CircularBuffer::<{delay::DELAY_SAMPLES}, f32>::new()

        }
    }
}

impl Default for AudioPluginParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            istime:BoolParam::new("Timed", true),
            time: IntParam::new(
                "Delay Time",
                6,
                IntRange::Linear { min: 2, max: 9 }
            ),
            feedback: FloatParam::new(
                "Feedback",
                0.3,
                FloatRange::Linear
                { min: 0.0, max: 1.0 }
            )
            .with_unit("%")
            .with_step_size(0.02)
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

    fn initialize(
            &mut self,
            audio_io_layout: &AudioIOLayout,
            buffer_config: &BufferConfig,
            context: &mut impl InitContext<Self>,
        ) -> bool {
            self.circbuf.fill(0.0);
        true
    }

    fn process(
            &mut self,
            buffer: &mut Buffer,
            aux: &mut AuxiliaryBuffers,
            context: &mut impl ProcessContext<Self>,
        ) -> ProcessStatus {
            let feedback = self.params.feedback.smoothed.next();
            for channel_samples in buffer.iter_samples() {
                for sample in channel_samples {
                    if 2.0*((120f64/context.transport().tempo.unwrap()) as f32*context.transport().sample_rate) < DELAY_SAMPLES as f32
                    {
                        delay::delay(
                            self.params.istime.value(),
                            sample, self.params.feedback.smoothed.next(),
                            self.params.time.smoothed.next(), context.transport().sample_rate,
                            context.transport().tempo.unwrap(),
                            delay::DELAY_SAMPLES,
                            &mut self.circbuf,
                        );
                    }

                }
            }


            ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl ClapPlugin for AudioPlugin {
    const CLAP_ID: &'static str = "com.dkdsp.dplug";
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
