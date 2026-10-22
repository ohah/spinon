# C11.4 auto-repeat line name 계획 실패 관점 검토 20개

**검토 대상:** CSS oracle 독립성, line-name metadata와 auto-repeat ordering, limit/fail-closed 경계, simulator 및 artifact 판정. 구현 전에 계획의 서로 다른 실패 경로를 점검했다.

| # | 독립 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | Chrome 관찰값을 규범 기대값으로 복사하는가 | 반복 개수와 item frame은 track/gutter 산술 및 CSS Grid 규칙으로 먼저 계산했고 Chrome 표는 별도 관찰로 보존한다. 통과. |
| 2 | auto-fill count가 gutter 수를 빠뜨리는가 | 180px 사례에서 40px track 3개+2 gutters는 140px, 네 개+3 gutters는 190px임을 명시했다. 통과. |
| 3 | auto-fit이 collapse 전에 occurrence를 바꾸는 것으로 오해하는가 | count 3을 먼저 고정한 뒤 빈 첫·셋째 track만 접으며 named placement line은 유지하는 기준을 분리했다. 통과. |
| 4 | bare, positive, negative occurrence가 모두 올바른 named line을 고르는가 | bare는 첫 line `(0,0)`, positive `2/3`은 두 번째 track `(50,0)`, negative `-2/-1`은 마지막 track `(100,0)`으로 각각 고정했다. 통과. |
| 5 | named span의 track 폭에서 gap을 빠뜨리는가 | 두 40px track 사이 10px gutter를 포함한 90px item을 예상값으로 고정했다. 통과. |
| 6 | two-track repeated fragment를 단일 track과 같은 line count로 처리하는가 | 40/30px tracks와 내부 gutter를 두 번 반복하는 별도 scene 및 frame을 둔다. 통과. |
| 7 | prefix/suffix tracks와 외부 gutter가 repeat count에 빠지는가 | 260px scene을 식으로 계산하고 count 2/3 사용량 220/310px를 비교했다. 통과. |
| 8 | repeat 마지막/첫 line-name group 병합이 새 occurrence를 만드는가 | 경계에서 같은 physical line의 중복 name을 별도 line으로 세지 않으며 `neighbors`, `merge`를 분리한다. 통과. |
| 9 | repeat 경계 외의 prefix/suffix name group이 손실되는가 | metadata를 outer component line groups와 repeated fragment groups로 분리해 전달하도록 계획했다. 통과. |
| 10 | 행 축만 이름을 보존하고 열 축만 구현하는가 | fixed row-axis named occurrence scene과 columns/rows metadata를 각각 검증하도록 둔다. 통과. |
| 11 | computed snapshot의 `N+1` 의미를 components 수로 잘못 바꾸는가 | auto-repeat 시 outer groups는 component 경계, repeat groups는 repeated track 경계로 별도 표현하고 non-repeat N+1은 보존한다. 통과. |
| 12 | numeric line/span placement가 새 symbolic path에서 깨지는가 | 기존 numeric placement와 auto-placement controls를 같은 회귀 집합에 포함한다. 통과. |
| 13 | non-repeat named line resolver를 불필요하게 옮기는가 | 일반 axis는 현재 projector resolver를 유지하고 auto-repeat axis에만 deferred name을 둔다. 통과. |
| 14 | named area alias나 implicit line fallback이 조용히 새로 지원되는가 | areas+auto-repeat 및 missing-name fallback은 계속 오류 경계로 명시한다. 통과. |
| 15 | line-name metadata 크기/구조 오류에서 panic 또는 partial output이 생기는가 | N+1/component group 수를 검증하고 Spinon bridge allocation failure는 구체 오류, layout 전체 거부로 계획했다. 통과. |
| 16 | auto-repeat count를 projection과 Taffy에서 중복 계산하는가 | 고정 Taffy 0.15.0의 size 계산 후 named resolver를 활용하고 별도 count algorithm은 추가하지 않는다. 통과. |
| 17 | 공개 CSS 지원으로 지나치게 넓혀 쓰는가 | 고정 8개 fixture 및 기존 auto-repeat track subset에 제한하고 일반 CSS admission을 부정한다. 통과. |
| 18 | 앱 box trace를 glyph/border paint로 잘못 확대하는가 | simulator acceptance는 layout trace와 surface box edge만이며 renderer의 border 미구현을 명시한다. 통과. |
| 19 | fixture ABI가 기본 제품 artifact에 섞이는가 | default/product와 opt-in fixture-test archive의 음성/양성 대조를 별도 요구한다. 통과. |
| 20 | 계획 PR의 Agent Browser, simulator, Tailnet 상태를 혼동하는가 | 계획/구현 evidence를 분리하고 Agent Browser만 사용, 세션 종료, simulator 결과 한계, 병합 뒤 Tailnet hash 검증을 각각 지정했다. 통과. |

이 검토는 계획의 정합성만 다룬다. 구현/runtime 적대 검토 20개 및 최종 PR diff 검토는 각각 구현 변경 이후 별도로 작성한다.
