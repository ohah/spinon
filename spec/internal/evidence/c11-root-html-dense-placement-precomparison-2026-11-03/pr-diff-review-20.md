# C11.1 Root HTML dense auto-placement 계획 PR diff 검토 · 20개 실패 관점

**검토 대상:** PR `docs: C11 dense 배치 Root HTML 계획 추가`의 전체 계획·상태·근거 변경. 기반 main `bd6f68e54f5f3a5013fee505b3d89bd6946c3dc0`.

1. **기능 중복 범위:** C11.2.3 dense 알고리즘을 중복 구현할 위험. **통과** — 기존 normalizer를 사용하고 parsed Root HTML admission만 새 제한 mode로 연결한다.
2. **computed style 경로:** 선언 문자열을 재파싱해 cascade를 다르게 해석할 위험. **통과** — computed typed flow만 사용하도록 적었다.
3. **dense shorthand 의미:** `grid-auto-flow:dense`의 기본 row 축이 빠질 위험. **통과** — 기본 row dense, `row dense`, `column dense`를 모두 명시했다.
4. **sparse 회귀:** dense 경로가 기존 row/column sparse 동작을 바꿀 위험. **통과** — sparse 대조 scene을 같은 matrix에서 보존하고 전용 mode만 dense item에 적용한다.
5. **전용 admission:** 일반 FFI 입력이 Root HTML 전용 입력 제한을 우회할 위험. **통과** — `RootHtmlDenseAutoPlacementV1`은 parsed Root HTML 검증 후 선택한다.
6. **explicit track 경계:** 비고정·빈 explicit track이 지원으로 확대될 위험. **통과** — 양 축 비어 있지 않은 finite non-negative fixed-px 목록으로 제한한다.
7. **implicit auto·track ceiling 경계:** implicit track 생성이 필요한데 `auto`를 허용하거나 두 축 track 수를 합산해 상한을 다르게 적용할 위험. **수정 후 통과** — 양 축 각각 `auto`는 해당 축에 새 track이 없을 때만 허용한다. `row-dense-fixed-row`의 `grid-auto-columns:auto` 기대 거부와 C11.2.3의 축별 10,000 상한을 지정했다.
8. **span·line 입력:** 0/음수 span이나 이름 line이 dense path에 들어올 위험. **통과** — 양수 span·positive definite line만 지원하고 negative/named line을 fail-closed한다.
9. **order 정렬:** nonzero `order`를 일부만 처리해 occupancy와 paint order가 엇갈릴 위험. **통과** — `order:0`만 admission한다.
10. **nested flow 격리:** nested mixed-flow의 한 container 판정이 이웃 결과에 번질 위험. **통과** — container별 독립 검증과 기존 nested candidate를 명시했다.
11. **visibility 참여:** `display:none`과 `visibility:hidden` 처리가 뒤바뀔 위험. **통과** — 전자는 점유에서 빼고 후자는 layout participant로 둔다.
12. **source·accessibility·paint order:** visual placement 순서를 tree 순서로 오인해 재정렬할 위험. **통과** — 세 순서를 변경하지 않는다고 계약했다.
13. **failure atomicity:** reject 뒤 partial track/frame/draw가 남을 위험. **통과** — 결과 publication 전에 실패하고 부분 결과를 남기지 않는 기준을 명시했다.
14. **allocation failure 복구:** reserve 실패가 panic이나 worker 오염으로 끝날 위험. **통과** — 실제 allocation site 주입과 같은 worker 재사용 검사를 구현 관문으로 뒀다.
15. **기존 scene 수:** oracle-only 장면을 제품 성공으로 세거나 candidate를 빠뜨릴 위험. **통과** — 기존 25 scene 중 21 candidate(19 success, 2 expected rejection), 4 oracle-only를 구분하고 92 node ID를 matrix와 대조했다.
16. **독립 기대값:** Chrome 관측만으로 expected를 순환 정의할 위험. **통과** — fixed track/gap에서 계산한 dense/sparse 좌표와 CSS Grid cursor 규칙을 분리해 적었다.
17. **비교 좌표·overflow:** root-relative frame이나 viewport 밖 item을 놓칠 위험. **통과** — node frame, expanded implicit track, root 바깥 item frame까지 비교 대상으로 지정했다.
18. **플랫폼 근거 과장:** AVD software renderer나 iOS pre-compositor 표본을 device/compositor 증거로 부를 위험. **통과** — API 37 ARM64 AVD와 iOS Simulator 범위 및 표본 한계를 분리했다.
19. **fixture 누출:** 시험 scene·FFI가 기본 제품 artifact에 포함될 위험. **통과** — 기본 artifact와 opt-in fixture-test positive control을 별도 확인하도록 했다.
20. **공식 완료 부풀리기:** 제한 dense slice를 C11 완료나 더 높은 완료 수로 계산할 위험. **통과** — C11 checkbox와 공식 `72/213`을 유지하고 구현 후 명세·상태를 갱신하도록 했다.

**결론:** matrix 원본에서 candidate count/outcome/node 수를 확인했고, implicit `auto` admission 경계를 기존 expected rejection과 맞췄다. 계획에는 제품 성공 또는 C11 완료 주장이 없다. 구현/runtime review는 구현 PR에서 새로 수행한다.
