# C11 기본 Root HTML Grid direct-text aspect-ratio 계획 검토 20개

이 파일은 제품 코드 변경 전 계획 자체를 검토한 원본 기록이다. 이후 기본 Root HTML 경로에서 Ahem webfont가 로드되지 않는 사실이 확인되어 입력·기대값의 일부 판정은 보정했다. 그 보정은 [추가 20개 검토](./c11-root-html-grid-text-aspect-ratio-plan-amendment-review-20.md)에 기록한다. 이 파일은 runtime 코드 검토나 PR 변경분 검토가 아니다.

| # | 실패 관점 | 판정과 반영 |
|---:|---|---|
| 1 | C11.4 fixture-only ratio 구현을 기본 앱 경로의 증거로 혼동 | 통과 — 두 profile을 명시적으로 분리하고 기본 parsed `RuntimeRootHtmlTextPaintV1`만 목표로 고정했다. |
| 2 | ratio allowlist가 다른 computed-style profile로 번짐 | 통과 — default profile의 allowlist/property vector와 profile regression을 별도 gate로 요구한다. |
| 3 | ratio가 Grid container 자체에 적용됨 | 통과 — 대상은 owner별 direct in-flow text item이고 Grid root ratio는 거부한다. |
| 4 | 한 Grid의 여러 ratio item이 Taffy track sizing에서 상호작용 | 통과 — owner당 한 ratio item 제한, 복수 ratio item은 owner 전체 거부다. |
| 5 | 서로 다른 Grid owner의 NodeId/style 상태가 공유됨 | 통과 — owner-scoped gate와 네 장면의 owner 분리, NodeId/revision 일치 조건을 적었다. |
| 6 | stale computed style 또는 layout revision을 최신 ratio로 오인 | 통과 — cascade snapshot, computed style, layout revision이 불일치하면 partial output 없이 실패한다. |
| 7 | CSSOM 문자열을 projection에서 임의 재파싱 | 통과 — cascade의 typed ratio와 computed serialization 대조를 요구하고 string reparse는 금지했다. |
| 8 | ratio 계산 방향을 width/height에서 뒤집음 | 통과 — CSS ratio를 width/height `2/1`로 고정하고 양 축 장면에서 60×30을 대조한다. |
| 9 | width-ratio 장면에서 auto row의 30px 기여를 놓침 | 통과 — 독립값을 `max(10px,30px)=30px`, 대조군은 10px로 명시했다. |
| 10 | height-control의 auto width를 60px로 가정 | 부분 보정 — 고정 Ahem을 직접 등록하는 Rust 단위 검사는 40px지만 기본 앱·Chrome fixture는 시스템 `monospace`를 쓰며 Chrome intrinsic width는 24.09375px이다. 플랫폼 간 intrinsic-width 비교는 판정에서 제외한다. |
| 11 | Ahem 미로드 또는 font fallback이 기댓값을 바꿈 | 최초 가정 실패 — 기본 Root HTML runtime은 `@font-face`를 지원하지 않고 고정 Ahem registry도 연결하지 않는다. 성공 fixture를 `monospace`로 바꾸고 OS별 intrinsic 폭 차이를 별도 기록했다. |
| 12 | track list/gap/root width가 바뀌어 ratio 효과와 혼합 | 통과 — width 70px, `[10px,60px]`, zero gap, single auto row를 고정한다. |
| 13 | Grid self-alignment가 stretch/ratio 계산을 가림 | 통과 — start alignment를 고정하고 computed value 및 frame을 함께 비교한다. |
| 14 | margins/padding/border/box sizing이 ratio box를 바꿈 | 통과 — root/item의 margin/padding/border는 0이다. scene 식별에는 sizing에 영향을 주지 않는 background-color만 쓴다. min/max는 기본값이다. |
| 15 | `auto 2/1`의 content-box 의미를 numeric ratio와 섞음 | 통과 — numeric `2/1`만 성공 입력이고 `auto <ratio>`는 제외·거부한다. |
| 16 | zero, negative 또는 overflow된 ratio를 전달 | 통과 — finite positive `2/1`만 받으며 degenerate ratio와 unsupported typed values는 fail closed다. |
| 17 | 중첩 Grid/replaced/anonymous text를 같은 direct text case로 취급 | 통과 — non-replaced direct HTML block text를 요구하고 다른 item topology는 제외했다. |
| 18 | default root route의 sibling block/flow 동작을 건드림 | 통과 — generic block, 다른 Grid mode 및 public CSS admission은 비목표다. |
| 19 | CSS Grid CRD와 CSS Sizing WD의 근거 등급을 동일하게 표현 | 통과 — Grid 2025 CRD는 published CRD, Sizing 4는 Working Draft로 문서에서 구분했다. |
| 20 | Chrome precomparison geometry를 raster/WPT/전체 C11 지원으로 확대 주장 | 통과 — geometry는 0 CSS px 판정이며 glyph pixels, WPT, device/GPU 한계와 전체 완료 판정을 제외했다. |

**당시 계획 판정:** 계획 수준에서 독립 기대값과 범위 경계를 고정한 뒤 구현에 진입했다. 10·11번의 글꼴 가정은 기본 앱 경로의 실행 전제가 아니었음이 드러나 보정했다. 후속 검토와 현재 구현 상태는 계획 amendment review와 runtime evidence에 연결한다.
