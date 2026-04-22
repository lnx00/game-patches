# MESOW Patches

Quality-of-life patches for the game "Middle-earth: Shadow of War".

## Features

- **Disable camera smoothing:** Makes the camera feel more responsive, especially when using a mouse.

## Installation

1. Download and extract the latest mod files (`mesow-patches-aio.zip`).
2. Copy **all** extracted files and folders (`winmm.dll`, `plugins/...`) to your game binary folder (`ShadowOfWar/x64/`).
3. Run the game and enjoy!

The mod files contain an ASI loader that will automatically load the mod DLL when launching the game. If the `winmm.dll` file conflicts
with any other mod, you can choose an alternative DLL [here](https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases/latest/).

## Uninstallation

1. Navigate to your game binary folder (`ShadowOfWar/x64/`).
2. Delete the `winmm.dll` file and the `plugins` folder (or just the specific `mesow_patches` files inside it if you have other plugins).

## Configuration

You can configure the mod by placing a `mesow_patches.toml` file next to `mesow_patches.asi` in the `plugins` folder.
An example configuration file can be found [here](./config/mesow_patches.toml).

## Credits

- [libmem by rdbo](https://github.com/rdbo/libmem)
- [Ultimate-ASI-Loader by ThirteenAG](https://github.com/ThirteenAG/Ultimate-ASI-Loader)
