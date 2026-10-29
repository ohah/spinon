# C11.4 direct text first-baseline 계획 적대 검토 20개

**대상:** [집중 계획](./c11-4-grid-text-first-baseline.md), [Agent Browser 사전 비교](../spec/internal/evidence/c11-4-grid-text-first-baseline-precomparison-2026-10-30/README.md), 고정 HTML fixture. 구현 검토가 아니다.

| # | 계획을 깨뜨릴 수 있는 입력·해석 | 판정 근거 | 결과 |
|---:|---|---|---|
| 1 | 한 item만 baseline-aligned여서 shim 계산이 no-op인데도 8px 보정을 기대한다. | 모든 positive scene은 같은 row에서 first baseline에 참여하는 direct text item이 두 개다. | PASS |
| 2 | item이 다른 row에 배치되어 baseline group을 공유하지 않는다. | fixture는 한 `auto` row와 두 `auto` columns를 고정하며 scene JSON이 공통 row를 검사한다. | PASS |
| 3 | spanning item이 두 row 사이에서 예상하지 못한 baseline group 선택을 일으킨다. | 양 item의 row span은 1이고 spanning scene은 명시적으로 제외했다. | PASS |
| 4 | `align-items`가 가로 축만 조정하는 것으로 잘못 해석된다. | 부모는 `horizontal-tb`; Grid block axis와 단일 row의 block-start가 baseline shim 축이다. | PASS |
| 5 | first baseline 계산이 last baseline 또는 font descent를 사용한다. | scope를 first baseline으로 제한하고 Ahem ascent 800/1000 및 고정된 line metrics로 독립 계산했다. | PASS |
| 6 | UA 기본 `line-height:normal`이 플랫폼별 line box 차이를 넣는다. | 두 text item에 `line-height:1`을 명시하고 CSS computed line-height를 관찰한다. | PASS |
| 7 | 폰트 미로드/fallback 뒤 다른 glyph ascent로 기대값을 틀린다. | 저장소 Ahem 파일 SHA를 고정하고 browser font load를 확인했다. runtime도 동일 font registry 입력을 검사해야 한다. | PASS |
| 8 | baseline offsets를 Chrome에서 역산해 규범 계산인 것처럼 기록한다. | expected JSON은 font metrics와 Grid shim 공식으로 계산하고 Chrome observation은 별도 산출물이다. | PASS |
| 9 | Ahem을 “XX”가 아닌 글자에 사용해 폭/advance가 달라진다. | 모든 positive/control은 정확히 두 `X`로 고정했고 grid columns도 20px/40px로 검사한다. | PASS |
| 10 | min-content baseline query가 soft-wrap된 다른 line을 반환한다. | 각 item은 `white-space:nowrap`이고 한 단락·한 line profile만 허용한다. | PASS |
| 11 | Grid row가 baseline shim을 intrinsic contribution에 포함하지 않는다. | 독립 계산은 `max(10+8,20+0)=20px`; runtime plan은 Rows stage track과 item frames를 함께 검사한다. | PASS |
| 12 | margin이 baseline origin에 더해져 item y 기대값을 바꾼다. | margin은 0으로 고정하고 item margin·baseline shim은 서로 다른 값으로 trace한다. | PASS |
| 13 | border/padding 때문에 text baseline과 border-box baseline의 원점이 다르다. | 이 slice는 nonzero border/padding을 제외하고 baseline 좌표를 line-top/item origin에 그대로 대응한다. | PASS |
| 14 | `align-self`가 부모 `align-items`보다 우선하는 규칙을 검사하지 않는다. | 두 번째 positive scene은 부모 `start`와 각 child의 `first baseline`을 사용한다. | PASS |
| 15 | baseline 전달 변경이 일반 start alignment까지 이동시킨다. | 동일 입력의 `align-items:start` control은 두 item y=0, 서로 다른 glyph baseline을 고정한다. | PASS |
| 16 | source order가 baseline group 최대값이나 shim을 바꾼다. | 역전 scene은 column 순서만 바꾸며 baseline y=16 및 small/large y=8/0을 고정한다. | PASS |
| 17 | Taffy의 definite-size leaf early return이 text 측정 없이 baseline을 생략한다. | 계획 입력은 auto inline/block size와 no edge로 제한하고, implementation test는 baseline query 경로가 실제로 output baseline을 채우는지 확인한다. | PASS |
| 18 | missing paragraph/font/line에서 height fallback을 성공으로 통과시킨다. | 계획은 실패 주입과 unsupported 입력의 partial frame 부재를 요구하며 success matrix는 font revision을 검사한다. | PASS |
| 19 | Chrome screenshot/geometry가 Spinon Android/iOS 결과로 오인된다. | 사전 비교에는 Chrome-only 표기를 하고 구현 PR에서 별도 Android AVD·iOS Simulator 캡처와 pixel 계층을 기록한다. | PASS |
| 20 | opt-in FFI fixture가 기본 artifact에 노출되거나 내부 계약 숫자/공식 완료 체크가 과승격된다. | C11.4 전용 fixture-test feature와 기본/positive-control archive 비교를 구현 gate에 둔다. 계약 `0.1.0` 유지, STATUS/roadmap narrative만 갱신하고 C11.4/C11 체크는 미완료로 남긴다. | PASS |

20개의 서로 다른 입력·계약 경계를 검토했다. 발견 사항은 계획과 fixture에 반영되어 구현 전 기준이 고정됐다.
