# Branch: main

**Purpose:** Primary development branch

_Commits will be appended below._

## Commit 6a5a32b0 — 2026-07-17 13:48 UTC

### Branch Purpose
Primary development branch

### Previous Progress Summary


### This Commit's Contribution
Re-tagged after macos-13 fix (PR9, squash 84e1a12). Release run 29585121343 green; assets: linux-x86_64, mac x86_64+aarch64, windows zip. Verified: latest-API resolves v0.1.0, asset downloads, tar layout matches install.sh extraction. Gap found: this dev machine is linux-aarch64 (WSL2 on ARM) - no prebuilt binary for it; ubuntu-24.04-arm runner could add aarch64-unknown-linux-gnu in v0.1.1.

---

