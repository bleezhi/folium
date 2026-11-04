# Folium

**The open-source creative suite.**

Folium is a cross-platform creative software project built around a shared Rust engine and native application interfaces. Development starts on Android with **Folium Paint**, a digital painting and illustration app. Desktop support and the other creative tools will grow from the same foundations.

## Apps

| App | Purpose | Status |
|---|---|---|
| **Folium Paint** | Raster painting and illustration | In development |
| Folium Animate | 2D animation | Planned |
| Folium Vector | Vector illustration | Planned |
| Folium Motion | Video editing, compositing and motion graphics | Planned |
| Folium Audio | Audio editing and sound design | Planned |
| Folium 3D | 3D creation | Future |

## Project principles

- Open-source, with a shared and reusable creative engine.
- Native interfaces rather than a WebView-based editor.
- Rust for core document, input and rendering systems.
- Android first; desktop support is part of the architecture.
- Portable, versioned project files and dependable export.
- Build useful app features early, not infrastructure for its own sake.

## Repository layout

```text
apps/
  android/paint/       Android application (Kotlin + Compose)
crates/
  folium-core/         Shared engine primitives
  folium-document/    Document model and project serialization
docs/                 Architecture and development notes
.github/workflows/    Continuous integration
```

## Getting started

### Rust workspace

Install the stable Rust toolchain, then run:

```sh
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

### Android app

Open `apps/android/paint` in Android Studio with a compatible Android SDK installed, or build it with Gradle:

```sh
cd apps/android/paint
./gradlew assembleDebug
```

The Android app is an early scaffold; features will be added incrementally.

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Development setup](docs/DEVELOPMENT.md)
- [Project format](docs/PROJECT_FORMAT.md)

## Contributing

Folium is at an early stage. Issues and focused pull requests are welcome. Please read the development notes before making architectural changes.

## License

A repository-wide license has not yet been selected. Until one is added, the source is not granted an open-source license by default.
