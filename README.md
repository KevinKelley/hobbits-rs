# An implementation of the Hobbits protocol, in Rust.


- To run unit tests: `cargo test`

- To run the demo server: `cargo run` and, in another terminal, start the conformance
test like `./conformance --port 12345`

- (To use a different port: `cargo run -- --port 8888` and `./conformance --port 8888`)

(note: conformance may need to run as sudo, depending on whether your system allows ping)


There is still work-in-progress, to simplify usage (and to support streaming of
Envelopes over a single TCP connection); that's what `src/encoding/codec.rs` is
for, along with some other stuff not committed yet.
