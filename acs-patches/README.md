# ACS Patches

This mod contains some quality-of-life patches for Assassin's Creed Syndicate to make the gameplay feel better, especially by disabling mouse/camera smoothing.

## Features

- **Disable mouse smoothing:** Makes the mouse feel more responsive.
- **Disable integrity checks:** Allows modifications to the game binary.

## Installation

1. Download and extract the latest mod files (`acs-patches-aio.zip`)
2. Copy **all** extracted files and folders (`dinput8.dll`, `dinput8.ini`, `plugins/...`) to your game folder (`Assassin's Creed Syndicate/`)
3. Run the game and enjoy!

The mod files contain an ASI loader that will automatically load the mod DLL when launching the game. If the `dinput8.dll` file conflicts
with any other mod, you can choose an alternative DLL [here](https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases/latest/).

## Configuration

You can configure the mod by placing a `acs_patches.toml` file next to `acs_patches.asi` in the `plugins` folder.
An example configuration file can be found [here](./config/acs_patches.toml).

## Credits

- [libmem by rdbo](https://github.com/rdbo/libmem)
- [Ultimate-ASI-Loader by ThirteenAG](https://github.com/ThirteenAG/Ultimate-ASI-Loader)
