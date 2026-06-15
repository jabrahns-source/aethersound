// AetherSound UE5 MetaSound Module
// Even The Odds Foundry — June 2026
// Deterministic emotional audio synthesis for Unreal Engine 5

#pragma once

#include "CoreMinimal.h"
#include "Modules/ModuleManager.h"

class FAetherSoundModule : public IModuleInterface
{
public:
    virtual void StartupModule() override;
    virtual void ShutdownModule() override;
};
