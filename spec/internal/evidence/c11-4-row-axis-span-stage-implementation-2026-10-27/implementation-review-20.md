# C11.4 row-axis span-stage implementation review · 20 viewpoints

**검토 성격:** 구현 코드와 고정 oracle을 읽기 전용으로 대조했다. Reviewer는 테스트, browser, simulator를 실행하거나 코드를 수정하지 않았다. HEAD는 검토 시점의 구현 base `ba19d61a`; 구현 변경은 기존 trace test helper 수정 한 곳과 새 `row_axis.rs`다.

## 판정

**19 PASS / 1 PARTIAL.** 명백한 축 선택, 기대값, placement, 입력 범위 또는 regression 결함은 발견되지 않았다. PARTIAL은 test constants와 checked-in Chrome/normative JSON의 기계적 연결이 없어, 나중에 JSON만 바뀌어도 unit test가 이를 자동 탐지하지 않는다는 증거 동기화 한계다. 현재 상수와 JSON은 일치하고, artifact 해시는 사전 비교 README의 `SHA256SUMS`에 고정돼 있다.

| # | 실패 관점 | 판정 | 근거 |
|---:|---|---|---|
| 1 | Rows pass를 정확히 선택하는가 | PASS | Root trace에서 Rows pass 하나를 고른다. |
| 2 | Columns fixed-track 대조군이 유효한가 | PASS | 조기 반환 하나와 intrinsic batch 부재를 검사한다. |
| 3 | definite root/viewport 크기 | PASS | 둘 다 220×220px다. |
| 4 | 행/열 track 정의 | PASS | 220px fixed column, 4개 `minmax(0,max-content)` rows다. |
| 5 | gap/alignment 영향 통제 | PASS | 기본 0 gap, start alignment; helper가 gutter shape/size를 검사한다. |
| 6 | A/B/C placement 및 span 길이 | PASS | A/B span 2, C span 3이다. |
| 7 | DOM source-order controls | PASS | 네 장면에서 forward/reversed만 바뀐다. |
| 8 | item identity 귀속 | PASS | 같은 handle이 placement, contribution, frame assertion에 연결된다. |
| 9 | text/image intrinsic 기여 혼입 | PASS | 빈 div builder라 별도 intrinsic resource가 섞이지 않는다. |
| 10 | span batch 순서 | PASS | `[2]` 또는 `[2,3]` 전체 순서를 검사한다. |
| 11 | flex span 분류 | PASS | non-flex selector와 전체 관측 span sequence를 검사한다. |
| 12 | Initialize 상태 | PASS | 4개 base 0, growth limit `None`을 검사한다. |
| 13 | span-2 겹침 산술 | PASS | `[40,60,60,∞]` growth limit을 검사한다. |
| 14 | span-3 증가분 산술 | PASS | 기존 growth limit 반영 후 `[160/3,220/3,220/3,∞]`를 검사한다. |
| 15 | 미사용 네 번째 row 종료 | PASS | IntrinsicSizingComplete에서 0으로 닫히는지 검사한다. |
| 16 | MaximizeTracks 결과 | PASS | 최종 base sizes를 두 normative arrays와 대조한다. |
| 17 | maximize 뒤 남는 공간 | PASS | 60px 또는 20px을 계산해 검사한다. |
| 18 | Chrome final frame tolerance | PASS | root와 item x/y/width/height를 1/64 CSS px로 대조한다. |
| 19 | source-order 독립성 | PASS | stage kind/track shape/snapshots 및 ID별 item frames를 비교한다. |
| 20 | 범위, column 회귀, oracle 증거 연결 | PARTIAL | test-only 경계와 기존 column 기대값은 유지한다. JSON과 테스트 상수는 자동 생성/검증으로 연결되지 않는다. 현재 값과 JSON 및 해시는 일치한다. |

## 직접 확인한 경로

- `crates/spinon-layout/src/grid_multispan_chrome_oracle_tests/row_axis.rs`
- `crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs`
- `spec/internal/evidence/c11-4-row-axis-span-stage-precomparison-2026-10-27/expected.json`
- `spec/internal/evidence/c11-4-row-axis-span-stage-precomparison-2026-10-27/chrome156-observation.json`
- `plan/c11-4-row-axis-span-stage-oracle.md`

PARTIAL은 제한 사항으로 문서에 노출한다. 현재 값 불일치나 구현 오류를 나타내지는 않는다.

## 수정 후 재검토

- fixed Columns 대조군은 `FixedTrackEarlyReturn` snapshot에서 CSS track 하나의 base size가 220px인지도 검사한다. 독립 재검토 PASS.
- 전체 expected Rows sequence를 명시해 `ExpandFlexibleTracks`와 `StretchAutoTracks { applied: false }`까지 확인한다. 기존 column test에도 같은 exact sequence helper를 적용하며 회귀 재검토 PASS.
- 이 재검토에서는 테스트를 다시 실행하지 않았다. 최종 코드 source hash와 실행 결과는 [validation manifest](./validation.json)에 연결했다.
