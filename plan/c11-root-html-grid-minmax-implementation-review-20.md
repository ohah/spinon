# C11.1 implementation/runtime hostile review · 20 perspectives

**대상:** base `773cfe232` 위의 현재 미커밋 변경과 [runtime evidence](../spec/internal/evidence/c11-root-html-grid-minmax-runtime-2026-11-03/README.md).

이 검토는 계획의 20관점 검토와 별도로 구현 코드·runtime 경계를 대상으로 했다. PR diff 검토 30관점은 별도 gate이며 아직 수행하지 않았다.

| # | 실패 관점 | 적대 입력·경계 | 확인 결과 |
|---:|---|---|---|
| 1 | CSS admission | 임의의 typed `MinMax`를 broad profile에 통과시키기 | Root HTML validator만 유한 `px/%` minimum과 유한 `px/%/fr` maximum을 허용한다. 다른 profile 경계는 유지된다. |
| 2 | Flex minimum grammar | `minmax(1fr, 80px)` 및 직접 typed-value injection | CSS 무효 선언은 앞선 유효 선언으로 cascade fallback되고, Root HTML gate도 flex minimum을 거부한다. |
| 3 | Negative/non-finite breadth | 음수 px, 음수 fr, NaN/Infinity | 유효하지 않은 computed breadth는 admission 전 거부된다. negative CSS 선언 cascade fallback 검사가 통과했다. |
| 4 | Intrinsic grammar | `auto`, min/max-content, `fit-content()` breadth | 제한 목록 바깥 breadth는 projection 전에 unsupported computed value로 실패한다. |
| 5 | Mixed-unit floor | percentage minimum과 px/percentage maximum의 사용값 비교 | 같은 definite 축 basis에서 resolve한 후 maximum을 minimum에 floor한다. percentage floor·px floor·max-under-min 사례가 통과했다. |
| 6 | Percentage basis ownership | child·viewport 값을 container basis로 잘못 쓰기 | axis validator는 그 computed-style snapshot의 grid width/height를 요구하고 해당 축 basis를 사용한다. |
| 7 | Indefinite sizing | percentage breadth가 `auto`/math/cyclic size에 의존 | direct percentage와 같은 definite finite non-negative axis gate에서 거부한다. |
| 8 | Zero definite basis | 0px width/height를 indefinite로 취급하거나 나눗셈 | 0은 definite basis로 허용되며 zero-percentage-basis runtime 사례가 통과했다. |
| 9 | Cross-axis substitution | row percentage를 width로 resolve하거나 그 반대 | columns는 width, rows는 height를 검사한다. row max와 percentage-row 사례가 통과했다. |
| 10 | Min/max constraints | percentage/`fr` minimum·maximum 계산 중 min/max-size를 무시 | 공유 axis constraint gate가 computed min/max-size와 aspect-ratio를 검사한다. |
| 11 | Fixed growth limit | available space가 min과 max 사이·max 초과 | fixed minmax는 available size 안에서 자라고 max에 도달하면 멈춘다. single-max와 bounded-space 케이스가 통과했다. |
| 12 | Fraction distribution | mixed fixed/percentage tracks와 gap을 fr 분모에서 누락 | 기존 Taffy flexible sizing 경로를 재사용하며 gap을 뺀 잔여 공간으로 분배한다. fraction-max·percent-flex-max 케이스가 통과했다. |
| 13 | Zero flex maximum | `minmax(20px, 0fr)`에서 minimum을 0으로 축소 | min floor를 유지한다. zero-fr-max 사례가 통과했다. |
| 14 | Fraction accumulator overflow | 여러 유한 `fr` 값 합계가 f32 범위를 넘김 | 축 validator가 direct와 minmax `fr` maximum을 누적해 합계 overflow를 layout 전에 거부한다. rejection matrix 검사가 통과했다. |
| 15 | Fixed resolved overflow | 큰 percentage·gap·track 합계가 f32 범위를 넘김 | 개별 resolved breadth와 고정 track/gap 누계를 f32 상한과 비교해 fail-closed한다. |
| 16 | Repeat expansion | integer `repeat()`가 만든 minmax track을 직접 입력과 다르게 처리 | 기존 integer repeat 확장 뒤 같은 bounded track gate를 적용한다. repeat-bounds와 direct-equivalent 사례가 통과했다. |
| 17 | Auto-repeat/metadata quota | named line·auto-repeat expansion으로 범위를 넓힘 | auto-repeat·line name은 거부하고 기존 track/line metadata quota를 바꾸지 않는다. |
| 18 | Implicit sizing leakage | 명시 minmax를 이유로 default auto implicit row를 intrinsic sizing 허용 | 추가 implicit row를 만드는 3번째 item은 `grid-auto-rows`에서 scene publish 전에 거부된다. |
| 19 | Placement/template leakage | column flow나 named area가 minmax 지원으로 우회 | column-flow guard와 named-area fixed-track guard가 별도로 유지된다. out-of-scope 입력에는 성공 분기가 없다. |
| 20 | Partial frame / release boundary | 거부 중 일부 frame 게시 또는 test-only ABI를 product에 노출 | layout 실패는 completed scene 없이 반환한다. Android/iOS surface capture 인자는 feature-enabled test build에서만 작동하고 default FFI build에는 C11 test ABI가 없다. |

**판정:** 이번 20관점 검토에서 해결되지 않은 구현/runtime 결함을 새로 찾지 못했다. 관련 검증은 42개 runtime tests, projection/parser focused tests, changed-crate Clippy, Android AVD PixelCopy 두 번, iOS Simulator Metal sample이다. 별도 30관점 구현 PR diff 검토와 PR CI/merge는 남아 있다.
