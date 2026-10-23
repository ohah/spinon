# C11.4 zero-percent 계획 PR 변경 검토 20개

이 검토는 계획 PR의 실제 파일 집합을 대상으로 한다. 계획 내용 자체의 [실패 관점 20개](../../../../plan/c11-4-auto-repeat-zero-percent-minimum-plan-review-20.md), 그리고 이후 구현/runtime 검토와 각각 분리했다. 공식 상태 대장·공개 roadmap 동기화 및 Tailnet preview 반영 결과도 현재 diff에 포함한다.

| # | PR 변경 적대 관점 | 결과 |
|---:|---|---|
| 1 | 계획 PR에서 제품 Rust/C++/Swift/Kotlin 실행 코드가 함께 바뀌어 구현 전 단계 경계를 흐린다. | **통과.** 코드 변경 없이 계획·fixture·관찰·검증기·로드맵 문서만 포함한다. |
| 2 | 공식 C11.4 상위 checkbox 또는 완료 집계가 계획 단계에서 증가한다. | **통과.** `STATUS.md` 기준일과 계획 전용 C11.4 기록을 추가했지만 상위 checkbox는 미완료, 공개 집계는 `72/213`이다. |
| 3 | C13 상태 변경을 다시 열어 사용자가 본 “진행 중” 오해를 만든다. | **통과.** C13 문구는 미완료·비활성 그대로 두며 새 active plan은 C11.4로 명시한다. |
| 4 | 새 계획 설명을 이미 구현된 기능이나 simulator 결과처럼 쓴다. | **통과.** 계획·사전 oracle 단계라고 적고 runtime, Android/iOS simulator 및 product support를 미검증으로 분리한다. |
| 5 | 계획의 현재 코드 설명이 실제 preflight와 다르다. | **통과.** `auto_repeat_track_has_fixed_minimum`은 `Percentage(0)`에 `> 0.0`을 요구하고 Taffy count는 raw per-repeat size를 나눈다. |
| 6 | 규범 링크가 auto-repeat count floor의 다른 절 또는 다른 draft를 가리킨다. | **통과.** CSS Grid 2025 CRD §7.2.3.2의 repeat syntax fragment에 연결하고, 1px 제안과 Chrome 관찰을 구분한다. |
| 7 | 각 repeat track의 count contribution과 실제 used size를 혼합한다. | **통과.** 계획·roadmap 모두 1px를 count-only로 쓰고 actual minimum/auto-fit sizing은 보존한다고 명시한다. |
| 8 | 사전 scene ID가 계획 표·HTML fixture·Chrome observation·Agent Browser rerun 사이에 빠지거나 추가된다. | **통과.** 검증기는 양쪽 관찰에서 독립된 13개 ID 집합과 중복 여부를 확인한다. |
| 9 | 120px/gap 0 또는 gap 4의 repeat count가 잘못된 산술에서 나온다. | **통과.** validator가 exact count를 검사하고 사전 계산은 120/24다. |
| 10 | subpixel explicit prefix/suffix를 repeated track으로 취급한다. | **통과.** fixture와 독립 histogram은 0.5px explicit 두 track과 23 repeated tracks를 분리한다. |
| 11 | gutter가 first repetition과 후속 repetition 사이에서 한 칸씩 틀린다. | **통과.** 24 repeats/23 gaps와 neighbor case의 25 total tracks/24 gaps를 고정한다. |
| 12 | auto-fit의 count를 placement 뒤 줄이거나 collapse 후 1px floor를 누출한다. | **통과.** empty 120-track zero collapse, single item, span-2와 row controls를 별도 기대값으로 둔다. |
| 13 | percentage basis가 rows/columns 또는 border-box 경계에서 잘못 선택된다. | **보완 완료.** 모든 scene의 120px basis와 border-box scene의 116px content basis를 expected JSON 및 검증기에 추가했다. |
| 14 | border-box 산술에서 border width가 빠진다. | **보완 완료.** fixture 공통 border 2px와 padding 8px를 plan 표, README 및 독립 기대값에 반영해 `136−16−4=116`을 명시했다. |
| 15 | item 크기/부재 검사가 없는 track 관찰을 완전한 frame 확인처럼 설명한다. | **보완 완료.** 기대값과 validator가 item 부재 및 세 item의 width/height를 함께 확인한다. |
| 16 | Chrome observation이 다른 fixture source 또는 browser environment의 결과여도 validation이 통과한다. | **통과.** fixture SHA-256, Agent Browser fixture path, Chrome 156 user agent, viewport, DPR를 양 observation 사이에서 확인한다. |
| 17 | JSON 표현식, 중복/추가 ID, histogram count 오류가 검증기를 통과한다. | **보완 완료.** 유효 JSON, unique/exact ID 집합, total histogram count, uniform track histogram을 검사한다. |
| 18 | child plan과 parent plan·공개 roadmap 경로가 끊기거나 다른 slug로 연결된다. | **통과.** 선택 Rspress build에서 parent plan, standalone plan, evidence README/review, raw·generated screenshot 및 공식 상태 route를 만들고 27개 preview route를 확인했다. |
| 19 | PR diff 검토가 계획 20개 또는 이후 구현 검토와 같은 관점으로 재사용된다. | **통과.** 이 표는 변경 파일·표시·oracle integrity를 대상으로 하며 계획/구현 리뷰 표와 따로 기록한다. |
| 20 | 선택 build나 앱 tests를 실행하지 않고 pass라고 쓴다. | **통과.** 사전 비교 validator는 Chrome 156 및 Agent Browser pass02 각각 13/13, Rspress 2.0.22 선택 build는 성공, Tailnet 27/27 route는 HTTP 200·SHA 일치를 확인했다. 앱/runtime test와 Simulator build는 하지 않았다고 분명히 기록한다. |

발견 후 수정한 계획·검증 결함은 네 가지다: 모든 scene의 basis 검증, border-box border 누락, item presence/size 비교, observation ID·histogram 무결성 검사. 마지막 validator 결과는 Chrome 156 13/13, Agent Browser pass02 13/13, fail 0이다. 공식 대장에는 10월 24일 계획 단계만 추가했고 checkbox와 `72/213`은 보존했다. Tailnet preview는 교체 4개를 백업하고 27개 route 해시를 검증했다. 계획 PR diff에는 해결되지 않은 blocker가 없다.
