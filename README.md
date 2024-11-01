# Wynnbuilder Tools UI

A GUI application built with Rust and Iced to interface with Wynnbuilder tools for Wynncraft players.

## Features

- Search through Wynncraft items using various parameters
- Edit configuration files with live preview
- Multiple theme options including:
  - Light/Dark modes
  - Popular themes (Dracula, Nord, Gruvbox, etc.)
  - Tokyo Night variants
  - Catppuccin variants
  - And more!
- Automatic theme persistence
- Status indicators for required binaries and config files

## Prerequisites

- Rust (latest stable version)
- `search_item` binary in the root directory
- `builder` binary in the root directory (upcoming feature)

## Installation

1. Clone the repository
2. Build the project:
```bash
cargo build --release
```
3. Place the required binaries (`search_item`/`search_item.exe`) in the same directory as the executable
4. Create a `config` directory for configuration files

## Usage

### Search Items
Navigate to the Search tab to look up Wynncraft items. Examples:
- List all boots with highest HP: `--type boots -s hp -l 10`
- Get help with parameters: `--help` or `-h`

### Configuration
- The Config File tab allows you to edit the configuration in real-time
- Changes are automatically saved
- Configuration files are stored in the `config` directory

### Themes
- Access the Theme tab to change the application's appearance
- Theme preferences are automatically saved in `settings/theme.toml`

## Project Structure

- `src/main.rs`: Main application logic and UI components
- `src/theme_serde.rs`: Theme serialization/deserialization
- `config/`: Configuration files
- `settings/`: User preferences (theme settings)

## Development

To run the project in development mode:
```bash
cargo run
```

The application uses the Iced GUI framework with the following key components:
- Tab-based navigation
- Text editors for search results and configuration
- Theme picker with persistent settings
- Status indicators for required files

## License

The license can be found in the LICENSE file.