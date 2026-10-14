# C14.1 · 구현 계획 적대 검토

검토 대상은 [`C14 intrinsic sizing 계획`](../../../plan/c14-intrinsic-sizing.md)의 C14.1 범위다. 계획 수정 뒤의 최종 20개 관점이며, 코드를 검토하거나 기능 구현 검토 횟수로 재사용한 것이 아니다.

| # | 독립 실패 관점 | 검토 결과와 계획 반영 |
| ---: | --- | --- |
| 1 | 다른 DocumentGeneration에서 같은 NodeId에 붙은 오래된 image size를 재사용한다. | source `LayoutSourceRevision`을 measurement snapshot에 묶고 input source와 다르면 거부하도록 했다. |
| 2 | 비동기 decode 완료 때 revision이 포화되어 이전 번호로 되돌아간다. | document-scoped revision의 `checked_next()`와 overflow 실패를 명시했다. |
| 3 | 측정 map 값은 새 값인데 LayoutOutput revision은 이전 값으로 남는다. | measurement revision을 `LayoutInputRevision`과 output에 끝까지 보존하도록 했다. |
| 4 | 계산 뒤 자원 크기가 바뀌었는데 현재 입력 gate가 이전 output을 받는다. | `CurrentLayoutInputs` 비교에 measurement revision을 추가하도록 했다. GPU queue 재검증은 별도 미완료 경계로 명시했다. |
| 5 | 이미지 크기 누락이 기존 zero-size leaf fallback으로 조용히 성공한다. | 보이는 auto/auto HTML image의 누락 측정은 node별 오류로 처리한다. |
| 6 | decoder 실패가 0×0 natural size로 들어가 NaN/0 division을 만든다. | 측정 snapshot은 양의 유한 width·height만 받으며 broken-image UI는 U05 후속으로 뒀다. |
| 7 | NaN 또는 infinity가 Taffy 계산 후 frame 집계에 퍼진다. | 입력 검증에서 유한성을 검사하고 계산 출력 검증도 유지한다. |
| 8 | decoder의 물리 pixel 또는 DPR 배율 값을 CSS px처럼 전달한다. | 입력 단위를 density 보정된 CSS px로 고정하고 DPR을 비교 변수와 분리했다. |
| 9 | HTML이 아닌 namespace의 `<img>`가 HTML replaced element로 오인된다. | 대상 판정을 HTML namespace의 `img`로 한정한다. SVG `<image>`는 제외한다. |
| 10 | `<img>`가 DOM 자식을 가져도 leaf measure가 자식 배치를 건너뛴다. | measurement가 있는 대상은 leaf여야 하며 자식이 있으면 오류로 처리한다. |
| 11 | 다른 일반 요소나 버튼에 이미지 measurement map을 잘못 적용한다. | map key가 HTML image marker가 아니면 오류다. 버튼 크기는 C15/U04에 남긴다. |
| 12 | width만 auto 또는 height만 auto일 때 2축 자연 size가 그대로 사용되어 ratio가 깨진다. | C14.1 성공 조건을 양 축 auto로 제한하고 부분 지정은 오류로 둔다. |
| 13 | author `aspect-ratio`가 자연 ratio와 섞여 우선순위가 모호해진다. | aspect-ratio가 설정된 입력은 C14.1 밖으로 두고 실패 처리한다. |
| 14 | padding/border를 decoder size에 먼저 더한 뒤 Taffy가 다시 더한다. | snapshot 측정은 content size만 담으며 Taffy box edge가 한 번 적용되도록 했다. |
| 15 | `min/max-width/height`가 natural frame을 잘못 제한하거나 무시된다. | 기존 Taffy min/max를 한 번 적용하고 clamp case를 별도 확인한다. |
| 16 | flex auto basis 측정에는 intrinsic size가 사용되지 않고 최종 leaf에만 반영된다. | flex item의 auto basis와 실제 layout pass를 별도 시험한다. |
| 17 | 상위 `display:none` 아래에서 필요 없는 이미지 decode/measurement를 요구한다. | 조상 visibility를 함께 판정하고 hidden subtree는 measurement 요구에서 제외한다. |
| 18 | 같은 snapshot 안의 반복 NodeId가 수집 과정에서 map insert로 덮어써진다. | decoder→snapshot 변환 단계에서 중복을 덮어쓰지 말고 오류로 처리하도록 계획에 추가했다. |
| 19 | Chrome 변경 또는 viewport 조건 변경으로 기준 결과를 섞는다. | 실행 버전·viewport·DPR를 기록한다. Chrome 155 결과는 기존 pinned Chrome 154와 구분하고 재실행 없이는 같은 reference로 표시하지 않는다. |
| 20 | layout input admission 성공을 실제 GPU 표시까지 stale 차단 완료로 확대 해석한다. | queue/display stale gate는 S02 미완료로 분리하고 C14.1 증거로 주장하지 않는다. |

검토 뒤 보완한 항목은 measurement snapshot의 source binding, duplicate rejection, CSS px 단위, generation-scoped overflow 정책, hidden ancestor 처리, stale gate 한계다. 구현 전 계획 검토 20개에서 남은 차단 결함은 없다.

구현 전 추가 재검토: 자원 owner가 CSS size와 관계없이 decode된 모든 HTML image의 natural-size map을 제공해도 고정된 width/height image 때문에 계산 전체가 실패하지 않아야 한다. 양 축 definite이면 입력 measurement를 무시하고, auto/auto만 필수로 하며, partial auto만 명시적으로 거부하도록 계획을 수정했다.

구현 경계 재검토: generic `LayoutInput`은 tag/resource 의미를 소유하지 않고 다수의 직접 구성 fixture가 공개 입력 형태를 공유한다. measurement map은 새 `TaffyLayoutEngine::compute_with_intrinsic_size_snapshot` 인자로 분리하고, HTML image 누락·tag·visibility 판정은 HostDocument를 보유한 style-to-layout preflight로 이동하도록 계획을 수정했다. 이 재검토 뒤 기존 LayoutInput 생성자를 유지하며, 새 compute 입력의 source와 revision은 engine output/current gate/render key까지 전달한다.
