# C11.4 `grid-layout-auto-tracks` PR diff 적대 검토 30개

구현 source, Android/iOS adapter, fixture boundary regression, runtime evidence와 로드맵 변경을 병합 후보 diff 기준으로 서로 다른 실패 경계에서 검토했다. 구현/runtime 20개 검토를 재사용하지 않았다.

| # | 실패 관점 | 확인 결과 | 판정 |
|---:|---|---|---|
| 1 | 새 FFI module이 `fixtures/c11_4/` 책임 경계를 벗어난다 | scene source와 report module이 작업별 fixture 경로에 있다 | PASS |
| 2 | report module이 제품 feature에서 컴파일된다 | report module은 fixture-test feature로 조건부 선언된다 | PASS |
| 3 | runtime source route의 기존 scene 동작을 가린다 | 새 scene 분기는 기존 allowlist route의 첫 번째 exact match로만 호출된다 | PASS |
| 4 | report JSON에서 item frame이 아닌 child frame을 B로 보고한다 | node IDs와 root/A/B/B-child/C frames를 별도 key로 기록한다 | PASS |
| 5 | rekeyed runtime diagnostic에서 새 observation이 사라진다 | `with_revision`이 observation을 보존하며 fixture 없는 진단은 `None`을 쓴다 | PASS |
| 6 | 다른 fixture의 `C11_4TextInput`과 새 Ahem engine이 혼합된다 | 새 input이 선택된 경우에만 cloned text input을 사용한다 | PASS |
| 7 | unrelated profile/runtime layout에 C11.4 adapter가 적용된다 | C11.4 fixture feature와 `BlockFlexGridTrackSizing` profile을 조건으로 삼는다 | PASS |
| 8 | CSS track input과 computed style만 따로 통과한다 | preparation 및 observation 단계에서 두 representation을 모두 검증한다 | PASS |
| 9 | A/B/C node identity가 충돌한다 | 다섯 element NodeId 고유성을 확인한다 | PASS |
| 10 | trace report가 진단 없는 runtime completion을 panic/unwrap한다 | report는 구체적 오류를 반환하고 observation 부재는 `None`으로 처리한다 | PASS |
| 11 | Rust fixture source가 DOM node/text 개수를 바꾼다 | FFI unit test는 div 5개, text 3개, NBSP와 주요 placement를 고정한다 | PASS |
| 12 | pure layout regression이 시스템 font에 의존한다 | 저장소 Ahem fixture bytes와 system-font-free engine을 사용한다 | PASS |
| 13 | pure layout test가 positive implicit row/column을 분리하지 못한다 | columns, rows, root/item/child frames와 row contributions를 각각 assertion한다 | PASS |
| 14 | root sizing policy가 viewport 크기에 따라 달라진다 | pure input의 root width를 fixed 100, height를 auto로 지정하고 viewport를 명시한다 | PASS |
| 15 | item minimum contribution의 source order가 잘못됐다 | row minimum entries A/B/C 세 개의 수와 값을 고정한다 | PASS |
| 16 | static analysis warning을 새 코드가 만든다 | 전체 workspace all-target Clippy `-D warnings`가 통과했다 | PASS |
| 17 | FFI test adapter가 기본 host artifact에 노출된다 | 기본/제품 archive에서 fixture marker 0개, positive control에서 8개를 확인했다 | PASS |
| 18 | Android scene allowlist는 갱신했지만 기대 root/item frame을 빠뜨린다 | root 100×20, item 100×10, 5 node frames를 고정했다 | PASS |
| 19 | Android report parser가 marker 이후 trailing text를 성공으로 처리한다 | JSONTokener object parse와 typed property/revision/trace assertions를 사용한다 | PASS |
| 20 | Android pixel matrix에 중복 또는 unknown sample이 들어온다 | 고정 ID 집합, 정확한 개수, target ID와 CSS 좌표를 검사한다 | PASS |
| 21 | Android RGB가 맞지만 alpha가 다른 pixel을 놓친다 | RGBA 전체 `6,214,160,255` 및 `0,0,0,255`를 비교한다 | PASS |
| 22 | iOS matcher가 trace axis 순서를 바꾸어도 통과한다 | 정확히 두 pass에서 Columns 뒤 Rows를 검사한다 | PASS |
| 23 | iOS typed JSON optional field 누락이 정상으로 취급된다 | observation, revision, style, input, IDs, frames, traces의 각 field를 guard한다 | PASS |
| 24 | iOS sample payload에 다른 alpha/coordinate가 들어온다 | exact sample ID·node·CSS coordinate·RGBA tuple을 검증한다 | PASS |
| 25 | Android/iOS 내부 stretch flag 차이가 track geometry regression을 숨긴다 | matcher는 해당 flag를 추론하지 않고 stage 순서와 final bases를 비교한다 | PASS |
| 26 | feature catalog의 C11.4 scene 수 회귀가 테스트에서 가려진다 | capture runner의 실제 508과 불일치한 이전 496 기대값을 갱신하고 Bun suite를 통과했다 | PASS |
| 27 | multiline Cargo feature의 새 dependency가 regression test에서 누락된다 | boundary test의 기존 한 줄 regex를 multiline feature 선언에 맞췄다 | PASS |
| 28 | 문서가 이 normalized fixture를 upstream WPT와 동일한 CSS라고 주장한다 | Ahem normalization과 Chrome default-font observation을 분리해 기록한다 | PASS |
| 29 | simulator/AVD 표본을 hardware 또는 compositor 결과로 확대한다 | AVD SwiftShader 및 iOS pre-compositor 한계를 기록한다 | PASS |
| 30 | 제한 fixture가 C11.4 완료 수를 올리거나 전체 WPT PASS로 보인다 | 계획, STATUS, README가 C11.4 미완료와 `72/213`을 유지하고 scope를 제한한다 | PASS |

## 판정

PR diff 검토 PASS 30/30, PARTIAL 0, FAIL 0. 계획 단계 30개 diff 검토, 구현/runtime 20개 검토 및 이 PR diff 검토는 별도 기록이다.
