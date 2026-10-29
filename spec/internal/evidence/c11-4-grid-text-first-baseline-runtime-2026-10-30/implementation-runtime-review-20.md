# C11.4 direct text first-baseline 구현/runtime 적대 검토 20개

**대상:** baseline 전달 코드, 고정 runtime fixture, Android/iOS Simulator 실행 및 실행 근거. 계획 검토와 별개로 실제 변경과 관찰값을 기준으로 확인했다.

| # | 공격 관점 | 확인한 근거와 판정 |
|---:|---|---|
| 1 | 기본 feature에서도 baseline metadata가 일반 Grid layout에 영향을 준다. | `GridItemIntrinsicMetadata`의 두 새 필드와 baseline 전달부가 `c11-4-grid-track-sizing-fixture-test`로 cfg 제한된다. Product archive와 positive control 비교에서도 named fixture symbol/scene IDs가 각각 제외/포함됐다. **PASS** |
| 2 | 기존 projection API가 새 baseline admission을 묵시적으로 얻는다. | 일반 entrypoint들은 `allow_c11_4_grid_text_first_baseline_fixture: false`를 넘긴다. `ordinary_track_sizing_projection_keeps_grid_first_baseline_closed`도 일반 wrapper에서 거부를 확인한다. **PASS** |
| 3 | 범용 Grid 스타일이 고정 fixture opt-in을 빌려 baseline을 통과한다. | allow 경로는 `RuntimeBlockFlexGridTrackSizingV1`에 한정하고 runtime 특수 분기는 네 고정 root ID recognizer와 C11.4 text input이 모두 있을 때만 선택한다. **PASS** |
| 4 | Stylo의 `baseline` alias가 fixture 밖까지 받아들여진다. | `baseline`/`first baseline` alias는 전용 option 분기에서만 허용된다. 기본 projection 회귀 테스트는 alias 두 형태를 모두 거부하도록 확인한다. **PASS** |
| 5 | Flex baseline이나 `align-content` baseline으로 잘못 분기한다. | 변경은 Grid `align-items`/Grid item `align-self`의 고정 text fixture 경로에만 적용되고, 다른 속성은 기존 rejection에 남는다. **PASS** |
| 6 | SVG 또는 foreign namespace root가 같은 문자열 ID만으로 실행된다. | runtime recognizer 이후 `validate_root`가 HTML namespace, computed profile, static position을 검사한다. **PASS** |
| 7 | 다른 Grid track profile, repeat, named line, template area, intrinsic dimension이 고정 결과를 우회한다. | root 검증은 2 auto columns, 1 auto row, row auto-flow, repeat/template-area/named-line 부재, max-content width, auto height, 8px column gap을 확인한다. **PASS** |
| 8 | 0·1·3개 direct Grid item 또는 익명 non-whitespace text가 baseline group을 바꾼다. | runtime adapter는 정확히 두 element item만 받고, 익명 일반 text는 거부한다. HTML collapsible whitespace만 건너뛴다. **PASS** |
| 9 | 중첩 descendants나 바뀐 문자열을 direct Ahem leaf처럼 취급한다. | 각 item의 meaningful text, 직접 Text source, 정확한 `XX`, leaf 여부를 확인하고, 중첩 inline/text 구조는 고정 profile 밖으로 거부한다. **PASS** |
| 10 | 다른 font, font size, weight, style 또는 line-height가 browser별 baseline 변동을 숨긴다. | 저장소 Ahem을 system font 없이 등록하고 10px/20px 각각 `line-height` 일치, normal weight/style을 검사한다. 근거 README에 Ahem SHA와 trace의 `fontRegistryRevision: 1`을 함께 기록했다. **PASS** |
| 11 | 두 item의 크기가 같거나 두 item 모두 같은 크기라 열/row 기대값이 우연히 통과한다. | adapter는 정확히 하나의 10px와 하나의 20px item만 허용하며 trace는 각 text layout과 `[20,40]` 또는 reversed `[40,20]` tracks를 검사한다. **PASS** |
| 12 | 부모 `align-items`와 자식 `align-self` 우선순위가 뒤바뀐다. | 네 scene은 부모 first-baseline, start control, 부모 start+양쪽 first-baseline self, reversed source order를 각각 고정한다. native frame과 Rust frame assertions가 모두 연결된다. **PASS** |
| 13 | start control에 baseline 값을 전달해도 대조 결과가 그대로 정렬된다. | `start` scene은 item y=0/0으로 검사하며 text snapshot은 서로 다른 8px/16px line baselines를 유지한다. **PASS** |
| 14 | 여러 line에서 first baseline을 고르는 로직이 두 번째 줄이나 마지막 baseline으로 바뀐다. | 전달 helper는 정확히 한 줄만 허용한다. multiline 입력 회귀 테스트는 구체적인 `UnsupportedTextParagraph` 오류를 요구한다. **PASS** |
| 15 | border/padding/margin 때문에 text baseline 원점과 Taffy border-box 원점이 다르다. | helper는 item box edges가 전부 0이 아니면 거부한다. nonzero padding 회귀 테스트가 오류 경로를 확인한다. **PASS** |
| 16 | vertical writing mode, 잘못된 폭 또는 NaN/범위 밖 baseline이 layout 결과로 유출된다. | helper는 horizontal-tb, 유한 비음수 폭, 유한 baseline 및 `[0, snapshot.height]` 범위를 확인한다. 이 값 주입은 단위 테스트로 직접 만들지 않고 해당 방어 분기를 코드 검토했다. **PASS, 직접 주입 미검증** |
| 17 | cached text result와 새 baseline 조회가 다른 width/layout을 사용한다. | helper는 실제 Taffy `output.size.width`를 normalized-bit cache key로 사용하고, 같은 text engine/cache를 먼저 확인한 뒤에만 새 layout을 계산한다. **PASS** |
| 18 | missing paragraph/font engine에서 높이를 baseline으로 대체하거나 부분 output을 성공으로 낸다. | missing paragraph는 no result로 조용히 통과하지 않고 고정 leaf/text precondition에서 차단되며, 필요한 engine 부재는 `MissingTextLayoutEngine` 오류, text layout 실패는 propagated error다. missing-engine 주입 실행은 하지 않았다. **PASS, 직접 주입 미검증** |
| 19 | baseline 값이 비정렬 item이나 다른 layout modes까지 전파된다. | `LayoutOutput.baselines.first`는 track-sizing metadata와 first-baseline alignment가 모두 참일 때만 기록한다. 나머지 Grid child는 기존 Taffy output을 그대로 둔다. **PASS** |
| 20 | Simulator 증거가 실기기·hardware GPU·glyph paint 또는 iOS compositor 이후 pixels로 오인된다. | Android runner는 `qemu=1`, API 37, ARM64와 APK hash 일치를 단언한다. iOS runner는 iPhone 17 Pro/iOS 26.2, executable hash 일치를 확인한다. 각 플랫폼에서 4 scene, draw/present, root/item fill·gap 표본 12/12 RGBA를 검사하고 `finally`에서 앱을 종료한다. README는 AVD/QEMU, glyph-paint 미검증, Metal pre-compositor 경계를 구분한다. **PASS** |

결과는 **20개 검토 항목, 해결되지 않은 동작 결함 0개**다. NaN baseline·missing engine 분기는 코드 guard를 검토했지만 현재 adapter에서 강제 주입하는 runtime test는 만들지 않았다. Simulator surface sample은 text glyph paint가 아니라 root/item fill과 gap을 확인한다. 실행 결과는 고정된 두 simulator 환경과 이 제한 fixture에만 해당하며 C11.4/C11 전체 완료를 뜻하지 않는다.
