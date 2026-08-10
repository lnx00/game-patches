# SoDYOSE Patches

Quality-of-life patches for the game "State of Decay: YOSE".

## Features

- **Disable Input Clamping:** Removes the minimum and maximum turn speed limits (also referred to as "Negative Mouse Acceleration").

## Installation

1. Download and extract the latest mod files (`sodyose-patches-aio.zip`).
2. Copy **all** extracted files and folders (`dinput8.dll`, `plugins/...`) to your game binary folder (`State of Decay - Year One/`).
3. Run the game and enjoy!

The mod files contain an ASI loader that will automatically load the mod DLL when launching the game. If the `dinput8.dll` file conflicts
with any other mod, you can choose an alternative DLL [here](https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases/latest/).

## Uninstallation

1. Navigate to your game binary folder (`State of Decay - Year One/`).
2. Delete the `dinput8.dll` file and the `plugins` folder (or just the specific `sodyose_patches` files inside it if you have other plugins).

## Configuration

You can configure the mod by placing a `sodyose_patches.toml` file next to `sodyose_patches.asi` in the `plugins` folder.
An example configuration file can be found [here](./config/sodyose_patches.toml).

## Credits

- [libmem by rdbo](https://github.com/rdbo/libmem)
- [Ultimate-ASI-Loader by ThirteenAG](https://github.com/ThirteenAG/Ultimate-ASI-Loader)
