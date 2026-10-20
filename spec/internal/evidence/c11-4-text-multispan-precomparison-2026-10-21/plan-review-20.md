# C11.4 direct-leaf text multi-span 계획 적대 검토 20개

검토 대상은 [집중 계획](../../../../plan/c11-4-text-multispan-contributions.md), [Chrome fixture](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-text-multispan-precomparison.html), [Chrome observation](./chrome155-observation.json), [사전 비교 화면](./chrome155-precomparison.png)과 공식 CSS Grid §11.5.1이다. 구현 전 서로 다른 실패 경로 20개를 검토했다.

| # | 실패 경로 | 계획의 차단 기준 |
| ---: | --- | --- |
| 1 | 구현 결과를 보고 기대 track을 역산 | CSS Grid §11.5.1에서 ideal 기대를 도출하고 코드 구현 전에 고정한다. |
| 2 | Ahem font가 늦게 로드돼 fallback 측정값 사용 | `document.fonts.ready`와 `document.fonts.check('10px Ahem')`가 true일 때만 캡처한다. |
| 3 | Ahem 텍스트의 min-content 폭 가정이 틀림 | min/max-content probe를 같은 Chrome run에서 저장한다: 50/80, 70/120, 180/180px. |
| 4 | `min-content` track과 `min-content` item contribution을 혼동 | track sizing expectation과 item intrinsic widths를 별도 JSON 필드 및 Rust assertion으로 다룬다. |
| 5 | Chrome used-value serialization을 규범 기대값으로 오인 | ideal thirds와 1/64px로 양자화된 Chrome 문자열을 별도 보존한다. |
| 6 | fractional track 차이를 엄격한 문자열 비교로 거부 | runtime은 root-relative numeric comparison과 최대 1/64 CSS px 허용치를 사용한다. |
| 7 | source order 변경과 item content 변경이 함께 일어남 | forward/reverse pair는 text·span·row가 같고 DOM 순서만 다르다. |
| 8 | span-3이 span-2보다 먼저 계산됨 | 두 mixed scene 모두 trace stage가 `[2,3]`인지 확인한다. |
| 9 | span 밖의 마지막 track이 기여를 받음 | 네 번째 track을 두 expectation 모두 0px로 고정한다. |
| 10 | column 결과가 row sizing 변화에 가려짐 | 20px fixed row 3개와 서로 다른 row placement를 사용한다. |
| 11 | row gap이 track 값에 포함됨 | 모든 gap은 0으로 고정한다. |
| 12 | `align-content:start`가 fixture profile에서 거부되거나 정렬 결과를 바꿈 | profile과 맞게 `align-items:start`만 지정하고 `align-content`는 normal 기본값으로 둔다. |
| 13 | grid item의 text가 anonymous Grid item으로 취급됨 | 이번 input은 root direct child `div`에 직접 포함된 text로 제한하고 anonymous root text는 범위 밖으로 둔다. |
| 14 | 한 item의 여러 text node를 잘못 하나로 합침 | item당 non-empty text node 하나를 exact topology로 검증한다. |
| 15 | nested item 또는 inline descendant가 무심코 허용됨 | root direct child와 text ownership을 exact allowlist로 검사하고 다른 topology는 거부한다. |
| 16 | 서로 다른 paragraph에 같은 item ID/style이 적용됨 | 각 item node ID를 paragraph-style map의 별도 key로 사용한다. |
| 17 | node order 검사에서 ID와 frame 대응이 뒤섞임 | Chrome, Android, iOS에서 각 item ID와 frame을 개별 비교한다. |
| 18 | viewport/page offset이 결과에 섞임 | root frame을 기준으로 item의 x/y를 상대화한다. |
| 19 | fixture 결과가 제품 CSS 지원으로 과장됨 | 전용 fixture-test feature를 사용하고 C11.4 checkbox/완료 수/계약 버전을 그대로 둔다. |
| 20 | 한 플랫폼만 통과했는데 전체 검증으로 보고 | Android API 37 AVD, SM-S731N 실기기, iPhone 17 Pro Simulator 결과를 각각 분리 기록한다. |

계획의 blocker는 발견되지 않았다. Chrome observation과 기기 matrix가 선행 조건이며 아직 구현 근거로 계산하지 않는다.
