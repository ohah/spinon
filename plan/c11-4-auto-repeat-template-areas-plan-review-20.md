# C11.4 auto-repeat + template areas 계획 실패 관점 검토 20개

**검토 대상:** [계획](./c11-4-auto-repeat-template-areas.md), [사전 비교](../spec/internal/evidence/c11-4-auto-repeat-template-area-lines-precomparison-2026-10-23/README.md), [고정 fixture](../tests/fixtures/css/c11/grid-auto-repeat-template-areas-c114.html)

**판정:** area가 expanded template보다 큰 입력을 지원하는 근거까지 확인해 계획을 고친 뒤 20개 독립 실패 관점을 다시 검토했다. 계획 PR만 가능하며 구현/runtime을 완료로 취급하지 않는다.

| # | 실패 관점 | 검토 결과와 계획의 방어선 | 판정 |
| --- | --- | --- | --- |
| 1 | `hero-start`가 auto-repeat 확장 전에 숫자 line으로 변환되어 index가 틀린다. | symbolic area placement를 Taffy까지 보존하고 확장 뒤 resolve한다. | 통과 |
| 2 | Repeat count를 Spinon이 재계산해 Taffy 결과와 달라진다. | Taffy 반복 산식 재구현을 금지하고 pinned dependency가 계산한다. | 통과 |
| 3 | CSS area cell을 auto-repeat fragment 단위로 간주해 wider viewport에서 이름을 이동한다. | `columns-area-shorter`/`rows-area-shorter`에서 area line index를 고정하고 마지막 fixed track을 별도 검사한다. | 통과 |
| 4 | area dimension이 template보다 클 때 track이 빠지거나 잘못된 크기를 받는다. | Chrome 156의 `columns-area-larger`/`rows-area-larger`는 template 4 tracks 뒤에 area가 요청한 explicit 5번째 track을 만든다. pinned Taffy도 explicit count를 `max(template count, area count)`로 정하고 추가 track에는 `grid-auto-*`를 적용한다. 13-scene runtime에서 20px added track 및 16px overflow를 확인한다. coordinate limit 초과만 publish 전에 거부하며 detailed `explicit_tracks`는 최종 count 검증에만 쓴다. | 통과 |
| 5 | grid-template-areas 정의만으로 auto-fit repeat track이 occupied 상태가 된다. | `columns-fit-empty-area`/`rows-fit-empty-area`에서 hero item을 제거하고 모든 repeat의 collapse를 검사한다. | 통과 |
| 6 | hero 하나가 여러 repeat을 걸쳐 점유해 빈 track을 잘못 collapse한다. | occupied `columns-fit`/`rows-fit`과 expanded used-track list를 고정해 spanning item이 걸친 track은 남기는 조건을 본다. | 통과 |
| 7 | collapsed track 옆 gutter가 남아 suffix frame이 4px 밀린다. | 규범 기대 44px/20px와 Chrome 48px/24px 관측을 분리하고 simulator pass를 규범으로 고정한다. | 통과 |
| 8 | Chrome 차이가 area alias 로직 결함으로 오분류된다. | 동일한 `auto-fit` 숫자 line controls와 empty controls에 같은 차이가 관찰됐다. 브라우저 차이를 area 원인으로 돌리지 않는다. | 통과 |
| 9 | authored `hero-start`와 generated alias 충돌에서 area 배치와 integer named line이 같은 위치가 된다. | `columns-area-line-collision`의 bare area와 second occurrence frame을 다르게 고정한다. | 통과 |
| 10 | line-name pair와 area edge가 repeat seam에서 결합할 때 occurrence가 중복 계산된다. | pinned Taffy `NamedLineResolver`의 merge/dedup 순서를 새 direct placement test에서 검사한다. | 통과 |
| 11 | area-name shorthand와 generated line longhand가 서로 다른 code path를 지나 한쪽만 지원된다. | collision scene은 `grid-area: hero`; fill/fit/shorter scene은 `hero-start`/`hero-end` longhand를 함께 고정한다. | 통과 |
| 12 | 열에서는 맞고 행에서는 area bounds, 추가 track 또는 gutter 방향이 반전된다. | fill/fit/shorter/larger/empty scene을 행 축에도 mirrored CSS로 고정한다. | 통과 |
| 13 | gap을 빼고 반복 수를 세거나 fixed neighbor를 한 번만 차감한다. | `116=20+3×20+20+4×4`, `140=20+4×20+20+5×4`를 precomparison 및 runtime matrix에 고정한다. | 통과 |
| 14 | dual-axis repeats나 multiple repeat이 조용히 지원 범위를 넓힌다. | 이 slice는 한 축 한 repeat만 받는다. dual-axis/복수 조합을 fail-closed control로 유지한다. | 통과 |
| 15 | 새 영역 결합이 C11.3 non-repeat area projection을 바꾼다. | C11.3의 no-repeat dimension equality와 기존 placement path는 유지하고 별도 회귀한다. | 통과 |
| 16 | C11.4 numeric auto-repeat placement가 area metadata 유무에 따라 바뀐다. | `columns-fit-numeric`/`rows-fit-numeric`을 areas 없는 양성 대조로 유지한다. | 통과 |
| 17 | `usize` area dimensions가 Taffy `u16`로 잘리거나 invalid rectangle이 들어간다. | checked conversion, half-open coordinate validation, overflow/malformed negative tests를 계획한다. | 통과 |
| 18 | name copy/rectangle vector allocation이 실패해 partial input 또는 layout을 남긴다. | Spinon-owned allocations는 `try_reserve` failpoint를 검증하며 실패 시 완전한 error/no-layout을 요구한다. | 통과 |
| 19 | simulator frame pass가 표면 paint, border stroke, hardware 또는 compositor 결과로 과장된다. | Android AVD PixelCopy/iOS compositor 전 Metal 표본의 읽은 점만 보고하고 미검증 경계를 PR에 분리한다. border stroke가 그려지지 않으면 통과를 주장하지 않는다. | 통과 |
| 20 | 이 slice가 전체 C11.4 완료 수를 올리거나 public CSS support로 승격된다. | `spec/STATUS.md` C11.4 checkbox와 `77/213`, 내부 계약 `0.1.0`, WPT 미검증을 유지한다. | 통과 |

## 반영 후 판정

Taffy 0.15.0의 `explicit_tracks`는 이미 area dimension을 포함한 최종 explicit count다. 그러므로 template보다 큰 area를 거부할 수 없으며, 이 slice는 fixed `grid-auto-*`로 해당 explicit track을 양성 지원한다. Taffy가 내부에서 clamp하는 최대 track 수를 넘는 malformed/oversized area 입력만 layout publish 전에 fail-closed한다. fixture와 runtime은 area가 template보다 짧음·같음·큼을 각각 검증하고, 구현 PR에서 13개 scene과 final explicit count를 대조한다.
