# Nested Grid text + replaced item 구현/runtime 적대 검토 20개

검토 기준은 현재 feature diff의 Rust adapter, combined layout routing, tests, FFI scene source, capture runner, Android/iOS harness와 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator result JSON이다. 계획 검토와 다른 20개 failure perspective를 사용했다. 각 행은 입력이나 통합 경계가 실패할 때 잘못 승인되거나 잘못 보고될 경로를 공격적으로 확인했다.

| # | 실패 관점 | 검토 결과 / 근거 |
|---:|---|---|
| 1 | 기존 허용 scene ID가 새 scene으로 잘못 라우팅되는가 | 새 ID만 `SCENES` allowlist에서 승인한다. FFI route는 기존 route보다 앞서 있으나 ID는 서로 겹치지 않고 `exposes_fixed_nested_grid_text_image_scenes`가 세 ID와 unknown negative를 확인한다. |
| 2 | 같은 이름 family의 미등록 scene이 부분 layout으로 빠지는가 | `nested-grid-text-image-` family의 비등록 root ID는 unsupported error로 fail closed한다. 다른 C11.4 family ID는 이 adapter에서 `None`으로 반환한다. |
| 3 | 유사 root의 namespace, tag 또는 parent 변형이 승인되는가 | runtime adapter는 HTML namespace, `div`, document root의 direct child와 정확한 fixture profile을 확인한다. |
| 4 | nested Grid 옆에 sibling이나 불필요한 node가 있어도 성공하는가 | root direct child는 nested Grid 하나, nested direct child는 정확히 text item·image item 두 개여야 한다. |
| 5 | 자식 순서가 뒤집혀도 잘못된 intrinsic source order로 계산되는가 | text는 index 0, image는 index 1이며 정확한 node ID와 parent owner를 검사한다. runtime trace source order 0/1을 capture runner가 대조한다. |
| 6 | nested Grid ID나 owner가 다른 Grid를 끌어오는가 | scene-derived nested ID와 `HostParent::Node(root)`를 모두 확인한다. 결과의 inner trace는 nested node, parent trace는 root node를 따로 검사한다. |
| 7 | text item 아래 추가 element, 빈 node 또는 다른 문자열이 Ahem text로 취급되는가 | text block에는 정확히 하나의 child text node와 `XX XXXXXXX`만 허용한다. descendant가 추가되거나 텍스트가 달라지면 거부한다. |
| 8 | system font fallback, weight/style, 줄바꿈 차이가 결과를 바꾸는가 | text의 computed typography subset을 확인하고 system font 없는 `TextLayoutEngine`에 고정 Ahem bytes를 등록한다. intrinsic min/max는 70/100px로 검증한다. |
| 9 | item의 margin/border/padding, explicit size 또는 position이 contribution을 오염시키는가 | text/image item box edge, size, max/min, flex basis, position을 검사한다. image aspect-ratio도 명시적으로 거부한다. |
| 10 | outer/nested Grid의 gap·row·column·template 변형이 hidden input이 되는가 | dimensions, column tracks, row track, gap, line names, repeat, areas, auto-flow, auto-track과 border/spacing을 검사한다. |
| 11 | grid placement의 row/column/span 변형을 올바른 source order처럼 받아들이는가 | nested/text/image placement를 각 scene의 exact line/span expectation과 비교한다. direct layout tests는 outer contribution span 1/2를 분리한다. |
| 12 | CSS `justify-items` 기본값과 앱 computed profile이 불일치하는가 | 최초 `start` 입력은 runtime profile 거부를 일으켜 규범 initial value `normal`로 고쳤다. Agent Browser 재측정에서 geometry가 그대로 유지됐고 addendum에 hash를 남겼다. |
| 13 | image URL만 맞고 다른 PNG bytes나 decode size가 쓰이는가 | 고정 URL을 검사하고 동일 document의 `ImageRequest`를 고정 PNG bytes로 decode한다. decoded natural size가 정확히 24×16px인지 확인한다. |
| 14 | image가 leaf가 아니거나 HTML image가 아닌 node가 들어오는가 | HTML namespace/tag/ID, nested Grid owner, leaf 조건, exact `src`를 확인한다. 직접 HTML `<img>`만 허용한다. |
| 15 | text와 image intrinsic source가 서로 다른 document/revision에서 섞이는가 | decoded completion으로 단일 `ImageResourceSnapshot`을 만들고, combined layout input과 output stamp에 해당 intrinsic revision을 보존한다. runtime test와 capture runner에서 revision 1을 확인한다. |
| 16 | revision overflow, decode 실패, 누락된 decoded metadata가 부분 성공으로 처리되는가 | `checked_next`, decode, snapshot conversion은 구체 `RuntimeLayoutFailure`로 반환하며 missing metadata/natural size는 unsupported error다. |
| 17 | 새 adapter가 기존 anonymous/direct image fallback에 동시 선택되어 입력을 중복 반영하는가 | 새 adapter 성공 시 후속 C11.4 adapter 준비와 layout dispatch를 모두 건너뛴다. runner는 outer item이 text/image intrinsic leaf input으로 나타나는 것도 거부한다. |
| 18 | definite flex control에서 zero minimum이 text/image natural size와 혼동되는가 | direct test와 runtime capture가 `Minimum=0`, span 2, outer tracks 58/58, nested used width 120px를 동시에 검사한다. 나머지 두 장면은 별도로 `MinContent=98px`를 검사한다. |
| 19 | 플랫폼 harness의 scene gate가 통과해도 frame/node 수가 틀린 실행을 성공 처리하는가 | Android와 iOS harness 모두 세 ID의 expected root/node frames와 box count 4를 고정한다. runner가 실제 trace/input/frame과 `drawPresented`를 추가로 확인한다. |
| 20 | host screenshot이나 draw marker가 surface pixel parity/제품 지원으로 과장되는가 | 화면은 host whole-app capture이며 PixelCopy/Metal drawable 표본은 실행하지 않았다. README에서 exact surface pixels, texel paint, hardware GPU, compositor-after, 일반 CSS admission 및 전체 C11.4를 명시적으로 제외한다. |

## 결론

이번 검토에서 새 변경을 막는 미해결 결함은 확인하지 않았다. #12는 browser/profile mismatch를 실제 focused test에서 발견해 입력과 코드 기대값을 수정한 닫힌 항목이다. 나머지 제한은 위 범위 선언처럼 별도 기능·실행 근거를 요구한다. 이 검토는 PR diff 검토를 대체하지 않으며 두 검토는 별도 파일로 유지한다.
