# FC3BD Patches

Quality-of-life patches for the game "Far Cry 3 - Blood Dragon".

## Features

- **Disable input clamp:** Removes the maximum mouse input limit (also referred to as "Negative Mouse Acceleration").
- **Sensitivity fix:** Reduces the sensitivity scale to make the game playable with modern high-DPI mice.

## Installation

1. Download and extract the latest mod files (`fc3bd-patches-aio.zip`).
2. Copy **all** extracted files and folders (`dsound.dll`, `plugins/...`) to your game binary folder (`Far Cry 3 Blood Dragon/bin/`).
3. Run the game and enjoy!

The mod files contain an ASI loader that will automatically load the mod DLL when launching the game. If the `dsound.dll` file conflicts with any other mod, you can choose an alternative DLL [here](https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases/latest/).

## Uninstallation

1. Navigate to your game binary folder (`Far Cry 3 Blood Dragon/bin/`).
2. Delete the `dsound.dll` file and the `plugins` folder (or just the specific `fc3bd_patches` files inside it if you have other plugins).

## Configuration

You can configure the mod by placing a `fc3bd_patches.toml` file next to `fc3bd_patches.asi` in the `plugins` folder.
An example configuration file can be found [here](./config/fc3bd_patches.toml).

## Credits

- [libmem by rdbo](https://github.com/rdbo/libmem)
- [Ultimate-ASI-Loader by ThirteenAG](https://github.com/ThirteenAG/Ultimate-ASI-Loader)
