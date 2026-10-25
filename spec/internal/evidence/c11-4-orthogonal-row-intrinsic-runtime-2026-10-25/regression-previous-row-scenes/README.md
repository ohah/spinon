# C11.4 직교 row 및 기존 Grid regression source 비교

최종 style-only guard가 들어간 동일 staged source snapshot으로 새 직교 text/image scene 4개와 기존 #611/#620 scene 8개를 Android와 iOS에서 각각 재실행했다. 두 플랫폼 모두 12/12 scene의 native matcher, frame, track, contribution, trace와 draw 검사를 통과했다. 상세 scene별 log, trace, PNG, 체크섬 및 종료 근거는 [Android 최종 실행](./android-avd/final-post-style-only-guard/README.md)과 [iOS 최종 실행](./ios-simulator/final-post-style-only-guard/README.md)에 있다.

## 최종 source와 앱

| 플랫폼 | HEAD | staged index tree | source 식별자 | 앱 SHA-256 |
| --- | --- | --- | --- | --- |
| Android API 37 ARM64 16 KB AVD | `d37ee6ea8c985ab27eb32770491c8998dcbf756c` | `45bb3313f5c126fd724eb54e877f86746107a715` | source snapshot SHA-256 `2cae262c3b1cc8bfcb420b84313f53540909c711d834f001d69058dde249ce11` | `cf978a383a3abceeaa99fe37d2f5c202f859193f20a352d4eea67dd0626371af` (설치본과 일치) |
| iPhone 17 Pro / iOS 26.2 Simulator | `d37ee6ea8c985ab27eb32770491c8998dcbf756c` | `45bb3313f5c126fd724eb54e877f86746107a715` | 807개 source/config manifest SHA-256 `954ef11bb9478de7391a0cc06ec63ccd6455ff43f4792627e5a680c7da94435f` | `ceadb3552b5bb50fcc9a75e813e58c9927f988ed4a10a20315d3c284f3563b93` |

index tree에는 HEAD 단독에 없는 staged 구현이 포함된다. 두 플랫폼은 Android `emulator-5570`과 전용 iOS UDID만 사용했고, 실제 기기는 사용하지 않았다. 두 전용 시뮬레이터는 검증을 마친 뒤 정상 종료했으며 사용자의 다른 booted simulator는 유지했다.

시뮬레이터 실행 후 PR 검토에서 확인한 계약 설명 두 곳을 Rustdoc comment만 좁혀 적었다. 실행 코드와 fixture는 변경되지 않았다. 테스트 당시의 staged source tree는 `45bb3313f5c126fd724eb54e877f86746107a715`이다. 수정 뒤 `cargo fmt --all -- --check`가 통과했다.

## 이전 iOS source 실행 기록

이 표는 최종 동일 source 12-scene 실행 전의 역사적 캡처다. 아래 이전 해시를 최종 앱 결과로 사용하지 않는다.

| 실행 | scene run IDs | build log | source manifest | 앱 executable SHA-256 |
| --- | --- | --- | --- | --- |
| 기존 8개 control | `c114reg01`–`c114reg08` | [build.log](./ios-simulator/build.log) | [source-hashes.json](./ios-simulator/source-hashes.json), `32361ecb24262f4698109d6b7f9680ac0e4d6fd0443798a33cf3e92342835617` | `de9a1aa3e8b1ad715d05afc5f753b21c8463365c3c3ef1411d1d7dad91c4864c` |
| 중간 orthogonal 실행 | `c114cur13`–`c114cur16` | [build-current-orthogonal.log](./ios-simulator/build-current-orthogonal.log) | [source-hashes-current-build.json](./ios-simulator/source-hashes-current-build.json), `b3c7da0984d1ebbb8c4d89ea3e9205a423342da5c1680f7ab434b8f6bcf9ae92` | `2a343098e9008567bf49093479fa6bfda921d6a1c0fd4cbf86a9dc2252f1e4ba` |
| 이전 최종 orthogonal 4개 실행 | `c114fin17`–`c114fin20` | [build-final-orthogonal.log](./ios-simulator/build-final-orthogonal.log) | [source-hashes-final-build.json](./ios-simulator/source-hashes-final-build.json), `b79c74d3b86087a09a95907124cc12a507a024087975302b31f538f4e2d01a6f` | `7ae420cbaaf542429c6e4e054aee3ea613b6f721375b532b58136c9075ed8189` |

세 이전 manifest는 각 파일의 `path NUL file-sha256 LF`를 정렬해 해시했다. 기존 8-scene build와 이전 orthogonal build 사이에는 source file 다섯 개가 달랐고, 중간·이전 최종 orthogonal build 사이는 `grid_text_stretch_tests.rs` 하나만 달랐다. 원본별 비교는 [source-hash-version-comparison.json](./ios-simulator/source-hash-version-comparison.json)에 있다. 이전 8개 iOS scene validation은 [previous-eight-scenes-validation.json](./ios-simulator/previous-eight-scenes-validation.json), 이전 orthogonal 네 scene validation은 [final-orthogonal-validation.json](./ios-simulator/final-orthogonal-validation.json)에 보존했다.

## 최종 12 scene geometry

| Scene 묶음 | Root | Text | Image | Columns / rows |
| --- | --- | --- | --- | --- |
| direct min-content column 2개 | `120×48` | `50×20` at `(0,0)` | `24×16` at `(0,24)` | `[50]` / `[24,24]` |
| direct max-content column 2개 | `120×48` | `80×10` at `(0,0)` | `24×16` at `(0,24)` | `[80]` / `[24,24]` |
| direct row min-content 2개 | `74×20` | `50×20` at `(0,0)` | `24×16` at `(50,0)` | `[50,24]` / `[20]` |
| direct row max-content 2개 | `74×20` | `50×20` at `(0,0)` | `24×16` at `(50,0)` | `[50,24]` / `[20]` |
| orthogonal row min-content 2개 | `74×50` | `50×50` at `(0,0)` | `24×16` at `(50,0)` | `[50,24]` / `[50]` |
| orthogonal row max-content 2개 | `74×80` | `50×80` at `(0,0)` | `24×16` at `(50,0)` | `[50,24]` / `[80]` |

각 묶음은 source order 정방향과 역방향을 모두 포함하며 예상 node identity와 geometry를 유지했다. 최종 orthogonal text source provenance는 child `VerticalRl`, parent `HorizontalTb`, orientation `Mixed`, logical inline min/max `50/80`, logical block min/max `20/10`, font registry revision `1`이다. 각 contribution의 span은 `1`, flexible track 교차는 `false`다.

## 종료 및 한계

Android trace parser가 처음 두 번 JSON 조각을 재조립하지 못했으나, scene matcher와 draw는 통과했다. 진단용 원본은 Android 최종 디렉터리의 `harness-diagnostics/`에 보존했고, 수정된 parser로 12개 scene을 모두 재수집했다. 이건 capture harness 재시도이며 layout scene 실패가 아니다.

두 플랫폼의 draw 결과는 surface 제출을 보여주고 image/glyph paint counter는 0이다. 이 근거는 layout geometry·trace·제출을 확인하며 실제 image/glyph pixel painting, Android 실기기/hardware GPU 또는 iOS compositor 이후 출력을 확인하지 않는다. WPT, 일반 CSS admission, 전체 C11.4도 이 결과의 범위가 아니다. 이전 iOS 원본 파일 체크섬은 [SHA256SUMS.txt](./ios-simulator/SHA256SUMS.txt)에 있다.
