# MESOM Patches

Quality-of-life patches for the game "Middle-earth: Shadow of Mordor".

## Features

- **Disable camera smoothing:** Makes the camera feel more responsive, especially when using a mouse.
- **Uniform camera speed:** Forces a 1:1 camera speed, instead of it being damped on the y-axis.
- **Mouse sensitivity fix:** (Experimental) Fixes the mouse sensitivity being tied to the framerate. Only works when FPS are being limited to ~90.

## Installation

1. Download and extract the latest mod files (`mesom-patches-aio.zip`).
2. Copy **all** extracted files and folders (`winmm.dll`, `plugins/...`) to your game binary folder (`ShadowOfMordor/x64/`).
3. Run the game and enjoy!

The mod files contain an ASI loader that will automatically load the mod DLL when launching the game. If the `winmm.dll` file conflicts
with any other mod, you can choose an alternative DLL [here](https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases/latest/).

## Uninstallation

1. Navigate to your game binary folder (`ShadowOfMordor/x64/`).
2. Delete the `winmm.dll` file and the `plugins` folder (or just the specific `mesom_patches` files inside it if you have other plugins).

## Configuration

You can configure the mod by placing a `mesom_patches.toml` file next to `mesom_patches.asi` in the `plugins` folder.
An example configuration file can be found [here](./config/mesom_patches.toml).

## Credits

- [libmem by rdbo](https://github.com/rdbo/libmem)
- [Ultimate-ASI-Loader by ThirteenAG](https://github.com/ThirteenAG/Ultimate-ASI-Loader)
