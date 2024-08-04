.PHONY: debug release clean

debug:
	@ EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sGL_ENABLE_GET_PROC_ADDRESS -sWASM=1 -sALLOW_MEMORY_GROWTH=1 -sWASM_MEM_MAX=512MB -sTOTAL_MEMORY=512MB -sABORTING_MALLOC=0 -sASYNCIFY -sFORCE_FILESYSTEM=1 -sASSERTIONS=1 -sERROR_ON_UNDEFINED_SYMBOLS=0 -sEXPORTED_RUNTIME_METHODS=ccallcwrap" cargo build --target wasm32-unknown-emscripten
	@ cp target/wasm32-unknown-emscripten/debug/snake.wasm snake.wasm
	@ cp target/wasm32-unknown-emscripten/debug/snake.js snake.js
release:
	@ EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sGL_ENABLE_GET_PROC_ADDRESS -sWASM=1 -sALLOW_MEMORY_GROWTH=1 -sWASM_MEM_MAX=512MB -sTOTAL_MEMORY=512MB -sABORTING_MALLOC=0 -sASYNCIFY -sFORCE_FILESYSTEM=1 -sASSERTIONS=1 -sERROR_ON_UNDEFINED_SYMBOLS=0 -sEXPORTED_RUNTIME_METHODS=ccallcwrap" cargo build --target wasm32-unknown-emscripten --release
	@ cp target/wasm32-unknown-emscripten/release/snake.wasm snake.wasm
	@ cp target/wasm32-unknown-emscripten/release/snake.js snake.js
clean:
	@ cargo clean
