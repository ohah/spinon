# C11.4 · repeated-text row-span one-fit `min-height:auto`

**상태:** 구현 검증 중 · C11.4/C11 미완료 · 공식 `78/214` 유지

**선행 범위:** [direct-text row-span](./c11-4-root-html-repeated-text-row-span.md) · [one-fit 위치 1–6](./c11-4-root-html-repeated-text-row-span-fit-content-positions-4-6.md) · [two-fit row-span `min-height:auto`](./c11-4-root-html-repeated-text-row-span-two-fit-content.md) · [one-fit auto-placement `min-height:auto`](./c11-4-root-html-repeated-text-autoplacement-one-fit-auto-minimum.md)

**사전 비교:** [Chrome 157 5,760-scene matrix와 독립 관계 검증](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum-precomparison-2026-11-10/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-one-fit-auto-minimum-precomparison.html) · [관측 NDJSON](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum-precomparison-2026-11-10/chrome157-observation.ndjson)

**계획 실패 관점 검토:** [20개](./c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum-plan-review-20.md) · **계획 PR diff 검토:** [실제 변경분 20개](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum-precomparison-2026-11-10/plan-pr-diff-review-20.md)

## 결손과 제한 목표

기본 parsed Root HTML의 one-fit repeated-text row-span profile은 여섯-member `grid-auto-rows` list, 한 개의 `fit-content()`, 두 개 또는 세 개의 직접 text payload, 명시적 2행·3행 span을 제한적으로 연결한다. 이 profile은 현재 text item의 computed `min-height:0`만 허용한다. PR #1311은 two-fit row-span의 `min-height:auto`를, PR #1318/#1322는 two-fit/one-fit auto-placement의 `min-height:auto`를 각각 별도 predicate로 추가했지만 one-fit row-span은 열지 않았다.

이번 slice는 기존 single-fit row-span 중 3개 implicit row를 만드는 topology만 대상으로, 모든 text item이 같은 computed `min-height:auto`인 경우를 추가한다. 같은 scene에서 `min-height:0`은 paired baseline이다. 첫 explicit-row empty control은 두 모드에서 계속 `min-height:0`이다. fit-content 위치 4–6을 쓰는 six-row topology, 두 번째 fit track, item별 mixed minimum, auto-placement, 새 text grammar는 범위 밖이다.

## 구현 전 비교 모델

- **규범 출처:** [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2 track sizing functions](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing) 및 [§11.5 track sizing algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items). §6.6은 비-scrollable item, span 내 `auto` minimum track, flexible track 미포함 조건을 정한다. §7.2는 `fit-content()`의 min track sizing function이 `auto`임을 정의한다. 이 입력의 여섯-member 목록에는 flexible track이 없다.
- **고정 scene:** Root HTML Grid `40×220px`, explicit column `[40px]`, explicit row `[10px]`, gap 0, sparse row flow, LTR, static, visible overflow, `align-content:start|stretch`. 여섯 auto-row member는 `fit-content(cap)` 1개, `min-content` 2개, `max-content` 2개, `auto` 1개이며 fit 위치는 1–3만 쓴다.
- **순서와 cap:** fit track을 제외한 다섯 member의 고유 순서 30개, typed cap `20px|20%|0%|120%` 네 개를 쓴다. Percentage basis는 definite 220px block-size다. computed track serialization은 authored 순서·단위 유형을 보존해야 한다.
- **Text/topology:** fixed monospace 10px/10px direct-text payload와 기존 ASCII grammar를 유지한다. `span2-forward`/`span2-reversed`는 A=`row 2 / span 2`, B=`row 3 / span 2`; `span2-before-span3`/`span3-before-span2`는 여기에 C=`row 2 / span 3`을 추가하고 DOM order만 바꾼다. 모든 item은 column 1 하나를 차지한다.
- **비교 cohort:** fit 위치 3 × tail 순서 30 × cap 4 × alignment 2 × topology 4 × minimum mode 2 = 5,760 scene. 같은 2,880 scene key를 `min-height:0` baseline과 `min-height:auto` candidate에서 짝지었다. 첫 explicit-row empty item은 늘 `min-height:0`이다.
- **자동 최소 크기 eligibility:** item이 scroll container가 아니고 span한 row 중 적어도 하나가 `auto` minimum track이며 span 안에 flexible track이 없을 때 content-based automatic minimum을 적용한다. 이 scene의 `fit-content()`와 `auto` member는 `auto` minimum track이고 flexible member는 없다. 나머지 span은 `auto` minimum이 0이라는 control로 남긴다.
- **독립 관계 검증:** item frame은 점유한 used row들의 합과 일치해야 한다. 대응하는 forward/reverse topology의 track와 semantic item frame은 0.01 CSS px 이내에서 같아야 한다. `align-content:stretch`는 materialized `auto` max track에만 추가 공간을 주며 non-auto row는 `start`와 같아야 한다. Candidate는 모든 eligibility-positive item에서 content child 높이 아래로 줄지 않고 visible content overflow가 없어야 한다. 이 대조는 화면 외형의 동등성 또는 전체 Grid 적합성을 주장하지 않는다.

## Chrome 사전 관측

Agent Browser HeadlessChrome 157에서 5,760 scene을 실행했다. 두 mode는 각 2,880 scene이며 2,880 paired key, 자동 최소 크기 조건을 만족하는 6,048 item occurrence가 있다. Candidate는 content floor 위반 0, visible content overflow 0이고 baseline도 content floor 위반 0이다. 따라서 이 후보의 주된 관측은 overflow 복구가 아니라 automatic minimum이 바꾸는 span track/frame sizing이다.

자동 최소 크기 전후 item frame 6,036개가 달라졌고 최대 used-row 차이는 30 CSS px, 최대 semantic item frame 차이는 45 CSS px였다. paired text line count 차이는 0이다. source-order reverse 최대 차이 0 CSS px, stretch에서 non-auto track 최대 변화 0 CSS px였으며 auto track만 커졌다(최대 180 CSS px, 성장 관측 1,152건). 허용 오차는 frame/track마다 0.01 CSS px다.

대표 `fit-content(0%) auto max-content max-content min-content min-content`, `align-content:start`, span-2 forward 장면은 baseline row `[10,0,20,30]px`에서 candidate `[10,10,15,15]px`로 바뀌었다. Candidate의 A/B frame 높이는 25/30px이고 payload 높이는 20/30px다. Content floor와 overflow는 둘 다 보존된다. [Chrome 화면](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum-precomparison-2026-11-10/chrome157-one-fit-span-auto-sample.png) · [요약 JSON](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum-precomparison-2026-11-10/capture-summary.json) · [실행 script](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum-precomparison-2026-11-10/capture-chrome.mjs)

## 허용·거부 경계

- 기존 one-fit, three-implicit-row span topology, fixed direct text grammar, positions 1–3, 30 tail orders, 네 cap, `start|stretch`, visible overflow 및 같은 Grid owner/revision에서 모든 span text item이 `min-height:auto`일 때만 추가 허용한다.
- Explicit-row control은 계속 `min-height:0`이어야 한다. 기존 같은 profile의 전 item `min-height:0`, one-fit auto-placement, two-fit auto-placement, two-fit row-span, positions 4–6 six-row row-span 경로는 각각 별도 회귀로 보존한다.
- mixed `auto|0`, auto item 중 일부 누락·중복, row-span과 auto-placement 혼합, fit 위치 4–6, 두 fit, 다른 list length/multiplicity, indefinite/negative/non-finite/calc cap, scrollable/hidden overflow, dense/column flow, implicit column, nested/replaced/inline payload, text/style/writing-mode 변형, stale owner/revision 또는 incomplete/duplicate/wrong-owner/out-of-order trace는 fail-closed한다.
- 잘못된 입력은 scene, frame, paint/render revision을 부분 공개하지 않고 같은 `RuntimeSession`에서 atomic rejection 뒤 다음 유효 revision을 허용해야 한다.

## 구현 및 검증 관문

1. 계획 자체의 서로 다른 실패 관점 20개와 계획 PR의 실제 diff 실패 관점 20개 검토가 끝나기 전에는 제품 코드를 바꾸지 않는다.
2. Style projection은 기존 one-fit row-span predicate에 all-auto minimum mode만 추가한다. two-fit span/one-fit auto-placement 검사를 합치지 않고 profile별 입력 경계를 명시적으로 유지한다.
3. Layout은 item span이 실제 `fit-content()` 또는 `auto` minimum track을 포함하는지, overflow가 scrollable이 아닌지, span 내 flexible track이 없는지 확인한다. `min-height:auto` contribution, per-row base/growth limit/final row 및 complete same-owner/current-revision trace를 검증한다. Cap을 automatic minimum으로 잘못 사용해 content floor를 clamp하지 않는다.
4. 기본 parsed Root HTML `RuntimeSession`에서 Chrome 2,880 baseline과 2,880 candidate를 같은 key로 비교한다. Track order/type, computed min-height, content floor, span/frame 합, line count, source-order invariance, cap/cycle, `start|stretch` auto-only distribution과 visible overflow를 확인한다.
5. 거부·복구 matrix는 mixed min-height, control auto, wrong span line, auto-placement, fit 위치 4–6, two-fit/zero-fit/invalid multiplicity, unsupported cap/percentage basis, scrollable overflow, wrong item count/payload/style, extra owner, stale revision, trace omission/duplication/order/owner/stage mismatch를 포함한다. 거부 전후 published render/layout revision과 동일 session 복구를 검사한다.
6. 기존 one-fit span 960/1,920/4,320 scene, two-fit span 3,840 scene, one-/two-fit auto-placement 1,440/2,880 scene, anonymous repeated-text 810 scene 및 Grid track-sizing 회귀를 실행한다. 변경 crate의 focused tests, `Clippy -D warnings`, `cargo fmt --all -- --check`를 기록하고 실행하지 않은 suite/CI는 통과로 쓰지 않는다.
7. 동일 source SHA의 기본 Root HTML route를 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 light/dark로 실행한다. baseline/candidate 대표 start/stretch scene, `layout=ready`, draw/present, HTML·app hash 및 OS screenshot을 남긴다. screenshot은 visual smoke이며 PixelCopy/Metal readback, compositor 뒤 pixel equality 또는 실기기 증명이 아니다.
8. 공용 Android AVD와 iPhone 17 Pro simulator는 재사용 후 shutdown한다. 별도 task-owned device/profile을 새로 만들면 실행 종료 후 제거한다. S03 이름의 simulator profile은 존재하지 않으며 shared profile/user data는 보존한다. Agent Browser는 고유 session으로 실행하고, 캡처 뒤 session과 브라우저 process가 종료됐는지 확인한다. Terminal Browser는 사용하지 않는다.
9. 구현/runtime은 계획 검토와 다른 실패 관점 20개로, 구현 PR diff는 별도 30개로 검토한다. Android/iOS 화면이 있으면 commit 고정 raw 링크를 PR 본문에 넣고 Agent Browser에서 링크 load를 확인한다.
10. 병합 뒤 실제 결과를 내부 계약 0059, `spec/STATUS.md`, 공개 roadmap, 이 계획 및 지정 Tailnet 미리보기와 동기화한다. 이 slice 자체로 C11.4/C11 checkbox나 공식 `78/214`, 내부 계약 숫자 `0.1.0`을 올리지 않는다.

## 완료 판정

이 bounded slice는 paired RuntimeSession matrix, automatic-minimum eligibility와 track-stage trace, atomic rejection/same-session recovery, 기존 profile 회귀, focused Rust/Clippy/rustfmt, Android/iOS default-route visual smoke, 별도 implementation/runtime 20관점, implementation PR diff 30관점 및 merge 후 상태/spec/roadmap/Tailnet 동기화를 마친 뒤에만 구현 완료로 기록한다. 이는 six-row positions 4–6 auto minimum, 일반 CSS admission, arbitrary row-span/text, WPT 전체 또는 C11.4/C11 완료가 아니다.

## 2026-11-10 실행 기록

- Runtime paired matrix 2,880쌍/5,760 layouts, mixed-profile·atomic reject/recovery와 기존 row-span/auto-placement/anonymous-text matrix가 통과했다. 한 two-fit `min-content` control이 새 explicit-row guard에 잘못 걸리는 회귀를 확인했고, guard를 새 one-fit auto-minimum 후보로 한정해 two-fit 3,840-scene matrix 및 floor control을 다시 통과시켰다.
- Android API 37.1 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 default Root HTML route를 baseline/candidate, light/dark로 실행했다. 각 route의 source SHA, `layout=ready`, draw `presented` 로그 및 OS screenshot은 [화면·실행 근거](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum-2026-11-10/README.md)에 있다. AVD와 iOS Simulator는 shutdown했고 원래 앱 HTML asset 및 기본 앱 빌드를 복원했다.
- `cargo fmt --all -- --check`는 통과했다. 변경 crate 전체 Clippy는 기존 anonymous-text test helper 2개의 `too_many_arguments` lint를 개별 허용했을 때 나머지 `-D warnings`가 통과했다.
- 최종 diff 검토에서 보강한 Taffy min/max cap 단위 테스트 4/4, layout track/nearest rounding 테스트 9/9, malformed `fitPosition` 거부 1/1, zero-cap overlap 및 auto-minimum trace 각 1/1, 기존 960-scene row-span matrix 통과를 재확인했다. Runtime paired matrix 최종 재실행은 2,880쌍/5,760 layouts 248.75초이며 Runtime glyph baseline line count도 Chrome과 대조한다. 신규 2,880-pair 모듈은 fixture-test feature 뒤에 둔다.
- standalone Taffy unit-test build는 이 작업에서 만들어진 `vendor/taffy/target`을 대상으로 `cargo clean --manifest-path vendor/taffy/Cargo.toml`로 정리했다(667 files/719.0 MiB). 오래된 무관한 untracked `vendor/taffy/Cargo.lock`과 다른 작업 파일은 PR stage에서 제외했다.
- Broad Rust suite는 clean하지 않다. `spinon-layout`은 524/525이며 미변경 HostDocument 테스트 1개가 실패했다. `spinon-style-to-layout`은 455/465이며 C07.3 aspect-ratio serialization과 C11.1/C11.5 grid fixture 9개가 실패했다. strict Clippy 기본 실행도 미변경 test helper lint 2개로 막힌다. 이 결과와 재현 필터는 화면·실행 근거 README에 분리해 기록했다.
- 독립 implementation/runtime 20관점과 별도 PR diff 30관점 검토를 마쳤다. 최초 diff review에서 제안된 Taffy minimum/maximum 회귀 테스트, nearest rounding 다중 행 선택, Runtime glyph line count, fail-closed fixture key, feature gate와 exact repro commands를 반영했다. 최종 re-review에는 코드 수정 요청이 없고 PR 생성·스크린샷 링크 검증 단계만 남았다.
- 구현 PR [#1326](https://github.com/ohah/spinon/pull/1326)은 `2026-11-10T05:39:19Z` Rebase and merge됐고 merge SHA는 `b5925aa274718a16c4e8f0bd9ef4bed4423aec6c`다. PR의 고정 Android/iOS 후보 화면 링크 네 장은 Agent Browser에서 load를 확인했고, check-run/status/review는 각각 0개라 CI green으로 보지 않는다. close 뒤 남아 있던 해당 세션의 고유 Chrome/agent-browser 프로세스만 종료해 후속 조회에서 사라졌음을 확인했다. AVD·Simulator는 모두 shutdown, ADB 연결 없음, S03 이름의 profile 없음이며 공유 프로필 데이터는 보존했다.
- PR #1326 병합 후 내부 계약 `0059`, 공식 `STATUS`, 공개 roadmap에 merge metadata와 제한 구현 결과를 반영했다. 기존 Tailnet preview server PID `90711`을 재시작하지 않고 선택 RSPress 문서를 동기화했으며 260개 route의 local·HTTPS HTTP 200 및 SHA-256과 교체 전 파일 백업 hash를 확인했다. Agent Browser desktop 1440×1000과 mobile 390×844에서 최신 PR #1326, merge SHA, 공식 `78/214`, C11.4/C11 미완료 및 PR checks 0개를 확인했고 mobile 가로 overflow는 없었다. 이 문서 동기화 PR은 merge 후 spec·status·roadmap·Tailnet 증거를 기록한다. C11.4/C11 checkbox와 공식 `78/214`는 미완료로 유지한다.
