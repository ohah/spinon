# C19.1 incremental color reuse 계획 실패 관점 20개

검토한 계획 blob은 `plan/c19-1-incremental-color-reuse.md`의 Git blob SHA `97d31e561670a3a3de128bdf84c9a5c0c86ee071`다. 기준 source는 `29449e4f794e53255909b58ed8fb2fd4995dd5d2`이다. 구현은 이 재검토 뒤 시작한다.

| # | 독립 실패 관점 | 계획에 둔 방어 또는 판정 |
|---:|---|---|
| 1 | C19.1만 열어야 하는데 paint/layout property 전체를 incremental snapshot에 끌어옴 | C19.1 full 및 incremental 경로가 동일한 전용 2-Longhand `background-color`·`color` 목록을 쓴다. author CSS 허용 목록은 변경하지 않는다. |
| 2 | 이전 snapshot profile이 다른데 C19.1 output을 섞음 | 기존 profile 일치 guard를 유지하고 불일치 negative case를 둔다. |
| 3 | author stylesheet가 이전 snapshot과 다른데 재사용 | 현재 incremental API의 외부 author stylesheet 거부를 유지한다. |
| 4 | media environment 또는 viewport가 달라진 결과를 재사용 | viewport/environment mismatch는 `None` 유지로 판정한다. |
| 5 | 문서 generation이 바뀌어 NodeId가 우연히 재사용됨 | generation mismatch 거부를 회귀 검사한다. |
| 6 | 새 문서 revision이 아닌 이전 snapshot에 다시 reuse | `previous.document_revision >= current.document_revision` 거부를 검사한다. |
| 7 | style revision이 여러 단계 건너뛰어 변경 내역이 손실 | 직전 revision만 수용하고 먼 revision은 거부한다. |
| 8 | caller가 dirty root를 빼먹어 낡은 computed color를 반환 | dirty container의 inherited color 변경을 full 결과와 비교한다. |
| 9 | dirty ancestor를 context에서 계산하지 않아 하위 상속 결과가 stale | child가 새 parent color를 상속하는지 검사한다. |
| 10 | dirty subtree 바깥 sibling까지 잘못 다시 계산 | 변경 없는 sibling node의 이전 output 재사용 및 통계를 확인한다. |
| 11 | context ancestor의 출력이 바뀌지 않았는데 stats가 재계산으로 집계 | context ancestor 출력 일치와 `context_style_elements`를 별도로 확인한다. |
| 12 | `computed_color`만 비교하거나 legacy paint field가 C19 carrier에 노출 | 각 요소의 두 typed carrier 전체, 색 property key 두 개, legacy `background_color`·`background_paint`가 `None`임을 full cascade와 비교한다. |
| 13 | carrier의 source space는 같지만 gamut-mapped bytes/alpha가 낡음 | `ComputedElementStyle` 전체 동등성으로 모든 carrier field를 비교한다. |
| 14 | currentColor background가 color 상속 변경을 놓침 | 선언 fixture에 상속된 color와 background carrier를 함께 포함한다. |
| 15 | output element 순서가 재사용/재계산 뒤 달라짐 | 전체 ordered `elements` vector를 full cascade와 비교한다. |
| 16 | stylesheet diagnostics가 재사용 때 누락 또는 중복 | 전체 diagnostics를 full result와 비교한다. |
| 17 | root-html synthetic wrapper 의미를 fragment fixture로 과장 | 범위는 고정 fragment child view로 명시하고 wrapper 동작 주장을 하지 않는다. |
| 18 | C19.1 incremental cache 지원을 paint 지원으로 오해 | paint, GPU, hit-test, WCAG, public API 제외를 계획/계약에 적는다. |
| 19 | 색상 경로가 달라져 Chrome/Stylo 불일치를 재분류 | 전체 cascade equivalence만 검사하고 기존 color oracle은 바꾸지 않는다. |
| 20 | 근거 링크를 추가하면서 C19 상태를 실수로 완료 처리하거나 공식 수를 올림 | `spec/STATUS.md` C19 설명에는 제한 구현 evidence 링크만 추가하고 checkbox 미완료 및 `72/213`을 유지한다. roadmap은 수정하지 않는다. |

**검토 판정:** 구현을 진행할 수 있다. updated plan blob을 기준으로 20개 관점을 재확인했다. 이 계획은 색 carrier에 필요한 두 Longhand만 직렬화하는 C19.1 내부 snapshot reuse와 STATUS 설명의 제한된 evidence 링크 갱신에 한정되며 기존 incremental API의 거부 조건을 보존한다.
