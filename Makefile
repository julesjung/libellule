.PHONY: all ios-build ios-bindings ios-xcframework ios-package clean

all: ios

ios: ios-package

ios-build:
	cargo build -p pronote-uniffi --target aarch64-apple-ios
	cargo build -p pronote-uniffi --target aarch64-apple-ios-sim

ios-bindings: ios-build
	mkdir -p ./build/bindings
	mkdir -p ./build/headers
	cargo run --bin uniffi-bindgen generate --library ./target/aarch64-apple-ios/debug/libpronote.a --language swift --out-dir ./build/bindings
	mv ./build/bindings/pronoteFFI.h ./build/headers/pronoteFFI.h
	mv ./build/bindings/pronoteFFI.modulemap ./build/headers/module.modulemap

ios-xcframework: ios-build ios-bindings
	rm -rf ./build/pronote.xcframework
	xcodebuild -create-xcframework \
		-library ./target/aarch64-apple-ios/debug/libpronote.a \
		-headers ./build/headers \
		-library ./target/aarch64-apple-ios-sim/debug/libpronote.a \
		-headers ./build/headers \
		-output ./build/pronote.xcframework

ios-package: ios-bindings ios-xcframework
	rm -rf ./ios/PronoteKit/pronote.xcframework
	mkdir -p ./ios/PronoteKit/Sources/PronoteKit
	mv ./build/pronote.xcframework ./ios/PronoteKit/pronote.xcframework
	mv ./build/bindings/pronote.swift ./ios/PronoteKit/Sources/PronoteKit/pronote.swift

clean:
	cargo clean
	rm -rf ./build
	rm -rf ./ios/PronoteKit/.build
	rm -rf ./ios/PronoteKit/pronote.xcframework
	rm -rf ./ios/PronoteKit/Sources
