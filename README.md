# ME: SoM Patches

Quality-of-life patches for the game "Middle-earth: Shadow of Mordor".

## Features

- ???

## Installation

1. Download and extract the latest mod files (`mesom-patches-aio.zip`)
2. Copy **all** extracted files and folders (`dinput8.dll`, `dinput8.ini`, `plugins/...`) to your game folder (`ShadowOfMordor/x64/`)
3. Run the game and enjoy!

The mod files contain an ASI loader that will automatically load the mod DLL when launching the game. If the `dinput8.dll` file conflicts
with any other mod, you can choose an alternative DLL [here](https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases/latest/).

## Configuration

You can configure the mod by placing a `mesom_patches.toml` file next to `mesom_patches.asi` in the `plugins` folder.
An example configuration file can be found [here](./config/mesom_patches.toml).

## Credits

- [libmem by rdbo](https://github.com/rdbo/libmem)
- [Ultimate-ASI-Loader by ThirteenAG](https://github.com/ThirteenAG/Ultimate-ASI-Loader)
