# AetherSound Commercial Licensing

## License Overview

AetherSound is offered under a **dual licensing model**:

1. **MIT Open Source License** — For open-source projects and non-commercial use
2. **Commercial License** — For game studios, audio software companies, and commercial products

## Commercial License Terms

### Licensee Rights

A Commercial License grants the licensee **non-exclusive** rights to:

- **Integrate** AetherSound core into commercial products (games, audio software, VR/XR applications)
- **Distribute** binaries compiled with AetherSound to end users
- **Modify** AetherSound source code for internal optimization and integration
- **Use** for unlimited concurrent users/seats within a single organization
- **Deploy** across multiple platforms (PC, console, mobile, XR)

### License Restrictions

Licensees may NOT:

- **Redistribute** AetherSound source code as a standalone library
- **Sublicense** or resell the technology to third parties
- **Remove** copyright notices or license headers
- **Compete** — Use AetherSound to create a competing audio synthesis engine for commercial redistribution

### Pricing Tiers

| Tier | Use Case | Annual Cost | Support |
|------|----------|-------------|---------|
| **Indie** | Revenue < $1M/year | $2,500 | Community (Slack) |
| **Studio** | Revenue $1M–$50M/year | $12,500 | Email (48h response) |
| **Enterprise** | Revenue > $50M/year | Custom | Dedicated engineer + consultation |
| **VR/XR Specialist** | AR/VR/spatial audio focus | $7,500 | Email + priority bug fixes |

### License Key System

All Commercial Licenses include:

- **License Key** — unique identifier for product verification
- **Platform Binaries** — precompiled optimized library for Windows, macOS, Linux, Nintendo Switch, PlayStation, Xbox (if applicable)
- **License Validation SDK** — runtime check to ensure licensing compliance
- **Support Portal** — Documentation, FAQs, and issue tracking
- **Quarterly Updates** — Bug fixes, performance improvements, new features

### Renewal & Support

- **License Term** — 1 year, automatic renewal unless cancelled
- **Support Period** — Ends with license expiration
- **Maintenance Release** — Free updates during license term
- **Major Version Upgrade** — New tier pricing applies

## Open Source (MIT License)

For **non-commercial and open-source projects**, AetherSound remains fully open under the MIT License:

- No licensing fee
- Full source code access
- No commercial use restrictions
- Attribution required

## Evaluation License

**Free 30-day evaluation** for studios and companies considering AetherSound:

- Full feature access
- Evaluation key expires after 30 days
- No support included
- Request via licensing@aethersound.io

## Compliance & Enforcement

### License Validation

Runtime license validation occurs:
- At plugin initialization (UE5, API)
- Via `AetherLicense::verify()` in C++ / `aether_license_verify()` in FFI
- Offline-capable (license key stored locally)
- No phone-home or telemetry (privacy-first)

### Audit Rights

Even Odds Foundry reserves the right to audit licensee usage annually to ensure compliance with license terms.

### Termination

Licenses terminate immediately upon:
- Non-payment of renewal fees
- Material breach of license terms
- Unlicensed redistribution of AetherSound
- Competitive use without consent

Licensee retains perpetual rights to products already distributed with valid license keys.

## Contact & Licensing Sales

**Even Odds Foundry**  
Jay Sanders (Jacarri Sanders)  
📧 licensing@aethersound.io  
📞 (530) 315-3784  
🐦 @GirthyLengths95

---

## FAQ

**Q: Can I use AetherSound in my game and distribute it?**  
A: Yes, with a Commercial License. The MIT license is for non-commercial use only.

**Q: What if my revenue crosses $1M?**  
A: Upgrade to the Studio tier ($12,500/year). Retroactive billing is waived if you upgrade within 30 days of crossing the threshold.

**Q: Can I modify AetherSound internally?**  
A: Yes. Commercial licenses allow internal modifications. You cannot redistribute modified source.

**Q: Is there a perpetual license option?**  
A: Contact licensing@aethersound.io for Enterprise custom agreements.

**Q: What if I go out of business?**  
A: Products already distributed remain licensed. No refunds for prepaid license terms.

**Q: Do I have to "phone home"?**  
A: No. License validation is local. No telemetry or analytics are collected.

