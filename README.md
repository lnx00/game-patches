# Game Patches

This repository contains a set of quality-of-life patches for several PC games. Each crate targets one game and has its own installation and configuration details.

## Games

- [Assassin's Creed Unity](./x64/acu-patches/)
- [Assassin's Creed Syndicate](./x64/acs-patches/)
- [Middle-earth: Shadow of Mordor](./x64/mesom-patches/)
- [Middle-earth: Shadow of War](./x64/mesow-patches/)
- [Watch Dogs](./x64/wd-patches/)
- [Far Cry 3](./x86/fc3-patches/)
- [Far Cry 3 - Blood Dragon](./x86/fc3bd-patches/)
- [State of Decay: YOSE](./x86/sodyose-patches/)
- [Lego Star Wars: The Skywalker Saga](./x64/lswtss-patches/)

## Workspace

The repository is managed as a Rust workspace. Build everything with:

```bash
cargo build --workspace
```

or build a specific project with:

```bash
cargo build -p acu-patches
```
