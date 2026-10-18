# C04.12 direct text paint 구현 실패 관점 검토 20개

계획의 Chrome 사전 검토와 분리해, 구현·runtime·native fixture를 실패 경계별로 다시 확인했다. 모든 항목은 코드와 연결된 실행 근거를 확인했다.

| # | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | 별도 parser나 synthetic DOM이 성공 경로에 끼어듦 | fixture 생성은 `ParsedRootHtml::parse`로 한 번 파싱하고 같은 RuntimeSession HostDocument를 사용한다. 통과. |
| 2 | foreign root나 잘못된 root handle이 HTML fixture로 수용됨 | root node kind, HTML namespace, `html` local name과 generation을 검사한다. 통과. |
| 3 | fixture ID 중복으로 다른 node를 선택함 | 문서 전체 ID map에서 중복을 즉시 실패 처리한다. 통과. |
| 4 | body/main/owner의 parent나 DOM 순서가 바뀜 | parent identity와 body/main element child order를 검사한다. 통과. |
| 5 | direct text의 값이 달라지거나 앞뒤 text가 추가됨 | direct child 수와 exact `XX` source를 검사한다. `changed-text-source` mutation test가 거부됨. |
| 6 | `X<span>Y</span>` 같은 nested/split text가 합쳐져 성공함 | direct child 수 검사가 nested inline 전 scene을 실패시킨다. `split-direct-text` mutation test 통과. |
| 7 | 빈 Text가 보이는 owner의 줄 상자로 취급됨 | empty direct child는 profile gate가 거부한다. `empty-direct-text` mutation test에서 scene 없음. |
| 8 | HTML block 사이 collapsible 공백이 추가 anonymous paragraph를 만듦 | layout 전용 입력은 HTML collapsible ASCII whitespace-only Text를 생략하고 runtime frame을 고정 비교한다. 통과. |
| 9 | CJK/Unicode 공백이나 다른 visible Text가 조용히 버려짐 | 허용된 두 Text 밖의 visible non-whitespace source는 거부한다. 통과. |
| 10 | SVG/비HTML namespace나 예상 밖 visible element가 출력됨 | visible preorder의 namespace, element allowlist, display를 검사한다. 통과. |
| 11 | `display:none` `HIDDEN` control이 layout 또는 paint에 들어감 | hidden node는 visible set에서 제외하고 source identity만 보존한다. mutation으로 노출하면 거부한다. |
| 12 | 다른 viewport나 device scale에서 fixture가 잘못 성공함 | profile 진입 시 viewport `320×240`, scale `1`을 exact 제한한다. 통과. |
| 13 | root/body/main box cascade가 sample과 달라도 고정 scene을 냄 | typed dimensions, display, overflow와 zero edges를 검사하고 frame을 `0.05px` 이내 비교한다. 통과. |
| 14 | owner width, auto height, padding/border, overflow가 다름 | computed owner style과 spacing/border/overflow axes를 각각 gate한다. visible overflow mutation 거부. |
| 15 | 다른 font-family나 font size를 fallback으로 그려 통과 | no-system-font registry에 고정 Ahem만 등록하고 font style을 검사한다. wrong-font mutation 거부. |
| 16 | text color가 틀리거나 wide-gamut 변환이 오라클을 통과 | computed sRGB RGBA를 고정 `#092170`과 비교한다. wrong-color mutation 거부. |
| 17 | layout text snapshot이 다른 Text node/revision에서 유래함 | owner node, source `NodeId`, input revision, paragraph와 available width를 대조한다. 통과. |
| 18 | UTF-16 source range와 glyph mapping이 어긋남 | 두 glyph의 `[0..1]`, `[1..2]` source range·advance·x를 검사한다. 통과. |
| 19 | raster가 다른 font registry/glyph id/source를 사용함 | raster와 layout font revision, glyph ID/source, alpha bitmap 수를 대조한다. 통과. |
| 20 | stale surface, CSS 좌표의 물리점 오투영 또는 clip 차이가 sample을 속임 | surface 비율로 pixel center를 계산하고 generation/renderer/크기를 확인한다. Android PixelCopy와 iOS Metal 표면에서 visible overflow 및 hidden clip을 각각 5/5 exact로 재실행했다. 통과. |

검토 중 발견한 1,101줄 단일 fixture 모듈을 tree/style 검증·text mapping 모듈로 나눠 각각 475줄 이하로 제한했다. 성공 전용 테스트만 있던 상태에 source/tree/style mutation 거부 사례를 추가했다. Swift array indexing compile 오류, hidden-owner pixel expected value, Clippy의 신규 `map_or`/block condition lint도 수정하고 영향받은 검사·Simulator 실행을 다시 확인했다. 잘못된 host 종류를 막는 FFI guard는 helper true/false 단위 테스트로 확인했다. live host를 만드는 FFI 단위 테스트는 V8 isolate 초기화 오류로 실행할 수 없어 해당 통합 호출은 미검증이다. 표준 workspace Clippy에서 남은 `too_many_arguments`는 `origin/main`에도 동일한 `compute_cascade_with_reuse` 서명에 걸리는 기존 lint다.
