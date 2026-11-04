# Development

## Requirements

- Stable Rust toolchain with `rustfmt` and `clippy`.
- Android Studio or Android SDK command-line tools for the Android application.
- JDK 17 for Android Gradle builds.

## Rust checks

From the repository root:

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Android

The first app lives in `apps/android/paint`. Open that directory in Android Studio. CI uses Gradle 8.x through the Gradle setup action. A Gradle wrapper will be added once its wrapper distribution and checksums are pinned.

## Scope discipline

Prefer small vertical slices that make the app more useful: canvas interaction, input capture, brush preview, drawing, undo/redo, layers, save/load and export. Keep the document model independent of Android UI types.
