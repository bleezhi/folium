# Folium project format

## Current format: version 1

The current prototype serializes a `ProjectFile` as UTF-8 JSON. Its top-level fields are:

- `format`: the literal identifier `folium`
- `version`: integer format version
- `document`: document metadata and layers

This is a development format, not yet the final binary container. Files use the `.folium` extension in examples, but version 1 is plain JSON.

## Compatibility rules

1. Never silently reinterpret an unknown format version.
2. Keep document identity stable when loading and saving.
3. Add migrations when a future version changes persisted data.
4. Treat embedded assets as untrusted input and validate sizes before allocation.

## Planned package layout

A later ZIP-based container is expected to include `manifest.json`, `document.json`, raster tile payloads and an `assets/` directory. Compression, checksums and resource limits will be specified before that format is finalized.
