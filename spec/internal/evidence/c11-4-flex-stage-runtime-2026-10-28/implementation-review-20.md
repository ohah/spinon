# 구현·runtime 실패 관점 검토 20개

검토 대상은 고정 scene builder, Android/iOS strict matcher, log reassembly verifier, 두 simulator의 최종 실행이다. 각 항목은 서로 다른 입력·경계 실패를 다룬다. 통과는 이 네 입력에 한정한다.

| # | 공격 가설 | 판정과 근거 |
| ---: | --- | --- |
| 1 | unlisted scene ID가 임의 JavaScript로 그대로 삽입될 수 있다. | **PASS** — Rust builder는 네 ID exact lookup만 제공한다. dispatcher test가 `weighted-columns-unlisted`, unknown 문자열을 거부한다. 양 플랫폼의 앱 unknown-scene route도 argument error를 냈다. |
| 2 | Android/iOS가 Rust와 다른 scene 목록을 받아 fixture 요청을 정상으로 오인할 수 있다. | **PASS** — 두 플랫폼의 allowlist에 네 ID를 추가했고, `verify.py`가 JSON·Java·Swift track table의 정확한 ID 집합을 비교한다. |
| 3 | Rows scene에서 target axis와 고정 control 축을 바꿔 검증할 수 있다. | **PASS** — matcher는 scene ID의 축 표에서 target을 고르고, trace의 `sequence`/`axis` 두 pass를 함께 확인한다. 네 scene 모두 통과했다. |
| 4 | target-axis CSS gap이 반대 축에 설정되거나 0 gap으로 빠질 수 있다. | **PASS** — Rust source test에서 두 gap 배치를 확인하고 runtime trace는 내부 gutter `10px`, 경계 gutter `0px`를 검사한다. |
| 5 | auto-placement가 두 번째 marker에 implicit column/row를 만들어 expected track topology를 우회한다. | **PASS** — 두 marker 모두 `grid-column`과 `grid-row`를 명시한다. trace는 target axis 다섯 internal entry와 고정 반대축 세 entry만 허용한다. |
| 6 | marker 내용·intrinsic minimum이 대상 track base를 올려 §11.7 결과를 가장할 수 있다. | **PASS** — marker는 빈 `div`, `min-width:0;min-height:0`, padding/border 0이며 text/image를 만들지 않는다. source test가 node 수와 text 미사용을 검사한다. 양 플랫폼 track/frame 값이 독립 JSON과 맞았다. |
| 7 | root padding, border, margin 때문에 200px definite available space가 달라질 수 있다. | **PASS** — root는 `border-box`, margin/padding/border 0이며 target sizing axis를 200px, 반대축을 120px로 고정한다. 앱 summary와 세 node frame이 두 플랫폼에서 일치했다. |
| 8 | 첫 pass나 배열 인덱스를 무조건 flexible axis로 사용하면 다른 scene 축이 검사되지 않는다. | **PASS** — Android/iOS matcher는 `axis` 값으로 target pass를 선택하고 두 axis pass 순서도 확인한다. |
| 9 | trace가 없거나 여러 grid trace 중 하나만 우연히 맞아 통과할 수 있다. | **PASS** — matcher와 Python verifier 모두 trace 개수가 정확히 1, grid node가 1인지 요구한다. verifier의 오염 JSON/누락 축 대조도 거부했다. |
| 10 | `ExpandFlexibleTracks`가 누락·중복되거나 인접한 다른 stage로 바뀌어도 통과할 수 있다. | **PASS** — target pass에서 `Initialize < MaximizeTracks < ExpandFlexibleTracks < StretchAutoTracks`와 네 stage의 유일성을 검증한다. Android matcher도 `Initialize` 순서 검사를 추가했고 독립 verifier가 초기화·최대화 역순을 음성 대조로 거부한다. Expand 누락 대조도 거부했다. |
| 11 | Columns/Rows `sequence`가 뒤바뀌거나 같은 axis가 두 번 실행될 수 있다. | **PASS** — 두 pass의 정확한 sequence `[0,1]`과 axis `[Columns,Rows]`를 확인하고, target stage가 선언된 축에 있는지 별도로 검사한다. |
| 12 | internal gutter를 CSS track으로 세거나 gap을 한 번 더 더할 수 있다. | **PASS** — track마다 `gutter` boolean을 읽어 두 CSS base와 한 internal gutter를 분리하고, 10px gap 및 boundary sentinel을 확인한다. |
| 13 | 내부 track index 누락·중복 또는 implicit track이 고정 수량 검사에서 빠질 수 있다. | **PASS** — target trace는 5개, 고정축 trace는 3개를 요구하며 각 `internalIndex`, gutter, collapsed flag를 비교한다. |
| 14 | `weighted-columns`에서 1fr:2fr 비율이 틀려도 전체 너비만 맞으면 통과할 수 있다. | **PASS** — 두 개별 base를 독립 유리수 `[190/3,380/3]`와 `1e-4 CSS px`로 비교한다. 두 플랫폼은 각각 `63.333332`/`126.666664`를 기록했다. |
| 15 | 최소값 동결 뒤 나머지 flex track을 다시 계산하지 않는 오류가 가려질 수 있다. | **PASS** — `frozen-row`는 독립 기대값 `[80,110]` 및 y/frame `[0,80]`, `[90,110]`을 stage와 최종 geometry 양쪽에서 대조한다. |
| 16 | 1 미만 flex factor를 1로 취급하지 않아 남은 공간을 잃을 수 있다. | **PASS** — `subunit-columns`는 `[85,20]` 및 남은 공간 85px를 계산하고 trace와 frame에서 검사했다. |
| 17 | `0fr`가 zero minimum으로 축소되어 24px base floor를 잃을 수 있다. | **PASS** — `zero-fr-row`의 target track bases와 item frames가 `[24,166]`, `[0,24]`/`[34,166]`으로 Android/iOS에서 모두 일치했다. |
| 18 | NaN/Infinity 또는 숫자가 아닌 track value가 tolerance 비교를 우회할 수 있다. | **PASS** — 플랫폼 matcher와 Python verifier는 finite base/frame만 허용한다. verifier의 non-finite track 대조가 거부됐다. growthLimit의 내부 무한 의미는 이 작업의 판정에 사용하지 않는다. |
| 19 | layout 결과만 있고 실제 draw/presentation은 일어나지 않아 screenshot이 이전 화면일 수 있다. | **PASS** — run ID별 summary·node frame·trace chunk와 `SPINON_C0410_DRAW ... presented boxes=3`를 확인하고, foreground 앱에서 screenshot을 저장했다. 8개 screenshot의 PNG signature와 SHA-256도 검증했다. |
| 20 | 실기기나 다른 boot 상태를 바꾸고도 AVD/Simulator 결과로 표시할 수 있다. | **PASS** — Android runner는 `emulator-*`만 허용하고 API 37, `qemu=1`, ARM64, 16 KB를 확인했다. iOS runner는 기존 boot 상태를 기록하며 이번 run에서 기기를 boot/shutdown하지 않았고 앱만 종료했다. ADB의 실기기 serial에는 명령을 보내지 않았다. |

## 남은 범위

검토 결과는 definite 200px sizing axis, 120px 반대축, 단일 10px target gap, 빈 두 marker를 쓰는 네 scene에만 적용된다. Intrinsic item contribution, spans, indefinite free space, auto-repeat·implicit track 조합, 일반 author CSS admission, 전체 WPT, 실기기와 hardware GPU는 이 작업에서 통과로 판정하지 않는다. Screenshot은 compositor 이후 pixel equality 근거가 아니다.
