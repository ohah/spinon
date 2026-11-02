# C11.1 explicit `fit-content(px)` 계획 PR 변경 검토 20개

계획 PR 후보의 실제 계획·fixture·expected·observation·image 변경을 다음 20개 adversarial 관점으로 검토했다. `row-min-content` probe와 고정 named-area 장면을 추가한 뒤의 최종 후보를 대상으로 한다.

| # | PR diff 관점 | 결과 |
| --- | --- | --- |
| 1 | 변경이 Root HTML 기본 profile을 정확히 가리키는지 | `RuntimeRootHtmlTextPaintV1` 및 `validate_root_html_grid_tracks()`를 명시했고 신규 profile/ABI는 제외했다. |
| 2 | 현재 gap과 기존 converter 사실이 코드에 맞는지 | computed `FitContent` 변환과 Taffy mapping은 이미 있고 Root HTML validator가 admission을 막는 위치임을 source에서 확인했다. |
| 3 | allowed value가 broader `length-percentage`로 새지 않는지 | 계획을 finite non-negative `LengthPx` variant로 제한했다. |
| 4 | row와 column을 모두 다루는지 | 기대 입력에 explicit row/column 장면과 property별 gate tests를 요구한다. |
| 5 | 독립 기대값이 Chrome 수치에서 역산되지 않는지 | 규범 공식과 Ahem glyph/space advance 계산으로 각 cap을 산출하고 Chrome을 cross-check로 분리했다. |
| 6 | auto minimum을 항상 min-content와 같다고 단정하는지 | W3C의 auto-min 예외를 확인하고, positive scenes를 비-scrollable plain Ahem text에 한정하며 overflow/aspect-ratio는 제외한다. |
| 7 | row 기대가 대상 grid 자체와 순환하지 않는지 | `width:20px;height:min-content` independent probe를 별도 요소로 두고 row scene과 비교한다. |
| 8 | fixed repeat를 auto-repeat으로 확대 해석하지 않는지 | fixed `repeat(2,...)` 한 장면만 넣고 auto-repeat을 명시적으로 제외했다. |
| 9 | area 장면이 area-created auto track을 혼합하지 않는지 | `"fixed fit"`과 authored tracks `[20px, fit-content(40px)]`의 dimensions가 정확히 일치한다. |
| 10 | implicit sizing이 explicit support로 잘못 승인되지 않는지 | plan은 `grid-auto-*`, area-created unsized track을 제외하고 기존 guards 유지 기준을 둔다. |
| 11 | zero와 invalid float 경계가 빠지지 않는지 | projection matrix에 0, positive, negative, NaN, ±infinity를 나눈다. |
| 12 | percentage와 calc가 우연히 포함되지 않는지 | 두 computed variants 모두 positive list 밖이며 각각 명시적 거부 대상이다. |
| 13 | fallback이 unsupported 값을 성공처럼 숨기지 않는지 | cascade rejection과 node/property-bound projection diagnostic을 구분해 수락한다. |
| 14 | Ahem 원본이 잘못 참조될 가능성 | fixture relative URL, source asset path, font SHA-256을 함께 기록했다. |
| 15 | capture 전에 font가 늦게 바뀌는 가능성 | `document.fonts.ready`, `document.fonts.check`, loaded status를 관찰에 기록했다. |
| 16 | browser가 Chrome인지 reference가 최신인지 | Chrome 156.0.8078.80과 Agent Browser CLI 0.31.1의 `chrome://version` 값을 저장했다. |
| 17 | viewport와 CSS/device pixel 좌표 혼동 | 1280×633 CSS px와 DPR 1을 observation에 고정했다. |
| 18 | 눈대중 색을 pixel proof로 오기재 | PNG의 세 RGBA sample을 실제 파일에서 읽고 screenshot size/hash를 함께 고정했다. |
| 19 | HTML scene과 expected/observation ID가 어긋나는지 | 여섯 scene, track values, item origins를 서로 대조했고 Chrome observation이 일치했다. |
| 20 | 계획-only evidence와 roadmap 기록이 완료로 오해되는지 | C11 umbrella plan은 최신 병합 PR #1086과 새 계획 후보를 구분해 적고, `spec/STATUS.md`/official counter는 바꾸지 않는다. |

미해결된 계획 PR diff 결함은 없다. 이 검토는 계획 PR 변경에만 해당하며 구현/runtime 20개와 구현 PR diff 20개를 대체하지 않는다.
