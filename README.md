# Barotrauma Conflict Finder

`barotrauma_conflict_finder` is a tool designed to identify conflicts between different mods that are installed and active for the game Barotrauma.
This project also allows you to create a Patch mod that fixes conflicted files using a GUI editor.

## Features

- Identify conflicts between different mods installed for Barotrauma.
- Create a "Patch mod" to fix conflicted files using the GUI editor.

## Installation

### From Source

To install `barotrauma_conflict_finder` from source, you need to have [Rust](https://www.rust-lang.org/learn/get-started) installed.

1. Clone the repository:
    ```sh
    git clone https://github.com/FoLZer/barotrauma_conflict_finder.git
    cd barotrauma_conflict_finder
    ```

2. Build and run the project:
    ```sh
    cargo run --release
    ```

### From GitHub Releases

1. Download the latest release for your os from the [Releases](https://github.com/FoLZer/barotrauma_conflict_finder/releases) page.
2. Run the executable.

## Usage

### Running with GUI

Simply run the executable (either from Cargo or the downloaded binary).

### Running with Arguments

You can also run the tool by passing arguments directly. This does not bypass the GUI but allows you to pass settings as arguments instead of setting them within a GUI.

```sh
barotrauma_conflict_finder.exe [GAME_PATH] [CONFIG_PLAYER_PATH] [PATCH_MOD_PATH]
```

- `GAME_PATH`: Path to the Barotrauma game directory. (default: "C:\Program Files (x86)\Steam\steamapps\common\Barotrauma")
- `CONFIG_PLAYER_PATH`: Path to the player's configuration file. (default: "%GAME_PATH%\config_player.xml")
- `PATCH_MOD_PATH`: Path to the directory where the patch mod will be saved. (default: "%GAME_PATH%\LocalMods\conflict_finder_patchmod")

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with your improvements.

## License

This project is licensed under the GPL-2.0 License. See the [LICENSE](LICENSE) file for details.

## Contact

For any questions or suggestions, feel free to open an issue on the GitHub repository.
Alternatively, you could write to me directly on Discord: @folzer
