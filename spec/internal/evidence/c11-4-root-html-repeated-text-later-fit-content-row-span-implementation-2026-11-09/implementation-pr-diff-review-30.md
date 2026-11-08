# C11.4 later fit-content row-span 구현 PR diff 실패 관점 검토 30개

검토 대상은 구현 후보의 staged 전체 diff다. Layout·Style projection admission, RuntimeSession/trace tests, Android·iOS 기본 route smoke, plan·상태 대장·내부 계약, 실행 자료와 screenshots를 함께 확인했다. 이 표는 계획 검토 20개나 구현/runtime 검토 20개를 재사용하지 않는다.

| # | PR diff에서 찾은 실패 가능성 | diff와 근거에서 확인한 방어 | 판정 |
|---:|---|---|---|
| 1 | CSS의 1-based position 계산이 Rust `insert` index와 어긋남 | Style test·RuntimeSession builder는 모두 `fit_position - 1`에 삽입하고 2·3 위치의 exact typed list를 검사한다. | 통과 |
| 2 | Layout helper와 Style helper가 서로 다른 허용 position을 사용 | 각 typed enum용 predicate가 6개 list, index `< 3`, 단일 supported fit track과 동일한 multiplicity를 확인한다. | 통과 |
| 3 | position 1 기존 track orders가 새 helper 도입으로 거부됨 | position-1 30-order Style, 960-scene span, 810-scene text regression이 통과했다. | 통과 |
| 4 | 위치 2·3 말고 4·5·6이 기존 repeated helper 경계를 우회 | Style 근접값 test와 RuntimeSession atomic rejection test가 positions 4–6을 각각 확인한다. | 통과 |
| 5 | position 2·3 목록이 9-item auto-placement도 넓힘 | separate `later_six_member_candidate` guard와 RuntimeSession auto-placement rejection test가 새 위치를 row-span profile로 한정한다. | 통과 |
| 6 | 이 guard가 기존 position-1 9-item profile도 거부 | 발견했던 over-broad count guard를 candidate distinction으로 좁혔고 기존 30 orders 회귀를 다시 통과시켰다. | 통과 |
| 7 | 하나 이상의 fit-content가 list에서 허용 | predicate는 fit track를 하나만 세며 Style projection과 RuntimeSession에서 duplicate-fit input을 atomic rejection한다. | 통과 |
| 8 | `min-content`·`max-content`·`auto` 개수가 변한 list가 통과 | Layout/Style 양쪽에서 정확히 `[2, 2, 1]` 개수를 검사하고 Runtime matrix도 각 scene multiset을 assertion한다. | 통과 |
| 9 | 6개가 아닌 list에서 다른 profile이 새 경로를 빌림 | 기존 4-member 및 3/5-item profile 분기와 새 6-member candidate 분기가 독립되어 있다. Layout·Style focused suite가 통과했다. | 통과 |
| 10 | 지원하지 않는 track kind가 fit-content list에 섞임 | 양쪽 helper의 `match` default는 거부하며 fit-content cap은 기존 finite non-negative direct px/percentage helper를 재사용한다. | 통과 |
| 11 | percentage cap이 다른 axis나 viewport에서 풀림 | Fixed definite `40×220px` content Grid, four authored cap forms와 exact computed order를 Chrome corpus·RuntimeSession에서 대조했다. | 통과 |
| 12 | cascade가 typed order를 바꾸거나 CSSOM serialization이 달라짐 | Matrix가 `ComputedCssGridAutoRows::TrackList`와 serialized `grid-auto-rows` 순서를 모두 exact 비교한다. | 통과 |
| 13 | 이 지원이 일반 Style/CSS route로 노출 | Projection은 기존 `RuntimeRootHtmlTextPaintV1` root reservation/profile 검증 안에 있으며 새 공개 API·FFI나 general author CSS gate 변경은 diff에 없다. | 통과 |
| 14 | 새 cap order에서 indefinite size 또는 implicit column이 생김 | projection/Layout validator는 fixed 40px single column, 10px explicit row, row flow 및 기존 definite basis 경계를 유지한다. | 통과 |
| 15 | `align-content` 값이 Layout과 Style에서 어긋남 | 양쪽 기존 allowlist를 유지하고 new matrix는 computed `start|stretch`를 exact 검사한다. | 통과 |
| 16 | hidden descendants가 item count를 바꾸며 허용 topology를 위조 | Layout은 hidden subtree를 제외한 child order를, Style은 visible parsed child list와 owner count를 각각 검증한다. | 통과 |
| 17 | Grid item wrapper나 payload가 임의 중첩 구조를 받음 | Span helper는 각 direct wrapper의 단일 child와 supported direct text payload grammar를 요구한다. | 통과 |
| 18 | 첫 empty cell이 실제 item content 또는 예상 밖 위치를 가짐 | 첫 item의 children empty 여부와 row/column `1/2` placement를 검사한다. | 통과 |
| 19 | text items의 span placements가 중복·누락되어도 success | topology validator는 row 2/span 2와 row 3/span 2를 각각 1회, 4-item에서만 row 2/span 3을 1회 허용한다. | 통과 |
| 20 | row span이 예측한 4개 used row 바깥으로 나감 | trace normalizer는 row start/span을 checked conversion/addition으로 바꾸고 trace의 4-row 경계 안인지 확인한다. | 통과 |
| 21 | 다른 Grid owner의 track trace를 사용 | Layout은 대상 `grid_id`와 일치하는 trace 하나만 허용하고 missing/mismatched owner trace atomicity tests가 rejection을 확인한다. | 통과 |
| 22 | negative implicit rows, collapsed rows, gap gutter가 일반 row처럼 계산 | Trace validator가 negative count 0, positive count 3, collapsed 없음, gutter 제외와 internal index 연속성을 검사한다. | 통과 |
| 23 | trace의 오래된 sizing pass나 incomplete stage 사용 | row axis의 최대 sequence pass와 마지막 stage만 선택하며 missing trace와 잘못된 count/index trace가 mutation 전에 error가 된다. | 통과 |
| 24 | item frame을 추정해 CSS Grid track size를 만들어냄 | Span normalizer는 final row trace를 source로 두고 기존 span·frame 관계가 원시 trace와 일치하는지 0.01px 경계로 검사한다. | 통과 |
| 25 | 1/64px correction이 payload child frame을 item과 분리 | item subtree의 `frames`와 `flow_frames`를 같은 `delta_y`로 이동하고 wrapper height만 snapped span height로 조정한다. | 통과 |
| 26 | 뒤 item 오류가 앞 item correction 뒤 partial layout을 노출 | RuntimeSession rejection은 `completed=None`을 확인하고 same-session valid recovery가 새 key와 기대 frame으로 돌아오는지 검사한다. | 통과 |
| 27 | matrix가 1,920이라고 기록하면서 곱집합 일부를 빠뜨림 | test가 unique key 1,920, position별 960, tail 30, cap 4, topology별 480, align·position·cap 전체조합을 assertion한다. | 통과 |
| 28 | source order reversal 또는 stretch relation을 scene count만으로 놓침 | `span2`·`span3` semantic frames의 order reversal과 non-auto used tracks의 start/stretch 관계를 각각 scene 전부에 걸쳐 비교한다. | 통과 |
| 29 | iOS/Android 화면이 서로 다른 HTML이거나 실기기 pixel parity라고 오인 | platform script/manifest와 runtime logs가 네 조합 각각 동일 input hash, default route, ready·10-box draw를 연결하며 README는 screenshot 한계를 명시한다. | 통과 |
| 30 | 새 구현 기록이 C11.4 체크 또는 internal version을 잘못 승격 | STATUS와 0059는 후보·지원 경계, `78/214`, C11.4/C11 미완료와 `0.1.0`을 유지한다. PR body에는 Android/iOS screenshots와 비검증 범위를 연결한다. | 통과 |

## 결과

서로 다른 30개 변경 실패 경로를 검토했다. 구현 중 발견한 기존 profile 회귀는 코드 조건과 regression test를 수정하고 position-1 matrix 재통과로 확인했다. 제품 지원 및 C11.4 전체 범위는 제한 상태로 남기며 이 검토는 완료 선언이 아니다.
