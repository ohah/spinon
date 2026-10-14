# C13.1 전체 viewport hit identity follow-up · 2026-10-14

## 목표

기존 C13.1 13개 유효 Chrome 행의 hit identity 검증을 각 target frame 내부 정수 grid에서 전체 browser viewport 정수 grid로 넓힌다. target 사각형 밖에서 overflow clip parent, 중첩 clip box, viewport/root fallback이 잘못 hit되는지 exact `NodeId`로 확인한다. 이 작업은 host render snapshot hit query 범위의 검증이다.

C13.1 구현의 새 CSS 지원, Android/iOS 앱 통합, surface paint 비교, DOM/CSSOM event dispatch, WPT 또는 C13 완료를 주장하지 않는다. 기존 renderer query가 잘못된 identity를 내면 원인을 재현하고 해당 제한 profile을 수정한다.

## 고정 oracle과 fixture

- Browser: agent-browser `0.31.1`의 Chrome for Testing `155.0.8059.40`, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`.
- User agent: 기존 HeadlessChrome 155 고정 문자열.
- Viewport: `1280×1100` CSS px, DPR 1, scroll offset `(0,0)`.
- Fixture: `tests/fixtures/css/c13/overflow-clip-c13-1.html`, SHA-256 `b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a`.
- 유효 행: `visible`, `clip-x`, `clip-y`, `clip-both`, `clip-margin-padding`, `clip-margin-content`, `clip-margin-border`, `clip-margin-hidden`, `mixed-visible-hidden`, `mixed-clip-auto`, `positioned-contained`, `positioned-escape`, `nested-clip`.
- invalid margin 행은 runtime reject이고 float 세 행은 현재 의도적으로 fail-closed이므로 성공 scene의 exact identity matrix에서 제외한다.
- 13개 유효 행을 Chrome과 같은 page 위치에 담은 단일 Rust scene을 만든다. 브라우저에서 각 행의 target/descendant는 `<case>:target`, nested inner는 `<case>:inner`, clip box는 `<case>:box`, section의 다른 영역은 `<case>:case`로 분리한다. invalid/float rows와 page 바깥 영역은 viewport root `root`로 접는다. Chrome `null`은 별도 `null` identity로 보존해 Rust `None`과 비교한다.

## 합격 기준

1. 사전 비교 전용 계획과 별도 20관점 계획 검토를 기록한다.
2. Chrome browser, fixture hash, DPR, viewport, scroll offset과 유효 ID 13개를 capture 전후 확인한다.
3. 각 행에서 `(x,y)` 모든 정수 좌표 `x=0..1279`, `y=0..1099`를 조회한다. 기대 표본은 단일 full-page scan `1280 × 1100 = 1,408,000`점이다. 같은 화면을 13 scene으로 중복 scan하지 않는다.
4. 각 y 행은 RLE x-run으로 저장한다. 모든 run과 y 행에 빈틈·겹침이 없고 각 행 총 폭은 1280, viewport 행 수와 총점은 각각 1100과 1,408,000이어야 한다.
5. Rust test는 이전 target frame/기존 187,320점 검사를 유지하면서 전체 viewport 각 점의 Chrome identity를 13개 case별 `case`, clip `box`, `target`, nested `inner`, viewport `root`의 exact `NodeId`로 비교한다. Chrome `null`은 `None`으로 exact 비교한다.
6. 좌상/우상/좌하/우하 viewport 경계, target 밖 clip box·page fallback, visible overflow와 clip에서 달라지는 target reach를 negative/control 행으로 포함한다.
7. unresolved identity, 누락 행/run, 잘못된 frame 또는 표본 수는 validator와 Rust test에서 즉시 실패한다.
8. Chrome screenshot은 PR 본문에 inline으로 첨부하고 실제 표시를 agent-browser로 확인한다. Capture는 browser oracle이며 앱 화면이나 GPU surface 근거로 해석하지 않는다.
9. 한 batch에서 13개 scene의 viewport grid를 반환하려던 첫 capture는 agent-browser daemon의 `Resource temporarily unavailable` 응답 한계에 걸렸다([초기 시도 결과](../spec/internal/evidence/c13-1-overflow-clip-full-viewport-hit-identity-2026-10-14/capture-initial-batch-error.json)). oracle 범위는 줄이지 않고 단일 page/combined scene scan으로 바꾸어 재수집한다. 구현 실패 관점 20개와 PR 변경 관점 검토를 각각 수행한다. 코드 문제가 없더라도 검증 수치만으로 C13.1 완료를 주장하지 않는다.

## 제외와 후속 경계

이 작업은 target frame 밖 full-page viewport hit query만 다룬다. 전체 paint surface sample, PixelCopy/Metal drawable readback, Android/iOS 앱 전체 clip matrix, DOM event dispatch, WPT 실행과 scroll state는 여전히 별도다. 모든 전체 viewport pixel을 다뤄도 C13.1 완료 게이트는 닫히지 않는다.

## 구현 순서

1. [계획 실패 관점 20개 검토](../spec/internal/evidence/c13-1-overflow-clip-full-viewport-hit-identity-plan-review-2026-10-14.md)를 기록하고 기준/제외 범위를 고정한다.
2. agent-browser Chrome capture를 만들고 JSON integrity validator로 metadata·ID·RLE 연속성·1,408,000점 합계를 확인한다.
3. 기존 Chrome 13 row 위치를 그대로 재현하는 combined runtime scene builder를 추가한다. root는 1280×1100 CSS px, 각 section/box/target/inner frame은 Chrome 사전 관찰과 대조한다.
4. C13.1 Rust render test 하나에서 combined scene의 full viewport exact identity를 비교하고 capture negative controls를 검증한다. 기존 187,320 target-frame query는 유지한다.
5. 기존 target-frame 검증, renderer tests, Clippy, rustfmt, workspace gate와 artifact boundary를 실행해 근거를 저장한다.
6. 새 20개 구현 실패 관점과 PR 변경 검토를 남긴다. STATUS와 C13 계획은 새로 확인한 범위만 기록하고 C13.1/C13 완료는 유지하지 않는다.
7. PR 본문에 inline Chrome fixture screenshot을 올린다. 최신 main, mergeability, required CI/review, linear history를 다시 확인하고 rebase 방식으로 병합한다.
