# C11.4 later fit-content 위치 · 구현 후보 근거

## 변경 범위

기존 기본 parsed Root HTML repeated-text row-span profile에서 여섯-member `grid-auto-rows`의 `fit-content()`를 두 번째 또는 세 번째 위치에 둘 수 있게 한 bounded implementation candidate다. 각 위치에서 나머지 member는 `min-content` 두 개, `max-content` 두 개, `auto` 한 개로 고정한 30개 순서뿐이다. 첫 위치의 기존 순서와 기존 9-item auto-placement profile은 보존한다. 새 위치에서 9-item auto-placement 입력은 거부한다.

나머지 topology와 content grammar는 [선행 row-span slice](../c11-4-root-html-repeated-text-row-span-implementation-2026-11-09/README.md)를 따른다. 이 작업은 공개 CSS 지원, 일반 author CSS admission, arbitrary Grid row span, image/nested content 또는 전체 C11.4 완료가 아니다.

## 독립 기준과 RuntimeSession matrix

기준은 [Chrome 157 사전 비교](../c11-4-root-html-repeated-text-later-fit-content-row-span-precomparison-2026-11-09/README.md)의 고정 1,920-scene corpus와 별도 관계식이다. RuntimeSession은 각 장면에서 computed six-member 순서, explicit row와 positive implicit rows 3개, semantic item·payload frame, 줄 수, placement, source-order reversal을 검사한다. Feature `c11-4-grid-track-sizing-fixture-test`를 켠 실행은 마지막 row sizing trace의 네 used row size까지 검사한다.

새 기본-path RuntimeSession matrix는 1,920/1,920 scene을 통과했다. Trace-enabled matrix도 1,920/1,920 scene을 통과했다. Position 4–6 및 이 span profile 밖의 9-item auto-placement는 거부하며, 실패에서 partial layout을 공개하지 않고 같은 session의 다음 유효 입력으로 복구되는지 확인한다. 이전 position-1 repeated-text row-span 960-scene와 810-scene 회귀도 통과했다.

[`run-focused-verification.sh`](./run-focused-verification.sh)의 로그는 Style projection 2/2, Layout fit-content 14/14, Grid track sizing 123/123, 신규 matrix 1/1, 신규 rejection/recovery 4/4, 기존 position-1 span 1/1, 기존 810-scene text 1/1, trace-enabled matrix 1/1을 기록한다. 변경 crate all-target Clippy `-D warnings`, `cargo fmt --all -- --check`, `git diff --check`도 통과했다. 로그는 [`rust-verification/`](./rust-verification/)에 있다. Broad `spinon-layout --lib` 또는 workspace test를 이번 후보의 green 결과로 주장하지 않는다.

## Android·iOS 기본 route smoke

검증은 opt-in fixture FFI가 아닌 C04.12 기본 Root HTML route로 수행했다. [스크립트](./platform-smoke/run-platform-smoke.sh)는 시작 전 기기 상태를 확인하고 테스트 입력만 플랫폼 asset에 임시로 복사했다. Android는 기존 API 37 `arm64-v8a` AVD `spinon_api37_2_presenttime_probe_20261018`, iOS는 iPhone 17 Pro / iOS 26.2 Simulator를 사용했다.

각 플랫폼에서 `fit-content()` 위치 2·3 × `align-content:start|stretch`의 네 sentinel을 각각 빌드·설치·실행했다. 모든 실행은 `layout=ready`, `boxes=10`, 세 text draw와 `SPINON_C0410_DRAW status=0 presented`를 남겼다. Android/iOS의 해당 조합은 같은 HTML SHA를 사용했다.

| 위치 / alignment | 입력 SHA-256 | Android 캡처 | iOS 캡처 |
|---|---|---|---|
| 2 / start | `a36d3fb51fdc249e732b66fb16bbba608ce91a1d4510365add972f2d93f8a64d` | [화면](./platform-smoke/android-position-2-start.png) | [화면](./platform-smoke/ios-position-2-start.png) |
| 2 / stretch | `62f5f319c4375112fba97596853fe945b6dd9ca80c6048c46e749a572207e6aa` | [화면](./platform-smoke/android-position-2-stretch.png) | [화면](./platform-smoke/ios-position-2-stretch.png) |
| 3 / start | `c9080f41002ee0936cf0239097588850b8ce15d592576a5be6bf0fece8fe3f7d` | [화면](./platform-smoke/android-position-3-start.png) | [화면](./platform-smoke/ios-position-3-start.png) |
| 3 / stretch | `60a1b429768cd434c71005a71c73d5a786bedda88db2f6847e8011ea4a8991b7` | [화면](./platform-smoke/android-position-3-stretch.png) | [화면](./platform-smoke/ios-position-3-stretch.png) |

이 캡처는 화면과 runtime draw/present를 확인하는 visual smoke다. Android PixelCopy, iOS Metal drawable readback, OS compositor 이후 pixel 비교, Android 실기기·hardware GPU, 네 scene 이상의 native matrix로 해석하지 않는다.

## Cleanup과 실행 기록

Smoke script는 실행 전에 Android device가 연결되어 있지 않고 선택 iOS simulator가 boot되지 않았을 때만 시작한다. 종료 trap에서 테스트 앱을 멈추고 이번 실행에서 시작한 AVD와 iOS Simulator를 shutdown한 뒤 두 플랫폼 resource 파일을 원본으로 되돌린다. 복구된 두 파일 SHA-256은 모두 `38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85`이며 [복구 기록](./platform-smoke/source-restore.txt)에 남겼다. `examples/bootstrap/index.html`은 교체하지 않았다.

빌드·런타임·Rust 검증 로그의 내용은 보존하고, 저장소 whitespace 검사에서 걸리는 행 끝 공백과 중복 EOF 공백만 제거했다.

최종 기기 확인에서 `adb devices -l`은 빈 목록이고 `xcrun simctl list devices booted`에도 기기가 없었다. AVD 정의와 iPhone simulator 정의는 공유 개발 리소스라 제거하지 않았다. 실행별 build/install/launch/runtime 로그, device metadata, HTML sentinel과 원본 source hash는 `platform-smoke/` 아래에 보존한다.

## 남은 범위

별도의 [구현/runtime 실패 관점 20개](./implementation-runtime-review-20.md)와 [PR 변경 diff 실패 관점 30개](./implementation-pr-diff-review-30.md)를 기록했다. 이 후보는 일반 CSS parser admission이나 C11.4 상위 완료를 올리지 않는다. 공식 완료 수는 `78/214`로 유지한다. 구현 PR [#1299](https://github.com/ohah/spinon/pull/1299)은 2026-11-08T23:28:36Z Rebase and merge됐고 merge SHA는 `f0c18c8fead17fb379b77eb6191193a066a5115d`다. GitHub check rollup은 0건이며 CI 통과로 기록하지 않는다.
