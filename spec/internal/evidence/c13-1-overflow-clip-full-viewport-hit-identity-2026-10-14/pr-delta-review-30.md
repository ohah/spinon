# C13.1 전체 viewport hit PR 변경 관점 30개 · 2026-10-14

이 검토는 계획 단계의 20관점 표와 분리한 최종 PR diff 공격 검토다. 대상은 Rust test, observation/capture/validator, 계획·상태 문서, 증거와 본문 screenshot이다.

| # | 변경 경계 공격 | 결과와 근거 |
|---:|---|---|
| 1 | 새 test 모듈 선언이 중복되거나 기존 matrix를 대체한다 | PASS — 기존 module을 유지하고 하위 `full_viewport_hit`만 추가한다. |
| 2 | test가 기본 library에 runtime fixture ABI를 넣는다 | PASS — `#[cfg(test)]` Rust unit-test tree 안의 test-only include다. |
| 3 | 테스트 입력이 이전 fixture 내용과 달라진다 | PASS — pinned fixture SHA로 validator와 Rust test를 연결한다. |
| 4 | 로컬 Chrome 버전만 적고 revision을 생략한다 | PASS — Chrome version·revision·UA를 함께 pin한다. |
| 5 | viewport 설정이 browser batch에서 분리되어 기본값으로 실행된다 | PASS — 원본 batch는 open, wait, viewport 설정과 capture eval을 한 호출에서 순차 실행한다. |
| 6 | capture가 스크롤 후 원점에 있다고 가정한다 | PASS — script가 scroll origin을 설정하고 metadata에서 0을 기록한다. |
| 7 | JSON의 case 순서가 Rust style 배열과 어긋난다 | PASS — 각 위치의 case ID와 target ID를 검증한다. |
| 8 | target frame 관측치가 바뀌었는데 full grid만 갱신된다 | PASS — 기존 2D observation의 SHA와 각 target frame을 비교한다. |
| 9 | section frame 누락이 scan identity를 무효화한다 | PASS — 각 section frame 비교 실패 시 pixel loop 전에 테스트가 실패한다. |
| 10 | clip box frame 누락으로 clip 좌표 shift가 숨는다 | PASS — 각 box frame을 independently 비교한다. |
| 11 | target frame mismatch가 tolerant threshold를 넘어가도 통과한다 | PASS — threshold는 0.5 CSS px이며 네 필드마다 assertion한다. |
| 12 | nested inner frame이 다른 case에 붙는다 | PASS — nested case만 inner frame을 요구하고 다른 12개는 null을 요구한다. |
| 13 | 실제 Chrome identity에 미지원 문자열이 들어가 root로 임의 변환된다 | PASS — `expected_node`는 알려진 kind/case만 허용하고 unknown은 panic한다. |
| 14 | target/inner descendant 경계 순서가 바뀐다 | PASS — Chrome capture의 owner 우선순위를 보존하고 exact identity를 Rust와 대조한다. |
| 15 | unique ID 결과에서 target/box/case가 빠져도 지나간다 | PASS — Rust가 고정 기대 ID 집합과 관측 집합을 비교한다. |
| 16 | run이 x=0에서 시작하지 않거나 중간 gap이 생긴다 | PASS — 모든 run `from`은 누적 `next_x`와 같아야 한다. |
| 17 | 역방향 또는 viewport 밖 run이 포함된다 | PASS — `to >= from` 및 `to < width`를 확인한다. |
| 18 | 마지막 x가 누락되어도 점 수만 다른 방식으로 맞춘다 | PASS — 모든 행 끝이 정확히 width가 되어야 한다. |
| 19 | y 행 누락/중복/재정렬이 발생한다 | PASS — 각 row `y`는 0부터 순서대로 고정하고 1,100행 수를 확인한다. |
| 20 | RLE 합계와 실제 point loop 수가 다르다 | PASS — Rust loop에서 누적한 점 수를 1,408,000과 비교한다. |
| 21 | browser identityCounts와 실제 JSON run 수가 달라진다 | PASS — validator가 모든 run 길이 합계·identity count·unique set을 재계산한다. |
| 22 | null 외부 control 좌표가 내부 점과 중복 계수된다 | PASS — controls는 별도 top-level 배열이며 full-grid count에 포함되지 않는다. |
| 23 | 바깥 null이 한 축이나 한쪽 경계에서만 확인된다 | PASS — left/top/right/bottom을 각각 정확한 좌표 및 null tag/id로 검증한다. |
| 24 | Rust query의 viewport 바깥 결과를 단순 문서 주장으로 남긴다 | PASS — 네 Chrome control 각각을 `hit_test_css_point`의 `None`과 assertion한다. |
| 25 | screenshot이 앱 또는 GPU output처럼 제시된다 | PASS — README와 PR 본문에서 Chrome fixture oracle이라고 표시하고 앱/surface 증거에서 제외한다. |
| 26 | screenshot이 전체 변경에 필요 없는 용량을 크게 추가한다 | PASS — PNG 한 장만 첨부하며 fixed oracle scene을 직접 보여준다. |
| 27 | 캡처 첫 실패를 삭제해 재시도 한계를 감춘다 | PASS — daemon 실패 JSON은 별도 보존하고 최종 단일 page scan과 구분한다. |
| 28 | validator만 통과하고 실제 Rust renderer를 실행하지 않는다 | PASS — focused Rust test 및 전체 workspace test에서 같은 observation을 사용한다. |
| 29 | C13/C13.1이 full-grid 결과만으로 완료 표시된다 | PASS — official status와 roadmap에서 surface/event/WPT/scroll gate를 미완료로 유지한다. |
| 30 | 새 테스트 브랜치가 최신 main 뒤에 있거나 merge base가 달라져 불필요한 충돌을 숨긴다 | PASS — branch base는 C14.2 merge SHA `18d3ff13b15ce42a4824167b2febdb9727995091`이며 PR 생성 전 최신 `origin/main` ancestry와 mergeability를 다시 확인한다. |

최종 delta에서 남은 코드 P1/P2는 없다. 화면 증거는 fixture/oracle에 한정한다. Android/iOS 앱 전체 matrix, PixelCopy/Metal surface, DOM event dispatch와 WPT는 후속 범위다.
