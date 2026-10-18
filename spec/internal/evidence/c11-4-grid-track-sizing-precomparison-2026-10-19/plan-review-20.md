# C11.4 계획 적대 검토 · 20개 독립 관점

**검토 대상:** [`plan/c11-4-grid-track-sizing.md`](../../../../plan/c11-4-grid-track-sizing.md), Chrome precomparison fixture 및 README.
**검토 시점:** 구현 시작 전.
**결론:** 초기 20관점 검토에서 찾은 oracle 누락을 수정하고 다시 확인했다. 현재 고정 Chrome oracle은 26개다. 계획 검토이므로 runtime 통과나 제품 지원 주장을 의미하지 않는다.

## 검토 결과

처음 대조할 때 WPT의 `fit-content(calc(...))` 입력은 있었지만 Chrome fixture에는 없었다. definite 100px basis의 `fit-content(calc(10% + 5px))`와 Ahem break-all text를 추가해 15px track을 수집했다. 또한 text advance만 측정해도 padding/border·`min-width`·`max-width`와 Grid item automatic minimum을 놓칠 수 있어 content-box edge, 90px min, 40px max oracle을 추가했다. rows-only image track과 두 flexible track을 span한 200px unbreakable word도 추가했다. 재수집 전 관찰을 버리고 최종 26개 JSON·screenshot·hash만 계획 입력으로 남겼다.

계획 PR 전 이 20개 서로 다른 실패 경로를 문서·fixture에 대조했다. 추가로 계획 밖 implementation controls는 코드 진입 전에 독립 oracle 또는 normative expected value를 붙이는 조건으로 남겼다.

| # | 적대 관점 | 반례·위협 | 계획상 차단·판정 |
| ---: | --- | --- | --- |
| 1 | 문법만 지원으로 셈 | Stylo가 `minmax()`를 파싱하지만 계산기는 모든 track을 기존 0px/auto로 바꿈 | typed projection, track stage와 final frame 일치 필요; 조용한 fallback 금지 |
| 2 | 한 축만 구현 | 열 알고리즘은 맞지만 row auto/min-content track이 zero-height로 남음 | rows와 columns를 B단계 필수 범위로 지정, app matrix에서 두 축 분리 |
| 3 | `minmax()` 허용 조합 오해 | `minmax(1fr, auto)` 같은 허용되지 않는 최소 함수가 Taffy 내부로 전달 | Stylo computed form preflight, min/max 함수 조합 검증, node/property error |
| 4 | `auto`와 `minmax(auto,auto)` 합침 | automatic minimum과 growth limit 구분이 없어 min contribution이 유실됨 | auto min/max 의미를 track sizing stage에서 각각 관찰 |
| 5 | `fit-content()` cap만 적용 | cap보다 큰 track auto-minimum이 0 또는 cap으로 clamp됨; item `min-width:auto`를 track 함수와 혼동 | normative max-content/cap/track-auto-min floor 확인; image24/cap12 및 text70/cap40 controls |
| 6 | percent/calc cap에 틀린 basis 사용 | padding/border 또는 final track 크기를 percent argument의 basis로 재사용; calc를 raw string으로 남겨 15px를 잘못 계산 | definite 100px basis의 `fit-content(calc(10% + 5px))` → 15px oracle, indefinite argument는 별도 control |
| 7 | item contribution을 final size와 혼동 | item의 stretching 결과가 min-content input으로 재사용되어 feedback loop 발생 | contribution source와 track sizing stage를 분리 기록 |
| 8 | span을 단순 평균 분배 | 3-span, 기존 큰 track, gap, 다른 intrinsic 함수가 있는 경우 잘못 동일 분배; flex track을 span한 item의 automatic minimum 예외가 누락됨 | span length/function group별 normative stage; 48px+48px flex-track oracle, span3/multiple span/mixed tracks controls |
| 9 | gap을 두 번 계산 | gap이 span contribution에도 들어가고 track free-space에서도 차감됨 | span oracle에 gap4, 각 단계 track 합과 container width를 각각 검사 |
| 10 | track ordering 누락 | 같은 span 길이의 item 순서 변경이 결과를 바꾸거나 결과를 정렬로 우연히 맞춤 | item-order permutation을 implementation control로 고정하고 표준 순서·함수 그룹을 검사 |
| 11 | cyclic percentage 누출 | indefinite 50% 최종 12px이 intrinsic pass에서 contribution으로 사용됨 | 24px max-content oracle와 12px 최종 track·24px overflow를 동시에 검사 |
| 12 | auto-repeat 문법 과다 허용 | repeat 안에 `auto`, `fr` 등 non-fixed track이 포함되거나 한 축에 두 repeat가 있음 | 반복 전 typed syntax validation; 허용 fixed-size만; unsupported는 명시 오류 |
| 13 | auto-repeat count 경계 | gap 포함 횟수에서 1개가 빠지거나 zero/overflow count가 발생 | definite preferred/max/min 순서와 gap을 독립 관찰, WPT 고정 후보 |
| 14 | indefinite auto-repeat 잘못 계산 | preferred/max/min basis가 모두 indefinite인데 viewport width를 읽어 여러 repeat를 생성 | 세 basis가 모두 indefinite이면 한 번 반복; viewport 의존 금지 |
| 15 | auto-fill과 auto-fit 동일 처리 | 빈 repeated track이 남거나 채워진 track도 collapse됨 | auto-fill 3-track, auto-fit 100/0/0, all-empty 0/0/0 분리 |
| 16 | span-occupied track을 empty로 오판 | item은 여러 track을 차지하지만 시작 track만 occupied로 표시 | placement 이후 span이 걸치는 각 repeated track의 occupancy를 확인 |
| 17 | collapsed track 뒤 gutter 유지 | 빈 auto-fit track은 0px지만 주변 gap이 남아 전체 폭이 초과 | repeated track과 인접 gutter collapse를 separately record; 전체 합 검사 |
| 18 | implicit sizing 누락 | explicit track만 새 타입이고 implicit `grid-auto-rows/columns`는 fixed parser로 남음 | positive implicit track을 B단계에 포함; auto-repeat 및 explicit/implicit interaction controls |
| 19 | text/image contribution 손상 | min/max query가 definite snapshot을 덮거나 padding/border·min/max-width·natural ratio/revision을 무시 | Ahem 80px outer min, min-width 90px/max-width 40px, C14.3 source/generation checks와 회귀 |
| 20 | 실패·화면 경계 은폐 | 일부 item 오류 뒤 partial frames 게시, 12px border 밖 overflow를 안쪽 sample만으로 통과 | error/output atomicity, 10,000-track/allocation failure, 바깥 좌표·표면 sample, AVD+iOS simulator 구분 |

### 검토 뒤 고정한 주의점

- Chrome oracle 26개는 C11.4 전체를 덮지 않으므로 span ordering, transferred-size suggestion, invalid input, allocation failure controls를 implementation 전에 더한다.
- WPT revision과 각 파일 hash를 보존한다. upstream test runner가 없으므로 WPT pass를 주장하지 않는다.
- C11.4는 C14.3 image와 C15 text intrinsic inputs를 함께 요구한다. 특히 현재 text callback의 non-definite-width error를 0으로 바꾸어 회피하지 않는다.
- 계획 PR merge, B단계 통과, auto-repeat phase 통과는 서로 다른 gate다. C11.4/C11 parent는 모든 해당 범위가 실제 실행 근거로 닫힐 때까지 미완료다.
