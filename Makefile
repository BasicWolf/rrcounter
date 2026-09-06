ANSIBLEOPTS=-vv

DEPLOY_DRY_RUN ?= 0

ifeq ($(DEPLOY_DRY_RUN), 1)
	ANSIBLEOPTS += --check
endif

DEV_BINARY_PATH = target/debug/rrcounter
RELEASE_BINARY_PATH = target/release/rrcounter
build: $(DEV_BINARY_PATH)

build: $(DEV_BINARY_PATH)

$(DEV_BINARY_PATH):
	cargo build

test: $(DEV_BINARY_PATH)
	cargo test

$(RELEASE_BINARY_PATH):
	cargo build --release

release: $(RELEASE_BINARY_PATH)

deploy:
	ansible-playbook ./.github/workflows/ansible/deploy.yml $(ANSIBLEOPTS)

clean:
	cargo clean

.PHONY: build test release deploy clean
