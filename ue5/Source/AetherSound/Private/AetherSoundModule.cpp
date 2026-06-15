// AetherSound UE5 MetaSound Module
// Even The Odds Foundry — June 2026

#include "AetherSoundModule.h"

#define LOCTEXT_NAMESPACE "AetherSound"

void FAetherSoundModule::StartupModule()
{
    UE_LOG(LogTemp, Warning, TEXT("AetherSound Module Started - Deterministic Audio Engine Ready"));
}

void FAetherSoundModule::ShutdownModule()
{
    UE_LOG(LogTemp, Warning, TEXT("AetherSound Module Shutdown"));
}

#undef LOCTEXT_NAMESPACE

IMPLEMENT_MODULE(FAetherSoundModule, AetherSound)
