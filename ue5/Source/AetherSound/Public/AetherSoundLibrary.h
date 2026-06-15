// AetherSound Rust Core Binding - C FFI Interface
// Even The Odds Foundry — June 2026

#pragma once

#include "CoreMinimal.h"

// C FFI declarations matching Rust core
extern "C"
{
    // Initialize synthesizer with emotional telemetry vector
    // Returns 0 on success, negative on error
    int32 aether_create(double tension, double brightness, double agitation);

    // Render audio samples into output buffer
    // output_buffer: pointer to i16 PCM samples
    // num_samples: number of samples to render
    // Returns 0 on success, negative on error
    int32 aether_render(int16* output_buffer, uint32 num_samples);

    // Clean up synthesizer resources
    // Returns 0 on success
    int32 aether_destroy();

    // Get native sample rate (always 48000 Hz)
    uint32 aether_get_sample_rate();
}

/**
 * Wrapper class for Rust AetherSound core
 * Provides C++ convenience interface for UE5
 */
class AETHERSOUND_API FAetherSoundLibrary
{
public:
    // Initialize with emotional vector (0.0 - 1.0 each)
    static bool Initialize(float Tension, float Brightness, float Agitation);

    // Render audio frame (48kHz, mono i16 PCM)
    static bool RenderFrame(TArray<int16>& OutBuffer, uint32 NumSamples);

    // Shutdown
    static bool Shutdown();

    // Get sample rate (48000)
    static uint32 GetSampleRate() { return 48000; }
};
