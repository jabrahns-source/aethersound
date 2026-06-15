/*
AetherSound Rust Core Library
Even The Odds Foundry — June 2026

Provides C-FFI interface for UE5 MetaSound and other engines.
100% deterministic audio synthesis via u64 phase accumulation.
*/

use std::sync::Mutex;

const SAMPLE_RATE: u32 = 48000;
const TWO_POW_64: u128 = 1u128 << 64;

#[derive(Clone, Copy, Debug)]
pub struct EmotionalVector {
    pub tension: f64,
    pub brightness: f64,
    pub agitation: f64,
}

pub struct DeterministicOscillator {
    phase: u64,
    phase_step: u64,
}

impl DeterministicOscillator {
    fn new(frequency_hz: f64) -> Self {
        let phase_step = ((frequency_hz * (TWO_POW_64 as f64)) / (SAMPLE_RATE as f64)) as u64;
        Self {
            phase: 0,
            phase_step,
        }
    }

    #[inline(always)]
    fn next_sample(&mut self) -> i16 {
        self.phase = self.phase.wrapping_add(self.phase_step);
        ((self.phase >> 48) as i16) - 16384
    }
}

pub struct AetherSynthesizer {
    oscillator: DeterministicOscillator,
    vector: EmotionalVector,
    sample_index: usize,
}

impl AetherSynthesizer {
    pub fn new(vector: EmotionalVector) -> Self {
        let base_freq = 220.0 + (vector.tension * 110.0);
        Self {
            oscillator: DeterministicOscillator::new(base_freq),
            vector,
            sample_index: 0,
        }
    }

    pub fn render_sample(&mut self) -> i16 {
        let mut sample = self.oscillator.next_sample();

        // Brightness modulation: add harmonic saturation
        if self.vector.brightness > 0.5 {
            sample = sample.saturating_add((sample / 3) as i16);
        }

        // Agitation modulation: rhythmic amplitude modulation
        let agitation_step = (self.vector.agitation * 8.0) as u64;
        if agitation_step > 0 && (self.sample_index as u64 % (agitation_step + 1)) == 0 {
            sample = sample.saturating_mul(12) / 10;
        }

        self.sample_index += 1;
        sample
    }

    pub fn render_buffer(&mut self, buffer: &mut [i16]) {
        for sample in buffer.iter_mut() {
            *sample = self.render_sample();
        }
    }
}

// Thread-safe global synthesizer for C-FFI
lazy_static::lazy_static! {
    static ref SYNTH: Mutex<Option<AetherSynthesizer>> = Mutex::new(None);
}

// C FFI Interface
#[no_mangle]
pub extern "C" fn aether_create(tension: f64, brightness: f64, agitation: f64) -> i32 {
    let vector = EmotionalVector {
        tension: tension.clamp(0.0, 1.0),
        brightness: brightness.clamp(0.0, 1.0),
        agitation: agitation.clamp(0.0, 1.0),
    };
    let mut synth = SYNTH.lock().unwrap();
    *synth = Some(AetherSynthesizer::new(vector));
    0 // Success
}

#[no_mangle]
pub extern "C" fn aether_render(output_buffer: *mut i16, num_samples: u32) -> i32 {
    if output_buffer.is_null() {
        return -1; // Error: null pointer
    }

    let mut synth = SYNTH.lock().unwrap();
    if synth.is_none() {
        return -2; // Error: synthesizer not initialized
    }

    let buffer = unsafe { std::slice::from_raw_parts_mut(output_buffer, num_samples as usize) };
    synth.as_mut().unwrap().render_buffer(buffer);
    0 // Success
}

#[no_mangle]
pub extern "C" fn aether_destroy() -> i32 {
    let mut synth = SYNTH.lock().unwrap();
    *synth = None;
    0 // Success
}

#[no_mangle]
pub extern "C" fn aether_get_sample_rate() -> u32 {
    SAMPLE_RATE
}
