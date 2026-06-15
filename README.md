# AetherSound — Deterministic Procedural Audio Core

**100% deterministic audio synthesis for multiplayer and spatial environments.**

Integer phase accumulation (u64 wrapping) eliminates stochastic drift across clients, runs, and platforms. Same Emotional Telemetry Vector → bit-identical PCM output every time.

This is the foundation for zero-drift multiplayer audio, compute-efficient ITD spatialization at synthesis time, and zero-disk procedural generation.

## Core Guarantee (Verified)

### Python Verifier (Colab / zero install)
Run `verifier.py`:

```bash
python verifier.py
```

Expected:
```
Buffers are bit-identical: ✅ YES
```

### Rust Verifier (production path)
`verifier.rs` is the same logic in Rust. Compile and run:

```bash
rustc verifier.rs -O -o verifier && ./verifier
```

Both produce bit-identical PCM from the same Emotional Telemetry Vector. This is the multiplayer state sync guarantee.

## Visual Diagrams

Three production-grade visuals are ready for outreach and documentation:

1. **UE5 MetaSound Node** — Shows exactly how the custom AetherSound operator appears in Unreal Engine 5 with Tension/Brightness/Agitation inputs and PCM + JSON outputs.
2. **Determinism Proof** — Side-by-side comparison: traditional IEEE 754 floating-point drift vs AetherSound u64 integer phase accumulation (with the exact formulas).
3. **ITD Spatialization** — Psychoacoustic Interaural Time Difference generated at the oscillator level (Woodworth model, microsecond-precise, no convolution cost).

High-resolution versions available on request or included with licensing discussions.

## Technical Foundation

- Master Integer Clock + u64 phase step: `ΔΦ = floor(f · 2⁶⁴ / fs)`
- Phase update: `Φₙ = (Φₙ₋₁ + ΔΦ) mod 2⁶⁴`
- Emotional Telemetry Vectors (tension, brightness, agitation) map directly to deterministic waveform parameters
- ITD spatialization generated at synthesis time (no post-process convolution)

## Integration Path

- Rust core + strict C-FFI (dynamic libs for any C++ engine)
- UE5 MetaSound Operator (non-blocking wrapper, JSON state sync)
- Python/FastAPI zero-disk streaming bridge (Unix pipe model)
- Deterministic JSON state packets for low-bandwidth multiplayer

## Why Studios Need This Now

**Competitive Multiplayer (Riot, Epic, Respawn)**: Peeker's advantage and audio desync ruin fairness. One telemetry vector from the server renders identically on every client.

**High-Fidelity Mobile (Tencent, NetEase, Supercell)**: Infinite variation, zero added .wav files, lower battery draw, stays under app store size caps.

**VR / XR (Meta Reality Labs, Skydance)**: Accurate 3D spatial cues at wave generation time instead of expensive post-process HRTF convolution. Frees GPU/CPU for rendering.

## Current Status (June 2026)

- Determinism verifiers (Python + Rust) — proven and committed
- Rust core + C-FFI — in active hardening
- UE5 MetaSound node — in integration
- Full compiled plugin + commercial licensing — contact for early access

**Even The Odds Foundry**  
Jay Sanders (Jacarri Sanders)  
eventheoddsfoundry@gmail.com | (530) 315-3784  
X: @GirthyLengths95

This repository exists so technical audio leads and engine architects can verify the core claim in under two minutes. No gatekeepers. No fluff. Just the math and the code.
