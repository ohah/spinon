# C11.4 pinned WPT implicit-column PR 변경 검토

검토 대상은 이 PR의 pinned WPT assertion fixture, runtime/FFI 진단 연결, Android/iOS harness 통합, 상태·계획·실행 근거다. 계획 검토와 구현/runtime 검토와 분리해 각기 다른 변경 경계와 실패 유입 경로를 확인했다. 발견한 구현/실행 수정은 [구현 검토](./implementation-review-20.md)에 기록했다.

| # | 변경 경계와 실패 관점 | 적대 확인 결과 |
|---:|---|---|
| 1 | 공개 CSS admission이 fixture 추가를 따라 일반화되는가 | scene은 `c04-runtime-gpu-fixture-test` 경로에만 추가됐다. author CSS 지원이나 parser admission 코드는 바뀌지 않았다. |
| 2 | 내부 계약 버전이 범위보다 넓게 승격되는가 | `0059`와 내부 숫자 버전은 바뀌지 않았다. 일반 `RuntimeBlockFlexGridTrackSizingV1` 계약을 넓히지 않은 진단 fixture다. |
| 3 | C11.4 module wiring이 다른 fixture 파일을 대체하거나 중복 등록하는가 | 새 Rust 파일은 전용 `#[path]` 모듈로 연결됐고 기존 fit-content scene과 별도 route다. |
| 4 | 새 scene ID가 부분 문자열·접미사·스크립트 변형으로 허용되는가 | Rust source allowlist는 exact ID 일치이며 `;throw` 접미와 미등록 이름 대조를 거부한다. FFI route focused test는 1/1 선택되어 통과했다. |
| 5 | 새 ID가 기존 scene route와 충돌하는가 | C11.4 통합 dispatcher의 기존 fixture 집합에서 같은 ID를 찾지 못했고 단일 source 반환 경로를 확인했다. |
| 6 | scene source가 pinned WPT의 검증된 세 요소 topology를 바꾸는가 | root/item/child를 각각 한 번 생성하고 3-node parent-child 관계와 고정 ID를 사용한다. source allowlist test도 개수와 연결을 검사한다. |
| 7 | fixture CSS가 source hash가 고정된 WPT 입력에서 sizing 관련 값을 벗어나는가 | 300×300 root, 60px explicit column, 40px implicit fit-content cap, line 2, `min-width:0`, 400×10 child가 `expected.json`과 일치한다. |
| 8 | box sizing·spacing 변경이 기대 track size를 우연히 맞추는가 | 세 요소의 box sizing과 margin/padding/border, root gap을 고정 검사한다. 색 배경은 크기 입력을 바꾸지 않는다. |
| 9 | root clip 또는 child overflow 설정이 overflow assertion을 숨기는가 | root에 clipping/overflow 규칙이 없고 child는 400px로 유지된다. 표면 샘플은 300px root 바깥 child까지 읽는다. |
| 10 | raw CSS 문자열만 맞고 typed cascade가 다른 grid input으로 projection되는가 | fixture preparation은 허용된 computed style snapshot과 `GridContainerInput`을 함께 확인하고 source tree/style 변조를 실패 처리한다. |
| 11 | 두 번째 grid나 모호한 root가 diagnostic observation에 섞이는가 | 관찰은 고정 root/node ID와 해당 root의 단일 columns/rows pass를 대상으로 하며 대상 외 node/frame 집합은 성공 조건에 포함되지 않는다. |
| 12 | node ID 재사용, extra style 또는 빠진 style이 관찰을 통과하는가 | 세 고정 node를 일대일로 확인하고 style/node cardinality와 parent identity를 검사한다. |
| 13 | item placement와 explicit/implicit 경계가 다른데 track 결과만 같아 통과하는가 | computed placement와 typed input에서 explicit `[60]`, implicit `fit-content(40px)`, `grid-column:2`를 각각 확인한다. |
| 14 | columns trace의 minimum과 intrinsic contributions가 혼동되는가 | item `Minimum=0`/`SpecifiedMinimum`, `MinContent=400`, `MaxContent=400`과 span/axis identity를 검증한다. |
| 15 | rows 기여값이 바뀌어도 최종 10px frame만으로 통과하는가 | Rust validator와 두 host matcher가 row `Minimum=10`/`AutomaticMinimum`, `MinContent=10`, `MaxContent=10`을 각각 확인한다. |
| 16 | 한 축 pass가 누락되거나 반복되는데 다른 축만으로 성공하는가 | columns/rows 각각의 trace identity, 축 순서 및 pass 수를 검사하고 누락·extra pass를 거부한다. |
| 17 | trace 배열이 잘못된 단계에서 올바른 숫자만 내는가 | 단계 종류, 여섯 단계 순서, gutter slot, visible track 수와 stage별 base 배열을 별도로 확인한다. |
| 18 | 내부 `StretchAutoTracks.applied`를 normative track growth로 과장하는가 | Columns false / Rows true를 내부 Taffy regression 값으로만 기록한다. row base 10px을 별도 확인하고 docs가 CRD/Chrome per-stage oracle가 아님을 명시한다. |
| 19 | 기여 항목 순서·source·finite tolerance 변조가 느슨한 비교로 통과하는가 | contribution count/kind/order/source/span과 작은 수치 오차 기준을 검사한다. Android/iOS parser도 누락·비유한 값·잘못된 cardinality를 거부한다. |
| 20 | pre-render layout frame 누락·extra·node 재배치가 표면 결과만으로 통과하는가 | root/item/child 전체 frame map과 300×300, `(60,0,40,10)`, `(60,0,400,10)`을 검사하며 표면 샘플은 layout oracle를 대체하지 않는다. |
| 21 | 스타일/환경 revision과 다른 layout 또는 draw가 한 실행처럼 합쳐지는가 | runtime key와 report revision을 보존하고 Android/iOS가 environment와 presentation identity 및 sequence/generation을 묶는다. |
| 22 | serialization 실패·partial report가 성공 경로를 계속하는가 | 누락 frame/pass/style은 report 생성 오류가 되고 runtime validation 실패는 completed output 경로에서 반환된다. partial presentation을 성공으로 표시하지 않는다. |
| 23 | FFI report의 새 optional field가 다른 C11.4 trace payload를 덮는가 | `fitContentImplicitColumn`은 별도 optional key로 연결됐고 기존 fixture 값은 `None`인 test expectation을 추가했다. |
| 24 | FFI report가 최종 frame·trace 대신 예상값만 반환하는가 | report는 completed output에서 frame, stages, contributions와 revisions를 직렬화하고 expected metadata를 별도 key로 둔다. |
| 25 | production archive에 test scene id/marker/payload가 노출되는가 | 기본 product static archive에는 marker 4종 0개, opt-in fixture-test positive control에는 포함됨을 archive logs/hash가 기록한다. 판정 범위는 static archive다. |
| 26 | Android scene dispatch가 기존 route를 가로채거나 잘못된 viewport를 고르는가 | dedicated scene matcher가 exact ID를 사용하고 480×320 viewport/expected frame 경로를 기존 scene별 조건과 함께 고정한다. |
| 27 | Android JSON coercion, missing field, NaN/Infinity가 통과하는가 | parser는 typed object/array, scalar, finite geometry, expected scene 및 trace/contribution fields를 검증한다. 실패 시 presentation gate가 닫힌다. |
| 28 | Android surface pixels가 root 안의 쉬운 영역만 읽어 overflow를 놓치는가 | PixelCopy 표본은 root 안, item 안 child, root 바깥 x=350 child, x=470 clear를 나눠 4/4 exact RGBA로 확인한다. |
| 29 | iOS Swift source가 빌드 대상에서 빠지거나 Metal sample이 다른 viewport를 읽는가 | 새 Swift 파일이 PBXBuildFile·Sources build phase에 등록됐다. drawable 1440×960/scale 3은 480×320 CSS viewport와 일치한다. |
| 30 | iOS 표면 근거와 공식 문서가 증명 범위를 넘기는가 | Metal 표본은 OS compositor 전이라고 기록하고, 계획/status/evidence는 WPT assertion 하나의 opt-in slice로 제한한다. C11.4 checkbox와 72/213은 유지한다. |

## 후속 문서 갱신 확인

PR 번호를 기록한 뒤 status/plan/runtime evidence의 PR 링크를 갱신하고, 최종 문서 diff를 이 검토 관점에 다시 대조한다.
