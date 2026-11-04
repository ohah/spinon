# C11.1 parsed Root HTML direct-text aspect-ratio · simulator pass 04

이 자료는 기본 `RuntimeRootHtmlTextPaintV1` 경로의 좁은 aspect-ratio 후보를 기록한다. C11.1 또는 C11 완료, 일반 CSS 지원, WPT 적합성을 뜻하지 않는다. 공개 로드맵은 상위 78개·ID 세부 134개·ID 없는 하위 체크 2개를 합쳐 현재 `78/214`로 집계하며 이 후보로 완료 수는 바뀌지 않았다.

## 입력과 결과

- 사전 비교 및 앱 입력 fixture SHA-256: `d44069e298552707307ddcb2116dd7758c630af28a8c943e527984c8a5f80005`.
- 후보는 base HEAD `54d05c424a3e7b296b10264c8e4b3407f3db26a0` 위의 현재 working-tree 변경이다. 수정된 tracked/new Rust source SHA-256 목록은 [rust-source-sha256.txt](./rust-source-sha256.txt), fixture와 build marker hash는 [source-hashes.txt](./source-hashes.txt)에 있다.
- 동일한 parsed HTML fixture를 Android·iOS 기본 앱 route에서 열었다. 각 앱의 `root-app.js`는 fixture marker만 세우며 레이아웃을 만들거나 바꾸지 않는다. 임시 bootstrap 입력은 복구했고 원래 `examples/bootstrap/index.html`/`root-app.js` 해시는 각각 `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`, `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`다.
- Chrome 156 precomparison은 고정 columns `[10px,60px]`, 70px root, 단일 auto row와 width/height ratio controls에서 ratio target `60×30 CSS px`를 관찰했다. 플랫폼 시스템 monospace의 intrinsic control width는 Chrome과 native 간 동등 판정에서 제외한다. 세부 독립 기대값은 [precomparison README](../../c11-root-html-grid-text-aspect-ratio-precomparison-2026-11-05/README.md)에 있다.

## Rust 검증

- Cascade numeric-ratio 허용과 `auto <ratio>` 거부: [focused-style-cascade.log](./focused-style-cascade.log), 2 tests passed.
- layout width/height ratio geometry: [focused-style-to-layout-positive.log](./focused-style-to-layout-positive.log), 1 passed.
- 비지원 ratio, sizing, Grid/content/item alignment 입력 거부: [focused-style-to-layout-negative.log](./focused-style-to-layout-negative.log), 1 passed.
- 같은 parsed fixture를 기본 runtime에서 읽고 지정 NodeId frame을 확인: [focused-runtime.log](./focused-runtime.log), 1 passed.
- `cargo fmt --all -- --check`와 `cargo clippy --locked --workspace --all-targets -- -D warnings`: [rustfmt.log](./rustfmt.log), [workspace-clippy.log](./workspace-clippy.log), 둘 다 통과.
- 전체 `spinon-style-to-layout`: 후보 387 passed/10 failed, 기준 HEAD 385 passed/10 failed. 실패 test name 10개는 동일하며 후보에 추가된 ratio tests 2개는 통과했다. [candidate log](./full-style-to-layout-candidate.log) · [기준 HEAD log](./full-style-to-layout-test.log) · [후보 실패 이름](./candidate-failure-names-style-to-layout.txt) · [기준 실패 이름](./head-failure-names-style-to-layout.txt).
- 전체 `spinon-layout`: 후보와 기준 HEAD 모두 440 passed/1 failed. 동일한 기존 C15 `host_document_collects_direct_text_and_inherited_spans_in_source_order` assertion이 `host_document.rs:305`에서 실패한다. [candidate log](./full-layout-candidate.log) · [기준 HEAD log](./full-layout-test.log) · [후보 실패 이름](./candidate-failure-names-layout.txt) · [기준 실패 이름](./head-failure-names-layout.txt). 전체 package suite는 green으로 판정하지 않는다.

## Android API 37 AVD

- `sdk_gphone64_arm64`, Android 17/API 37, physical display 1080×2400 at density 420; ARM64 emulator.
- 앱 APK SHA-256: `e33827b3b68e0c653bf9788af3e911719f47c66989416764fa29393f63b5249f`. APK 안 `assets/index.html`은 위 fixture hash와 일치한다.
- 기본 app route에서 `layout=ready boxes=14`, root frame `980×100 CSS px`; draw는 `status=0 presented boxes=14`, `runtime_text_count=8`, `glyph_batch_draw_count=8`을 기록했다. backend는 ANGLE/Vulkan SwiftShader software renderer다.
- [screen.png](./android/screen.png)는 시각 smoke 자료다. viewport meta가 없는 fixture라 화면 좌상단에 작게 보이는 것이 관찰 결과이며, 개별 item frame을 앱 screenshot/log에서 측정했다고 주장하지 않는다. 개별 frame은 위 Rust default-runtime test에서 확인했다. PixelCopy는 하지 않았다.
- 로그: [build-native.log](./android/build-native.log), [build-app.log](./android/build-app.log), [runtime-logcat.txt](./android/runtime-logcat.txt), [기기 정보](./android/adb-devices.txt).

## iOS 26.2 Simulator

- iPhone 17 Pro Simulator `AC268F2F-76B5-4B8E-90A1-1D35D9CD36ED`, iOS 26.2; Metal simulator surface 1206×2334.
- 앱 executable SHA-256: `3207e0628dede6884bfe2a9113420dbe8eaf3446f7f450064b1e0901aedf1c64`. Bundled `index.html`과 `root-app.js` hash는 Android 패키지와 일치한다.
- 기본 app route에서 `layout=ready boxes=14`, root frame `980×100 CSS px`; draw는 `status=0 presented boxes=14`, `runtime_text_count=8`, `glyph_batch_draw_count=8`을 기록했다. renderer는 Apple iOS simulator GPU/Metal이다.
- [screen.png](./ios/screen.png)는 시각 smoke 자료다. viewport meta가 없어 콘텐츠가 화면 좌상단에 작게 표시된다. Metal drawable readback·compositor 이후 픽셀 검사는 하지 않았다.
- 로그: [build.log](./ios/build.log), [runtime-log-unified.txt](./ios/runtime-log-unified.txt), [simulator 목록](./ios/booted-devices.txt).

## 남은 범위

지원 입력은 정확한 `2/1` numeric ratio, 한 owner의 direct HTML `div` text items `A`/`AAAA`, fixed columns `[10px,60px]`, 단일 auto row, 70px root, zero gaps/box edges, `align-items:start`·`justify-items:start`, `align-content:normal`·`justify-content:normal`, item self-alignment `auto`, LTR/horizontal-tb·visible overflow 및 지정 size pair로 제한한다. 잘못된 값·비슷한 scene은 fail-closed로 거부한다. Android 결과는 software renderer이고 iOS는 Simulator다. PixelCopy, Metal readback, 실기기, 넓은 CSS/WPT 범위는 확인하지 않았다.

독립 구현/runtime 실패 검토는 [implementation-runtime-review-20.md](./implementation-runtime-review-20.md), PR 변경분 검토는 [pr-diff-review-30.md](./pr-diff-review-30.md)에 기록한다. 모든 로그·화면·요약 artifact의 무결성 목록은 [SHA256SUMS](./SHA256SUMS)다. 계획 검토는 [계획](../../../../../plan/c11-root-html-grid-text-aspect-ratio.md) 및 그 연결 자료를 참조한다. 구현 PR [#1161](https://github.com/ohah/spinon/pull/1161)은 2026-11-04에 Rebase and merge되었다 (merge SHA `f774af7f1b7441708495b74a063796122a4f1128`).
