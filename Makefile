ARCH=linux/arm64
PY_VERSION=3.12

.PHONY: dev
dev:
	maturin develop --skip-install

.PHONY: release
prod:
	maturin develop --release

.PHONY: test
test: dev
	rye run test

.PHONY: bench
bench: prod
	rye run python benchmark.py

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: build
build:
	maturin build --release

.PHONY: build-docker
build-with-docker:
	docker run --rm --platform ${ARCH} -v ${PWD}:/io ghcr.io/pyo3/maturin build --release --out dist --interpreter ${PY_VERSION}