browser:
	wasm-pack build --target web

node:
	wasm-pack build --target bundler

wasm: 
	make browser
