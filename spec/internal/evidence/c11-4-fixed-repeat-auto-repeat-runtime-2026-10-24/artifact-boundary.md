# Debug artifact fixture-marker 경계

| Artifact | SHA-256 | Nine scene markers |
| --- | --- | --- |
| Default Android Debug APK | <code>2eabb7bc100872ee541d406cfae1687ca4e412ef4d682c1160988bf709579cb2</code> | 0/9 present across every uncompressed APK entry |
| Opt-in Android Debug APK positive control | <code>8a6b10d62692930ace3114e73f8071935ef62161cc40471fbee5fda573538acc</code> | 9/9 present in <code>libspinon_bootstrap.so</code> |
| Default iOS Simulator app executable | <code>36b996f6eb1842273e1bffdd23ecad58a04e32d72fdf1cffe420320089b3be1b</code> | 0/9 present across files in the app bundle |
| Opt-in iOS Simulator executable positive control | <code>6fbb9c865d5e8b5ba2a6652a8c193e854bacc9a5b36d164dfab402a17353257e</code> | 9/9 present |

The scanner records the exact scene IDs and matching archive entries/files in [artifact-boundary.json](artifact-boundary.json). The positive controls were copied to temporary paths during verification and are not committed.

This is a marker-only comparison for the default Android Debug APK and iOS Simulator app. It does not prove Release APK or IPA symbol absence, final package size, or hardware behavior.
