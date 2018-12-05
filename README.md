# lpsettings
A cli tool and library for managing `lovepack` settings.

`lpsettings` is primarily designed to act as a rust library for `lovepack` to make it easier to write and read settings from various lovepack project sources. It can be compiled into a standalone binary for stand-alone settings interaction.

## Using the Library
Add the library to your Cargo.toml `lpsettings = "0.2"` and then start getting settings.

```rust
// get a setting
lpsettings::get_value("user.name");

// set a setting
lpsettings::set_value("user.email","user@email.com");
```

`lpsettings` use a enum to contain all the possible types in a settings file, you will need to match these to get the underlying values

```rust
use lpsettings::Type // repeated from settingsfile

if let Some(Type::Text(username)) == lpsettings::get_value("user.name") {
    println!("Your username is set to {}", username);
}

```

## Compiling the Binary
Clone this project and run cargo inside the src-binary

```bash
cd src-binary
cargo build --release
```

The binary follows the version of the library, so build to the latest tagged repository to get the correct  release version.