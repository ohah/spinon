# C11.4 anonymous Grid text `word-spacing` 구현·runtime 실패 관점 검토

## 검토 범위

이 검토는 `ca0627b89c2a76b82347627dceccbc6e0bd84872` 위 현재 후보의 새 Rust 경로, 기존 `ParagraphStyle` 호출부 조정, Android/iOS fixture host, 24-scene runtime evidence를 대상으로 한다. 사전 계획 검토를 재사용하지 않았다.

## 독립 실패 관점

1. **`normal` CSSOM 정규화** — Stylo가 `normal`을 `0px`로 계산하는지 확인했다. 6개 `normal` 장면의 computed style 및 simulator trace가 `0.0`과 일치한다.
2. **명시적 0의 구분** — authored `0px`도 6개 별도 장면으로 실행하며 `normal`과 동일한 used result를 내되 scene identity는 구분한다. 통과.
3. **장면 바꿔치기** — 새 runtime adapter는 24개 고정 ID와 각 expected serialization을 매핑하고, 미등록 scene 및 다른 값은 거부한다. Rust validator negative case 통과.
4. **문자열 형식 우회** — 숫자 parser는 `normal`, `1em`, `NaNpx`, `infpx`, 허용 범위 밖 값을 거부하고, scene validator는 `-0px`, `0.0px`, `+1px`, `1.00px`처럼 값이 같은 비정규 alias도 exact serialization 비교에서 거부한다. 앞선 테스트가 alias 책임을 parser에 잘못 두어 실패했고, validator negative assertions로 경계를 수정한 뒤 word-spacing runtime tests 4/4 통과했다.
5. **비유한 내부 값** — `ParagraphStyle::validate`가 NaN word spacing을 거부하고 기본값은 `0.0`이다. 새 unit test 통과.
6. **기존 text profile 회귀** — 새 field를 추가한 기존 explicit initializer와 root HTML 제한 profile은 모두 `0.0`을 명시한다. focused Rust tests와 Android/iOS 빌드 통과.
7. **Parley 전달 누락** — intrinsic/layout builder가 `StyleProperty::WordSpacing`을 설정하고 layout snapshot에 value를 남긴다. Ahem text unit 및 24-case runtime test 통과.
8. **공백 collapse 전 적용 오류** — `XX   XX XX  `를 사용해 source whitespace 묶음과 trailing whitespace를 포함한다. raw source 보존과 normalized `XX XX XX`가 두 platform trace에서 일치한다.
9. **min-content에 separator 포함 오류** — min-content width는 가장 긴 단어 20px이고 word-spacing 부호와 무관하게 유지된다. 네 value 모두 통과.
10. **max-content separator 누락·중복** — 두 단어와 세 단어 장면에서 남은 separator 한 개 또는 두 개에 `10px + d`를 적용한다. −1/0/+1 기대 폭과 두 platform trace가 exact 일치한다.
11. **공백 없는 대조군** — 단일 `XX`는 spacing 변화로 폭이 바뀌지 않아야 한다. 8개 단일 단어 scene이 20px로 통과한다.
12. **min/max track 혼동** — 각 scene은 정확히 하나의 `min-content` 또는 `max-content` column을 생성하고 final Columns trace를 기대 크기와 비교한다. 48회 통과.
13. **줄 수와 높이 전파** — min-content 줄 수는 단어 수, max-content는 한 줄이며 auto row와 root height를 함께 검사한다. 48회 통과.
14. **text source 소유권** — anonymous direct text는 item과 source node identity가 같다는 예외를 Android/iOS verifier에 반영했다. 실제 trace의 node/text 및 빈 inline-ancestor 목록 검사 통과.
15. **computed style 대상 노드** — runtime adapter는 root node의 `word-spacing` 값을 정확한 scene serialization과 비교한다. 24-scene Rust test와 platform trace에서 통과.
16. **revision 혼합** — layout report의 HostDocument source와 nonnegative style/environment/intrinsic revisions를 검사한다. 48개 trace가 유효하다.
17. **부분·잘린 trace** — Android logcat과 Apple unified log의 chunk 번호·총수·중복을 검사하고 전부 합친 JSON만 parse한다. 각 log가 실패 marker 없이 complete다.
18. **Android scene route 및 gate** — fixture-test `BuildConfig` 조건과 scene route가 연결되고 설치 APK hash가 빌드 APK와 같다. API 37 ARM64 AVD 24/24 통과.
19. **iOS scene route 및 gate** — fixture-test compile define 아래에서 scene expected/validator가 연결되며 설치 executable hash가 빌드 executable과 같다. iPhone 17 Pro / iOS 26.2 Simulator 24/24 통과.
20. **화면 증거 과장** — 두 screenshot은 `layout=ready`를 보이지만 renderer 영역이 검다. glyph paint나 surface pixel 검증으로 해석하지 않도록 실행 README와 PR 범위를 제한했다. 실제 판정은 layout/track trace에 한정한다.

## 판정

검토 중 제품 로직 결함은 발견하지 않았다. 초기 Android stale APK와 두 native host의 anonymous source/frame 기대값 문제는 이 보고서 전에 수정하고 두 플랫폼 전체 24-scene 행렬을 다시 실행했다. 전체 runtime suite에서 기존 anonymous-text mutation test가 이제 word-spacing 전용 validator가 맡는 검사를 옛 typography helper에서 계속 요구하는 것을 찾아 해당 mutation을 새 validator negative test로 옮겼다. 수정 후 text-layout 74/74, runtime 352 passed·3 ignored, FFI 175 passed·2 ignored다. 기본/fixture archive 구분은 별도 evidence에 기록했다. 예외 없이 실행한 Clippy의 단일 경고는 HEAD의 변경되지 않은 `runtime_layout.rs:960`에 이미 있는 `suspicious_else_formatting`; 해당 경고만 command-line에서 제외한 scoped Clippy는 통과했다. 화면 캡처는 glyph 출력에 대한 해결 증거가 아니다.
