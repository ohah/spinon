# C11.1 Root HTML direct-text `aspect-ratio` 값 · 플랫폼 실행 · 2026-11-05

기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1` 경로에서 finite positive `aspect-ratio` 값의 행렬을 실행했다. 주 fixture는 14개 scene이며 SHA-256은 `08cb15bf6def149576f78bb1d96323946ae1ece6fa933607e621b418ac8687eb`이다. 자동 최소 크기 보충 fixture는 2개 scene이며 SHA-256은 `fb79dfdaaabe42b1044c56fd6217da04b9671e5f1f0794456d4f0451992562b2`다. 각 fixture를 Android와 iOS Simulator의 기본 앱 경로로 실행했다.

## 실행 환경

- Android: API 37, `sdk_gphone64_arm64`, ARM64, 1080×2400 physical pixels, density 420 DPI. 렌더러 로그는 Android Emulator OpenGL ES Translator / ANGLE / SwiftShader다.
- iOS: iPhone 17 Pro, iOS 26.2 Simulator, UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`, 1206×2622 physical pixels, 3× scale.
- 네 실행 모두 후보 소스에서 Debug 앱을 빌드하고 fixture hash와 APK/app bundle의 `index.html` bytes를 비교한 뒤 설치·실행했다. 임시 bootstrap verifier는 번들 안의 Root HTML marker와 document-tree marker도 확인한다. 러너는 `run-simulator-matrix.py`다.
- 실행 뒤 `examples/bootstrap/index.html`과 `root-app.js`를 저장 전 byte content로 복원했다. 복원 SHA-256은 각각 `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`, `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`이며, 러너가 재실행 후에도 두 해시를 검사했다.

## 관찰 결과

| fixture | 플랫폼 | Root HTML layout / frame (CSS px) | draw/present 로그 |
| --- | --- | --- | --- |
| 주 14 scene | Android API 37 AVD | `ready`, 72 boxes, viewport 411.42856×841.52374, root 411.42856×853 | `presented boxes=72`, text 42, glyph batch 42 |
| 주 14 scene | iPhone 17 Pro iOS 26.2 Simulator | `ready`, 72 boxes, viewport 402×778, root 402×853 | `presented boxes=72`, text 42, glyph batch 39 |
| 자동 최소 크기 2 scene | Android API 37 AVD | `ready`, 12 boxes, viewport 411.42856×841.52374, root 411.42856×104 | `presented boxes=12`, text 6, glyph batch 6 |
| 자동 최소 크기 2 scene | iPhone 17 Pro iOS 26.2 Simulator | `ready`, 12 boxes, viewport 402×778, root 402×104 | `presented boxes=12`, text 6, glyph batch 6 |

주 fixture 캡처: [Android](./android/screenshot.png), [iOS](./ios/screenshot.png). 보충 fixture 캡처: [Android](./android/automatic-minimums/screenshot.png), [iOS](./ios/automatic-minimums/screenshot.png). 원본 build/install/launch/run/stop 로그는 각 플랫폼 폴더와 `automatic-minimums/` 아래에 있다. 앱은 각 캡처 뒤 종료했다.

`presented`는 앱 runtime의 GPU draw 경로가 Surface에 frame을 제출했다는 로그다. 앱 로그는 C11.1 scene별 `NodeId` frame을 별도로 내보내지 않으며, PixelCopy 표본 비교, Metal drawable readback, OS compositor scanout 검증은 하지 않았다. 14개 장면과 자동 최소 크기 frame의 개별 기대값은 Rust layout/style-to-layout/runtime 행렬이 검사한다. Simulator 결과는 실기기 결과가 아니며 screenshot은 표면의 개별 CSS frame을 증명하지 않는다.

## 코드 검증

- Focused regression: [style-to-layout 6/6](./focused-style-to-layout.log), [Root HTML runtime 1/1](./focused-runtime.log), [typed cascade 1/1](./focused-style.log), [finite transfer 1/1](./focused-layout.log) 통과.
- [`cargo fmt --all -- --check`](./format-final.log), 변경 네 crate all-target [Clippy `-D warnings`](./clippy-final.log), [`git diff --cached --check`](./diff-check-final.log) 통과.
- 전체 `spinon-layout`: 441/442 통과. 기존 C15 `host_document_collects_direct_text_and_inherited_spans_in_source_order` assertion 1건이 실패했다.
- 전체 `spinon-style-to-layout`: 391/401 통과. C07 aspect-ratio computed serialization assertion 1건과 C11.1/C11.5 범위 밖 Grid admission assertion 9건이 실패했다. broad suite는 green이 아니다. 원본 출력은 [layout](./spinon-layout-final.log), [style-to-layout](./spinon-style-to-layout-final.log)에 보존했다.

최근 자동 최소 크기 및 한 번 계산한 target set 최적화까지 포함한 새 구현/runtime 검토는 [20개 관점](./implementation-runtime-review-20-after-auto-minimum-fix.md)이다. 별도 [PR diff 검토 20개](./pr-diff-review-20.md)와 [PR 적대 검증 30개](./pr-hostile-review-30.md)도 기록했다. 이 bounded slice는 C11.1, C11 또는 공식 완료 수를 완료로 표시하지 않는다.
