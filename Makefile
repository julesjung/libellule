IOS_VERSION := 26.0

.PHONY: all ios-build ios-bindings ios-xcframework ios-package clean

all: ios

ios: ios-package

ios-build:
	IPHONEOS_DEPLOYMENT_TARGET=$(IOS_VERSION) cargo build -p libellule-uniffi --target aarch64-apple-ios
	IPHONEOS_DEPLOYMENT_TARGET=$(IOS_VERSION) cargo build -p libellule-uniffi --target aarch64-apple-ios-sim

ios-bindings: ios-build
	mkdir -p ./build/bindings
	mkdir -p ./build/headers
	cargo run --bin uniffi-bindgen generate --library ./target/aarch64-apple-ios/debug/liblibellule.a --language swift --out-dir ./build/bindings
	mv ./build/bindings/libelluleFFI.h ./build/headers/libelluleFFI.h
	mv ./build/bindings/libelluleFFI.modulemap ./build/headers/module.modulemap

ios-xcframework: ios-build ios-bindings
	rm -rf ./build/libellule.xcframework
	xcodebuild -create-xcframework \
		-library ./target/aarch64-apple-ios/debug/liblibellule.a \
		-headers ./build/headers \
		-library ./target/aarch64-apple-ios-sim/debug/liblibellule.a \
		-headers ./build/headers \
		-output ./build/libellule.xcframework

ios-package: ios-bindings ios-xcframework
	rm -rf ./ios/LibelluleKit/libellule.xcframework
	mkdir -p ./ios/LibelluleKit/Sources/LibelluleKit
	mv ./build/libellule.xcframework ./ios/LibelluleKit/libellule.xcframework
	mv ./build/bindings/libellule.swift ./ios/LibelluleKit/Sources/LibelluleKit/libellule.swift

clean:
	cargo clean
	rm -rf ./build
	rm -rf ./ios/LibelluleKit/.build
	rm -rf ./ios/LibelluleKit/libellule.xcframework
	rm -rf ./ios/LibelluleKit/Sources
