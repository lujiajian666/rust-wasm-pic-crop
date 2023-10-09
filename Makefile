.PHONY: build
build: build-wasm npm-link

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

