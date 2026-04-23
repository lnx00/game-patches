# ACU Patches

This mod contains some quality-of-life patches to make the gameplay feel better, especially by disabling mouse/camera smoothing and fixing the mouse sensitivity at high frame rates.

## Features

- **Disable mouse smoothing:** Makes the mouse feel more responsive.
- **Mouse sensitivity fix:** Fixes the mouse sensitivity being tied to the framerate.
- **Disable integrity checks:** Allows modifications to the game binary.

## Installation

1. Download and extract the latest mod files (`acu-patches-aio.zip`)
2. Copy **all** extracted files and folders (`dinput8.dll`, `dinput8.ini`, `plugins/...`) to your game folder (`Assassin's Creed Unity/`)
3. Run the game and enjoy!

The mod files contain an ASI loader that will automatically load the mod DLL when launching the game. If the `dinput8.dll` file conflicts
with any other mod, you can choose an alternative DLL [here](https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases/latest/).

## Configuration

You can configure the mod by placing a `acu_patches.toml` file next to `acu_patches.asi` in the `plugins` folder.
An example configuration file can be found [here](./config/acu_patches.toml).

## Credits

- [libmem by rdbo](https://github.com/rdbo/libmem)
- [ACUFixes by NameTaken3125](https://github.com/NameTaken3125/ACUFixes)
- [Ultimate-ASI-Loader by ThirteenAG](https://github.com/ThirteenAG/Ultimate-ASI-Loader)
