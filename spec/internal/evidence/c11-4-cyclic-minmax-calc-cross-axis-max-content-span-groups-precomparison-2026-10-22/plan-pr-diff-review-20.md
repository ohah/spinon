# C11.4 max-content mixed cross-axis span-groups 계획 PR 변경 적대 검토 20개

**범위:** 계획, 상위 계획, 상태 대장, 공개 roadmap, 고정 fixture와 독립/Chrome 사전 비교 evidence. Runtime 구현 코드는 없다. 서로 다른 실패 방향 20개를 검토하고 수정사항은 같은 변경에 반영했다.

| # | 실패 경로·불변 조건 | 검토 결과 |
|---:|---|---|
| 1 | 변경이 계획 범위를 넘어 runtime 제품 코드를 건드리는가? | 변경 파일 범위에는 runtime Rust/C/Swift/Kotlin 구현이 없고, 계획·고정 fixture·문서·관측 evidence만 있다. 통과. |
| 2 | 사전 oracle과 tests fixture가 서로 다른 CSS 입력을 갖는가? | 두 HTML의 diff는 Ahem font URL의 상대 경로만 다르며 나머지 markup/style/script는 같다. 통과. |
| 3 | Ahem 로드 실패가 track 값으로 오인되는가? | Agent Browser의 `document.fonts.check('10px Ahem')`가 true이고 10px/10px 및 font hash를 README/manifest에 고정했다. 통과. |
| 4 | Chrome 값을 독립 기대값으로 역산하는가? | spec-derived JSON은 min/max-content contributions와 §11.5.1 planned-increase 및 §11.6 maximize 산술만 사용한다. Chrome JSON은 별도 관측이다. 통과. |
| 5 | Intrinsic cyclic pass와 resolved layout pass가 합쳐지는가? | 110px max-content width 결정 전/후를 JSON의 두 stage로 분리하고 final pass의 13px minimum을 적었다. 통과. |
| 6 | Max-content root width에 gutter와 border가 중복 산입되는가? | 세 column tracks 합 106px, 두 2px gutters와 합쳐 110px content, 4px border 후 114px outer box로 표를 대조했다. 통과. |
| 7 | 80px definite control에서 column percentage basis가 border box를 쓰는가? | width basis 80px content box에서 minimum 10px을 계산하고 4px border는 root outer box에만 더했다. 통과. |
| 8 | 80px control의 equal tracks가 growth limit을 넘는가? | B/A/C contributions 이후 limits [205/6,259/6,86/3]와 46px maximize remainder를 나눠 76/3씩 얻고 모두 limit 아래임을 확인했다. 통과. |
| 9 | Rows의 percentage basis를 80px width로 잘못 잡는가? | 80px control의 wrapped content height 20px를 row basis로 삼아 4px resolved row minimum을 계산했다. 통과. |
| 10 | Columns와 Rows 사이의 text wrap 측정 순서가 바뀌는가? | definite control A/C line widths [40,10]/[80,20]을 final columns 뒤의 20px row contribution에 연결했다. 통과. |
| 11 | Three-item span order가 source-order를 따라 달라지는가? | 2/2 spans의 per-track planned maxima 뒤 3-span C를 계산하고 forward/reverse item-keyed equality를 요구한다. 통과. |
| 12 | Reverse fixture가 placement나 item identity까지 변경하는가? | 네 scene은 order arrays만 바꾸며 key/text/placement/style 정의를 공유한다. 통과. |
| 13 | CSSOM quantization이 rational expected 값과 충돌하는가? | exact fractions를 기대값으로 유지하고 Chrome track output에만 1/64 CSS px tolerance를 적용했다. 통과. |
| 14 | Chrome의 auto/pixel-min diagnostic variant가 규범 근거처럼 보이는가? | Chromium diagnostic JSON은 normative derivation 및 baseline observations와 별도 파일이며 README에서 역할을 한정했다. 통과. |
| 15 | Evidence README screenshot이 앱 runtime 결과로 읽히는가? | 이미지 설명과 판정 경계는 Chrome 156 browser oracle로 한정하고 Spinon/Android/iOS 검증을 부인하지 않고 명시적으로 미실행으로 기록했다. 통과. |
| 16 | public roadmap이 계획을 runtime 완료로 표시하는가? | roadmap 문구는 계획·runtime 미시작을 말하고 checkbox 및 공식 77/213을 유지한다. 통과. |
| 17 | STATUS/parent plan의 신규 항목이 C11.4 완료 checkbox를 바꾸는가? | 변경은 진행 메모와 후속 계획이며 C11.4와 공식 완료 집계 행은 체크되지 않았다. 통과. |
| 18 | 상대 경로가 plan/evidence/public route마다 다른 root에서 깨지는가? | Agent Browser에서 plan 4-row matrix, precomparison screenshot의 naturalWidth 1280, roadmap links 6/6 HTTP 200을 확인했다. 첫 pass에서 roadmap의 raw PNG path가 404여서 README route로 바꿨다. Raw HTML/JSON/source links는 site output에 없는 경로를 제거하고 merge 후 영구적으로 유효할 GitHub main blob URLs로 바꿨다. 통과. |
| 19 | 고정 fixture 밖 입력으로 확대 해석할 여지가 있는가? | plan은 정확히 네 scene, 세 direct text item, Ahem/font hash와 고정 width/placement로 제한하고 arbitrary CSS admission/WPT를 제외했다. 통과. |
| 20 | 계획 PR 병합 후 미실행 검증을 완료로 보고할 위험이 있는가? | 문서에서 runtime 미시작, simulator·artifact gate, 실기기/hardware/compositor 경계를 구분하고 C11.4 전체 미완료를 유지한다. 통과. |

**판정:** 변경의 20개 실패 경로를 확인했다. 문서 route 항목은 selected RSPress build와 Agent Browser의 최종 링크 확인 이후 해당 근거를 이 표에 반영한다. 이 표는 계획의 20개 관점 검토와 기능 구현 후 runtime 검토를 대체하지 않는다.
