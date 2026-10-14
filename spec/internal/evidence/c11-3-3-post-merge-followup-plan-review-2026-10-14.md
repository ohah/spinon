# C11.3.3 병합 후 증거 보강 계획 검토 · 실패 관점 20개

검토 대상은 <a href="../../../plan/c11-3-template-repeat-named-lines-areas.html">PR #152 병합 후 남은 증거 보강 계획</a>이다. 이는 구현 전 계획 검토이며 기존 기능 구현 검토와 PR 적대 검토를 대체하지 않는다.

| # | 독립 실패 관점 | 계획 판정 |
| --- | --- | --- |
| 1 | 1-based/0-based 변환에서 row start 0이 유효 index로 wrap된다 | 직접 typed injection에서 거부 오류와 node/property를 확인한다. |
| 2 | row end가 dimension보다 한 칸 커도 통과한다 | 마지막 유효 end와 초과 end를 분리하고 초과는 전체 거부한다. |
| 3 | column 범위만 잘못된 경우 row 검사 성공이 전체 성공으로 가려진다 | row/column start·end를 독립 입력으로 만든다. |
| 4 | 역전 범위가 비어 있는 영역으로 조용히 바뀐다 | start>end를 별도 입력으로 거부한다. |
| 5 | start=end인 빈 범위를 layout의 0-span item으로 흘린다 | 빈 row와 빈 column 범위를 각각 거부한다. |
| 6 | 손상된 입력 테스트가 실제 projection 함수 대신 helper 계산만 검사한다 | Stylo typed value를 public test seam이 아닌 실제 `computed_grid_template_areas` 경계에 전달한다. |
| 7 | 오류가 올바른 property/node를 잃고 일반 layout 오류로 바뀐다 | `UnsupportedComputedGridValue`의 node와 property를 정확히 단언한다. |
| 8 | area가 없다는 전제가 template `none`이나 빈 목록과 혼동된다 | area metadata 부재와 named line metadata를 fixture에서 명시한다. |
| 9 | row 축에만 동일 이름이 있으면 implicit area로 오판한다 | 두 축의 `name-start`/`name-end` pair를 모두 부재로 둔다. |
| 10 | line occurrence 1이 마지막 중복 line에 붙는다 | pinned Chrome frame과 첫 occurrence의 numeric line을 함께 고정한다. |
| 11 | bare ident test가 `1 ident` 문법 경로를 잘못 재사용한다 | item style은 bare `grid-area`만 사용하고 integer prefix는 두지 않는다. |
| 12 | frame이 같아 resolver가 실제로 실행됐는지 구분되지 않는다 | grid 각 축의 첫/두 번째 동일 이름 line이 다른 frame을 만드는 입력을 택한다. |
| 13 | Chrome 기준이 Spinon 코드와 같은 helper에서 유도된다 | 고정 Chrome 154 브라우저 결과를 독립 oracle로 저장한다. |
| 14 | mutation 두 번째 eval이 새 host에서 실행되어 cache/revision 재사용을 못 본다 | 같은 앱 host handle을 사용하고 host 생성 ID를 로그로 연결한다. |
| 15 | mutation이 템플릿만 바꾸고 item area는 고정돼 위치 변화가 없다 | 기존 area name은 유지하고 셀 위치만 반전한다. |
| 16 | 두 번째 eval 결과만 기록되어 첫 frame/revision과 비교할 수 없다 | 단계별 run marker에 revision과 동일 item NodeId frame을 남긴다. |
| 17 | document revision만 증가하고 render tree/cascade revision은 stale하다 | document와 render revision 모두 엄격 증가를 요구한다. |
| 18 | 두 번째 평가 오류가 예상 거부로 받아들여진다 | 두 mutation 단계는 모두 `layout=ready`, status 0이어야 통과한다. |
| 19 | 새 scene을 기존 고정 matrix count에 조용히 끼워 과거 증거를 무효화한다 | 원래 26-scene 결과는 보존하고 후속 run/evidence ID를 별도로 둔다. |
| 20 | AVD 결과를 physical/hardware/WPT 완료로 과장한다 | 결과 문구를 Android API 37 AVD로 한정하고 다른 관문을 미검증으로 유지한다. |

## 판정

20개 관점은 모두 실행 계획에서 입력·관찰값·실패 판정을 확인할 수 있다. 직접 typed range injection은 unit 경계, area 없는 fallback은 pinned Chrome과 Rust projection/layout 경계, mutation/revision은 같은 Android AVD runtime host 경계를 각각 사용한다. 계획상 테스트가 구현을 대신하지 않으며 발견된 결함은 별도 수정·재검증한다.
