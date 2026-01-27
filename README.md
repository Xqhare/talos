# Talos
A simple, (almost) no dependency, TUI immediate mode unix toolkit.

It can be used to create a simple TUI for any kind of application.

## Naming
In Greek mythology, `Talos`, was a giant automaton made of bronze to protect Europa in Crete from pirates and invaders. He circled the island's shores three times daily.

## Motivation
This project is educational in nature. I hope it may prove useful to myself in the future.\
I have limited the scope of the project at some points to make it easier on myself, one point would be the emulation of old school code pages.\
To support unicode, I would have to implement `unicode-segmentation` and probably `unicode-width`.

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

Talos never halts execution, meaning it does not wait on input or similar. To adjust the speed of the program, one can use `thread::sleep` as the user of Talos.

Support for mouse input is planned at a much later - stable - state.

### Code Pages

There are a total of 256 possible code pages. 2 are reserved for windows-1252 and cp437.

Each code page has 256 entries. Each entry represents a character.\
Each entry must have a displayed width of 1.\
Each entry must be stored in valid utf-8.

Talos builds a cache of the code pages and checks if a char is in a code page before displaying it.\
Should a char not be in a code page, it will be displayed as a question mark.

