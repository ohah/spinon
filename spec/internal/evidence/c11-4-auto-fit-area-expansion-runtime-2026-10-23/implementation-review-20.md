# 구현 변경 독립 실패 관점 20개

**검토 범위:** 7 scene 구현 diff, host/capture fixture 경계와 실제 runtime. 계획의 사전 검토 표는 재사용하지 않았다. 초안 검토 뒤 full run에서 발견한 빈 item-rect map 누락을 수정했고, 수정된 host mapping을 정적 검사·단독 scene·전체 matrix에서 다시 확인했다.

| # | 별도 실패 관점 | 확인 내용과 판정 |
| ---: | --- | --- |
| 1 | 신규 scene ID가 서로 겹치거나 기존 ID를 덮는다. | JSON 29개 ID unique 검사와 Node 경계 검사 7/7. 통과. |
| 2 | fixture 전체 개수가 바뀌었는데 validator나 합산 scene 목록에 옛 수가 남는다. | 첫 검토 뒤 `--list-scenes`에서 `336 !== 329`를 재현했다. capture aggregate를 336으로 고치고 Node test에 336 guard를 추가했다. Node 경계 검사 7/7 및 336-ID 목록 생성 통과. |
| 3 | empty 3×3 area가 반복 track을 점유하는 item을 몰래 포함한다. | empty scene `items=[]`, layout case는 root 단독으로 구성했다. layout matrix 통과. |
| 4 | area dimensions가 template repeat보다 크지 않아 새 track을 만들지 않는다. | empty/item/span cases가 각각 3×3 area, column case 1×3, row case 3×1이다. Chrome 사전 비교와 layout matrix의 expected tracks가 일치한다. 통과. |
| 5 | area-added track을 repeat track으로 잘못 계산한다. | `grid-auto-columns:16px`, `grid-auto-rows:12px` expected 값과 final non-collapsed track을 독립 축으로 검사한다. 통과. |
| 6 | 반복 안에 item이 없는데 `auto-fit` track이 남는다. | empty와 added-cell-only cases에서 columns/rows 각각 collapse count를 2로 검사한다. Rust layout test 7 case 통과. |
| 7 | full-span item이 반복 track을 교차하지 않아 collapse된다. | 3×3 named area의 expected frame 64×60과 collapse count 0을 검사한다. layout test 통과. |
| 8 | columns/rows의 `auto-fit`·`auto-fill` 축 순서를 뒤집는다. | fit-fill과 fill-fit expected tracks 및 item frame을 Node test와 layout test에서 각각 대조한다. 통과. |
| 9 | added-column case에서 비어 있는 두 column repeat가 유지된다. | named area는 1×3, item은 세 번째 column에 두고 collapse count 2와 `[16]` used size를 확인한다. 통과. |
| 10 | added-row case에서 비어 있는 두 row repeat가 유지된다. | named area는 3×1, item은 세 번째 row에 두고 collapse count 2와 `[12]` used size를 확인한다. 통과. |
| 11 | 한 축만 area가 늘어도 다른 축의 repeat/collapse 상태가 섞인다. | column-only에서 rows `[20,0]`, row-only에서 columns `[20,0]`을 별도로 고정한다. 통과. |
| 12 | item의 auto stretch가 빠져 CSS area frame과 달라진다. | Rust layout child와 runtime fixture item의 width/height를 `auto`로 유지하고 frame을 비교한다. layout test 통과. |
| 13 | named area의 start/end lines가 생성되지 않거나 다른 셀에 놓인다. | layout regression은 `hero-start`/`hero-end` named lines를 사용하며 Chrome fixture는 같은 `gridArea: hero` 배치를 사용한다. 통과. |
| 14 | root 또는 scroll extent가 overflow를 숨긴다. | span `[64,60]`, row overflow `[52,60]`, column overflow `[64,52]`를 JSON과 Node test로 고정했다. 통과. |
| 15 | Android expected frame이 iOS와 다르다. | 두 host에서 41 scene 전체를 실행했고 node frame 교차 비교가 41/41 exact다. 통과. |
| 16 | full-span item이 root 안 표본을 덮는데 표본이 root 배경색으로 기대된다. | span scene root-center 표본이 Android PixelCopy와 iOS Metal drawable에서 모두 exact다. 전체 area 표본도 각 116/116 통과. |
| 17 | overflow 방향 표본이 item 내부 또는 root 밖 clear를 놓친다. | span·row·column overflow를 포함한 29 area scene의 Android/iOS sample ID·좌표·색이 각 116/116 exact다. |
| 18 | empty scene에 item sample을 기대하거나 sample 수가 어긋난다. | 첫 Android full run에서 scene 35의 missing item-rect map을 재현했다. 두 host에 명시적 빈 목록을 넣고 29개 area ID가 각 root/name/rect helper에 존재하는 Node guard를 추가했다. 단독 Android/iOS 재검사와 full run 41/41, area samples 각 116/116 통과. |
| 19 | 새 runtime fixture가 기본 FFI product에 섞인다. | product archive에서 scene marker와 C11.4 host symbol이 모두 없고 fixture-test positive control에는 모두 있다. Rust LLVM 22.1.2 `llvm-nm`으로 확인했다. |
| 20 | 고정 fixture 결과가 일반 CSS 지원 또는 C11.4 완료로 과장된다. | 제품 layout 계산 코드는 수정하지 않았다. 공식 `spec/STATUS.md`와 `77/213`은 유지하고 계획/evidence는 제한된 simulator slice로 기록한다. 통과. |

## Runtime 발견 후 재검토

| 관점 | 재검증과 판정 |
| --- | --- |
| empty area가 frame metadata는 있지만 item geometry가 없는 경우도 유효한 빈 rect로 전달된다. | Android/Swift helper에 `[]`/empty `int[][]` case를 추가하고, 실패 scene을 두 플랫폼에서 단독 실행했다. 통과. |
| host mapping guard가 모든 area fixture를 검사하며 새 ID의 omission을 잡는다. | Node 경계 검사가 29개 ID 각각 root-frame·item-name·item-rect helper 소속을 확인한다. 7/7 통과. |
| Android와 iOS의 빈 scene sample cardinality가 동일하다. | root interior와 after-root 2점이 양쪽 readback에서 exact로 확인됐다. 통과. |
| scene route 수와 실패 뒤 재실행 대상 집합이 변경되지 않는다. | 336 전체 route에서 manifest와 동일한 41개를 선택해 두 플랫폼 full run을 각각 통과했다. 통과. |
| surface sample 검증 실패가 layout summary 통과로 숨겨지지 않는다. | 최초 실패는 summary 전 config error로 중단됐고 성공 재실행은 runtime summary·trace·present·pixels를 모두 검증했다. 통과. |

최신-source layout/projection/FFI tests, Node 7/7, workspace 및 fixture-feature Clippy, Android/iOS app build, FFI archive negative/positive control과 양쪽 41-scene runtime은 evidence README와 원본 logs에 기록했다. 남은 관문은 final PR diff 30관점 재확인, PR 생성 뒤 현재 required check/merge state 조회, Agent Browser에서 PR 이미지 렌더링 확인 및 해당 browser session 정리다.
