# C11.4 repeated intrinsic text content · implementation/runtime review 20

**대상:** `RootHtmlFitContentAutoRowsV1` 제한 텍스트 grammar, Blink 1/64 CSS px row distribution normalization, 세 Chrome 157 corpus의 810-scene RuntimeSession matrix.

| # | 실패 관점 | 확인한 방어 또는 근거 | 결과 |
|---:|---|---|---|
| 1 | UTF-8 byte 길이와 Unicode 문자 수 혼동 | 대문자 ASCII 외 byte를 모두 거부하고 길이는 `as_bytes().len()`으로 판정한다. | 통과 |
| 2 | 빈 문자열이나 공백만 있는 payload admission | grammar는 마지막 단어 길이가 0보다 커야 통과하며 RuntimeSession empty-text mutation을 거부한다. | 통과 |
| 3 | 1자·6자·7자 단어 경계 | 각 단어 길이를 검사하며 1·6자 허용과 7자 거부 단위 검사가 있다. | 통과 |
| 4 | 전체 32·33 byte 경계 | 32 byte 경계 corpus와 RuntimeSession 33 byte rejection을 사용한다. | 통과 |
| 5 | 선행·후행·중복 공백 정규화 | 단어가 비어 있을 때 ASCII space를 거부한다. 세 mutation을 RuntimeSession에서 검사한다. | 통과 |
| 6 | 탭·개행 또는 다른 공백 문자의 CSS collapse 우회 | ASCII space 외의 모든 byte를 거부하고 tab/newline mutation을 검사한다. | 통과 |
| 7 | 대소문자·숫자·문장부호·비 ASCII 유입 | `A..Z` 범위만 허용하며 grammar 단위 검사에서 각 문자군을 거부한다. | 통과 |
| 8 | 여러 TextSource를 하나로 합쳐 profile 우회 | Layout paragraph source가 정확히 하나인지 확인하고, source tree에서는 단일 direct Text child만 허용한다. | 통과 |
| 9 | inline ancestor/run style에 의한 glyph 기여 변화 | 기존 empty-inline 조건을 유지하며 nested inline mutation을 거부한다. | 통과 |
| 10 | font·크기·weight·line-height 차이가 row 기여를 바꿈 | exact typography gate를 유지하며 font family/size/weight/line-height와 nowrap negative control을 둔다. | 통과 |
| 11 | 임의 topology 또는 style gate를 우회 | bounded validator의 단일 Grid·고정 column/explicit row·9 item 및 payload 구조 조건을 유지한다. | 통과 |
| 12 | 새 grammar가 #1290의 여덟 입력을 깨뜨림 | `legacy-1290` corpus를 기존 layout/style/runtime 경로로 계속 실행한다. | 통과 |
| 13 | 새 ASCII 길이 입력이 기존 문자열 allowlist에서만 통과 | `varied-ascii`와 32-byte corpus의 HTML source 및 expected text descriptor를 대조한다. | 통과 |
| 14 | 한 corpus의 30 tail 순서 일부 누락 | 각 corpus에서 30개 distinct tail을 확인하고 순서마다 8개 220px 및 1개 100px scene을 검사한다. | 통과 |
| 15 | cap/alignment 조합 또는 장면 수 누락 | corpus별 270 scene을 고정하고 tail마다 네 cap × 두 alignment와 overflow 장면을 요구한다. | 통과 |
| 16 | text glyph 수나 line 수의 잘못된 하드코딩 | 각 descriptor의 ASCII 글자 수와 Chrome Range line count를 실제 RuntimeSession glyph에 대조한다. | 통과 |
| 17 | Grid 밖 overflow geometry가 row 검사만으로 가려짐 | 100px scene마다 실제 text payload bottom이 Grid bottom보다 0.01px 넘는지 단언한다. | 통과 |
| 18 | 절대 row boundary별 floor에서 잔여 1/64 단위가 잘못된 row에 감 | 첫 후보가 1,176 frame component에서 허용치 밖으로 나간 것을 보존하고, track trace growth-potential 순 잔여 분배 뒤 810/810을 통과시켰다. | 통과 |
| 19 | normalization 전에 trace를 버리거나 일반 route에 비용 추가 | `compute`가 intrinsic inputs와 trace를 수집한 뒤 보정하고, style projection은 반복 intrinsic six-member profile에서만 trace를 요청한다. | 통과 |
| 20 | 거부 중 부분 output 누출 또는 session 복구 실패 | mutation마다 failed/no-completed-output을 확인하고 같은 session의 dark/light environment 거부·복구 테스트를 실행한다. | 통과 |

## 실행 결과와 남은 검증

`cargo test --locked -p spinon-runtime parsed_root_html_repeated_intrinsic_text_ -- --nocapture`는 3/3 통과했다. 포함된 810-scene strict geometry matrix의 허용치는 각 frame component ±0.01 CSS px다. `cargo test --locked -p spinon-layout --lib track_sizing -- --nocapture`는 127/127 통과했고 grammar 단위 검사는 Layout과 Style projection에서 각각 2/2 통과했다. 변경 3 crate Clippy `-D warnings`, `cargo fmt --all -- --check`, `git diff --check`도 통과했다.

전체 `spinon-style-to-layout --lib` suite는 C07.3 aspect-ratio computed-value 비교에서 `1.7777778 / 1`과 pinned 기대값 `1.77778 / 1`의 차이로 실패했다. focused 재실행에서도 같은 값이 나왔다. 해당 테스트·aspect-ratio serializer는 변경하지 않았으며 baseline branch 실행은 하지 않아 원인 귀속은 미확정이다. Android/iOS Simulator 실행과 full-suite 결과의 기준 branch 분류가 남아 있다. 이 검토는 PR 변경 diff의 별도 검토를 대신하지 않는다.
