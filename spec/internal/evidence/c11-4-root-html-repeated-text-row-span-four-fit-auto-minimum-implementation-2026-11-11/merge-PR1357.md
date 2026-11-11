# PR #1357 병합 기록

- PR: [#1357 — C11.4 parsed Root HTML repeated-text row-span four-fit `min-height:auto`](https://github.com/ohah/spinon/pull/1357)
- 병합 방식: Rebase and merge
- 병합 시각: `2026-11-11T11:46:18Z`
- PR 최종 head: `f5a85607fb1a3678a5a30b55ad032a34d08b9585`
- 병합 후 `main` SHA: `30bc0acceeb1cdc90b55d3ee29bc938e05c61f83`
- PR diff 검토 후 원격 `gh pr diff 1357`와 `origin/main...HEAD`는 동일했다. SHA-256은 `87e28573a8d2a869d77c2e5f6e2c7f5b762d2eb6bf8c7a59d165d720ebfeaeaa`이며 2,697 lines / 42 files다.
- GitHub check rollup에는 check가 보고되지 않았다. CI green으로 세지 않는다.
- PR labels: `enhancement`, `area:testing`, `area:layout`.
- PR 본문에 첨부한 iOS/Android light/dark 이미지 4장은 Agent Browser에서 모두 로드됐으며, 브라우저 세션은 닫혔다.

## 병합된 범위

정확히 네 개의 typed `fit-content()` cap (`0px`, `10px`, `20%`, `30px`), `min-content`와 `auto` 각 하나로 구성된 고정 six-member repeated-text row-span profile만 지원한다. Chrome 157 RuntimeSession matrix 11,520/11,520 scene, 기존 four-fit auto-placement regression 5,760/5,760 scene, focused projection/layout, rejection/recovery, trace, rustfmt 및 workspace Clippy 결과는 같은 폴더의 `verification.json`과 원본 로그에 기록했다.

시뮬레이터 화면은 iOS 26.2 iPhone 17 Pro Simulator 및 Android API 37 ARM64 AVD의 **기본 Root HTML 앱 시작 smoke**다. C11.4 four-fit fixture geometry는 화면에서 검증하지 않았다. Android는 SwiftShader였다. PixelCopy, Metal drawable readback, compositor 이후 pixel parity, 실기기, hardware GPU, WPT 및 일반 CSS Grid 적합성은 확인하지 않았다.

병합은 C11.4 또는 C11 전체 완료를 뜻하지 않는다. 공식 `78/214`, 미완료 상위 상태와 internal numeric contract `0.1.0`을 유지한다.

## 후속 정리 확인

후속 문서 동기화 시 `simctl list devices booted`는 비어 있었고, Android `adb devices -l`에도 기기가 없었다. 등록된 S03 iOS Simulator profile은 없었으며 Agent Browser 세션도 남아 있지 않았다. 공유 simulator/AVD profile과 다른 작업 프로세스는 삭제하거나 종료하지 않았다.
