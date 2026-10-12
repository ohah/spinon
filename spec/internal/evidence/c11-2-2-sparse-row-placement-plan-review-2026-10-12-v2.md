# C11.2.2 sparse placement 계획 추가 검토 · 2026-10-12

**범위:** definite-column 항목이 sparse cursor의 column보다 앞에 놓일 때 다음 row로 이동하는 규칙을 보강한 계획과 Chrome 기준 v4를 다시 대조한다. 이 검토는 계획·oracle fixture만 다루며 C11.2.2 runtime 구현이나 Spinon 적합성 검증이 아니다.
**표준 근거:** [CSS Grid Layout Module Level 1 §8.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-placement-algo)
**현재 검사:** v4 단독 Node 검사 4/4 통과. v3 reference는 보존했다. 고정 WPT의 기존 Chrome 실행은 이번 검토에서 다시 실행하지 않았다.

| # | 실패 관점 | 대조 결과 |
|---:|---|---|
| 1 | definite-column item의 다음 row 선택이 표준 규칙과 어긋남 | item column이 기존 cursor column보다 작으면 row를 먼저 증가시키도록 계획과 기준을 구체화했다. |
| 2 | 같은 column에서도 이전 배치 뒤의 점유 cell을 건너뛰지 않음 | cursor 행에서 시작해 충돌 시 다음 row를 검색하는 규칙을 유지했다. |
| 3 | definite item을 auto item과 DOM 순서대로 바로 배치함 | definite item을 먼저 occupancy에 등록하는 단계가 계획에 있다. |
| 4 | 행 고정·열 자동 item이 definite-column 단계에 섞임 | definite-row/auto-column item을 앞선 별도 단계로 유지한다. |
| 5 | 두 축 자동 item이 sparse row-major 순서를 잃음 | 마지막 단계에서 같은 cursor를 이어 row-major 검색하도록 명시했다. |
| 6 | 이미 지난 빈 칸을 다시 채워 sparse가 dense처럼 동작 | row wrap 뒤 이전 row의 hole을 재방문하지 않으며 Chrome v4 scene이 이를 확인한다. |
| 7 | span item의 점유 폭을 한 칸으로 축소 | span 전체 cell이 겹침 검사에 들어가야 한다고 계획에 명시했다. |
| 8 | 충돌하는 line/span 조합에서 시작·끝을 잘못 해석 | CSS conflict handling을 좌표 정규화 전에 적용하는 순서가 고정돼 있다. |
| 9 | 음수 line을 암시 track 생성으로 잘못 확대 | 명시 line 범위 밖 선언은 거부하고 implicit negative row/column은 제외한다. |
| 10 | definite row에 공간이 없을 때 조용히 implicit column을 만듦 | 해당 전체 계산을 거부하고 implicit column을 만들지 않는다. |
| 11 | 암시 행을 만들 때 auto sizing을 무조건 지원한다고 오인 | 암시 행이 실제 필요하면 `grid-auto-rows:auto`는 거부한다. |
| 12 | 여러 track 또는 `%`·`fr`·`minmax()` 패턴이 기본값으로 수용됨 | 단일 비음수 px만 허용하며 나머지는 명시적으로 거부한다. |
| 13 | implicit 행이 explicit track 제한 밖까지 무한히 확장 | explicit+positive implicit track 합을 checked arithmetic으로 10,000 이하인지 검사한다. |
| 14 | 중첩 Grid가 부모 cursor/occupancy를 공유 | 각 컨테이너가 독립 배치 모드를 가진다고 계약했다. |
| 15 | `display:none` subtree가 occupancy를 차지 | 숨겨진 subtree는 기존 cascade/layout 제외 규칙을 따른다. |
| 16 | `order`가 source order와 달라졌는데 DOM 순서를 사용 | 이번 profile은 `order:0`만 허용해 source order와 order-modified order가 같음을 제한한다. |
| 17 | auto item의 intrinsic size를 fixed-size로 오인 | text/replaced/intrinsic sizing 및 auto min-size는 의존 단계로 남긴다. |
| 18 | 기대 거부 장면이 구현 지원 후보에 포함 | oracle-only 장면과 각 rejection reason을 분리하고 구현 결과를 아직 주장하지 않는다. |
| 19 | 실패 중 일부 frame이나 이전 revision을 새 결과로 publish | 전체 validation·placement·track 확장 성공 뒤에만 새 revision을 publish하고 후속 회복을 요구한다. |
| 20 | 계획 단계 결과가 runtime 구현 착수나 상위 Grid 완료로 읽힘 | C11.2.1 별도 통합 gate와 Rust·Chrome fixture gate를 유지하고 C11.2.2 코드는 아직 시작하지 않는다. |

## 수정 및 남은 경계

이전 계획은 definite-column item이 더 작은 column으로 되돌아오는 sparse cursor 조건을 명시하지 않았다. 새 `column-cursor-wrap` 장면을 추가해 기존 cursor보다 낮은 column의 item이 새 implicit row에 놓이는 Chrome 좌표와 track 수를 고정했다. 계획과 상태표는 v4 기준을 가리켜야 한다. Chrome oracle은 Spinon 동작을 증명하지 않으며 Android/iOS runtime, surface pixel, 전체 CSS 적합성은 이 검토의 범위가 아니다.
