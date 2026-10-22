# C11.4 · auto-fit named-line implicit fallback 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [auto-repeat authored line names](./c11-4-auto-repeat-named-lines.md) · [auto-fill implicit fallback](./c11-4-auto-repeat-named-line-implicit-fallback.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 구현/runtime 검증 완료(고정 fixture slice). Chrome 사전 비교와 계획 검토에 이어 두 simulator에서 auto-fill 회귀 10개와 auto-fit 신규 10개를 각 20/20 통과했다. 구현/runtime 검토 20개도 별도로 마쳤다. 구현 PR [#537](https://github.com/ohah/spinon/pull/537)에 결과를 연결했다. C11.4 및 공식 `77/213`은 미완료로 유지한다.

## 결손과 가설

PR [#530](https://github.com/ohah/spinon/pull/530)은 authored auto-repeat line names의 `auto-fit` collapse를 검증했고, PR [#534](https://github.com/ohah/spinon/pull/534)은 단일 `auto-fill` axis의 missing/insufficient named-line fallback을 고정 scene으로 연결했다. 두 결과는 각 feature의 분리 사례다. 둘을 결합했을 때 placement 뒤 empty repeated track이 collapse된 geometry와 named-line implicit fallback이 일치하는지는 아직 Spinon simulator fixture에서 확인되지 않았다.

W3C CSS Grid 2025 CRD §7.2.3.2는 `auto-fit`이 item placement 뒤 비어 있는 repeated track을 collapse한다고 정의한다. §8.3은 integer named line의 occurrence가 부족하면 implicit grid lines를 해당 이름으로 간주하고, named span은 검색 방향 쪽 implicit line을 사용한다고 정한다. 독립 기대값은 [사전 비교](../spec/internal/evidence/c11-4-auto-fit-named-line-implicit-fallback-precomparison-2026-10-23/README.md)에 고정했다. 구현 가설은 기존 symbolic named placement와 Taffy auto-fit sizing 경로만으로 충분하다는 것이다. 첫 구현 단계에서 bridge·validator·Taffy resolver를 다시 추적하고, 실패가 재현되지 않으면 제품 로직을 바꾸지 않고 fixture 회귀만 추가한다.

## 구현 범위

- 단일 `auto-fit` repeat 축, definite `125px` axis, `[cell] 40px [cell]`, implicit track `20px`, gap 0인 고정 profile만 다룬다. 배치 전 repeat 수는 3이다.
- 신규 고정 scene 10개: positive/negative absent name, positive/negative insufficient `cell`, forward/backward named span fallback, 하나의 repeated track을 점유하는 control, 두 repeated track을 span하는 control, row axis positive/negative absent-name controls.
- 기존 `auto-fill` implicit fallback 10개 scene을 그대로 regression matrix에 유지한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 20개 전체 scene의 track list, line groups, item frame, trace와 draw/present를 확인한다.
- PixelCopy와 compositor 제출 전 Metal drawable에서 item 내부, 인접 track 및 implicit track의 고정 좌표를 판정한다. 0px collapsed track에는 interior pixel이 없으므로 sample에서 제외한다. screenshot과 실행 로그는 scene ID·앱 hash와 연결한다.
- fixture-only entry, marker 또는 source가 변경되면 기본 release와 fixture-test static archive negative/positive control을 나란히 확인한다. 제품 APK/IPA 제외는 별도 검사 없이는 주장하지 않는다.
- simulator 결과는 simulator에 한정한다. 실기기, hardware GPU, WPT, iOS compositor 이후 출력 또는 일반 CSS admission으로 확대 해석하지 않는다.

## 사전 기준과 합격 규칙

고정 HTML fixture는 Chrome 156, viewport `1280×1000`, DPR 1에서 실행했다. CSS Grid 규칙과 기존 auto-fill fallback 비교를 사용해 독립 기대값을 산출한 뒤 Agent Browser Chrome 관측을 JSON과 screenshot에 저장했다. Track/group serialization은 exact, scene별 item frame은 최대 `1/64 CSS px` 차이만 허용한다. 두 비교 중 하나라도 실패하면 scene은 실패다. Browser session/process는 관측 후 닫았다.

합격 집합은 20 scene이다. 기존 auto-fill 10개는 회귀 기준으로 포함하고, 신규 auto-fit 10개는 사전 비교의 각 track list와 frame을 독립 확인한다. `occupied-repeat` 결과가 `[0,40,0]`, `span-occupied-repeat`가 `[0,40,40]`이 아니면 collapse placement 경계가 실패다. 양수·음수 row control은 column control을 축만 바꿔 반복하며 두 축이 상호 영향받지 않는지도 확인한다.

## 구현 후 판정 순서

1. 이 계획 PR이 병합된 뒤 최신 `main`에서 implementation branch를 만든다. 기존 미추적 evidence, tmux와 simulator/AVD 상태는 보존한다.
2. `ComputedCssGridItemPlacement`부터 grid validator, auto-repeat expansion, Taffy named-line resolver와 post-placement collapse 순서를 추적한다. 코드 수정 전에 fixed fixture가 실패하는지 확인하고, 원인이 있으면 최소 경계만 고친다.
3. fixture harness는 opt-in `c04-runtime-gpu-fixture-test` 아래에서만 scene을 허용한다. 기존 10개 auto-fill cases를 보존하고 신규 10개 auto-fit cases를 고정 scene ID로 추가한다.
4. 해당 Rust focused tests/workspace checks와 Android API 37 ARM64 AVD, iPhone 17 Pro/iOS 26.2 Simulator를 실행한다. 각 플랫폼 20/20 scene의 track/frame/trace/draw-present가 모두 통과해야 한다. PixelCopy/Metal은 실제 읽은 좌표·픽셀에 한해 비교한다.
5. 변경이 fixture ABI에 닿으면 기본 release/fixture-test archive 음성·양성 대조를 하고 원본 심볼/문자열 근거를 남긴다. APK/IPA 전체 제외, 실제 device 또는 compositor parity는 검사하지 않은 범위로 유지한다.
6. implementation/runtime을 기존 plan review와 겹치지 않는 20개 실패 관점으로 검토한다. 최종 PR diff는 그 리뷰들과 구별되는 30개 관점으로 검토한다. 발견 결함을 수정하고 영향받은 각 관점을 다시 검토한다.
7. PR 본문에 상태 ID, 계약·Chrome oracle·simulator 근거, 화면 링크와 남은 한계를 적고 유형/영역 라벨을 단다. Tailnet 미리보기를 최신 `main`에 맞춰 hash 단위로 sync한 뒤 Agent Browser로 page/image를 확인하고 session 및 process를 종료한다. Required checks와 merge state를 live 확인하고 Rebase and merge한다.

## 구현/runtime 결과 · 2026-10-23

- Runtime의 symbolic named placement와 auto-fit collapse 경로가 고정 profile을 처리해 제품 layout algorithm source는 바꾸지 않았다. FFI fixture count와 scene data, Android/iOS allowlist·frame·surface sample mapping, simulator capture runner만 확장했다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 20/20 scene, 40 node frame, track trace와 draw/present를 확인했다. PixelCopy와 Metal drawable 표본은 각 60/60 exact다. 상세 app/source hashes와 실행 로그는 [runtime evidence](../spec/internal/evidence/c11-4-auto-fit-named-line-implicit-fallback-runtime-2026-10-23/README.md)에 있다.
- 기본 release static archive와 fixture-test positive control의 marker·fixture symbol 경계를 확인했다. 기본/양성 대조 hash는 evidence의 [artifact-boundary JSON](../spec/internal/evidence/c11-4-auto-fit-named-line-implicit-fallback-runtime-2026-10-23/artifact-boundary.json)에 보존했다.
- [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-4-auto-fit-named-line-implicit-fallback-runtime-2026-10-23/implementation-runtime-review-20.md)는 계획 리뷰와 분리했다. [PR diff 실패 관점 검토 30개](../spec/internal/evidence/c11-4-auto-fit-named-line-implicit-fallback-runtime-2026-10-23/pr-diff-review-30.md)는 구현/runtime 검토와 분리했다.
- 남은 제한: simulator 외 Android 실기기/hardware GPU, iOS compositor 이후 출력, CSS border stroke, WPT, 일반 CSS admission과 C11.4 전체 완료는 확인하지 않았다.

## 계획 실패 관점 검토

[계획 자체의 서로 다른 실패 관점 20개](./c11-4-auto-fit-named-line-implicit-fallback-plan-review-20.md)는 implementation review와 재사용하지 않는다.
[이 계획 PR 변경의 독립 실패 관점 30개](../spec/internal/evidence/c11-4-auto-fit-named-line-implicit-fallback-precomparison-2026-10-23/plan-pr-diff-review-30.md)는 위 계획 검토와 분리했다.

## 사전 비교 산출물

- [독립 계산·Chrome 156 관측·범위 제한](../spec/internal/evidence/c11-4-auto-fit-named-line-implicit-fallback-precomparison-2026-10-23/README.md)
- [Chrome 입력 fixture](../spec/internal/evidence/c11-4-auto-fit-named-line-implicit-fallback-precomparison-2026-10-23/grid-auto-fit-named-line-implicit-fallback.html)
- [원본 관측 JSON](../spec/internal/evidence/c11-4-auto-fit-named-line-implicit-fallback-precomparison-2026-10-23/chrome156-observation.json)
- [Agent Browser Chrome screenshot](../spec/internal/evidence/c11-4-auto-fit-named-line-implicit-fallback-precomparison-2026-10-23/chrome-auto-fit-named-line-implicit-fallback.png)
