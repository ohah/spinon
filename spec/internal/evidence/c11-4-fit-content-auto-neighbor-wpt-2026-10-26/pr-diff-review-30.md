# C11.4 `fit-content()` 옆 `auto` implicit row 계획 PR diff 검토 30관점

**결과:** 30 PASS / 0 PARTIAL / 0 FAIL · 차단 사항 없음

**검토 기준:** base `d48d31698842866241dcb3317c6f8ff1ee068d1c`, PR #661 head `2b715cf0ae159f22332e0483680ccd7bcd283a86`

**보고서 작성 당시 PR head:** `737c11a4ef68326b624c16f1d39604a148627c5a` (history 정리 및 evidence·본문 링크 delta 재확인)

**검토 시각:** 2026-10-26 14:02 UTC

**evidence·본문 재확인:** 2026-10-26 14:18 UTC
**현재 메타데이터:** PR base는 현재 `main`과 같은 `d48d31698842866241dcb3317c6f8ff1ee068d1c`; labels `planning`, `area: layout`; merge state `CLEAN`; `statusCheckRollup=[]` (CI 통과 아님).

계획 문서 자체의 20관점 검토는 [`plan-review-20.md`](plan-review-20.md)에 별도로 기록했다. 아래 30개는 계획 검토 표의 재집계가 아니라 17개 파일 전체 diff, PR 본문·라벨·상태를 대상으로 한 별도 실패 경로 및 계약 경계다.

| # | 독립 실패 경로·계약 경계 | 확인 결과 | 판정 |
|---:|---|---|---|
| 1 | WPT의 인접 assertion을 여러 개 한 계획 slice로 묶음 | `autoRowAndFitContentRow` 한 assertion만 고정한다. preceding `autoRow` 대조군과 나머지 cap/floor·percentage·spanning assertion은 범위 밖이라고 분명히 한다. | PASS |
| 2 | upstream fixture pin 누락 또는 이동 | WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`와 파일 SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`가 source pin과 실제 고정 checkout에서 일치한다. | PASS |
| 3 | 선택된 WPT case나 출력 문자열 오인 | source lines 130–132의 `autoRowAndFitContentRow`는 `10px 290px`을 요구하며 “남은 공간은 이웃 auto row가 받는다”는 설명과 맞는다. | PASS |
| 4 | 명시 행으로 implicit-row 검증을 대체 | expected input은 explicit rows를 비우고 `grid-auto-rows` 패턴으로 두 implicit row를 만든다. | PASS |
| 5 | auto-placement와 track-list 반복 순서가 바뀜 | 두 direct item의 row 순서는 `[fit-content(40px), auto]` pattern의 첫 두 implicit track 순서와 같다. 계획은 adapter가 실제 order를 보존하는지 확인하도록 관문을 둔다. | PASS |
| 6 | 300px 제약이 definite content block-size가 아님 | root height 300px과 zero border/padding으로 definite inner block-size 300px을 고정한다. | PASS |
| 7 | gap·padding·border가 free-space 계산에서 빠짐 | 모두 0px로 고정하며 산술에서 제외한다. | PASS |
| 8 | content contribution을 grid item 높이로 잘못 모델링 | 각 auto-height wrapper 안에만 10px fixed child를 둔다. 첫 wrapper는 10px, 두 번째는 stretch 후 290px이므로 contribution과 stretched frame을 분리한다. | PASS |
| 9 | `align-content:normal`을 `stretch`로 바꿈 | 계획 입력과 Browser 관측 모두 `normal`이다. test 계획은 기존 normal control과 같은 의미를 유지한다. | PASS |
| 10 | §6.6 automatic minimum 조건을 만족하지 않음 | WPT wrapper는 기본 `overflow:visible`이고 각자 한 행만 span한다. 해당 축에 `auto` min sizing function인 track이 있으므로 content-based automatic minimum 조건이 성립한다. | PASS |
| 11 | `fit-content()`의 min/max function을 혼동 | CSS Grid Level 2 2025 CRD §12.2의 `min=auto`, `max=max-content`를 적고 §12.8 stretch 대상에서 첫 track을 제외한다. | PASS |
| 12 | auto-min track의 §12.4 초기 base를 잘못 시작 | 두 track의 initial base `[0,0]px`를 기록한다. | PASS |
| 13 | §12.5 item minimum contribution 누락 | 두 단일-span item의 10px contribution이 bases를 `[10,10]px`로 올린다고 독립 계산한다. | PASS |
| 14 | fit-content cap과 max-content growth limit 혼동 | 첫 track의 max-content contribution 10px은 40px argument로 clamp되어 10px이다. 두번째 auto track의 growth limit도 10px이다. | PASS |
| 15 | §12.6 `Maximize Tracks`가 base를 더 늘린다고 가정 | 두 bases가 각 growth limit 10px에 이미 도달하므로 stage 뒤에도 `[10,10]px`이다. | PASS |
| 16 | §12.7 flexible-track expansion을 auto-track stretch와 혼동 | 입력에는 flex track이 없어 flexible expansion 뒤 bases는 `[10,10]px`로 유지한다. | PASS |
| 17 | §12.8 eligible auto track 선택 오류 | `fit-content()` max function은 `auto`가 아니고 두 번째 track만 `auto`이므로 eligible index `[1]`이다. | PASS |
| 18 | 양의 definite free space 산술 오류 | §12.8 직전 `300−10−10−0=280px`; 둘째 track만 받아 final bases는 `[10,290]px`다. | PASS |
| 19 | trace가 다른 node 또는 다른 axis pass를 검사 | 계획은 root grid node 소유 trace와 정확히 한 row pass를 요구한다. | PASS |
| 20 | stage sequence·중복·적용 flag 누락 | `IntrinsicSizingComplete → MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks` 순서를 검사하고 stretch stage가 한 번뿐이며 `applied:true`인지 본다. | PASS |
| 21 | gutter/collapsed sentinel을 CSS track으로 셈 | CSS bases 수집에서 `is_gutter`와 `is_collapsed`를 제외하고 각 flag를 별도 확인하도록 정한다. | PASS |
| 22 | 최종 frame에서 wrapper와 nested child를 혼동 | root `(0,0,60,300)`, items `(0,0,60,10)` 및 `(0,10,60,290)`, nested children 각 10px을 별도로 적고 허용오차 `1e-4 CSS px`를 둔다. | PASS |
| 23 | WPT grid 폭과 제안 test 폭 차이가 row oracle을 바꿈 | 원본 block grid는 viewport 폭 1264px을 사용하지만 유일한 60px column과 nested children은 60px이다. test root를 60px로 좁혀 inline 자유 공간을 없애며 고정 block-axis contribution에는 변화가 없다. | PASS |
| 24 | 현재 typed API가 제안 입력을 표현하지 못함 | base 코드에 `GridTrackSize::FitContent`, `GridAutoRows::TrackList`, `GridPlacementMode::TrackSizing`이 있다. 계획은 새 제품 API를 전제하지 않고 실제 implicit expansion/order를 검증하게 한다. | PASS |
| 25 | 새 테스트가 기존 책임 파일이나 제품 경로를 침범 | 새 `grid_fit_content_auto_neighbor_oracle_tests.rs` 및 `lib.rs`의 `#[cfg(test)] mod` 한 줄을 제안하고 기존 stretch oracle과 runtime/product source는 그대로 둔다. | PASS |
| 26 | 기존 coverage를 중복 또는 상위 완료로 계산 | 기존 oracle의 auto/auto/fixed stretch와 auto-track pattern tests가 혼합 fit-content/auto의 stage 결과를 직접 검사하지 않는다는 gap만 추가한다. C11.4 전체 완료로 확대하지 않는다. | PASS |
| 27 | 기대값이 Taffy/Spinon 후보 출력에서 역산됨 | `expected.json`은 plan-only로 표시하고 CRD 산식과 WPT expectation을 분리한다. 구현 결과를 oracle로 쓰지 않는다고 적는다. | PASS |
| 28 | precomparison 입력·환경·screenshot이 서로 다른 장면 | HTML, raw CSSOM/frame, screenshot은 60px column·300px height·normal 및 `10px 290px`을 일치시킨다. Chrome 156.0.8078.48/Agent Browser 0.31.1 환경 정보와 이미지 SHA가 보존돼 있다. | PASS |
| 29 | `file://` CSSOM 관측을 WPT 실행으로 과장하거나 문서 검증을 통과로 표시 | raw 요청은 testharness resources가 `file:///resources/...`로 resolve됐음을 보이고, plan/PR은 이를 WPT PASS로 부르지 않는다. 전체 docs build exit 1과 기존 evidence dead links, 신규 페이지 진단 0, 로컬 link/hash 결과를 각각 구분한다. | PASS |
| 30 | PR 본문·라벨·상태·완료 수가 변경 scope를 과장 | title은 한글 규칙에 맞고 labels 두 종류가 있다. body는 C11.4 및 `72/213` 미변경, 미구현, CI rollup empty를 밝힌다. Agent Browser에서 현재 본문 evidence links가 full `3b5b7053481edf8b8cf70cd4b35dada516380faf`을 가리키며 immutable screenshot URL `e172d501bec0a4de97f573d6e237632a4334bc52`는 HTTP 200·표시 완료 1280×1000이다. | PASS |

## 검증 근거의 커밋 상태 재확인

30개 substantive 관점은 PR 전체 diff base `d48d31698842866241dcb3317c6f8ff1ee068d1c`부터 head `2b715cf0ae159f22332e0483680ccd7bcd283a86`까지를 검토했다. 이후 PR branch history에서 manifest가 틀렸던 옛 `3c4c523f`와 `4fb50a20` commits를 force-with-lease로 제거했다. 이를 대체한 `3b5b7053481edf8b8cf70cd4b35dada516380faf`과 현재 head `737c11a4ef68326b624c16f1d39604a148627c5a`는 `pr-body-image-url-check.json`과 `SHA256SUMS`만 바꿨고 계획·코드 내용은 바꾸지 않았다. 각 commit tree의 `pr-diff-review-30.md` placeholder SHA `36936cbb15947be716c549dda1d9bc04f70dcd12e6cf988a0c03060ebbe4db00`와 manifest 항목이 일치함을 별도로 확인했다. image-check JSON은 Agent Browser로 확인한 PR head `737c11a4ef68326b624c16f1d39604a148627c5a`와 당시 본문 evidence blob links commit `3b5b7053481edf8b8cf70cd4b35dada516380faf`, screenshot asset SHA를 구분해 기록한다. PR 본문 evidence blob links는 full SHA `3b5b7053481edf8b8cf70cd4b35dada516380faf`에 고정되어 있고 screenshot은 삭제 가능한 branch나 `/merge` ref가 아닌 full immutable commit URL `e172d501bec0a4de97f573d6e237632a4334bc52`를 쓴다. 현재 head `737c11a4`에서 PR 본문을 다시 열어 링크·§6.6/§12.2/§12.4–§12.8·screenshot 표시를 확인했다. 이미지 complete·visible, 1280×1000이고 raw GET은 200이었다. 저장소 루트에서 `shasum -a 256 -c spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/SHA256SUMS`를 실행해 목록의 14개 evidence 파일이 모두 일치함을 확인했다.

이 보고서, PR 본문 이미지 확인 JSON, 갱신된 hash manifest를 추가한 commit은 substantive 30관점 검토 대상 head 이후의 evidence-only delta이므로 별도 inspect 대상이다. 이 보고서 작성 당시 PR 본문은 diff 검토가 진행 중이라고 표시했다. 테스트, simulator, WPT testharness는 이 review에서 실행하지 않았다. PR은 아직 병합되지 않았다.
