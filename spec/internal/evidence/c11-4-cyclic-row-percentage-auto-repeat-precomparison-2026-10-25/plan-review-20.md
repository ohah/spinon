# C11.4 cyclic percentage auto-repeat row-axis 계획 검토 20개

검토 대상: [`plan/c11-4-cyclic-percentage-auto-repeat-row-axis.md`](../../../../../plan/c11-4-cyclic-percentage-auto-repeat-row-axis.md), 사전 비교 evidence와 연결된 fixture. 각 관점은 규범 계산, source 경계, 오류 경로 또는 검증 범위를 따로 확인한다.

초기 검토에서 발견된 exact-scene, control 경로, phase/count/basis/track provenance, mutation, worktree 및 roadmap 명확성 문제를 계획에 반영했다. 수정본 재검토에서 이전 차단점은 해소됐고, 내부 Taffy `Block`과 runtime/FFI `Rows` 축 표기도 source와 대조했다.

| # | 실패 관점 | 판정 | 근거 / 통제 |
|---:|---|---|---|
| 1 | plan PR에서 모든 근거 fixture·evidence를 원격에서 열 수 있는가? | PASS, PR 구성 시 파일 목록 재확인 | 계획은 fixture, 전체 evidence directory, manifest, lifecycle 및 conformance 연결을 같은 PR에 포함하도록 한다. |
| 2 | 규범 근거와 Chrome 결과를 분리했는가? | PASS | CSS Grid 2025 CRD §7.2.1/§7.2.3.2/§11 및 CSS Sizing §2를 독립 계산 기준으로 둔다. |
| 3 | max-content block size가 auto-repeat count 시점에서 indefinite임을 다루는가? | PASS | `max-content`는 indefinite이고 root `min-height:auto`도 definite minimum size가 아니므로 Grid §7.2.3.2에 따라 count는 1회다. |
| 4 | cyclic percentage가 intrinsic 계산 중 auto처럼 동작하는가? | PASS | 16px intrinsic height를 얻은 뒤 final percentage basis로 사용한다. |
| 5 | final percentage 해소 후 count를 잘못 재확장하지 않는가? | PASS | Taffy row rerun은 기존 count를 재사용하고 expected count는 1→1이다. |
| 6 | definite control의 count/track 산술이 독립적인가? | PASS | 40px / 50% = 20px, 두 row `[20,20]`이다. |
| 7 | row overflow와 root 크기를 혼동하지 않는가? | PASS | item은 16px, track은 8px이며 item은 track보다 8px 크고 root는 16px이다. |
| 8 | scene ID가 CSS fixture와 evidence에 일치하는가? | PASS | target/control scene ID 및 입력을 대조했다. |
| 9 | browser 비교 환경이 재현 가능한가? | PASS | Chrome 156, viewport, DPR, CSS.supports 및 font 상태를 기록한다. |
| 10 | hash가 관측·기대값·입력 파일을 묶는가? | PASS | fixture, README, raw JSON, screenshot, expectation JSON, lifecycle을 SHA-256 manifest로 관리한다. |
| 11 | opt-in target gate가 이외 typed style/placement를 거부하는가? | PASS | root/item LayoutStyle, GridContainerInput, tracks, flow, placement, geometry, children의 exact 값과 mutation controls를 고정했다. |
| 12 | definite control이 일반 경로와 fixture gate로 혼동되지 않는가? | PASS | 일반 definite-axis admission과 opt-in trace activation을 분리한다. |
| 13 | row의 semantic phase와 root `min-height` 초기값, percentage basis provenance가 분명한가? | PASS | fixture는 `min-height`를 선언하지 않으며 CSS 초기 `auto`를 typed `LayoutDimension::Auto`로 요구한다. Chrome CSSOM의 관측 `0px`를 authored fixed zero로 쓰지 않고 첫 basis `null`과 재실행 `16px`을 구분한다. |
| 14 | trace axis와 같은 pass provenance가 runtime 출력에 맞는가? | PASS | Taffy `AbstractAxis::Block` → `LayoutGridTrackSizingAxis::Rows`; phase/count/basis/track snapshot을 동일 pass에서 검사한다. `Auto` typed min-height에서는 `axis_min_size` fallback이 없어 기존 `percentageBasis`가 null→16px을 구분한다. |
| 15 | fail-closed mutation에 입력·owner·typed error가 있는가? | PASS | 11개 구체 mutation과 `InvalidGridTrackList { property: "grid-template-rows" }`를 표에 고정했다. |
| 16 | 새 일반 row algorithm 대신 검증된 rerun을 재사용하는가? | PASS | indefinite parent + percentage-row rerun을 재사용하고 row re-expansion은 추가하지 않는다. |
| 17 | 병렬 lane 경계에 파일 충돌이 없는가? | PASS | layout/Taffy, 별도 tests, FFI scene, Android/iOS 경로를 분리하고 shared dispatcher/report는 단독 소유한다. |
| 18 | worktree 사용을 불필요하게 강제하지 않는가? | PASS | 비중첩 ownership으로 통합 branch 병렬 편집, 독립 branch 필요 시에만 worktree를 쓴다. |
| 19 | PR에서 동작 화면을 볼 수 있는가? | PASS | Chrome 사전비교 화면을 plan PR에 연결하고, 구현 PR에는 가능한 simulator screenshot을 첨부한다. |
| 20 | 공식 상태와 Tailnet roadmap preview의 source·대상을 재현 가능하게 갱신하는가? | PASS | 구현 후 `spec/STATUS.md`와 `spec/public/roadmap.html`을 갱신하고 C11.4 미완료·72/213을 유지한다. preview root/route, runtime README route, evidence 내 `tailnet-sync-manifest.json`, 네 simulator screenshot 대상을 적었다. 동기화 전 server root를 재확인하고 삭제 없이 backup·file manifest·served HTTP 200/hash를 검증한다. |

계획 검토는 runtime 구현 검토 20개 및 PR diff 검토 30개와 별도다. Plan PR에 명시된 입력, fixture, screenshot, hash manifest가 실제 변경 목록에 들어 있는지 생성 시 재확인한다.
