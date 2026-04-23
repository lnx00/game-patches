# Game Patches

This repository contains a set of quality-of-life patches for several PC games. Each crate targets one game and has its own installation and configuration details.

## Games

- [Assassin's Creed Unity](./acu-patches/)
- [Assassin's Creed Syndicate](./acs-patches/)
- [Middle-earth: Shadow of Mordor](./mesom-patches/)
- [Middle-earth: Shadow of War](./mesow-patches/)

## Workspace

The repository is managed as a Rust workspace. Build everything with:

```bash
cargo build --workspace
```

or build a specific project with:

```bash
cargo build -p acu-patches
```
