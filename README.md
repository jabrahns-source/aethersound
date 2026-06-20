# AetherSound

**Deterministic Procedural Audio Engine** — Bit-identical synthesis for multiplayer, spatial, and sustainable compute.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/Python-3.10%2B-blue?logo=python)](https://www.python.org/)
[![UE5](https://img.shields.io/badge/UE5-MetaSound%20Plugin-5C5CFF?logo=unrealengine)](https://www.unrealengine.com/)

> Same Emotional Telemetry Vector → **bit-identical PCM** on every client, every run, every platform.

AetherSound eliminates stochastic drift in procedural audio using strict `u64` integer phase accumulation. It delivers zero-drift multiplayer synchronization, compute-efficient psychoacoustic spatialization (ITD at synthesis time), and a zero-disk architecture ideal for cloud and sustainable compute pipelines.

## Quick Proof (Run This Now)

**Python (zero install, works in Colab):**
```bash
python verifier.py
```
Expected output:
```
Buffers are bit-identical: ✅ YES
```

**Rust (production path):**
```bash
rustc verifier.rs -O -o verifier && ./verifier
```

Both produce identical 48 kHz mono i16 PCM from the same input vector. This is the multiplayer guarantee.

## Why It Matters

| Use Case                    | Problem Solved                              | Business Impact                          |
|-----------------------------|---------------------------------------------|------------------------------------------|
| Competitive Multiplayer     | Audio desync & peeker's advantage           | Fairer matches, fewer player complaints  |
| High-Fidelity Mobile        | Asset bloat + battery drain                 | Smaller downloads, longer sessions       |
| VR / XR                     | Heavy convolution spatialization cost       | Better visuals + spatial audio together  |
| Cloud / Sustainable Compute | Disk I/O + FPU power draw                   | Lower server costs + verifiable ESG data |

## Key Technical Properties

- **Deterministic by Design**: `u64` wrapping phase accumulation (`Φₙ = (Φₙ₋₁ + ΔΦ) mod 2⁶⁴`)
- **Emotional Telemetry Vectors**: Tension, Brightness, Agitation map directly to waveform parameters
- **ITD Spatialization at Source**: Microsecond-precise Interaural Time Differences generated during synthesis (no post-process)
- **Zero-Disk Architecture**: Python/FastAPI + Rust pipe model for streaming PCM with minimal RAM/disk footprint
- **Multi-Language Integration**: Strict C-FFI + UE5 MetaSound Operator

## Current Status (June 2026)

- ✅ Determinism verifiers (Python + Rust) — public and passing
- ✅ Rust core + C-FFI — hardened
- ✅ UE5 MetaSound Operator — integrated and tested
- Full compiled plugin + commercial licensing — available for early access

## Integration

See `BUILD_UE5.md` for Unreal Engine 5 MetaSound plugin build and integration instructions.

The engine exposes a simple JSON state interface for low-bandwidth multiplayer synchronization.

## Repository Structure

```
aethersound/
├── verifier.py                 # Python determinism proof
├── verifier.rs                 # Rust determinism proof
├── src/                        # Rust core + C-FFI
├── ue5/                        # UE5 MetaSound plugin
├── LICENSE
├── .gitignore
└── README.md
```

## Citation

If you use AetherSound in academic work or reference it in proposals:

```bibtex
@software{aethersound2026,
  author = {Jay Sanders},
  title  = {AetherSound: Deterministic Procedural Audio Engine},
  year   = {2026},
  url    = {https://github.com/jabrahns-source/aethersound}
}
```

## License

MIT License — see [LICENSE](LICENSE) file.

## Contact & Affiliation

**Even The Odds Foundry**  
Jay Sanders (Jacarri Sanders)  
eventheoddsfoundry@gmail.com | (530) 315-3784  
X: @GirthyLengths95

This project exists to let technical audio leads and engine architects verify the core determinism claim in under two minutes. No gatekeepers. Verifiable math and code.

---

*Star this repo if you find the determinism guarantee useful.*
*Issues and pull requests welcome.*