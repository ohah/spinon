# Nested Grid positive implicit row 구현·runtime 실패 관점 검토 20개

기준 변경은 두 fixed positive-implicit-row scene, strict runtime image adapter, Android/iOS scene validator와 capture runner다. 계획 검토 표와 별도로 source, focused tests, 최종 simulator logs/results, artifact controls를 대조했다.

| # | 독립 실패 관점 | 확인 결과 | 근거 |
| --- | --- | --- | --- |
| 1 | 등록되지 않은 scene ID가 runtime script로 전달되는가 | 통과: FFI source는 정확히 두 scene만 반환한다 | `fixtures/c11_4/nested_grid_implicit_rows.rs`; `ffi-focused-test.log` |
| 2 | 기존 scene adapter가 새 prefix를 먼저 잡아 잘못 분기하는가 | 통과: prefix/dispatch가 신규 scene을 기존 general image adapter보다 앞서 선택한다 | `runtime_layout/fixtures/c11_4_image.rs`; runtime focused test |
| 3 | 다른 style profile에서 fixture 결과가 일반 지원처럼 노출되는가 | 통과: `RuntimeBlockFlexGridTrackSizingV1` 외 profile은 거부한다 | `runtime_layout/fixtures/c11_4_nested_grid_implicit_rows.rs` |
| 4 | HTML namespace나 root tag가 바뀐 입력을 허용하는가 | 통과: XHTML namespace, root `div`, document root ownership를 검사한다 | runtime fixture adapter |
| 5 | root에 nested Grid 외의 sibling이 추가되어도 layout을 진행하는가 | 통과: root child count는 정확히 1이어야 한다 | runtime fixture adapter; runtime focused test |
| 6 | nested Grid가 root 직속이 아니거나 ID/tag/owner가 어긋나는가 | 통과: exact nested ID, `div`, parent owner를 확인한다 | runtime fixture adapter |
| 7 | inner Grid child가 누락·추가·재정렬되어도 row source를 바꾸는가 | 통과: 세 child의 count와 source order를 고정한다 | runtime fixture adapter; app node frames |
| 8 | outer Grid column/row, dimensions, spacing, border 등 미지원 스타일이 통과하는가 | 통과: computed Grid style subset과 24px inline size, 4px row gap, zero edges를 검사한다 | `validate_grid_style`; runtime focused test |
| 9 | implicit row list의 순환 값이나 auto-repeat/line names/template areas가 달라져도 허용하는가 | 통과: `[MinContent, MaxContent]` list를 고정하고 repeat/areas/names 및 auto columns를 거부한다 | `validate_grid_style`; direct-layout test |
| 10 | nested item의 outer placement가 scene 별 span과 다르게 해석되는가 | 통과: scene별 row end line 3 또는 2, column auto placement를 검사한다 | runtime fixture adapter; app trace validator |
| 11 | explicit/first implicit block의 dimensions, row placement, leaf ownership이 달라지는가 | 통과: 각 fixed child의 identity, zero edges, dimensions, exact row/column placement와 leafness를 검사한다 | `validate_fixed_block`; runtime test |
| 12 | 두 번째 positive implicit row의 child가 PNG가 아닌 임의 element/resource가 되는가 | 통과: exact HTML `img`, ID, nested owner, allowlisted URL, leafness와 row 3/4 placement를 검사한다 | `validate_image`; direct/runtime fixture |
| 13 | image request/decode 실패나 natural-size 변조가 조용히 0-size layout이 되는가 | 통과: request/decode 오류를 반환하고 decoded dimensions가 24×16이 아니면 거부한다 | runtime fixture adapter; FFI source test |
| 14 | intrinsic snapshot revision이 빠지거나 초기 revision을 재사용하는가 | 통과: checked increment로 revision 1을 만들고 앱 validator도 revision 1을 요구한다 | runtime fixture adapter; Android/iOS validators |
| 15 | inner min-content/max-content rows가 authored pattern 순서와 다르게 정렬되는가 | 통과: direct-layout와 양 플랫폼 trace에서 `[8,8,16]` 및 gap slot을 확인한다 | layout/runtime tests; simulator `result.json` |
| 16 | nested contribution에서 내부 gap이 빠지거나 두 번 더해지는가 | 통과: contribution은 40px이며 두 4px gutter는 trace에서 분리된다 | direct/runtime trace assertions; platform validators |
| 17 | outer two-row span이 gap 포함 총량을 잘못 분배하는가 | 통과: span-two Rows `[18,18]`, nested contribution item/span/value를 검사한다 | layout/runtime tests; Android/iOS results |
| 18 | one-row case가 남는 두 번째 track을 지우거나 4px gap을 행 크기에 합치는가 | 통과: `[40,0]` track와 별도 4px gutter, root height 44px를 확인한다 | layout/runtime tests; Android/iOS results |
| 19 | frame order, trace truncation, async retry 또는 draw/present 오류를 성공으로 세는가 | 통과: 다섯 frame, trace chunks Android 8/iOS 16, retries 0, draw/present true를 검사한다 | simulator `result.json`, runtime logs |
| 20 | fixture symbol·scene/payload가 default product archive에 남는가, positive control이 빠지는가 | 통과: default archive는 absent, opt-in archive는 present이며 Android 설치 APK hash가 local과 일치한다 | `default-artifact-boundary.json`, `fixture-artifact-boundary.json`, `installed-apk-verification.json` |

## 리뷰 중 보정

Capture 단계에서 stale APK allowlist, gap gutter를 무시한 trace expectation, natural-size-only PNG에 요구한 image draw count 세 결함을 찾고 고쳤다. 각 수정은 영향받은 플랫폼 validator/runner와 양 플랫폼 최종 2/2 실행으로 재확인했다. 실패한 원본 시도는 삭제하지 않고 [실행 근거](./README.md)에 구분해 기록했다.

이 검토는 fixed fixture의 implementation/runtime 실패 경로만 본다. 일반 CSS admission, 임의 nested Grid, border/overflow pixel, physical Android, WPT와 전체 C11.4는 이 근거의 범위 밖이다.
