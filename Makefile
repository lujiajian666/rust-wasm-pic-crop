.PHONY: build
build: build-wasm npm-link

.PHONY: es5
es5: 
	wasm-pack build --target web --release --out-dir pkg-es

.PHONY: build-wasm
.ONESHELL:
build-wasm:
	echo "build-wasm"
	wasm-pack build && cp ./package-for-pkg.json ./pkg/package.json

.PHONY: npm-link
.ONESHELL:
npm-link:
	echo "npm link"
	cd www && npm i

