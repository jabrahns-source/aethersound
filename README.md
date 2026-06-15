# AetherSound — Deterministic Procedural Audio Core

**100% deterministic audio synthesis for multiplayer and spatial environments.**

Integer phase accumulation (u64 wrapping) eliminates stochastic drift across clients, runs, and platforms. Same Emotional Telemetry Vector → bit-identical PCM output every time.

This is the foundation for zero-drift multiplayer audio, compute-efficient ITD spatialization, and zero-disk procedural generation.

## Core Guarantee (Verified)

Run the included `verifier.py`:

```bash
python verifier.py
```

Expected output:
```
Buffers are bit-identical: ✅ YES
```

This is the exact property your multiplayer clients need: the server sends one telemetry vector, every client renders the identical waveform locally with zero audio streaming and zero desync.

## Technical Foundation

- Master Integer Clock + u64 phase step: `ΔΦ = floor(f · 2⁶⁴ / fs)`
- Phase update: `Φₙ = (Φₙ₋₁ + ΔΦ) mod 2⁶⁴`
- Emotional Telemetry Vectors (tension, brightness, agitation) map directly to deterministic waveform parameters
- ITD spatialization generated at synthesis time (no post-process convolution)

## Integration Path

- Rust core (production) + C-FFI for native engines
- UE5 MetaSound Operator (in active build)
- Python/FastAPI zero-disk streaming bridge
- JSON state serialization for low-bandwidth multiplayer sync

## Why This Matters for Studios

**Competitive multiplayer** — No more audio desync in lobbies. Every client hears the same event at the same microsecond.

**Mobile / high-fidelity** — Infinite procedural variation with zero added download size and lower battery draw.

**VR / XR** — Physically accurate spatial cues at synthesis time instead of expensive post-process.

## Current Status (June 2026)

- Determinism verifier (Python) — ready and proven
- Rust core + FFI — in hardening
- UE5 MetaSound node — in integration
- Full plugin + licensing — contact for early access

**Even The Odds Foundry**  
Jay Sanders | eventheoddsfoundry@gmail.com | (530) 315-3784  
X: @GirthyLengths95

This repo exists to let technical audio leads and engine architects verify the core claim in under 2 minutes.
