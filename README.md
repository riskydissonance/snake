# Snake

Snake in rust using [raylib](https://www.raylib.com/).

Available natively or via wasm at https://riskydissonance.github.io/snake/

## Building

### Native

Just need rust and then build and run:

``` shell
make debug-native
make release-native
```

### WASM

Need to install [emscripten](https://emscripten.org/docs/getting_started/downloads.html) for compiling to wasm and setup the path as it details.

``` shell
make debug-wasm
make release-wasm
```

This will build and then update the root `snake.js` and `snake.wasm` which is used when it is deployed via GitHub.
