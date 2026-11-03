# C11.5 구현 PR diff 실패 관점 검토 30개

검토 대상은 GitHub PR [#1128](https://github.com/ohah/spinon/pull/1128)의 실제 diff다. 기준 `a0dd16e668204acd3d177eb301229e0a007b205f`에서 구현 commit `b2d59ee170155c339d3bc8777e58533e2a27ac86`까지의 56개 구현 파일을 검토했다. 이후 PR diff에는 검토 결과·상태를 기록하는 문서 변경이 추가되어 57개 파일이다. 이 후속 변경은 제품 소스를 수정하지 않으며, 아래 보충 대조에서 문서와 metadata를 함께 확인했다. 계획 검토 20개와 구현/runtime 검토 20개를 반복하지 않고, 공개 직전 코드·artifact·명세 연결에서 서로 다른 실패 경로를 확인했다.

| # | 실패 관점 | 검토 결과 |
|---|---|---|
| 1 | 지원이 다른 computed-style profile이나 일반 CSS admission으로 번지는가 | **통과** — item owner 집합은 `RuntimeRootHtmlTextPaintV1`에서만 계산하고, 일반 author CSS profile은 추가 허용 경로를 받지 않는다. 이 제한 slice는 전체 C11.5 내부 계약을 승격하지 않는다. |
| 2 | item이 자기 소유 Grid가 아닌 적격 container의 허용을 빌리는가 | **통과** — `grid_item_owners`에서 실제 owner ID가 적격 container 집합에 속하는 경우만 허용한다. 서로 다른 owner의 projection test가 한쪽 item만 선택한다. |
| 3 | nested, spanning, text, replaced 또는 non-empty child가 1-cell item으로 오인되는가 | **통과** — 기존 fixed-track eligibility와 direct empty `div`, 단일 cell 용량, auto placement 경계를 재사용한다. 조건 밖의 비초기 computed self value는 거부된다. |
| 4 | implicit/fractional/dense/column-flow 또는 named-area Grid가 허용되는가 | **통과** — C11.5는 기존 eligible container의 정확한 owner만 사용한다. dense, column flow, implicit track, repeat, area와 fraction은 해당 owner admission에서 제외된다. |
| 5 | projection이 서로 다른 document/style revision의 self 값을 섞는가 | **통과** — 기존 `ComputedElementStyle` snapshot에서 node와 두 computed longhand를 읽고, runtime은 document/revision identity를 검사한다. |
| 6 | `align-self`와 `justify-self`의 block/inline 축이 뒤바뀌는가 | **통과** — 두 속성을 별도 typed layout field로 투영하고 비대칭 x/y expected frame 17개와 Chrome 비교가 일치한다. |
| 7 | `left`/`right`가 block 축이나 RTL·vertical writing mode로 일반화되는가 | **통과** — physical keyword는 `justify-self`만 받고 LTR `horizontal-tb` eligibility 안에서 start/end로 바꾼다. |
| 8 | CSS-valid keyword 집합이 실수로 과대 허용되거나 선언 문자열이 직접 mapping되는가 | **통과** — allowlist가 computed value에서 동작하며 `normal`, `stretch`, baseline, `safe start` 등은 계속 거부한다. |
| 9 | `safe`/`unsafe`가 Taffy에서 버려져 overflow 위치가 달라지는가 | **통과** — safety를 typed `AlignmentSafety`로 전달한다. safe center/end는 cell 안쪽, unsafe center/end는 음수 origin을 유지하는 고정 기대값과 일치한다. |
| 10 | computed `auto`가 parent의 두 축 값을 잘못 공유하는가 | **통과** — `align-self:auto`와 `justify-self:auto`는 대응하는 `align-items`와 `justify-items`에 맡긴다. 서로 다른 parent defaults와 child overrides가 비교에 포함됐다. |
| 11 | `place-self` shorthand의 축 순서 또는 단일 값 복제가 잘못되는가 | **통과** — authored shorthand parser를 추가하지 않고 Stylo의 computed longhand 두 값을 사용한다. `place-self` one-value와 two-value 비대칭 scene이 독립 frame과 맞는다. |
| 12 | 정렬이 fixed item width/height나 grid track 크기를 stretch로 바꾸는가 | **통과** — profile은 명시 finite px 크기와 zero edge만 받는다. Rust matrix는 16×10 및 overflow 50×40 item의 크기와 위치를 각각 확인한다. |
| 13 | 기본 `justify-self:auto`가 기존 replaced-item start fallback을 깨는가 | **통과** — Taffy 변환은 explicit value를 우선하고, auto replaced item만 이전 `start`를 유지한다. 전용 unit test가 네 경우를 검사한다. |
| 14 | unknown 또는 unsupported computed value가 조용히 기본값으로 바뀌는가 | **통과** — parse/projection 오류에 node·property·value를 보존하고 layout/scene publication을 중단한다. |
| 15 | 문법상 invalid CSS를 유효한 미지원 값처럼 거부해 cascade fallback을 깨뜨리는가 | **통과** — CSS-invalid `align-self:left|right|top|bottom`, `justify-self:top|bottom`만 좁은 Root HTML diagnostic path에서 무시하고 computed `auto` fallback controls 2개로 parent 결과를 확인한다. |
| 16 | invalid-diagnostic recovery가 다른 property나 value의 diagnostic을 숨기는가 | **통과** — property와 단일 invalid keyword를 고정한다. valid-but-unsupported `safe start`, 다른 alignment property 및 기타 diagnostic은 예외 목록에 들어가지 않는다. |
| 17 | comments, 대소문자, `!important` 처리가 diagnostic 분류를 우회하는가 | **통과** — CSS comment 제거, ASCII 대소문자 무시와 trailing importance를 다루고 정확히 하나의 invalid value token만 허용한다. 테스트는 CSS-invalid 후보와 주변 양성/음성 입력을 분리한다. |
| 18 | 같은 invalid 진단이 registry의 첫 오류 검사에는 걸리고 cascade 목록에는 남는가 | **통과** — author diagnostic 선별과 incremental cascade diagnostic 수집 양쪽에서 같은 predicate를 사용한다. 둘 다 `RuntimeRootHtmlTextPaintV1`에서만 적용된다. |
| 19 | 기존 Grid container alignment 허용이 item 허용과 혼동되는가 | **통과** — `is_grid_container`와 `is_grid_item` 분기를 분리한다. C11.1 container 정렬 regression과 item owner별 검사가 각각 남아 있다. |
| 20 | 실패 scene에서 일부 형제 frame 또는 `completed` 결과가 노출되는가 | **통과** — runtime rejection matrix는 exact node/property와 `completed=None`을 확인하며, sample fixture에 성공 scene만 일부 공개하지 않는다. |
| 21 | 기존 C11.1 rejection test가 새 지원을 모순되게 reject하거나 유효 미지원 값을 허용하는가 | **통과** — 과거 `end`/`start` reject assertion을 `baseline`/`safe start`로 바꾸어 새 positive behavior와 미지원 negative control을 각각 보존했다. focused C11.1 회귀는 7/7이다. |
| 22 | Android build flag 하나가 빠져 default/opt-in 조합이 엇갈리는가 | **통과** — Gradle BuildConfig, fixture source set과 C04 GPU/fixture flags를 맞췄다. default 0 조합 및 opt-in positive 1 조합을 각각 빌드했다. |
| 23 | Android default DEX에 capture adapter class name이나 fixture sample payload가 남는가 | **수정 후 통과** — runtime reflection name이 처음 발견되어 call을 compile-time BuildConfig branch로 옮겼다. 최종 default DEX에는 adapter name/scene IDs가 없고 opt-in DEX에는 있다. |
| 24 | PixelCopy가 다른 surface, viewport 또는 stale generation에서 읽히는가 | **통과** — draw status/presented, `980×735` viewport, drawable identity, source-current callback과 generation을 확인한 뒤 capture한다. |
| 25 | 표본 좌표가 픽셀 scale 또는 경계에서 어긋나지만 color만 우연히 일치하는가 | **통과** — CSS viewport 기준 center sample 좌표를 계산하고 coordinate/color를 각각 검사한다. cell edge, safe/unsafe overflow, viewport clip inside/outside를 포함한 7개 표본이 7/7 exact다. |
| 26 | iOS 인수 선택이 복수 fixture route를 허용하거나 C11.5 route를 잘못 선택하는가 | **통과** — 전용 prefix, 고정 run ID, 단일 root-html run 제한과 conflicting `--spinon-*` 인수 검사를 유지한다. default build에서는 parser 구현 자체가 opt-in flag로 제외된다. |
| 27 | iOS Metal 표본 좌표가 화면 viewport와 drawable 크기의 차이를 무시하는가 | **통과** — draw report의 C11.5 viewport가 정확히 `980×735`인지 확인하고 `960×720` drawable과 일치 배율을 검증한다. 표본은 compositor 제출 전 readback이다. |
| 28 | iOS default executable에 candidate 인수·scene ID가 남거나 opt-in positive가 빠지는가 | **수정 후 통과** — 사용되지 않는 argument exception을 제거했다. 최종 default executable에서 인수와 IDs가 없고 opt-in positive에는 둘 다 포함된다. |
| 29 | 실행 PNG, 직접 readback, SwiftShader와 실기기 evidence가 한 종류로 합쳐지는가 | **통과** — Android PixelCopy, iOS pre-compositor Metal drawable, OS 합성 screenshot을 metadata와 README에서 분리한다. 두 앱의 draw/present는 확인했지만 실기기나 Android hardware GPU로 확대하지 않는다. PR 화면도 Agent Browser에서 실제 이미지 크기 1080×2400과 1206×2622로 로드됨을 확인한 뒤 브라우저를 닫았다. |
| 30 | package 실패·체크 부재·상태 숫자가 성공 또는 C11 완료로 과장되는가 | **제한 기록** — broad package 결과는 layout 423 pass/1 fail, style-to-layout 372/1, style 283/1, runtime 261 pass/2 fail/3 ignored다. PR check rollup은 비어 있다. 따라서 CI 통과나 C11 완료를 주장하지 않고 공식 수 `72/213`을 유지한다. |

## 최종 diff 대조

- 30개 검토의 구현 diff 기준은 base `a0dd16e668204acd3d177eb301229e0a007b205f`, 구현 commit `b2d59ee170155c339d3bc8777e58533e2a27ac86`, 56 files다. 최종 PR diff는 검토·상태 후속 문서를 포함해 57 files다.
- 후속 변경은 review artifact, 상태 문구와 metadata만 추가·수정했다. 제품 소스가 바뀌지 않았음을 확인하고, 최종 PR 상태·문서의 test count·review 상태와 diff whitespace를 서로 대조했다. 구현/runtime 20개와 계획 검토 20개는 별도 문서로 유지한다.
- 최종 상태: PR #1128은 2026-11-03T17:52:20Z Rebase and merge로 병합됐고 merge SHA는 `0f704151cfd805ad2fef2e12535739f0d3a109dd`다. PR check rollup은 비어 있다. 병합은 제한 후보의 통합이며 C11.5 전체, C11 checkbox와 공식 `72/213`을 승격하지 않는다.
