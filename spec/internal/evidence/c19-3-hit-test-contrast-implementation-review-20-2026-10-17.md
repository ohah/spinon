# C19.3 hit-test·WCAG 계산 구현 적대 검토 20개

**대상:** C19.3의 첫 제한 조각 코드와 문서
**검토일:** 2026-10-17
**종류:** 서로 다른 구현 실패 관점 20개. 전체 CSS·접근성 적합성 또는 플랫폼 화면 검증이 아니다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | opacity 0인 상위 box가 hit-test에서 제거됨 | 통과. opacity `0` group에서도 overlap target은 상위 node다. |
| 2 | source alpha 0인 색이 hit target에서 제외됨 | 통과. source alpha `0`에서도 같은 target을 반환한다. |
| 3 | group opacity와 source alpha가 각각 따로는 되지만 함께 적용될 때 hit 결과가 변함 | 통과. outer/inner group opacity 3×3과 source alpha 3개를 교차한 27조합을 확인한다. |
| 4 | 중첩 group의 EndGroup 순서나 불균형이 잘못됨 | 통과. 새 fixture는 두 단계 group을 열고 역순으로 닫으며 snapshot constructor의 검증을 거친다. |
| 5 | 상위가 투명할 때 하위 box의 hit 영역이 사라짐 | 통과. 상위 경계 밖의 under-only 좌표는 모든 27조합에서 하위 node다. |
| 6 | transparent 상위가 overlap hit target을 계속 가리는지 불명확 | 통과. Chrome 155 사전 기준과 Rust fixture 모두 상위 node를 선택한다. |
| 7 | frame 시작 경계의 inclusive 동작이 alpha/group 별로 변함 | 통과. top-left 경계 `(20,20)`를 각 조합에서 확인한다. |
| 8 | frame 오른쪽 경계의 half-open 동작이 alpha/group 별로 변함 | 통과. `(60,35)`는 상위 box 끝 경계라 하위 node를 선택한다. |
| 9 | frame 아래쪽 경계의 half-open 동작이 alpha/group 별로 변함 | 통과. `(30,50)`도 하위 node를 선택한다. |
| 10 | paint clip과 hit-test clip이 opacity 처리에서 합쳐짐 | 통과. hit-test 구현은 `hit_test_clip_css_px`만 읽고 별도 기존 clip 회귀 테스트가 paint clip 분리를 확인한다. |
| 11 | alpha 변화가 hit-test order를 바꾸거나 paint order를 기준으로 재정렬 | 통과. `hit_test_css_point`는 box frame, hit-test clip, `hit_test_order`만 사용하며 paint command alpha를 읽지 않는다. |
| 12 | viewport 바깥 또는 비유한 좌표가 새 test 경로에서 허용됨 | 통과. renderer 기존 경로에서 viewport와 finite check를 적용하며 기존 regression tests가 이를 고정한다. |
| 13 | `RuntimePaint::Color` alpha fixture가 잘못된 encoded channel을 만들어 matrix가 무효 | 통과. RGB는 8-bit 범위로 나누고 alpha는 0–1 값으로 constructor 검증을 통과한다. |
| 14 | WCAG 입력 타입이 alpha 또는 premultiplied 값을 받아 합성 전 색을 조용히 비교 | 통과. public type에는 RGB 세 채널만 있고 문서가 backdrop 합성을 호출자 책임으로 둔다. |
| 15 | NaN/Infinity 또는 범위 밖 채널이 휘도 계산에 도달 | 통과. 유한성·범위 오류가 채널 인덱스와 함께 반환되고 테스트가 각각 확인한다. |
| 16 | sRGB encoded 채널을 이미 linear 값으로 오인하거나 piecewise 경계 한쪽을 잘못 계산 | 검토 중 경계 branch가 기존 회색/극값 fixture에서 직접 고정되지 않은 점을 발견했다. 0.04045 이하/초과 test를 추가했고 7개 WCAG 테스트가 통과했다. |
| 17 | 세 luminance 계수 오타가 grayscale test에서 상쇄 | 검토 중 계수별 색 fixture가 없음을 발견했다. red/green/blue primary 값별 기대 ratio를 추가했고 7개 WCAG 테스트가 통과했다. |
| 18 | lighter/darker 선택 또는 0.05 보정 오류가 대칭성 assertion만으로 놓침 | 통과. black/white 21:1, #767/#777 양쪽, 정확 `4.5:1` 및 소수 채널 양쪽 fixture가 식의 크기와 경계를 함께 확인한다. |
| 19 | 잘못된 ratio threshold를 helper 안에 넣어 법적 적합성 판정으로 오인 | 통과. helper는 ratio만 반환하며 4.5 threshold·실제 glyph·WCAG conformance 판단은 호출자 범위 밖으로 계약에 명시한다. |
| 20 | Agent Browser의 3×3 결과를 nested CSS alpha, image paint, text glyph 또는 앱 접근성까지 일반화 | 통과. browser oracle은 단일 element hit identity이고 nested group은 별도 Rust geometry invariant다. 이미지, text, compositing, Android/iOS 픽셀 및 접근성은 제한 사항으로 유지한다. |

## 실행 근거와 판정

- `cargo test --locked -p spinon-style wcag_contrast::tests` — 7/7 통과.
- `cargo test --locked -p spinon-render opacity_and_source_alpha_do_not_change_geometry_hit_targets` — 통과. 두 nested group opacity × source alpha 조합 27개 각각에서 overlap/under-only/세 경계 좌표를 확인한다.
- 세 이전 negative control은 복원 후 확인했다: `RuntimePaint::Color` alpha 0 skip은 `opacity=0, alpha=0` overlap에서 기대한 top 대신 under node를 반환해 실패했고, red 계수 `.2127` mutation은 black/white `21.002 != 21`로 실패했으며, sRGB exponent `2.2` mutation은 #767/#777 기대값에서 실패했다. 이번 review에서 추가한 piecewise 경계와 계수별 primary test는 해당 두 계산 경로의 빈틈을 줄인다.
- 전체 workspace test는 실행 근거가 아니다. 별도 C19.2 runtime regression 빌드는 disk-full로 중단됐다. Android/iOS 시뮬레이터가 이 pure Rust hit-test·formula slice의 완료 조건에 포함되지는 않는다.

계획된 범위에서 수정 사항을 반영했다. C19.3 전체, C19 parent 또는 C15/C16 text paint 연결 완료를 주장하지 않는다.
