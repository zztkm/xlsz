ARCH=linux/arm64
PY_VERSION=3.12
VERSION := $$(make -s show-version)

.PHONY: tag
tag:
	git tag $(VERSION)
	git push origin $(VERSION)

.PHONY: show-version
show-version:
	@rye version

.PHONY: dev
dev:
	maturin develop --skip-install

.PHONY: release
prod:
	maturin develop --release

.PHONY: test
test: dev
	rye test -- -vv -v -ra -n auto tests

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