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

## Roadmap

- [x] Core
    - [x] Mouse input support (eventually)
    - [x] Optimise Codex
- [ ] Addons
    - [ ] Theme to manage many different styles
        - [ ] Probably just a hashmap of styles with names
- [ ] Documentation
    - [ ] Examples
    - [ ] API
        - [ ] Public
        - [ ] Internal
    - [ ] Readme
- [ ] Widgets
    - [x] Simple Widgets
        - [x] Text wrapping in `Text` widget
        - [x] Boolean rendering widget
        - [x] Number rendering widget
    - [ ] Stateful Widgets
        - [ ] Text Input Widget
        - [x] List widget
        - [x] Table widget
        - [ ] Chart widget
            - [ ] Column / Bar
            - [ ] Stacked Column / Bar
            - [ ] Min - Max Chart (Two points per x coordinate, connected with vertical lines)
            - [ ] Line (Only points. No interconnecting lines)
            - [ ] Support for `usize`, `isize` and `f32` & `f64`
                - `isize` and `float` will need the x-axis to be in the middle to support negative values
        - [x] Progress bar / Fillable bar widget
            - [x] Horizontal
            - [x] Vertical
            - [x] Toggleable numeric display in %
- [ ] Maybe
    - [ ] Custom page loader
    - [ ] Mouse position reporting without a mouse button pressed
        - [ ] Backend
            - [x] Xterm Parser
            - [ ] Flags to enable mouse reporting in TerminalIO
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

## Project Design
`libc` will be used as the base, the bindings will be taken from the rust crate `libc`.

I need to enter and exit RawMode.

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

