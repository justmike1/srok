ARCH := $(shell uname -m)
ifeq ($(ARCH),arm64)
  TARGET_ARCH := aarch64-unknown-linux-gnu
  COMPILE_TARGET := compile-arm64
else
  TARGET_ARCH := x86_64-unknown-linux-gnu
  COMPILE_TARGET := compile-amd64
endif

TARGET ?= $(TARGET_ARCH)

ensure-cross = \
	command -v cross >/dev/null 2>&1 || { \
		echo "'cross' not found. Installing with 'cargo install cross'..."; \
		cargo install cross; \
	}

compile:
	@echo "Detected arch: $(ARCH), using target: $(COMPILE_TARGET)"
	$(MAKE) $(COMPILE_TARGET)

compile-amd64:
	$(ensure-cross)
	cross build --release --target x86_64-unknown-linux-gnu --features ssr --bin srok
	chmod +x target/x86_64-unknown-linux-gnu/release/srok

compile-arm64:
	$(ensure-cross)
	cross build --release --target aarch64-unknown-linux-gnu --features ssr --bin srok
	chmod +x target/aarch64-unknown-linux-gnu/release/srok

run:
	@echo "Running for target: $(TARGET)"
	TARGET=$(TARGET) docker compose -f docker-compose.yml up --remove-orphans -d --build

clean:
	docker compose -f docker-compose.yml down --remove-orphans