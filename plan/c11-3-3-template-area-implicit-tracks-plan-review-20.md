# C11.3.3 area-generated implicit tracks · plan review

**결과:** PASS 20/20. 계획 단계의 서로 다른 실패 관점을 점검했으며, 구현 검증을 대신하지 않는다.

**검토 기준:** CSS Grid 2025 CRD §7.3/§7.3.2/§8.5, pinned Taffy 0.15.0의 area expansion 경로, C11.3.3 현재 style/layout projection, Agent Browser HeadlessChrome 156의 고정 관측 JSON 및 screenshot.

| # | 독립 실패 관점 | 계획의 확인 항목 | 결과 |
|---:|---|---|---|
| 1 | area row가 `grid-template-rows`에 없다는 이유로 무효 처리한다 | §7.3의 area row 생성과 `short-row` Chrome 결과를 고정한다 | PASS |
| 2 | area column이 `grid-template-columns`에 없다는 이유로 무효 처리한다 | §7.3의 cell 기반 column 생성과 `short-col`을 고정한다 | PASS |
| 3 | 두 track list가 모두 없을 때 area matrix 크기만큼 track이 생기지 않는다 | `none` 장면에서 양축 track 수·크기를 검사한다 | PASS |
| 4 | missing area tracks를 `grid-auto-*` 대신 항상 `auto`로 만든다 | fixed `7px`/`9px`의 used track 비교를 둔다 | PASS |
| 5 | `grid-auto-rows`가 생성 column 크기에 사용되거나 반대가 된다 | row/column 독립 대조 장면 및 track 목록을 둔다 | PASS |
| 6 | area가 만든 `auto` track이 child intrinsic contribution을 무시한다 | `auto` 장면의 `[10,20]` rows와 `[20,30]` columns를 고정한다 | PASS |
| 7 | `align-content`/`justify-content` stretch가 자동 기여값과 혼동된다 | 고정된 `start` 정렬로 free-space stretch를 배제한다 | PASS |
| 8 | authored row prefix가 area expansion 중 사라지거나 덮인다 | `short-row`의 `12px` prefix와 생성 `7px` track을 각각 검사한다 | PASS |
| 9 | authored column prefix가 area expansion 중 사라지거나 덮인다 | `short-col`의 `13px` prefix와 생성 `9px` track을 각각 검사한다 | PASS |
| 10 | area matrix가 더 작을 때 authored trailing tracks가 잘린다 | `long`에서 세 authored rows/columns 전부 보존한다 | PASS |
| 11 | 기존 `area-track-count-mismatch`가 계속 오류이거나 잘못된 fallback을 선택한다 | Chrome의 `40px 120px 0px` 결과를 성공 회귀로 승격한다 | PASS |
| 12 | area item이 생성된 마지막 line에 놓이지 않거나 root-relative 좌표가 틀린다 | 이름 배치 item frame과 CSSOM used tracks를 따로 비교한다 | PASS |
| 13 | area의 named line alias가 Taffy에 전달되며 기존 numeric placement와 충돌한다 | validated area metadata만 전달하고 기존 alias/placement regression을 재실행한다 | PASS |
| 14 | malformed area가 layout 단계에서 재파싱되어 cascade fallback을 바꾼다 | Stylo typed computed value만 사용하고 잘못된 CSS fallback을 회귀한다 | PASS |
| 15 | C11.4의 auto-repeat+area 경로가 새 no-repeat 경로 때문에 바뀐다 | 기존 TrackSizing+auto-repeat 분기를 그대로 보존하고 행렬을 재실행한다 | PASS |
| 16 | named area가 없는 일반 `TemplateV1` dense placement가 우연히 area profile이 된다 | 전달 조건을 area metadata가 있는 `TemplateV1`로 제한한다 | PASS |
| 17 | `grid-auto-*` track-list/percentage/calc grammar가 의도치 않게 허용된다 | 기존 `auto`/single fixed-pixel subset만 허용하도록 경계를 명시한다 | PASS |
| 18 | 과대 area dimensions나 metadata가 Taffy 좌표/메모리 한도를 넘는다 | `GRID_MAX_TRACKS`, area metadata budget, 실패 원자성 검증을 계획한다 | PASS |
| 19 | 새 장면 집계가 기존 26-scene 결과·fallback·rejection을 잘못 덮는다 | base matrix 31 scene·90 node·25 success/5 fallback/1 rejection을 재계산하고 기존 근거를 분리한다 | PASS |
| 20 | 시뮬레이터 artifact나 screenshot이 다른 source revision을 증명한다 | 최종 source revision/hash와 Android PixelCopy·iOS pre-compositor Metal 경계를 각각 기록한다 | PASS |

## 남은 구현 검증

이 계획 검토는 adapter 구현, Android/iOS 실행, artifact inclusion, 실제 failing-control, PR merge 또는 C11.3.3 완료를 입증하지 않는다. 그 근거는 별도 구현/runtime review, 두 simulator의 source-bound 실행 기록, artifact 비교와 별도 PR diff review에서 수집한다.
