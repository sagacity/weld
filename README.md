# Weld
Very alpha GUI library that uses Webrender as a backend.

The concepts are quite similar to libraries like React, Elm and Flutter:
* Build up your GUI by composing components
* Every interaction from a user with a component will trigger a callback that allows you to return a new component tree
* Weld will (partially) re-render your GUI whenever the tree changes

# Build
    $ cargo build --release

# Example
    $ cargo run --release --example button
