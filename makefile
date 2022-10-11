MACOS_TARGET = aarch64-apple-darwin
LINUX_TARGET = x86_64-unknown-linux-musl
WINDOWS_TARGET = x86_64-pc-windows-gnu
ASSETS_DIR = ./src/assets
GREEN=\033[0;32m
NC=\033[0m 


move-assets:
	@echo "Moving assets 🔥"
	@cp -R $(ASSETS_DIR) $(BUILD_DIR)
	@echo "Assets are moved ✅"

build-macos:
	@echo "Generating MacOS binary 🍎"
	@cargo build --release --target $(MACOS_TARGET) --target-dir target/macos
	@make move-assets BUILD_DIR=target/macos/$(MACOS_TARGET)/release/assets 
	@echo "MacOs binary is generated ✅"
	@echo "Run ${GREEN}make run-macos${NC}to run the binary!"

build-linux:
	@echo "Generating Linux binary 🐧"
	@cargo build --release --target $(LINUX_TARGET) --target-dir target/linux
	@make move-assets BUILD_DIR=target/linux/$(LINUX_TARGET)/release/assets 
	@echo "Linux binary is generated ✅"
	@echo "Run ${GREEN}make run-linux${NC}to run the binary!"

build-windows:
	@echo "Generating Windows binary 🪟"
	@cargo build --release --target $(WINDOWS_TARGET) --target-dir target/windows
	@make move-assets BUILD_DIR=target/windows/$(WINDOWS_TARGET)/release/assets 
	@echo "Windows binary is generated ✅"
	@echo "Run ${GREEN}make run-windows${NC}to run the binary!"

run-macos:
	@echo "Running MacOS binary 🍎"
	./target/macos/aarch64-apple-darwin/release/snake_game

run-linux:
	@echo "Running Linux binary 🐧"
	./target/linux/x86_64-unknown-linux-musl/release/snake_game

run-windows:
	@echo "Running Windows binary 🪟"
	./target/windows/x86_64-pc-windows-gnu/release/snake_game.exe

remove-all-builds:
	@echo "Removing all builds 🧹"
	rm -rf target

build-all: 
	@echo "Building all binaries 🚀"
	@make build-macos
	@make build-linux
	@make build-windows
