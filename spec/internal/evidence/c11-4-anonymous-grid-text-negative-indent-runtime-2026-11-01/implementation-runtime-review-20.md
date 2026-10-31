# Negative `text-indent` 구현·runtime 실패 관점 검토 20개

계획 검토 20개와 PR diff 검토 30개를 재사용하지 않고, 새 allowlist·계산 기대값·Android/iOS runtime 결과를 각각 공격적으로 확인했다.

| # | 실패 관점 | 확인 및 결과 |
|---:|---|---|
| 1 | fixture-test feature가 꺼진 기본 runtime에 scene이 들어가는가 | 기본 release archive에서 30 scene 및 두 validator/adapter symbol이 모두 absent. 통과. |
| 2 | 값 하나가 scene 이름 검사 없이 일반 CSS admission으로 번지는가 | expected scene ID와 computed serialization을 함께 검증한다. 미등록 scene은 layout 전에 거부. 통과. |
| 3 | 기존 18개 회귀 scene이 새 값 확장 중 삭제·변경되는가 | 18 positive와 12 negative scene을 합쳐 30개 고유 ID를 검사했다. 통과. |
| 4 | 범위 밖 `-7px`가 수치 범위만으로 허용되는가 | exact scene/value mismatch를 거부하는 runtime test 통과. |
| 5 | 위험 경계 `-30px`가 구현 allowlist에 들어오는가 | `-30px` mismatch가 layout publication 전에 거부되고 browser evidence에만 남는다. 통과. |
| 6 | 음수 percentage나 다른 단위가 px로 잘못 읽히는가 | computed string은 exact `px` serialization과 fixed scene 값에 묶인다. percentage control 거부. 통과. |
| 7 | `hanging` 또는 `each-line`이 단순 길이로 잘리는가 | strict px parser와 unsupported suffix test가 해당 값을 거부한다. 통과. |
| 8 | `-0px`, `-5.0px`, 공백 또는 `+` 표기가 canonical 값으로 받아들여지는가 | exact serialization 비교로 expected `-5px`/`-10px` 외 표기를 거부한다. 통과. |
| 9 | 비유한 값이나 range 바깥 값이 typed `f32`로 넘어가는가 | finite/range parser와 NaN·inf·-11px controls 확인. 통과. |
| 10 | 다른 document/style revision의 computed 값이 재사용되는가 | source HostDocument revision을 layout 계산 결과와 비교하는 30 scene runtime 검증 통과. |
| 11 | 익명 text가 아닌 node에 indent가 전달되는가 | 기존 direct-text source identity와 child allowlist를 그대로 사용한다. runtime matrix 통과. |
| 12 | Ahem font/size 변화가 계산된 상수를 우연히 통과시키는가 | Android/iOS expected style과 trace에서 Ahem 10px을 고정한다. matrix 통과. |
| 13 | positive indent의 기존 min-content 공식이 negative 값에도 무조건 적용되는가 | `max(first word + d, later word widths)`로 독립 계산하고 긴 첫 단어 장면 통과. |
| 14 | 짧은 첫 단어의 음수 indent가 더 긴 뒤 단어 contribution을 잘라내는가 | `X XXX`에서 later 30px word가 floor가 되는 4장면 모두 30px min-content 통과. |
| 15 | collapsed separator를 빠뜨리거나 indent를 max-content에 여러 번 더하는가 | 두 단어 Ahem 50px unindented max-content에 d를 한 번 반영하고 4개 장면 통과. |
| 16 | separator 없는 한 단어에서 min/max-content가 불필요하게 달라지는가 | `XX`의 min/max 15px 또는 10px track을 양 플랫폼과 Rust에서 확인. 통과. |
| 17 | indent를 line advance에 포함해 뒤 line까지 이동시키는가 | 각 line의 unindented advance와 first glyph typed offset을 Rust에서 따로 검사. 통과. |
| 18 | min-content wrapping으로 row 높이 또는 max-content single-line 높이가 바뀌는가 | 두 word min-content 20px, max-content 10px, single word 10px을 세 경로에서 확인. 통과. |
| 19 | Android 기대값 array/viewport에서 max-content advance를 첫 단어 폭으로 오인하는가 | PR diff 검토에서 max-content line의 기대값이 첫 단어 폭인 결함을 찾아 전체 unindented max-content advance로 수정했다. static source assertion, fixture app 재빌드와 새 APK hash를 확인한 뒤 API 37 AVD 30/30을 재실행했다. min-content line advance는 첫째/둘째 line별로 유지한다. |
| 20 | iOS adapter가 새 text 순서·두 줄 advance·track을 다른 Android식 가정으로 검증하는가 | scene text, min/max values, per-line advances, root/item frame 및 Columns/Rows를 독립 검사. iOS 30/30 통과. |

기존 구현 검토에서 Android expectation array의 `minContentHeight` slot 누락과 C11.4 fixture의 environment scale 덮어쓰기를 발견해 수정했고, 양 플랫폼 matrix를 통과시켰다. 별도의 PR diff 검토에서는 Android max-content line advance 기대값 결함을 발견해 고쳤다. 영향받은 static 검사와 Android build/capture를 반복했고, 수정된 최종 Android 30/30과 기존 iOS Simulator 30/30을 근거로 유지한다. 검증 한계는 [runtime evidence](README.md)에 기록한다.
