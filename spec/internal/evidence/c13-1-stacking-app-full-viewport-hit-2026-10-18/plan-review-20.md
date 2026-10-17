# C13.1 중첩 stacking descendant 전체 viewport hit 계획 실패 관점 20개

이 검토는 구현 전에 계획의 oracle, query 계약, 앱 연결, 실패 판정과 제외 범위를 서로 다른 실패 경로로 확인한다. 아직 runtime 코드는 검토 대상이 아니다.

| # | 실패 관점 | 계획의 방어와 남은 판정 |
|---:|---|---|
| 1 | Chrome DOM의 `data-id` 순서가 Rust frame 배열과 다르면 다른 `NodeId`를 맞았다고 처리할 수 있다. | 고정 13 ID 순서, frame 개수·순서·중복 검사 뒤 join한다. mismatch는 전체 grid 실패다. |
| 2 | 정수 query를 pixel center 또는 half-pixel 좌표로 바꾸면 이전 Chrome 결과와 달라진다. | coordinate convention을 계획·JSON·validator에서 `integer CSS points`로 고정하고 fractional query는 분리한다. |
| 3 | Chrome right/bottom fractional point가 `null`인 관찰을 `root`로 정규화하면 명세 외 추정이 oracle에 섞인다. | fractional edge 3개와 viewport 밖 4개 control을 `null` exact로 보존하며 full integer grid와 독립 판정한다. |
| 4 | capture 직전 browser viewport가 툴바 또는 창 크기 영향으로 달라질 수 있다. | `innerWidth/innerHeight`, DPR 및 `collectC131StackingClipObservation()` 결과를 함께 확인하고 고정값 외 즉시 중단한다. |
| 5 | 브라우저 페이지 scroll offset이 0이 아니면 point 좌표와 scene 좌표가 어긋난다. | scrollX/Y를 capture metadata에 저장하고 두 값 모두 0이 아니면 validator가 거부한다. |
| 6 | DPR 2 이상의 capture가 들어오면 physical sample과 CSS query를 혼합할 수 있다. | DPR 1만 허용하고 screenshot dimension·viewport를 함께 pin한다. |
| 7 | 투명한 `.layer` 영역은 색 표본에 보이지 않아도 hit target일 수 있다. | 색에서 hit identity를 추론하지 않고 전 좌표의 Chrome `elementFromPoint()` node ID를 직접 기록한다. |
| 8 | parent와 자식의 겹침 영역에서 target frame보다 높은 sibling overlay가 topmost를 바꿀 수 있다. | Chrome의 실제 topmost ID를 전 좌표 저장하고 기존 18개 sample은 독립 regression으로 유지한다. |
| 9 | `clip`과 `hidden`이 같은 색/geometry를 내도 hit clip 경계가 달라질 수 있다. | 세 case를 한 viewport에서 각각 조회하고 13개 ID를 exact compare한다. |
| 10 | root/body fallback을 하나의 임의 node로 대체하면 요소 바깥 영역의 오답을 놓칠 수 있다. | root ID가 전체 viewport에서 실제 topmost인지 browser oracle에 저장하며 미등록/null grid point는 허용하지 않는다. |
| 11 | RLE row에 빈 구간이 생기면 누락 point가 default root로 조용히 채워질 수 있다. | row마다 run 폭 합이 정확히 540이어야 하며 decoder는 누락 채우기 없이 fail한다. |
| 12 | RLE run이 겹치거나 총폭을 넘으면 앞선 run이 뒤의 node를 덮어쓸 수 있다. | 각 run 시작은 누적 x에서만 계산하고 `x+width <= 540`와 마지막 폭 exact를 검사한다. |
| 13 | 같은 node ID의 이웃 run이 분리되면 canonicalization/hash 재현성이 깨진다. | 인접 동일 index는 비정규 입력으로 거부하고 capture generator가 연속 run을 병합한다. |
| 14 | JSON의 node index가 고정 ID 수 밖이면 Rust index 접근이 panic하거나 잘못된 node를 가리킬 수 있다. | 모든 index를 0..12로 검사하고 malformed oracle은 scene query 전에 반환 오류로 닫는다. |
| 15 | rows 길이와 `pointCount`가 서로 다르면 일부 viewport만 검사한 채 성공할 수 있다. | pointCount 69,120, rowCount 128 및 실제 배열 길이를 각각 독립 검사한다. |
| 16 | screenshot이 oracle JSON과 다른 capture에서 오면 첨부 이미지는 맞아도 data 근거가 바뀔 수 있다. | screenshot size/hash를 JSON·README·validator로 고정하고 fixture SHA와 browser metadata도 동일 evidence에 남긴다. |
| 17 | Chrome version metadata가 major-only면 patch 차이에 따른 engine 동작 변경을 놓칠 수 있다. | `fullVersionList`의 Google Chrome full version `155.0.8059.85`와 OS/architecture를 exact 비교한다. |
| 18 | JNI/Logcat report에 69,120개 항목을 그대로 직렬화하면 truncation·buffer exhaustion이 성공처럼 보일 수 있다. | report는 bounded counts·digest와 최대 16 mismatch만 담고 capacity 부족은 명시적 실패로 처리한다. |
| 19 | eval과 draw가 서로 다른 document/render/style revision이면 old snapshot의 완전한 grid가 새 화면과 맞다고 오인될 수 있다. | app route에서 revision tuple을 고정·대조하고 stale document negative control은 readback/성공 보고 전 거부한다. |
| 20 | opt-in fixture FFI가 product artifact에 들어가면 검증 도구가 배포 API가 된다. | 기존 C13.1 fixture-test feature만 사용하고 default artifact absence 및 opt-in positive marker를 각각 검사한다. |

**판정:** 계획은 정수 viewport hit grid와 fractional edge 관찰을 분리하고, source→oracle→runtime snapshot→bounded app report의 identity 경계를 정의한다. 구현 전에 수정할 계획 결함은 발견하지 못했다. 위 표는 구현 실패 검토나 PR 변경 검토를 대체하지 않는다.
