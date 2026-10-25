# C11.4 span-stage test-only PR #642 diff review · 30관점

**초기 30관점 검토 대상:** [PR #642](https://github.com/ohah/spinon/pull/642), head `9e41800f3528555d719d2907074bd445794267bd`, base `a33b245983fae050b66efb9ff259b3fe1132a051`. GitHub `gh pr diff`와 branch-local diff의 파일 목록을 대조했다. 변경된 test 코드, plan/evidence, 구현 검토 20관점·addendum, 당시 PR 본문을 읽기 전용으로 확인했다. 이 review 파일 자체는 검토 대상에서 제외했다. 테스트·브라우저·시뮬레이터는 실행하지 않았다.

## 판정

**초기 판정 PASS 28 / PARTIAL 2 / FAIL 0. 현재 유효 판정 PASS 30 / PARTIAL 0 / FAIL 0.** 초기 sizing assertion·negative control에서 correctness blocker는 발견하지 못했다. 최초 PARTIAL은 (1) 500행 초과 파일 사유의 부정확한 private-visibility 설명, (2) implementation evidence README의 잘못된 상대경로와 문서 언어 문제였다. PR head `e75dfae5e1e19054f2f47442f5868f071bd9038d`에서 해당 세 세부 지적을 targeted recheck해 모두 해소됨을 확인했다(아래 addendum). 상세 구현자의 test/fmt/Clippy 결과는 evidence와 PR 본문의 **보고값**으로만 확인했으며 재실행하지 않았다.

## 초기 PR diff 실패 관점 30개

| # | 별도 실패 관점 | 판정 | 확인 내용 |
|---:|---|:---:|---|
| 1 | test-only slice가 product/runtime 코드를 함께 바꾸는가 | PASS | 최종 GitHub diff의 유일한 Rust 파일은 `grid_multispan_chrome_oracle_tests.rs`다. 나머지는 plan/evidence 변경이며 CSS cascade·runtime·FFI·renderer 코드는 바뀌지 않는다. |
| 2 | `multi_span_input` 추출이 기존 positive case의 HostDocument 입력을 바꾸는가 | PASS | root와 item 생성, DOM 자식 순서, style map과 placement map은 기존 builder 로직을 보존하고 track 목록·placement mode만 인자로 받는다. Positive는 원래처럼 `TrackSizing`을 사용한다. |
| 3 | Grid의 definite basis·track 함수·gap이 oracle와 달라지는가 | PASS | 네 개 `minmax(0,max-content)` column, 220×40 root, 10px row, 0px gap이 유지된다. Fixed control은 별도로 네 55px column을 사용한다. |
| 4 | A/B/C item의 크기나 explicit span이 바뀌는가 | PASS | A 80px/span-2, B 120px/span-2, C 200px/span-3의 line 범위와 definite size가 기존 fixture·독립 JSON에 맞는다. |
| 5 | source-order independence 대조가 제거되거나 약해지는가 | PASS | `[A,B]`, `[B,A]`, `[A,B,C]`, `[C,B,A]` 네 케이스와 expected final columns, `[2]`/`[2,3]` batch sequence가 유지된다. |
| 6 | 다른 Grid의 trace 또는 다중 Columns pass 중 하나를 임의로 고르는가 | PASS | trace는 root `NodeId`로 선택한다. Columns pass는 전부 모아 정확히 하나인지 검사한 뒤 사용한다. |
| 7 | 필수 stage가 누락되거나 중복돼 첫 결과로 가려지는가 | PASS | `unique_stage`가 Initialize, 각 span batch, IntrinsicSizingComplete, MaximizeTracks 각각의 일치 수가 정확히 1인지 요구한다. |
| 8 | expected stage 순서는 맞지만 실제 ordering이 바뀌는가 | PASS | 선택한 stage index가 Initialize → batch 배열 → IntrinsicSizingComplete → MaximizeTracks 순으로 엄격히 증가해야 한다. |
| 9 | span group이 누락·추가되거나 source-order로 재정렬되는가 | PASS | 기존 observed span order와 expected 배열을 대조하고 모든 `IntrinsicItemBatch` 수가 expected batch 수와 같아야 한다. 각 span stage도 고유해야 한다. |
| 10 | non-flex oracle가 flexible track batch와 혼동되는가 | PASS | 각 batch stage 전체를 `crosses_flexible_track: false`와 비교한다. `true` batch가 있으면 span 숫자가 같아도 실패한다. |
| 11 | contribution trace의 item/kind/span/value가 다른 item에 붙는가 | PASS | 기존 기여 assertion은 NodeId·kind·span·값의 순서를 대조한다. source-order별 owner와 80/120/200px 기여를 독립 expected map으로 유지한다. |
| 12 | Initialize base가 0이 아니거나 비유한데 통과하는가 | PASS | CSS track base마다 `is_finite()`를 확인한 뒤 1e-4px tolerance로 0과 비교한다. |
| 13 | Initialize/batch `None`을 내부 `+∞`라고 잘못 확정하는가 | PASS | Test 주석, plan, expected JSON은 serializer가 `+∞/−∞/NaN`을 모두 `None`으로 projection한다고 제한한다. 판정은 실제 내부 `+∞`의 증명으로 확대하지 않는다. |
| 14 | finite expected growth가 `None`, NaN 또는 무한대와 일치하는가 | PASS | Expected `Some(finite)`에는 actual `Some(finite)`만 허용하고 1e-4px 안에서 비교한다. `None`/`Some` 혼합은 실패한다. |
| 15 | finite base stage가 오차를 허용하지 않거나 비유한 값을 숨기는가 | PASS | 매 CSS track base는 finite 확인 뒤 expected 값과 1e-4px 비교를 거친다. Chrome final의 별도 1/64px tolerance와 분리돼 있다. |
| 16 | gutter sentinel이 빠지거나 entry 수가 달라져도 값 비교가 통과하는가 | PASS | CSS track `N`개에 대해 stage vector 길이를 정확히 `2N+1`로 고정한다. |
| 17 | 내부 gutter를 CSS track으로 취급하거나 경계 sentinel을 일반 gap으로 세는가 | PASS | 짝수 entry는 gutter, 홀수 entry는 CSS track인지 순서대로 검사한다. 두 경계 gutter도 이 구조에 포함된다. |
| 18 | boundary 또는 내부 gutter/track의 collapse flag가 틀려도 통과하는가 | PASS | 각 entry에 `is_collapsed`를 직접 비교해 index 0과 마지막 boundary gutter만 collapsed, 나머지는 non-collapsed여야 한다. 이전 구현 review의 PARTIAL을 고친 diff와 addendum가 이 assertion을 확인한다. |
| 19 | collapsed/gutter filter가 track 누락을 감추는가 | PASS | flag pattern을 먼저 검증한 뒤 `!is_gutter && !is_collapsed`로 CSS tracks를 추출하고 개수와 vector order를 비교한다. |
| 20 | 0px gap인데 gutter base가 크거나 비유한 값이어도 통과하는가 | PASS | 모든 gutter entry의 base는 finite이며 절댓값 1e-4px 이내의 0이어야 한다. 이번 규범 JSON은 CSS track growth arrays를 기대값으로 정의하므로 gutter growth limit 자체는 별도 assertion 대상이 아니다. |
| 21 | 겹친 span-2 item이 planned increase를 합산해 중간 track을 과대 확대하는가 | PASS | span-2 snapshot `[40,60,60,None]`이 같은 group의 track별 maximum planned increase를 확인한다. 역순 controls도 같은 기대 배열을 요구한다. |
| 22 | span-3 계산이 앞선 span-2 batch를 반영하지 않는가 | PASS | span-3 snapshot `[160/3,220/3,220/3,None]`은 기존 growth 합 160px에서 남은 40px을 세 track에 분배한 결과와 맞는다. 두 source order 모두 같은 규범 batch 순서를 검사한다. |
| 23 | span 밖 네 번째 track의 IntrinsicSizingComplete 처리가 잘못되는가 | PASS | Complete stage에서 네 CSS track base는 계속 0이고 growth는 `[160/3,220/3,220/3,0]` 또는 span-2 전용 `[40,60,60,0]`이어야 한다. |
| 24 | MaximizeTracks가 최종 base 또는 growth를 잘못 닫는가 | PASS | span-2 전용 및 span-2/3 각각 expected final base와 growth array를 비교하며 non-finite projection에 의존하지 않는 finite 구간이다. |
| 25 | remaining free space를 track만 합산하지 않거나 잘못 계산하는가 | PASS | Maximize stage 전체 entry base를 합산해 220px에서 빼고, 각 case의 60px/20px 잔여 공간을 별도 판정한다. 앞 단계의 zero-gap gutter-base assertion이 합산 경계를 고정한다. |
| 26 | 기존 Chrome final geometry regression이 중간 trace로 대체되는가 | PASS | 기존 `expected_columns`와 item x/y/width/height assertion이 유지된다. Chrome quantization은 1/64px로 비교하며 spec-derived stage tolerance와 섞지 않는다. |
| 27 | fixed-only input이 intrinsic sizing 경로를 거쳐도 negative control이 통과하는가 | PASS | 네 fixed column case에서 Columns pass 1개, `FixedTrackEarlyReturn` 1개, intrinsic batch 부재를 검사한다. |
| 28 | 같은 start/end line이 실제 rejection 경계를 통과하지 않고 성공하는가 | PASS | Invalid case는 정상 placement normalizer를 건너뛰는 `TrackSizing`이 아니라 `Existing` mode를 쓰고, `LayoutError::InvalidGridPlacement`를 정확히 요구한다. `compute`가 `Err`라 성공 LayoutOutput/trace를 만들 수 없다. |
| 29 | 631행 단일 test module이 권장선 예외 없이 커지는가 | PARTIAL | PR 본문은 한 fixture builder·한 multi-span oracle 책임을 이유로 한 파일을 유지한다고 설명한다. 같은 책임으로 묶는 예외는 타당할 수 있지만, 별도 child module은 parent의 private 항목에 접근할 수 있으므로 “나누면 builder 복제 또는 private scope 확장 필요”라는 설명은 정확하지 않다. 한 책임의 응집 이유를 남기되 이 부정확한 주장은 고치는 편이 좋다. |
| 30 | 새 evidence가 reviewer에게 열리고 저장소 문서 언어 규칙을 지키는가 | PARTIAL | `implementation` evidence README의 코드 링크 `../../../crates/...`는 한 단계 부족해 `/spec/crates/...`로 해석되고 target을 찾지 못한다. Repo root까지 4단계 올라가는 경로가 필요하다. 이 새 README에는 한글로 쓰지 않은 설명 문장도 여러 곳 있어 `docs/project-rules.md`의 한글 문서 규칙과 맞지 않는다. PR 본문과 다른 relative links는 읽기 가능하고, 나머지 evidence/plan 링크는 존재한다. |

## 통합 payload 확인

- 구현 review20은 처음의 `19 PASS / 1 PARTIAL` 중 boundary gutter collapse flag 관점을 찾았고, addendum는 현재 assertion과 vendored Taffy boundary를 대조해 유효 `20/20 PASS`로 갱신했다. 두 자료의 이전·후속 판정 흐름은 서로 모순되지 않는다.
- Plan과 implementation evidence는 수치·fixture·검증 범위를 test-only로 한정하며 C11.4 Stage B, C11.4 checkbox와 공식 `72/213`이 미완료임을 유지한다. PR diff에는 `spec/STATUS.md`가 없다.
- PR 본문은 fixed/invalid controls와 gutter finding/fix를 설명하고, Chrome 155 이미지를 **기존 final-geometry baseline**이라고 표시한다. GitHub contents API에서 해당 b58de5df commit의 PNG 파일 존재를 확인했다. 이미지 render는 browser 사용 금지 범위라 확인하지 않았다.
- 최신 `gh pr view` 기준 라벨은 작업 유형 `chore`와 `area: layout`, `area: testing`, `area: docs`다. 작업 유형 및 변경 영역 라벨 요구를 충족한다.

## 초기 blocker 판정

최초 head에서 sizing assertions·trace 해석·negative controls에 구현 correctness blocker는 없었다. 당시 #29/#30 문서 PARTIAL은 아래 `e75dfae5` targeted recheck에서 해소 여부를 확인했다.

## Targeted recheck addendum · 2026-10-26

재확인 대상은 PR #642의 GitHub head `e75dfae5e1e19054f2f47442f5868f071bd9038d`와 base `a33b245983fae050b66efb9ff259b3fe1132a051`다. `gh pr view`에서 PR은 OPEN/CLEAN이며 head SHA가 해당 commit과 일치했다. 새 diff/body, 구현 evidence README, parent plan entry만 영향을 받은 지적에 한정해 다시 확인했다.

| 초기 지적 | 재확인 결과 |
|---|---|
| #29 단일 test 파일 사유가 private visibility/builder 복제를 주장함 | **해소.** 최신 PR 본문은 네 기존 Chrome multi-span case, 같은 HostDocument fixture 및 같은 oracle 책임을 근거로 들며, 독립 경계가 없고 module 연결만 늘어난다고 설명한다. 부정확했던 private visibility 또는 builder 복제 주장은 남아 있지 않다. |
| #30 evidence 코드 링크가 깨짐 | **해소.** README 링크가 `../../../../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs`로 바뀌었다. evidence 디렉터리 기준 상대경로가 저장소 루트의 파일에 도달하며 target 존재를 확인했다. 같은 README의 구현 review20 및 addendum 상대 링크도 모두 존재한다. |
| #30 evidence README가 한글 문서 규칙을 어김 | **해소.** README 제목·절·설명 문장은 한글 문장으로 작성되어 있고 영어 설명 문장은 남아 있지 않다. `Internal vector`, `Expected unbounded trace limit` 등 일부 기술 용어는 원어를 유지하지만 설명 문장 자체는 한국어다. |
| parent plan 및 PR 메타데이터 연결 상태 | **PASS.** `plan/c11-4-grid-track-sizing.md`는 PR #642와 실행 evidence/review 문서를 연결하고 C11.4 Stage B·상위 체크·공식 `72/213`이 미완료라고 유지한다. `spec/STATUS.md`의 C11.4 항목도 unchecked다. PR 본문 역시 Stage B/전체 C11.4 미완료와 `72/213` 유지를 설명한다. 라벨은 `chore`와 세 area 라벨이다. |

따라서 영향받은 세 PARTIAL 세부 지적은 모두 해소되었고 **유효 최종 결과는 PASS 30 / PARTIAL 0 / FAIL 0**이다. 이 addendum 검토는 읽기 전용이다. 테스트·브라우저·시뮬레이터는 실행하지 않았으며, 구현자의 기존 테스트 결과는 보고값으로만 인용한다.
