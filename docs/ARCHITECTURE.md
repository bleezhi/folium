# Architecture

Folium is organized as a set of native applications backed by reusable Rust crates.

## High-level design

- **Android application:** Kotlin and Jetpack Compose provide navigation, toolbars, dialogs, platform lifecycle integration and stylus event capture.
- **Rust engine:** owns portable document structures, brush/stroke logic, history, project serialization and rendering.
- **Rendering:** GPU rendering is planned around `wgpu`; it is intentionally not introduced until the first canvas/input path is specified and tested.
- **FFI boundary:** a small, explicit boundary will connect the Android UI to Rust. The exact binding approach (JNI or UniFFI) remains an implementation decision.
- **Desktop:** a later native desktop shell will reuse the same Rust engine.

## Current crates

- `folium-core`: IDs, geometry and color primitives.
- `folium-document`: document/layer model and versioned project serialization.

## Planned engine modules

As the Paint MVP develops, extract focused crates for canvas/tile storage, input normalization, brushes, history, assets and export. Avoid premature abstractions: each crate should exist when it has a real consumer or independently testable responsibility.

## Raster strategy

The initial canvas should use tiled raster storage so large documents do not require one enormous contiguous bitmap. Tiles should be allocated on demand, with a bounded cache and explicit dirty-region tracking. Stroke input should retain pressure and timestamp data so brush rendering can be improved without losing source input.

## Project format

The first implementation uses a versioned JSON representation for correctness and easy inspection. A production `.folium` container is expected to become a ZIP-based package containing a manifest, document metadata, layer/tile data and optional assets. Migration code must be version-aware.
