# computer_graphics

## Description

Computer graphics in Rust.

It's a workspace with different crates.

### Binary crate

- `cmprust`
    - Main program.

### Library crates

- `georust`
    - Provides types and methods for geometric concepts.

- `linrust`
    - Provides types and methods for computer graphics.

- `ppmrust`
    - Provides types and methods for PPM images.

- `renrust`
    - Provides methods for rendering (use this!).

## Dependencies

```mermaid
graph TD;
    linrust --> georust;

    georust --> renrust;
    ppmrust --> renrust;

    georust --> cmprust;
    renrust --> cmprust;
    ppmrust --> cmprust;

    linkStyle 3 stroke:green;
    linkStyle 4 stroke:green,stroke-width:6px;
    linkStyle 5 stroke:green;
```

## Commands

### Version

```bash
% rustc --version
# rustc 1.92.0 (ded5c06cf 2025-12-08)

% cargo --version
# cargo 1.92.0 (344c4567c 2025-10-21)
```

### Utils

```bash
% cargo build # Compile every crate
% cargo test # Test every crate

% cargo run -p cmprust # Run crate binary
% cargo test -p [name] # Test crate library
```
