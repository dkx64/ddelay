use circular_buffer::CircularBuffer;

pub const DELAY_SAMPLES: usize = 250000;

pub fn delay(
    istime: bool,
    sample: &mut f32,
    feedback: f32,
    delay_time: i32,
    sample_rate: f32,
    tempo: f64,
    circular_buffer_length: usize,
    circbuf: &mut CircularBuffer<DELAY_SAMPLES, f32>,
) {
    let delay_time_s = match delay_time {
        9 => 4.0,
        8 => 3.0,
        7 => 2.0,
        6 => 1.5,
        5 => 1.0,
        4 => 0.75,
        3 => 0.5,
        2 => 0.25,
        1 => 0.1,
        0 => 0.05,
        _ => 0.0
    };
    if istime {
        let delay_samples: usize = (delay_time_s*sample_rate) as usize;
        *sample += feedback*(circbuf.get(DELAY_SAMPLES-delay_samples).unwrap());
        circbuf.push_back(*sample);
    }  else {
        let bps: f32 = (120f64/tempo) as f32;
        let delay_samples: usize = (sample_rate*delay_time_s*bps).floor() as usize;
        *sample += (feedback*circbuf.get(DELAY_SAMPLES-delay_samples).unwrap());
        circbuf.push_back(*sample);
    }
}
