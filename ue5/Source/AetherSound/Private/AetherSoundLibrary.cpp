// AetherSound Rust Core Binding - Implementation
// Even The Odds Foundry — June 2026

#include "AetherSoundLibrary.h"

bool FAetherSoundLibrary::Initialize(float Tension, float Brightness, float Agitation)
{
    int32 Result = aether_create(
        FMath::Clamp(static_cast<double>(Tension), 0.0, 1.0),
        FMath::Clamp(static_cast<double>(Brightness), 0.0, 1.0),
        FMath::Clamp(static_cast<double>(Agitation), 0.0, 1.0)
    );
    return Result == 0;
}

bool FAetherSoundLibrary::RenderFrame(TArray<int16>& OutBuffer, uint32 NumSamples)
{
    OutBuffer.SetNum(NumSamples, false);
    int32 Result = aether_render(OutBuffer.GetData(), NumSamples);
    return Result == 0;
}

bool FAetherSoundLibrary::Shutdown()
{
    int32 Result = aether_destroy();
    return Result == 0;
}
