cargo build

# Web
wasm-pack build --target web --out-dir ./web/src/rust

# Android
cargo ndk -o ./android/app/src/main/jniLibs \
        --manifest-path ./Cargo.toml \
        -t armeabi-v7a \
        -t arm64-v8a \
        -t x86 \
        -t x86_64 \
        build --release

cargo run --bin uniffi generate --library ../target/debug/libexample.dylib --language kotlin --out-dir ./android/app/src/main/java/tech/forgen/example/rust

# iOS
# Add the iOS targets and build
for TARGET in \
        aarch64-apple-darwin \
        aarch64-apple-ios \
        aarch64-apple-ios-sim \
        x86_64-apple-darwin \
        x86_64-apple-ios
do
    rustup target add $TARGET
    cargo build --release --target=$TARGET
done

cargo run --bin uniffi generate --library ../target/debug/libexample.dylib --language swift --out-dir ./ios/bindings
mv ./ios/bindings/exampleFFI.modulemap ./ios/bindings/module.modulemap

rm ./ios/example/rust.swift
mv ./ios/bindings/example.swift ./ios/example/rust.swift

rm -rf "./ios/bindings/rust.xcframework"
xcodebuild -create-xcframework \
        -library ../target/aarch64-apple-ios-sim/release/libexample.a -headers ./ios/bindings \
        -library ../target/aarch64-apple-ios/release/libexample.a -headers ./ios/bindings \
        -output "./ios/rust.xcframework"

rm -rf ./ios/bindings


