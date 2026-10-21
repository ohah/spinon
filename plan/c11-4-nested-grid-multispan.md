# C11.4 · depth-three nested Grid text multi-span contribution 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [depth-three nested Grid 계획](./c11-4-nested-grid-depth-three.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** fixed-scene 구현, Android/iOS Simulator 4/4 검증, direct-leaf 회귀 4/4, feature artifact 경계 검사, 구현/runtime 20개 및 PR diff 30개 실패 관점 검토를 마쳤다. [PR #464](https://github.com/ohah/spinon/pull/464)는 2026-10-22 03:45:55 KST에 Rebase and merge로 병합했고 merge SHA는 `3c03b51979127a0c52f5737cdd2d003b9dcf39c3`이다. 병합 당시 main에 required status checks는 설정되어 있지 않았고 PR check rollup은 보고되지 않았다. C11.4 checkbox와 공식 완료 수 `72/213`은 유지한다.

## 목적과 미구현 경계

기존 direct-leaf text multi-span fixture는 단일 Grid에서 겹치는 span-2와 span-2/3 item 기여를 검사한다. 별도 depth-three fixture는 한 direct text leaf의 min/max-content 기여를 세 Grid 경계로 전달한다. 기존 depth-three anonymous text + image fixture는 inner Grid의 두 자동 배치 item을 결합하지만 multi-span track 분배를 하지 않는다. 따라서 inner Grid의 겹치는 multi-span text contribution이 계산된 뒤 그 intrinsic width가 root → middle → branch의 세 parent Grid 경계를 통과하는 고정 실행은 아직 없다.

이번 slice는 `RuntimeBlockFlexGridTrackSizingV1` opt-in fixture-test 경로에 네 개의 고정 scene을 추가한다. topology는 `root → middle → branch → inner` 네 HTML `div` Grid와 inner의 direct-leaf text item 둘 또는 셋이다. 일반 CSS cascade admission, 공개 API, 임의 nested Grid, anonymous text, image, WPT, 전체 C11.4 지원을 추가하지 않는다.

## 입력과 기대값

HTML 재현 입력과 Chrome 관측은 [precomparison evidence](../spec/internal/evidence/c11-4-nested-grid-multispan-precomparison-2026-10-22/README.md)에 고정했다. Ahem은 10px/10px, box edge·gutter는 0px다. 각 outer Grid는 `width:min-content`, `height:60px`, 단일 min-content column과 60px row다. inner Grid는 같은 intrinsic width·고정 height에 네 min-content column과 세 20px row를 사용한다. leaf item placement와 문자열은 exact allowlist로 검사한다.

| scene | inner item 순서 | 독립 inner Columns | 각 Grid width |
| --- | --- | --- | --- |
| `nested-text-span2-forward` | span-2 A → span-2 B | `[25,35,35,0]` | `95×60px` |
| `nested-text-span2-reversed` | span-2 B → span-2 A | `[25,35,35,0]` | `95×60px` |
| `nested-text-span2-before-span3` | span-2 A → span-2 B → span-3 C | `[160/3,190/3,190/3,0]` | `180×60px` |
| `nested-text-span3-before-span2` | span-3 C → span-2 B → span-2 A | `[160/3,190/3,190/3,0]` | `180×60px` |

CSS Grid 2025 CRD §11.5.1에 따라 A와 B의 min-content 50px·70px span-2 contribution 뒤에 C의 180px span-3 contribution을 분배한다. 이상적인 분수 track과 Chrome의 1/64 CSS px quantization은 각각 다른 비교 입력이다. Chrome 관측, 독립 기대값, 모든 fixed node frame은 사전 비교 파일에 별도 저장되어 있다.

## 구현 및 검증 결과

1. 계획 PR #463을 Rebase and merge한 뒤 같은 checkout에서 `feat/c11-4-nested-grid-multispan` branch로 진행했다. worktree를 만들지 않았다.
2. opt-in runtime fixture adapter와 FFI scene generator에 네 exact scene을 추가했다. 네 Grid의 HTML identity·owner·순서·track/box profile, 직접 text child, Ahem style, item placement/span을 확인하며 unlisted/malformed input은 fail closed한다. core Grid sizing algorithm과 공개 API는 변경하지 않았다.
3. Rust와 simulator capture verifier가 모든 text intrinsic sizes/revision/order, inner Columns/Rows, 네 Grid frame, span contribution 및 세 parent contribution trace를 검사한다. 독립 분수 기대값과 Chrome 156 관측값은 사전 비교 문서에서 분리했다.
4. 신규 runtime 정상·negative tests 2개, depth-three controls 2개, mixed span control 1개, 신규 FFI allowlist 1개와 direct-leaf FFI control 1개가 통과했다. 기존 direct-leaf text multi-span 네 scene도 Android/iOS에서 각각 4/4 통과했다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 신규 scene 각 4/4가 통과했다. frame/track, complete trace, draw/present와 Android local/installed APK SHA를 근거에 남겼다.
6. workspace all-target 및 runtime/FFI fixture-feature Clippy, formatter, fixture boundary 6/6, JS syntax, diff whitespace 검사, Android Gradle 및 iOS Simulator build가 통과했다. 기본 FFI archive에는 신규 scene ID가 없고 opt-in positive control에 네 ID와 payload가 있음을 확인했다.
7. 계획 검토와 별개인 구현/runtime 실패 관점 20개와 PR diff 실패 관점 30개를 각각 완료했다. PR 본문의 두 raw screenshot이 Agent Browser에서 로드되고 크기가 확인됐다. browser session과 해당 Chrome/Agent Browser process는 종료했다.
8. 구현 PR [#464](https://github.com/ohah/spinon/pull/464)는 2026-10-22 03:45:55 KST Rebase and merge로 병합했다(merge SHA `3c03b51979127a0c52f5737cdd2d003b9dcf39c3`). 병합 시 main의 required status checks는 설정되어 있지 않았고 `gh pr checks`는 check 미보고를 반환했으므로 CI green으로 기록하지 않는다. Tailnet roadmap 미리보기는 기존 경로와 실행 process를 확인하고, 교체 전 백업과 전송 후 SHA-256 대조를 거쳐 갱신한다. GitHub Pages workflow는 실행하지 않는다.

## 실패·범위 경계

- CSS Grid 알고리즘의 독립 기대값과 Chrome 관측이 다르면 어느 한쪽에 맞춰 고치지 않고 mismatch를 기록하고 명세 계산을 재검토한다.
- Ahem 미로딩, 잘못된 text/owner/order/span, 무제한 grid child, 중간 trace 누락, stale revision, 일부 scene만 통과, 잘못된 draw marker, fixture symbol의 기본 artifact 유입은 완료 거부 조건이다.
- 성공은 이 fixed input의 중첩 contribution 전달에만 적용한다. 일반 font/script, anonymous item, image 혼합, nested span/outer item span, subgrid, arbitrary CSS Grid, WPT 및 C11.4 전체 완료를 의미하지 않는다.
