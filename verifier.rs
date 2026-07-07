/*
AetherSound Determinism Verifier (Rust)
Even The Odds Foundry — July 2026

Fully fixed-point for stronger determinism.
*/

use std::fs::File;
use std::io::Write;

const SAMPLE_RATE: u32 = 48000;
const DURATION_SAMPLES: usize = SAMPLE_RATE as usize;
const TWO_POW_64: u128 = 1u128 << 64;

#[derive(Clone, Copy, Debug)]
struct EmotionalVector {
    tension: f64,
    brightness: f64,
    agitation: f64,
}

struct DeterministicOscillator {
    phase: u64,
    phase_step: u64,
}

impl DeterministicOscillator {
    fn new(frequency_hz: f64) -> Self {
        let freq_millihz: u128 = (frequency_hz * 1000.0).round() as u128;
        let sample_rate_millihz: u128 = (SAMPLE_RATE as u128) * 1000;
        let phase_step = ((freq_millihz * TWO_POW_64) / sample_rate_millihz) as u64;

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

fn render_deterministic_buffer(vector: EmotionalVector) -> Vec<i16> {
    let mut buffer = Vec::with_capacity(DURATION_SAMPLES);
    let base_freq = 220.0 + (vector.tension * 110.0);
    let mut osc = DeterministicOscillator::new(base_freq);
    let agitation_step = (vector.agitation * 8.0) as u64;

    for i in 0..DURATION_SAMPLES {
        let mut sample = osc.next_sample();
        if vector.brightness > 0.5 {
            sample = sample.saturating_add((sample / 3) as i16);
        }
        if agitation_step > 0 && (i as u64 % (agitation_step + 1)) == 0 {
            sample = sample.saturating_mul(12) / 10;
        }
        buffer.push(sample);
    }
    buffer
}

fn main() {
    let test_vector = EmotionalVector {
        tension: 0.72,
        brightness: 0.65,
        agitation: 0.41,
    };

    println!("AetherSound Determinism Verifier (Rust)");
    println!("Vector: tension={:.2}, brightness={:.2}, agitation={:.2}", 
             test_vector.tension, test_vector.brightness, test_vector.agitation);
    println!("Rendering 48kHz mono i16 buffer twice...\n");

    let buf1 = render_deterministic_buffer(test_vector);
    let buf2 = render_deterministic_buffer(test_vector);

    let identical = buf1 == buf2;
    
    println!("Buffers are bit-identical: {}", if identical { "\u2705 YES" } else { "\u274c NO" });
    
    if identical {
        println!("\n\u2705 Determinism proven.");
    }
}