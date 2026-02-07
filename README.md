# Talos

<p align="center">
    <img src="https://raw.githubusercontent.com/Xqhare/talos/refs/heads/master/pics/generated_logo.jpeg" alt="Talos Logo" width="560" height="300"/>
</p>

A simple, (almost) no dependency, TUI immediate mode unix toolkit.

It can be used to create a simple TUI for any kind of application.

## Naming
In Greek mythology, `Talos`, was a giant automaton made of bronze to protect Europa in Crete from pirates and invaders. He circled the island's shores three times daily.

## Motivation
This project is educational in nature. I hope it may prove useful to myself and maybe even others in the future.\
It is part of my larger goal to create my own development ecosystem.\
I have limited the scope of the project at some points to make it easier on myself, one point would be the emulation of old school code pages.

## Features

### Core

- Zero-Dependency: Built directly on top of `libc`
- Signal handling
    - Automatically handles `SIGWINCH` and `SIGINT`/`SIGTERM`
- Input handling
    - Xterm parser
    - Keyboard support
        - Standard keys
        - Functions keys (F1-F12)
        - Modifiers (Shift, Control, Alt)
    - Mouse support
        - Button support
        - Drags & Scrolls
- Rendering
    - Style system: Supports foreground/background colors (Normal, Bright, Extended 256-color, and `TrueColor` / RGB).
    - Text Attributes (Bold, Dim, Italic, Underline, Blink, Reverse, and Strikethrough)
    - Layout engine: Supports horizontal and vertical alignment and using Constraints
    - Codex
        - Emulation of old school code pages for character rendering
        - Includes as default:
            - Windows-1252
            - CP437
            - UTF Geometric Shapes block
            - UTF Miscellaneous Technical Symbols block

### Widgets

- Simple Widgets
    - Block: A container widget with configurable borders (Normal or Fat), titles, and subtitles in six different positions.
    - Text: Supports text wrapping, horizontal centering, and vertical alignment.
    - Number: Renders numeric values as text.
- Stateful Widgets
    - List: Supports vertical and horizontal scrollable lists with selection symbols and styles.
    - Table: Supports grid-based data with optional inner/outer borders and alternating row/column colors.
    - Fillable Bar (Progress Bar): Supports horizontal and vertical bars with optional percentage displays and "glow" (shading) effects.
    - Signal Box: A simple toggleable boolean indicator using geometric symbols.

## Roadmap

- [ ] Addons
    - [ ] Theme to manage many different styles
        - [ ] Probably just a hashmap of styles with names
    - [ ] `AreaManager` to manage many areas
        - [ ] Probably just a hashmap of areas with names
        - Simplifiy mouse support: `AreaManager.get_area(x, y)`
- [ ] Documentation
    - [ ] Examples
        - [x] Demos
        - [ ] Docs
    - [ ] API
        - [ ] Public
        - [ ] Internal
    - [ ] Readme
- [ ] Widgets
    - [ ] Stateful Widgets
        - [ ] Text Input Widget
        - [ ] Chart widget
            - [ ] Column / Bar
            - [ ] Stacked Column / Bar
            - [ ] Min - Max Chart (Two points per x coordinate, connected with vertical lines)
            - [ ] Line (Only points. No interconnecting lines)
            - [ ] Support for `usize`, `isize` and `f32` & `f64`
                - `isize` and `float` will need the x-axis to be in the middle to support negative values

## Usage

### Add to Cargo.toml

To use Talos, and keep it up to date, add the following to your `Cargo.toml`:

```toml
[dependencies]
talos = { git = "https://github.com/Xqhare/talos" }
```

## Technical Details

Most coordinate calculations are limited to `u16::MAX`.
This is because of cast truncation in the calculations to `u16`.\
This means, for example, that a `Table` may only ever have up to around 65,000 rows and columns.

However, it is very unlikely that this will cause any issues in practice.

### Custom Code Pages

It is recommended that any custom code pages use an ID of `16` or higher.\
The range of `0` to `15` is softly reserved for the default code pages.

## Project Design
`libc` will be used as the base, the bindings will be taken from the rust crate `libc`.

I need to enter and exit `RawMode`.

A small ANSI Engine will be used to create the output.
The engine will use Canvas - a way to store the output and call `write()` once per frame after frame creation.
The engine will have a `Widget` trait.

A basic layout engine.

All constructions will be done using the builder pattern.

The developer experience should be as simple, intuitive and fast as I can make it.

While Windows and Mac support are not planned at all, the architecture should be at least somewhat extendable if I ever change my mind.

Talos emulates the use of old school code pages. While this decision has major downsides, it not only simplifies but also adds the weird flair each of my projects needs and is one of the core tenants of Talos.
Windows-1252 and CP437 by default.
User can provide their own, but have to ensure that every displayed character has the same width of one. 
This is done for simplicity in the code.
To support unicode, I would have to implement `unicode-segmentation` and probably `unicode-width` to keep the dependencies to only `libc`.

Talos never halts execution, meaning it does not wait on input or similar. To adjust the speed of the program, one can use `thread::sleep` as the user of Talos.

To keep the scope small, I want to push as much on the user as I can. This includes state management, and managing the currently focused widget.
This also means that events (e.g. `on_click`) must be handled by the user. 

### Code Pages

There are a total of 256 possible code pages. The first two (Index 0 and 1) are reserved for windows-1252 and cp437 respectively.

Each code page has 256 entries. Each entry represents a character.\
Each entry must have a displayed width of 1 and must be stored in valid utf-8.

Talos builds a cache of the code pages and checks if a char is in a code page before displaying it.\
Should a char not be in a code page, it will be displayed as a question mark.

### Features to consider

- [ ] Maybe
    - [ ] Custom page loader
    - [ ] Mouse position reporting without a mouse button pressed
        - [ ] Backend
            - [x] Xterm Parser
            - [ ] Flags to enable mouse reporting in `TerminalIO`
                - Needs to be configurable
        - [x] Frontend
    - [ ] Layers (right now there is only one layer - cells are drawn over each other sequentially if there are multiple widgets overlapping)
        - If performance suffers, implement a layer system - seems like a lot of work for not much benefit in most applications.
- [ ] Probably never - Way too much work, but would be nice
    - [ ] Unicode support & remove Code pages
    - [ ] Windows support
        - [ ] Read the damn win docs again and determine needed foreign functions needed
        - [ ] FFI for `kernel32.dll` at the very minimum needed (I/O)
        - [ ] FFI for `shell32.dll` (Shellhook?) and `ole32.dll` (memory management & clipboard) probably needed
    - [ ] Mac support
        - [ ] Define what needs to be done - FFI wise

