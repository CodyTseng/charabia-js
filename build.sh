#!/bin/bash

# clean pkg
rm -rf pkg

# build
wasm-pack build --target nodejs --release