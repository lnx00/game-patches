# LSWTSS Patches

Quality-of-life patches for the game "LEGO Star Wars - The Skywalker Saga".

## Features

- **Disable camera smoothing:** Makes the camera/mouse input feel more responsive.

> [!NOTE]  
> The game heavily relies on camera smoothing to hide small jumps and transitions.
> Disabling smoothing reveals these jumps and might feel unpleasant.

## Installation

1. Download and extract the latest mod files (`lswtss-patches-aio.zip`).
2. Copy **all** extracted files and folders (`dinput8.dll`, `plugins/...`) to your game binary folder (`LEGOStarWarsTSS/`).
3. Run the game and enjoy!

The mod files contain an ASI loader that will automatically load the mod DLL when launching the game. If the `dinput8.dll` file conflicts with any other mod, you can choose an alternative DLL [here](https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases/latest/).

## Uninstallation

1. Navigate to your game binary folder (`LEGOStarWarsTSS/`).
2. Delete the `dinput8.dll` file and the `plugins` folder (or just the specific `lswtss_patches` files inside it if you have other plugins).

## Configuration

You can configure the mod by placing a `lswtss_patches.toml` file next to `lswtss_patches.asi` in the `plugins` folder.
An example configuration file can be found [here](./config/lswtss_patches.toml).

## Credits

- [libmem by rdbo](https://github.com/rdbo/libmem)
- [Ultimate-ASI-Loader by ThirteenAG](https://github.com/ThirteenAG/Ultimate-ASI-Loader)
