#!/bin/sh
cargo build --release
cp target/release/slide ~/.local/bin/slide
mkdir -p ~/.local/share/slide/
cp -r template ~/.local/share/slide/
