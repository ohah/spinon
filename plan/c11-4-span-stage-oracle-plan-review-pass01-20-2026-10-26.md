# C11.4 overlapping span-stage 계획 1차 적대 검토 20개


**대상:** `plan/c11-4-span-stage-oracle.md` 및 연결된 독립 산술 자료

**기준 커밋:** `b58de5df93204ba211a025f20b5bfe3790c1d385`

**방식:** 계획, CSS Grid 2025 CRD 근거, existing Rust test와 Taffy trace source를 읽기 전용 대조했다. Rust test·브라우저·시뮬레이터는 실행하지 않았다.

## 검토 결과

| # | 고유 실패 관점 | 판정 | 근거와 필요한 보완 |
| ---: | --- | --- | --- |
| 1 | 이 작업이 상위 Stage B의 빠진 관문만 다루는가 | PASS | 중간 `IntrinsicItemBatch` state만 추가하고 Stage B/C11.4 완료나 공식 `72/213`을 올리지 않는다고 제한했다([계획 L5-L11, L58-L63](./c11-4-span-stage-oracle.md#L5)). |
| 2 | 기대값이 구현 출력에서 역산되지 않았는가 | PASS | Fraction script는 고정 입력과 CSS Grid §11.5.1/§11.6 규칙만 사용하며 Taffy trace를 읽지 않는다([계획 L15-L17](./c11-4-span-stage-oracle.md#L15), [산술 근거](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/README.md#L7)). |
| 3 | 표준 판본과 source integrity가 고정됐는가 | PASS | CSS Grid Layout 2025 CRD와 원문 SHA-256을 고정했다([근거 L3](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/README.md#L3)). |
| 4 | test inputs의 track count·gap·available width가 기대값과 일치하는가 | PASS | 네 `minmax(0,max-content)` column, 0px gap, definite 220px content width와 네번째 empty track을 명시했다([계획 L17](./c11-4-span-stage-oracle.md#L17)). |
| 5 | span line 좌표가 실제 track span과 맞는가 | PASS | A lines 1→3, B 2→4, C 1→4로 정의해 overlap이 정확히 한 span-2 track과 span-3 prefix를 공유한다([계획 L17](./c11-4-span-stage-oracle.md#L17)). |
| 6 | contribution 값이 fixed-size item box와 같은 단위인가 | PASS | A/B/C는 definite 80/120/200px width, 10px height, margin/padding/border 0이고 기존 helper가 같은 값으로 block style을 만든다([계획 L17](./c11-4-span-stage-oracle.md#L17), [test helper L71-L129](../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs#L71)). |
| 7 | span-2 같은 batch에서 item source order가 결과를 바꿀 수 있는가 | PASS | `[A,B]`와 `[B,A]`를 함께 두고 track별 planned increase의 max가 `[40,60,60,0]`인지 비교한다([계획 L17-L27](./c11-4-span-stage-oracle.md#L17)). |
| 8 | span-3 batch가 span-2 state를 반영하는가 | PASS | C의 200px 기여에서 앞선 growth limit 합 160px을 빼고 40px을 분배하도록 계산을 고정했다([계획 L17-L25](./c11-4-span-stage-oracle.md#L17)). |
| 9 | 새 test가 `Initialize` input state를 독립적으로 검증하는가 | PARTIAL | Expected table에는 `[0,0,0,0]`/`[∞,∞,∞,∞]`가 있으나 구현 단계는 주로 batch 뒤와 final snapshots를 비교한다고 적고 `Initialize` stage assertion을 명시하지 않는다([계획 L19-L27, L49-L54](./c11-4-span-stage-oracle.md#L19)). 초기화 오류가 batch 산술을 우연히 가릴 수 있어 해당 stage도 assertion 목록에 넣어야 한다. |
| 10 | `IntrinsicSizingComplete`에서 span 밖 무한 growth limit이 닫히는가 | PASS | 네번째 track만 base size 0으로 growth limit을 닫는 기대값을 명시하고 span-2-only 및 span-2/3 controls에 적용한다([계획 L24-L27](./c11-4-span-stage-oracle.md#L24)). |
| 11 | `MaximizeTracks` definite free space 결과가 일관적인가 | PARTIAL | span-2/3 ideal과 20px free space는 명시했지만 span-2-only의 maximize snapshot을 `[40,60,60,0]`과 60px free space로 명시적으로 test하라는 단계는 없다([계획 L25-L27, L49-L54](./c11-4-span-stage-oracle.md#L25)). final Chrome assertion은 base track만 확인하므로 두 input group 모두의 `MaximizeTracks` expected state를 고정하는 것이 좋다. |
| 12 | 반대 DOM 순서가 normative span-group order를 유지하는가 | PASS | `[C,B,A]`에서도 batch span order `[2,3]`이어야 한다고 명시해 DOM 순서와 track-sizing grouping을 분리한다([계획 L27](./c11-4-span-stage-oracle.md#L27)). |
| 13 | vector 내부 gutter를 CSS track index로 잘못 읽는가 | PASS | gutter와 collapsed entry를 제외한 배열을 vector order대로 읽고 raw internal index 0..3을 기대하지 않는다([계획 L37](./c11-4-span-stage-oracle.md#L37)). |
| 14 | collapsed/gutter state로 필요한 explicit track이 조용히 빠지는가 | PASS | extraction 전 `is_gutter`/`is_collapsed` flags를 확인하고 최종 네 CSS track 수와 기대값을 대조하도록 했다([계획 L37](./c11-4-span-stage-oracle.md#L37)). |
| 15 | ∞와 finite growth limit이 같은 숫자 sentinel로 혼동되는가 | PASS | trace의 `None`을 무한대로 명시하고 0px을 유한 growth limit으로 구별한다([계획 L19-L27, L37](./c11-4-span-stage-oracle.md#L19)). |
| 16 | f32 차이 허용치가 규범 분수와 Chrome quantization을 혼합하는가 | PARTIAL | `1e-4 CSS px` stage tolerance와 기존 Chrome 1/64px final tolerance를 분리한 점은 적절하다. 다만 구현 gate에서 finite actual의 absolute-delta 비교, NaN/∞ 거부, `None` 대 ∞ 비교 규칙을 한 번 더 고정해야 한다([계획 L31, L37](./c11-4-span-stage-oracle.md#L31)). |
| 17 | 첫 Columns pass만 골라 rerun/multiple pass를 숨기는가 | PASS | pass 수를 확인하고 추가 pass가 있으면 임의 선택 대신 계획 검토로 되돌리도록 했다. Taffy는 `begin_pass`마다 sequence를 추가하므로 이 gate를 구현할 수 있다([계획 L36](./c11-4-span-stage-oracle.md#L36), [Taffy trace](../vendor/taffy/src/compute/grid/mod.rs#L1555)). |
| 18 | expected non-flex batch에 `crosses_flexible_track=true` stage가 섞이는가 | PARTIAL | 계획은 true batch를 분리하는 control을 요구하지만, 실제 true batch를 만드는 최소 fixed test input과 기대 stage/count가 없다([계획 L37-L38](./c11-4-span-stage-oracle.md#L37)). positive classification control을 정의하거나, flex interaction을 이 slice에서 빼고 selector가 false인지만 assertion한다고 범위를 줄여야 한다. |
| 19 | fixed-track early return·malformed placement가 잘못된 trace를 남기는가 | PASS | fixed-only는 `FixedTrackEarlyReturn` 뒤 intrinsic batch 부재를, degenerate line은 `InvalidGridPlacement` 및 output/trace 부재를 확인한다([계획 L38](./c11-4-span-stage-oracle.md#L38)). |
| 20 | test-only 변경을 runtime/platform/C11.4 지원 완료로 과장하는가 | PASS | product/runtime, FFI, 앱·simulator, WPT는 제외하고 review20/PR diff30 후 test slice만 완료 판정한다([계획 L35-L39, L43-L63](./c11-4-span-stage-oracle.md#L35)). |

## 판정 요약

16 PASS, 4 PARTIAL이다. 이 기록은 첫 검토이며, implementation-ready 판정으로 사용하지 않는다. `Initialize`/`MaximizeTracks`, finite/무한 comparator 및 non-flex batch gate를 수정한 뒤 별도 20관점 재검토가 필요하다. 다른 계산과 gutter-index 경계는 독립 자료 및 source로 확인했다.
