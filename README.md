# Talos
A simple, (almost) no dependency, TUI immediate mode unix toolkit.

It can be used to create an ui for any kind of application.

## Naming
In Greek mythology, `Talos`, was a giant automaton made of bronze to protect Europa in Crete from pirates and invaders. He circled the island's shores three times daily.

## Project Design
`libc` will be used as the base, the bindings will be taken from the rust crate `libc`.

I need to enter and exit RawMode.

A small ANSI Engine will be used to create the output.
The engine will use Canvas - a way to store the output and call `write()` once per frame after frame creation.
The engine will have a `Widget` trait.

A basic layout engine.

All constructions will be done using the builder pattern.

The developer experience should be as simple, intuitive and fast as I can make it.
