# C11.4 Root HTML auto-row gap 구현·runtime 실패 관점 검토 20개

검토 대상은 `RootHtmlAutoRowStretchV1`의 row-gap admission, 기존 Taffy gap/content-distribution 경로, 두 implicit auto-row runtime scene, 지원 밖 입력의 원자 거부다. 독립 기대값은 같은 폴더의 Chrome 157 precomparison JSON에 고정했다. 검토 때 공식 C11 상태는 `78/214`이며 이 slice는 완료 승격 대상이 아니다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | CSS 문자열 재해석으로 computed 단위·타입을 잃음 | Style projection은 `ComputedCssSpacingValue::LengthPx`만 검사한다. 일반 CSS 문자열 파싱은 추가하지 않았다. |
| 2 | 하한 `0px` 거부 | projection boundary와 Layout matrix의 gap 0 통과. |
| 3 | 내부 경계 `2.5px`, `5px`에서 잘못된 free-space 계산 | 독립 40-scene 기대값과 Layout/RuntimeSession matrix가 통과. |
| 4 | 상한 `7.5px`을 배타적으로 처리 | projection과 layout contract가 7.5를 허용하고 `end`/`center` 장면이 양쪽 시뮬레이터에서 실행됐다. |
| 5 | 상한 초과 `7.5001px` 허용 | Style 단위 검사, layout contract, RuntimeSession negative control에서 거부. |
| 6 | 음수 finite px gap 허용 | projection boundary와 CSS runtime rejection에서 거부. |
| 7 | NaN 또는 양·음 Infinity로 범위 검사를 통과 | projection boundary에서 모두 거부하며 layout helper도 finite 조건을 요구한다. |
| 8 | percentage 또는 non-length computed gap을 px처럼 취급 | projection/layout 검사와 RuntimeSession percentage negative control에서 거부. |
| 9 | `calc()` 선언이 이 bounded profile을 우회 | RuntimeSession의 `calc(5px)` negative control은 layout publication 전에 거부. |
| 10 | `column-gap`을 row-gap 상한과 함께 확장 | Style projection은 typed 0만 허용하고 layout contract도 exact zero만 허용. |
| 11 | non-zero column gap이 wrong error path로 허용 | Layout 및 RuntimeSession negative controls에서 거부, partial completed layout 없음. |
| 12 | 다른 Root HTML intrinsic profile의 percentage/fixed gap admission이 바뀜 | 새 범위 helper는 `RootHtmlAutoRowStretchV1`에만 선택 적용하고 나머지는 기존 generic validator를 호출한다. |
| 13 | track sizing과 content alignment에서 gutter를 두 번 더함 | 구현에 수동 간격 보정을 추가하지 않았고 Taffy trace/40-scene geometry가 독립 기대값과 일치. |
| 14 | 세 row 사이 gutter 수를 3으로 계산 | 입력에는 두 gutter만 있다. `40 - 30 - 2×gap` 경계 네 값 모두 expected JSON과 일치. |
| 15 | `normal|stretch`가 explicit row까지 늘림 | trace used rows는 `[10, 10+free/2, 10+free/2]`이며 explicit 10px row는 유지. |
| 16 | positional alignment가 auto-row bases를 stretch함 | start/end/center 계열은 `[10,10,10]` track base를 유지하고 item 위치만 정렬한다. |
| 17 | negative free space에서 `end`의 overflow를 clamp | 7.5px `end` 첫 item y=-5px가 Layout 및 RuntimeSession에서 확인됨. |
| 18 | negative free space에서 `center`를 start나 zero로 clamp | 7.5px `center` 첫 item y=-2.5px가 Layout 및 RuntimeSession에서 확인됨. |
| 19 | 분배 정렬의 negative-space fallback을 positional unsafe 값과 혼동 | `space-between/around/evenly`는 Chrome expected JSON의 safe-start 위치와 일치한다. |
| 20 | 실패 요청이 partial scene을 publish하거나 valid request 후 회복을 깨뜨림 | gap/shape rejection은 `completed=None`; same-session alignment rejection/recovery focused test가 통과. |

Rust test logs, 시뮬레이터 화면·runtime logs, source hashes와 restoration 결과는 [실행 근거](README.md)에 연결한다. Android는 API 37 ARM64 AVD이고 iOS는 iPhone 17 Pro/iOS 26.2 Simulator다. Screenshots는 visual smoke다. PixelCopy/Metal drawable readback, 실기기, hardware GPU, WPT 및 전체 C11.4/C11 검증은 이 검토에 포함하지 않았다.
