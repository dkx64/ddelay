use nih_plug::prelude::*;
use ringbuf::rb;
use std::sync::Arc;
use vizia_plug::{vizia::vg::{luma_color_filter::new, Vector}, ViziaState};
mod editor;
use circular_buffer::*;

const DELAY_SAMPLES: usize = 100000;

pub fn delay(
    istime: bool,
    sample: &mut f32,
    feedback: f32,
    delay_time: f32,
    sample_rate: f32,
    tempo: f64,
    circbuf: &mut CircularBuffer<DELAY_SAMPLES, f32>,
) {
    if istime {
        let delay_samples: usize = (delay_time*sample_rate) as usize;
        *sample += feedback*(circbuf.get(DELAY_SAMPLES-delay_samples-100).unwrap());
        circbuf.push_back(*sample);
    }  else {
        let bps: f32 = (120f64/tempo) as f32;
        let delay_samples: usize = (48000.0*bps) as usize;
        *sample += (feedback*circbuf.get(DELAY_SAMPLES-delay_samples).unwrap());
        circbuf.push_back(*sample);
    }
}

struct AudioPlugin {
    params: Arc<AudioPluginParams>,
    circbuf: CircularBuffer<DELAY_SAMPLES, f32>
}

#[derive(Params)]
struct AudioPluginParams {
    #[persist = "editor-state"]
    editor_state:  Arc<ViziaState>,

    #[id="bytime"]
    pub istime: BoolParam,

    #[id= "time"]
    pub time: FloatParam,

    #[id="feedback"]
    pub feedback: FloatParam,
}

impl Default for AudioPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(AudioPluginParams::default()),
            circbuf: CircularBuffer::<DELAY_SAMPLES, f32>::new()

        }
    }
}

impl Default for AudioPluginParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            istime:BoolParam::new("Timed", true),
            time: FloatParam::new(
                "Delay Time",
                0.5,
                FloatRange::Linear { min: 0.2, max: 2.0 }
            ),
            feedback: FloatParam::new("Feedback", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
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
                    delay(
                        self.params.istime.value(),
                        sample, self.params.feedback.smoothed.next(),
                        self.params.time.smoothed.next(), context.transport().sample_rate,
                        context.transport().tempo.unwrap(),
                        &mut self.circbuf);

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
