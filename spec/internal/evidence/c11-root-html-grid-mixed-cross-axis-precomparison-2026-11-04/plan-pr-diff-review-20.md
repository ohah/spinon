# C11.1 교차축 배치 계획 PR diff 적대 검토 20개

**대상:** 교차축 partial auto-placement 계획 PR의 실제 diff. 구현 코드는 아직 변경하지 않았다.

| # | PR diff 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 최근 #1147과 같은 동작을 새 기능으로 중복 기록 | #1147은 same-axis partial만 허용한다. 새 plan은 row/column locked partial 혼합이라는 거부 경계를 다룬다. PASS. |
| 2 | 기존 mode semantics를 조용히 넓힘 | 새 `RootHtmlMixedCrossAxisAutoPlacementV1` mode를 분리하고 기존 modes 보존을 요구한다. PASS. |
| 3 | 여러 Grid owner를 한 placement 집합으로 합침 | 분류 및 선택을 owner 단위로 고정했다. PASS. |
| 4 | row-locked items가 row-flow cursor phase를 건너뜀 | CSS Grid §8.5 phase와 Chrome scene을 각각 연결했다. PASS. |
| 5 | fully definite occupancy 순서가 DOM 선언순에 좌우 | `d-last` control이 fully definite 우선 배치를 확인한다. PASS. |
| 6 | mixed axis 뒤 fully auto items를 누락 | plan과 oracle에 `auto-last`를 포함했다. PASS. |
| 7 | display:none item이 교차축 유형 수를 만족 | 분류 전 제거와 별도 Chrome control이 있다. PASS. |
| 8 | visibility:hidden occupancy가 손실 | hidden row/column이 뒤 item을 밀어내는 frame을 고정했다. PASS. |
| 9 | span occupancy가 단일 cell로 줄어듦 | 두 축 span scene과 기대 영역 크기를 포함한다. PASS. |
| 10 | implicit extent overflow가 성공으로 게시 | Chrome의 row-four는 baseline으로만 두고 product atomic rejection을 요구한다. PASS. |
| 11 | numeric unsupported grammar가 guess/fallback 처리 | named lines/areas, dense/column, invalid endpoints는 fail-closed다. PASS. |
| 12 | order-modified ordering을 DOM order로 거짓 지원 | `order:0` 고정과 nonzero order exclusion을 명시했다. PASS. |
| 13 | item paint/HostDocument order를 배치순으로 재정렬 | DOM child vector와 paint/source order 보존을 명시했다. PASS. |
| 14 | browser observation 자체를 expected oracle로 재사용 | 독립 `expected.json`, browser `capture.json`, verifier를 분리했다. PASS. |
| 15 | fixture root/text/font가 profile 한계를 흐림 | source HTML은 zero edge/gap과 empty direct blocks를 사용한다. PASS. |
| 16 | visible frame sample을 surface pixel equality로 확대 | precomparison README는 browser frame 관측으로만 규정한다. PASS. |
| 17 | Agent Browser 실행이 남아 리소스 누수 | session list와 process 확인에서 세션/브라우저가 비어 있다. PASS. |
| 18 | expected/reference artifact가 바뀌어 검토 결과가 낡음 | README, expected, fixture, script, capture, screenshot의 SHA-256 manifest와 재실행 검증을 포함한다. PASS. |
| 19 | plan merge를 공식 feature completion으로 표현 | 계획 파일과 master plan은 후보일 뿐이라고 적고 C11 checks/count를 건드리지 않는다. PASS. |
| 20 | PR diff에 unrelated worktree changes가 섞임 | 단일 checkout에서 새 feature branch를 origin/main에 만들었고 staged file 목록을 제한한다. PASS. |

**결과:** 20개 모두 통과. 구현/runtime 검토와 구현 PR 검토는 이후 별도 수행한다.
