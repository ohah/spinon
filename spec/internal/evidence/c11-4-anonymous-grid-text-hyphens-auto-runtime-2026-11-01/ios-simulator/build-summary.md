# iOS Simulator build result

- Target: iPhone 17 Pro / iOS 26.2 Simulator, ARM64 Simulator executable.
- Build: Xcode `SpinonBootstrap` scheme with `iphonesimulator` SDK, fixture-test host enabled, and code signing disabled for the simulator run.
- Result: `** BUILD SUCCEEDED **`.
- Artifact identity: the installed executable SHA-256 matches the build SHA-256 recorded in the final run's `simulator.json`.
- Warnings emitted by the build: deprecated `traitCollectionDidChange`, two always-successful `[String: Any]` casts, duplicate `RawWindowMetalLayer` linker symbols, V8 debug-map duplicate names, and skipped App Intents metadata extraction. These warnings are retained as reported; this document does not classify their origin or claim they were fixed by this change.

The full Xcode output is preserved byte-for-byte in [build.log.gz](build.log.gz). It includes the local build command and paths from the original run.
