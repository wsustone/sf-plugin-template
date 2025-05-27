# sf-plugin-template
Template repository for StrategyForge plugins

A template repository for creating StrategyForge game plugins. This template provides a starting point for developing modular plugins that can be loaded by the StrategyForge game engine.

## Features

- Boilerplate code for a Bevy-based plugin
- Example systems, components, and resources
- Test setup with example test case
- Cargo.toml configured for plugin development
- Safe hot-reloading support

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo
- StrategyForge Core (as a local dependency)

### Installation

1. Use this template to create a new repository
2. Clone your new repository
3. Update `Cargo.toml` with your plugin's metadata
4. Start developing!

### Development

Build the plugin in debug mode:

```bash
cargo build
```

Build in release mode (recommended for production):

```bash
cargo build --release
```

Run tests:

```bash
cargo test
```

## Plugin Structure

- `lib.rs`: Main plugin definition and public API
- `Cargo.toml`: Project metadata and dependencies
- `src/`: Source code directory

## Example Usage

```rust
use bevy::prelude::*;
use your_plugin_name::TemplatePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TemplatePlugin)
        .run();
}
```

## Development Features

Enable development features in your `Cargo.toml`:

```toml
[dependencies]
your_plugin_name = { path = ".", features = ["dev"] }
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Bevy Engine](https://bevyengine.org/)
- [StrategyForge](https://github.com/wsustone/StrategyForge)
