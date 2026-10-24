# C11.4 fixed-repeat line-name merge 계획 적대 검토 20개 · 2026-10-24

대상은 [`c11-4-fixed-repeat-line-name-merge.md`](./c11-4-fixed-repeat-line-name-merge.md)와 계획-only Chrome 사전 비교다. 각 항목은 서로 다른 실패 경로·계약 경계를 검토했다. 제품 구현/runtime review나 PR diff review로 재사용하지 않는다.

| # | 실패 관점 | 공격과 확인 결과 | 계획 반영·판정 |
| ---: | --- | --- | --- |
| 1 | repeat 문법 과대 주장 | CRD §7.2.3.1에서 auto-repeat 단일성과 fixed-repeat 동시 사용을 분리 대조했다. | 한 auto-repeat만 목표에 넣고 second auto-repeat·nested repeat은 제외했다. 통과. |
| 2 | 기존 fixed breadth 확장 | fixed repeat breadth를 `LengthPx` 외로 넓히는지 확인했다. | #597의 1–2개 finite nonnegative computed `LengthPx` 제한을 고정했다. 통과. |
| 3 | 0/음수/overflow count | invalid count가 line name 처리 중 확장될 수 있는지 살폈다. | line metadata 전에 existing positive count와 checked expanded-track gate를 유지한다. 통과. |
| 4 | outer list N+1 경계 | `TrackList.line_names` 길이를 component 수와 대조하고, ordinary `TrackSize` 이웃이 빠진 oracle을 공격했다. | 입력 검증 `values.len()+1` 및 output의 generated Taffy boundary count를 각각 검사한다. 네 번째 scene으로 ordinary track 경계도 추가했다. 통과. |
| 5 | fixed pattern N+1 경계 | multi-track pattern에서 첫·끝 경계 group이 사라질 수 있는지 점검했다. | 각 fixed `TrackRepeat.line_names`는 `track_sizes.len()+1`이어야 한다. 통과. |
| 6 | auto pattern N+1 경계 | auto-repeat의 반복 시작/끝 name group 위치를 확인했다. | 기존 별도 `ComputedCssGridAutoRepeat.line_names`로 N+1을 보존한다. 통과. |
| 7 | 같은 repeat 내부 seam | 한 fixed repeat가 두 번 이상 반복될 때 이전 끝과 다음 시작이 분리될 수 있다. | output boundary group에 end→next start 순으로 이름을 붙이고 2회 이상 count로 검사한다. 통과. |
| 8 | 두 fixed group seam | 앞 group 끝, outer list boundary, 뒤 group 시작이 서로 다른 Grid line으로 기록될 수 있다. | authored component 순서의 이름을 한 generated physical-line group으로 합친다. Chrome CSSOM에 `[join join mid]`를 고정했다. 통과. |
| 9 | fixed/auto 시작 seam | fixed repeat 끝과 auto-repeat 첫 line의 이름이 두 line으로 분리될 수 있다. | outer prefix와 auto-repeat 첫 inner group을 Taffy의 같은 boundary로 전달한다. `[mid-end seam seam]` 관찰을 고정했다. 통과. |
| 10 | auto/fixed 끝 seam | auto-repeat 끝, outer boundary, fixed repeat 시작이 서로 다른 line으로 기록될 수 있다. | auto inner final group과 이후 outer group이 같은 resolver line에서 합쳐지는지 adapter 검사에 넣었다. 통과. |
| 11 | duplicate token occurrence | 한 physical line에 `seam` 두 개가 있을 때 named occurrence가 두 번 증가할 수 있다. | CSSOM은 중복 token을 유지하고 Taffy line map의 위치를 deduplicate해야 한다. `seam 5/6`의 첫 auto track 위치를 oracle로 고정했다. 통과. |
| 12 | line-name 순서 손실 | merged seam에 이름을 정렬하거나 조기 deduplicate하면 CSSOM order와 metadata 계약이 깨질 수 있다. | source 순서·중복 token을 보존하고 resolver가 physical position만 deduplicate한다. 통과. |
| 13 | 양수 occurrence off-by-one | merged fixed/auto 경계에서 `seam 5/6`이 잘못된 track을 가리킬 수 있다. | line index와 item rectangle을 독립 계산해 Chrome 156과 대조했다. 통과. |
| 14 | 음수 occurrence 계산 | `seam -3/-2`가 implicit line 또는 마지막 fixed suffix 기준으로 오인될 수 있다. | explicit 16-track list의 named line positions와 `(180,0,24,40)`을 고정했다. 통과. |
| 15 | named span의 반대 방향 검색 | `span 2 mid`의 시작 line 계산을 occurrence와 같은 방식으로 구현하지 않을 수 있다. | `mid-end 2`에서 뒤로 두 `mid` line을 세는 Chrome rectangle `(56,0,36,40)`을 추가했다. 통과. |
| 16 | auto-fit placement 순서 | placement 전에 빈 repeat track을 collapse해 name occurrence가 바뀔 수 있다. | 네 번째 auto-fit track을 이름으로 선택한 뒤 나머지 5개 track의 zero breadth를 검사한다. 통과. |
| 17 | rows/columns 축 전이 | column-only 구현이 row-axis metadata 또는 end negative line을 손상할 수 있다. | 독립 row auto-fill scene에 seam과 negative suffix placement를 포함했다. 통과. |
| 18 | auto-repeat count 산술 | gutter를 빼먹거나 반복 전후 fixed breadth를 빠뜨리면 5/6/7 count가 달라질 수 있다. | `84 + 24n + 4(9+n)`로 6회 288px, 7회 316px을 독립 산출하고 validator에서 16-track order를 확인한다. 통과. |
| 19 | generated group cardinality/memory | flatten된 fixed tracks에 반복 metadata를 붙여 vector length가 resolver component 개수와 달라질 수 있다. | `tracks.len()-auto_pattern_tracks+2` groups 및 checked 1 MiB metadata budget을 생성·검사하도록 계획했다. 통과. |
| 20 | allocation failure와 완료 상태 경계 | Spinon-owned allocation failure, vendored Taffy infallible vector, narrow Chrome oracle 또는 계획 병합을 C11 완료나 process-wide OOM recovery로 과대 해석할 수 있다. | owned path만 contextual fail-closed·no-partial-output으로 검사하고 vendor OOM recovery는 제외한다. opt-in runtime/artifact gate를 요구하며 `C11 [ ]`, `72/213`을 유지한다. 통과. |

## 판정

20개 관점에서 계획의 범위·oracle·line metadata 경계가 일관된다. 관찰 중 fixture가 implicit row/column auto-placement로 named placement와 섞이는 위험을 찾아 각 item의 반대 축을 명시 배치했다. ordinary `TrackSize` 이웃 경계가 부족해 네 번째 scene을 추가했다. 화면 캡처는 DOM observation JSON을 숨긴 상태로 다시 저장했다. Chrome fixture validator는 repeat count, 16-track sequence, merge group, item geometry와 auto-fit collapse를 모두 통과했다. 구현 시작 전 AST-to-boundary assertion은 별도 구현 gate로 남긴다. C11 전체는 여전히 미완료다.
