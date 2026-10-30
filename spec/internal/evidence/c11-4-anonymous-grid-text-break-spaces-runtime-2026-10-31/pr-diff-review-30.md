# C11.4 break-spaces 구현 PR diff 실패 관점 검토 30개

검토 대상은 구현 code, Android/iOS fixture validators, capture/Node module-boundary tests, 내부 contract/status, 계획 상태 및 새 evidence/screenshots다. 구현/runtime 검토 20개와는 검토 단위를 분리하고, 각 항목은 다른 diff boundary를 확인한다.

| # | PR diff 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 새 white-space mode가 공개 product API로 승격될 수 있음 | 변경은 내부 enum과 opt-in fixture path에 한정; 새 C ABI 없음. PASS |
| 2 | C11.4 fixture feature가 기본 archive로 새어 나갈 수 있음 | product/fixture-test archive symbol/string positive-control 대조에서 새 ABI/marker는 fixture archive에만 존재. PASS |
| 3 | enum variant 추가 뒤 exhaustive `match`의 누락/잘못된 fallback | workspace compile, focused tests, all-target Clippy 통과. PASS |
| 4 | Parley에 mode를 전달하며 `break-spaces`가 preserve-only로 축소될 수 있음 | `WhiteSpaceCollapse::BreakSpaces`와 wrapping 설정을 함께 전달. PASS |
| 5 | Normal collapsing 결과가 바뀔 수 있음 | 기존 Normal controls와 26-scene simulator matrix 통과. PASS |
| 6 | NoWrap이 wrap profile로 바뀔 수 있음 | NoWrap은 기존 `NoWrap` mapping 유지; 기존 scene 두 플랫폼 재검증. PASS |
| 7 | PreLine segment break가 preserved space mode로 바뀔 수 있음 | PreLine은 별도 branch/mapping 유지; 기존 scene 두 플랫폼 재검증. PASS |
| 8 | PreWrap hanging 또는 tab expansion이 변할 수 있음 | PreWrap의 tab expansion/whitespace branch는 그대로 mode scoped; 기존 matrix 통과. PASS |
| 9 | BreakSpaces tab을 구현된 tab-size처럼 조용히 취급할 수 있음 | normalize에서 전용 unsupported error; intrinsic/layout 경로 양쪽 test. PASS |
| 10 | LF/CR/CRLF normalization이 공백을 버리거나 중복할 수 있음 | segment break source mapping과 CRLF block-size unit test 확인. PASS |
| 11 | CRLF가 두 `TextSource` 사이에서 갈라질 때 source identity가 합쳐질 수 있음 | source split regression에서 CR 및 LF 각각의 node/source ranges 보존. PASS |
| 12 | normalization 뒤 source UTF-16/paragraph UTF-8 range가 틀릴 수 있음 | direct CRLF test에서 각 range를 검증. PASS |
| 13 | HTML comment 대응의 두 source가 anonymous item 두 개로 변할 수 있음 | layout intrinsic input 하나와 각 text source provenance를 검사. PASS |
| 14 | max-content가 내부/후행 공백을 누락할 수 있음 | `XX  XX` 60px, `XX  ` 40px expected values가 Rust 및 앱 validator에 동기화. PASS |
| 15 | min-content가 공백 뒤 break opportunity를 놓칠 수 있음 | max/min-content unit 및 track tests에서 30px 고정. PASS |
| 16 | soft-wrap이 preserved line-end space를 hang하거나 width에서 제외할 수 있음 | 25px track과 30px line advance, hanging advance 0을 layout test로 확인. PASS |
| 17 | min-content reflow 뒤 row height가 stale intrinsic 값일 수 있음 | unit/grid tests에서 min-content height, final Rows track 및 root/item frames 일치. PASS |
| 18 | forced break 앞 spaces의 contribution이 max/min axis에서 빠질 수 있음 | 40px max, 30px min, 3 line·30px min-content height 대조. PASS |
| 19 | whitespace-only direct text가 anonymous item/paint box를 만들 수 있음 | zero intrinsic item, no text layout, 0×0 root/frame와 simulator `boxes=0` 검증. PASS |
| 20 | whitespace-only text가 뒤 block의 placement를 밀 수 있음 | `[20,20]` tracks에서 block 첫 cell frame을 Rust와 양 platform으로 확인. PASS |
| 21 | computed root style 또는 source shape 완화로 일반 author input이 들어올 수 있음 | runtime root/style/child scene allowlist를 exact IDs와 required profile로 유지. PASS |
| 22 | capture, FFI, Rust runtime scene ID 배열이 갈라지거나 중복될 수 있음 | 각 fixed list length/uniqueness와 Node scene allowlist test 통과. PASS |
| 23 | FFI-generated source가 expected CSS values와 불일치할 수 있음 | 26-scene FFI matrix test 및 fixture boundary 검사 통과. PASS |
| 24 | runtime expected intrinsic min/max/line height가 틀릴 수 있음 | candidate의 각 신규 scene에 root/item/source expectations를 고정. PASS |
| 25 | Android validator가 box count만 보고 wrong tracks/contributions를 놓칠 수 있음 | full root frame, intrinsic source, Columns/Rows final tracks 및 contribution kind/value 확인. PASS |
| 26 | iOS validator가 Android와 다르게 intrinsic 또는 trace를 관대하게 볼 수 있음 | 같은 10 신규 scenes의 source/style/frame/track/contribution 검사를 확인하고 simulator matrix 통과. PASS |
| 27 | capture expected JSON이 runtime validators와 불일치할 수 있음 | scene별 expected root/node frame, intrinsic input, track trace와 box count를 대조. PASS |
| 28 | 전체 capture catalog count를 과거 값으로 두어 neighboring tests를 깨뜨릴 수 있음 | `633` 참조를 새 `643` catalog count로 갱신; Node boundary 24/24 통과. PASS |
| 29 | screenshot/evidence가 hardware/pixel proof로 과장될 수 있음 | AVD renderer와 iOS pre-compositor 경계, glyph pixel 한계를 README와 contract에 명시. PASS |
| 30 | C11.4 또는 공식 완료 수를 후보 하나로 잘못 올릴 수 있음 | 계획/0059/STATUS/README 모두 C11.4 미완료와 `72/213`을 유지한다. PASS |

## 판정

30개 diff 관점에서 차단 finding은 없었다. `cargo clippy --locked --workspace --all-targets -- -D warnings`, rustfmt, diff check와 CSS fixture-module boundary 24/24 통과했다. `cargo test --locked --workspace`는 변경 밖 C07.3 aspect-ratio computed-string assertion 한 건에서 실패했다(기대 `1.77778 / 1`, 실제 `1.7777778 / 1`); 이를 green으로 세지 않는다. Android/iOS app source와 installed artifact hashes가 맞고, 26-scene runtime matrices는 각각 26/26이다.

이 검토는 PR hosting/required-check 결과나 전체 WPT 적합성을 대체하지 않는다. 병합 직전 PR head diff를 재검증하고 현재 branch diff와 비교해야 한다.
