# C11.1 · parsed Root HTML dense Grid auto-placement 계획

**상태:** 계획과 Chrome 156 Agent Browser 6-scene 사전 비교를 고정했다. Root HTML 전용 dense mode, CSS projection, 14-scene parsed-runtime 비교를 구현했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 12개 성공 장면의 60 node geometry, 2개 기대 거부, 52개 surface 표본을 검증했다. 두 플랫폼 모두 geometry 최대 오차 0 CSS px, 표본 52/52 exact다. 구현/runtime 실패 관점 20개와 PR diff 실패 관점 30개 검토를 마쳤다. 아직 PR은 만들지 않았다. C11.1/C11 checkbox와 공식 완료 `72/213`은 유지한다. [Android/iOS runtime evidence](../spec/internal/evidence/c11-root-html-dense-auto-placement-runtime-2026-11-03/README.md) · [구현/runtime 검토 20개](../spec/internal/evidence/c11-root-html-dense-auto-placement-runtime-2026-11-03/implementation-runtime-review-20.md) · [PR diff 검토 30개](../spec/internal/evidence/c11-root-html-dense-auto-placement-runtime-2026-11-03/pr-diff-review-30.md)

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.2.3 dense·column 배치](./c11-2-3-dense-column-placement.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**계획 실패 관점 검토:** [20개](./c11-root-html-dense-auto-placement-plan-review-20.md)

**Chrome 사전 비교:** [6-scene 관측과 화면](../spec/internal/evidence/c11-root-html-dense-placement-precomparison-2026-11-03/README.md)

## 결손과 목표

기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1` 경로는 computed `row dense`와 `column dense`를 `grid-auto-flow`에서 거부한다. 반면 `GridAutoFlow` typed input과 layout core의 dense normalizer는 이미 두 flow를 표현하며, 별도 C11.2.3 opt-in profile은 dense row/column algorithm을 구현한다. 이 작업은 그 배치 알고리즘을 parsed Root HTML 경로에 연결하되, C11.2.3 profile의 기존 입력 경계와 일반 Grid mode를 바꾸지 않는 제한 runtime mode를 추가한다.

Chrome 156 관측에서 `row dense`는 앞선 span 배치가 비워 둔 `(88, 0)` cell을 채우고 sparse `row` control은 `(88, 24)`에 둔다. `column dense`는 `(0, 56)`을 채우며 sparse `column` control은 `(48, 56)`에 둔다. definite item 충돌 대조도 함께 고정했다. 독립 기대값은 CSS Grid Level 1 §8.5의 dense cursor 재시작 규칙과 고정 track/gap에서 계산했다.

## 제한 지원 계약

1. 기본 parsed Root HTML route에서만 적용한다. Computed `grid-auto-flow: dense`의 기본 row 해석과 `row dense`, `column dense`를 typed 값으로 구분한다. 선언 문자열을 재파싱하지 않는다.
2. 전용 `RootHtmlDenseAutoPlacementV1` placement mode를 둔다. Root HTML 검증을 거치지 않은 generic `DenseAutoPlacement` 입력을 제품 route로 승격하지 않는다. C11.2.3 기존 FFI entry point·matrix·오류 계약은 그대로 둔다.
3. 두 축의 explicit tracks는 비어 있지 않은 finite non-negative fixed-px 목록으로 제한한다. 각 축의 implicit sizing은 단일 finite non-negative fixed-px만 허용한다. `grid-auto-rows:auto`와 `grid-auto-columns:auto`는 각각 그 축에 implicit track을 만들지 않는 경우에만 허용하고, `auto`인 축에 새 track이 필요한 입력은 해당 `grid-auto-rows` 또는 `grid-auto-columns` 오류로 fail-closed한다. C11.2.3 기존 10,000-track 축별 상한, `direction:ltr`, `writing-mode:horizontal-tb`, `order:0` 경계를 유지한다.
4. Direct grid item의 배치는 `auto`, 양수 span 또는 positive definite line 조합으로 제한한다. Negative/named line, nonzero order, positioned item, unsupported track/gap, invalid span은 fail-closed한다. `display:none` item은 occupancy에서 제외하며 `visibility:hidden`은 layout participant로 유지한다.
5. Root HTML tree 안의 여러 Grid와 nested mixed-flow 장면은 각 container를 독립 검증한다. 한 container의 row-dense 거부나 성공이 이웃 container의 sparse/column 결과를 바꾸지 않아야 한다. `grid-template-areas`, named lines, `repeat()`·auto-repeat, intrinsic/text/replaced items와 non-fixed implicit sizing은 이 slice에서 기존 동작대로 거부한다.
6. Dense 재배치는 computed/source child order, accessibility order, paint order를 재정렬하지 않는다. Rejection은 item placement, expanded track, frame 및 draw publication의 부분 결과를 남기지 않는다.

## 고정 입력과 비교 기준

- 신규 Agent Browser 사전 비교는 [fixture](../spec/internal/evidence/c11-root-html-dense-placement-precomparison-2026-11-03/fixture.html), [Chrome 156 observation JSON](../spec/internal/evidence/c11-root-html-dense-placement-precomparison-2026-11-03/chrome156-observation.json), [화면](../spec/internal/evidence/c11-root-html-dense-placement-precomparison-2026-11-03/chrome156-six-scenes.png)을 사용한다. Row dense/sparse, column dense/sparse, definite-item collision의 root-relative track 및 item frame이 고정되어 있다.
- 기존 C11.2.3 fixture와 Chrome reference는 변경하지 않는다. Root HTML dense 제품 행렬은 dense flow가 실제로 설정된 candidate 14 scenes(성공 12·기대 거부 2, `nodeIds` 60개)만 parsed input으로 실행한다. row sparse baseline과 네 column sparse scene 및 기존 두 sparse hole control은 이 dense mode의 지원 범위 밖이라 제품 행렬에서 제외하고 Chrome oracle 대조로만 보존한다. 나머지 oracle-only scenes도 제품 성공 수로 세지 않는다.
- 독립 expected geometry는 CSS Grid Level 1 §§7.2, 8.5에서 산출한다. Chrome 156은 browser oracle일 뿐 Spinon 결과나 WPT 적합성으로 세지 않는다. Scene별 source, root/item line, expanded implicit tracks, root 바깥 item frame, error code/property/node를 비교한다.

## 구현·검증 순서

1. Baseline에서 기본 Root HTML route가 `row dense`와 `column dense`를 지정 property로 거부하고 기존 row/column sparse scene은 유지하는지 고정한다.
2. `GridPlacementMode`에 Root HTML 전용 dense mode를 추가한다. Projection에서 computed flow와 정확한 track/item/host topology를 검증한 뒤에만 해당 mode를 선택한다. Layout core는 기존 dense normalizer를 사용하되 mode별 container/item 제한, 축별 10,000 track 상한 및 failure atomicity를 다시 검사한다.
3. Style projection/layout 단위 테스트와 parsed Root HTML runtime 행렬에 dense candidate 14 scenes를 연결한다. Positive control의 dense-hole 재배치, definite-item 충돌, auto-track pattern, nested mixed flow 및 `display:none` occupancy를 분리 assert한다. `auto-track-no-implicit`의 양 축 `auto` 허용을 positive control로 두고, `row-dense-fixed-row`처럼 implicit column이 필요한데 `grid-auto-columns:auto`인 장면의 정확한 property 거부를 negative control로 둔다. 추가 negative controls는 nonzero order, negative line, unsupported repeat/areas, track limit 초과, stale/missing item handle 및 allocation failure를 검증한다.
4. Root HTML Android/iOS app matrix에서 같은 14 scenes(60 authored node IDs)를 실행한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 Chrome 156 independent expected에 대한 root/item geometry를 축별 0.5 CSS px 이내로 비교하고, out-of-container positive implicit item을 viewport 밖까지 추적한다. PixelCopy와 iOS Metal drawable 표본은 geometry assertion과 별도로 scene matrix의 고정 sample manifest대로 exact 비교한다. Android software renderer 및 iOS compositor 제출 전 한계를 기록한다.
5. Default product archive에는 신규 fixture scene, payload 또는 전용 FFI symbol이 없어야 한다. Opt-in Root HTML fixture-test positive control에는 신규 scene/source와 실행 entry point가 있어야 한다. Existing C11.2.2/C11.2.3 scene, default app boot/draw 및 product ABI를 회귀 확인한다.
6. Rust focused tests, scoped Clippy, rustfmt, CSS reference/fixture boundary, Android/iOS simulator matrix를 실행한다. 구현/runtime 검토 20개와 PR 실제 diff 적대 검토 30개를 별도로 기록한다. PR checks가 없으면 CI 성공으로 기록하지 않는다.
7. 병합 뒤 내부 계약 0059, 이 계획, `spec/STATUS.md`를 실제 통과 범위에 맞춘다. C11.2.3 기존 체크, C11.1/C11 parent checkbox와 공식 `72/213`은 이 limited slice에서 승격하지 않는다.

## 범위 밖과 미검증 항목

이 계획은 전체 CSS Grid, WPT 적합성, public CSS support 완료, 일반 text/image intrinsic sizing, named line/area와 dense placement 조합, repeat/auto-repeat, baseline/alignment, RTL·vertical writing mode, Android 실기기/hardware GPU, iOS compositor 이후 표면 또는 전체 C11 완료를 주장하지 않는다.
