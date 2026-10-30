# C11.4 anonymous Grid text `white-space:pre` preserved tab 제한 slice

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [내부 text 계약 0070](../spec/internal/0070-c15-1-inline-text-layout.md) · [CSS Text Level 3](https://www.w3.org/TR/css-text-3/)

**상태:** Agent Browser HeadlessChrome 156 사전 비교와 별도 [계획 실패 관점 20개 검토](./c11-4-anonymous-grid-text-pre-tabs-plan-review-20.md)를 마쳤다. 구현 전 계획이며 코드·runtime 동작이나 C11.4 완료를 주장하지 않는다. 기존 `white-space:pre` slice는 tab-size profile 미정으로 탭을 거부한다. 이번 후보는 CSS Text Level 3의 기본 `tab-size:8`을 Ahem 고정 fixture 경로에서 처리하고, 다른 tab-size와 임의 text profile은 계속 닫아 둔다. Chrome 관측과 독립 기대값은 [사전 비교](../spec/internal/evidence/c11-4-anonymous-grid-text-pre-tabs-precomparison-2026-10-31/README.md)에 있다.

## 해결할 차이와 규범 기준

CSS Text Level 3 §4.1.2는 보존 탭을 가장 가까운 block container ancestor의 inline content 시작점 기준 다음 tab stop까지 배치한다. §4.2의 `tab-size` 기본값은 8이고 숫자는 해당 ancestor의 U+0020 advance 배수다. 지정된 위치까지의 거리가 `0.5ch`보다 작으면 그 다음 stop을 사용한다. 강제 segment break 뒤 새 줄은 같은 inline content 시작점에서 다시 계산한다. CSS Grid 2025 CRD §6·§11.5.1의 anonymous text item 및 intrinsic contribution 규칙으로 Grid track 기대값을 계산한다. Chrome은 교차 관찰 oracle이며 규범 계산의 유일한 근거가 아니다.

고정 Ahem 10px/10px LTR profile에서 `X`와 U+0020 advance는 각각 10px, 기본 tab stop 간격은 80px다. 비교 fixture는 root/ancestor/Grid content 시작점이 x=0이고 border·padding·margin·letter-spacing은 0, word-spacing은 0이다. 이 정렬을 벗어나는 offset ancestor, 다른 spacing 또는 writing mode는 이번 slice 범위 밖이다.

## 고정 장면과 기대값

[Chrome 156 사전 비교 fixture](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/anonymous-grid-text-pre-tabs-precomparison.html), [관측 JSON](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-anonymous-grid-text-pre-tabs-precomparison-2026-10-31/chrome-observation.json), [독립 기대값](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-anonymous-grid-text-pre-tabs-precomparison-2026-10-31/expected-values.json)과 [화면](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-anonymous-grid-text-pre-tabs-precomparison-2026-10-31/chrome-156-agent-browser.png)을 고정 기준으로 쓴다. fixture 폰트 hash는 저장소 Ahem의 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`이다.

| 장면 | Chrome track/root | text line advance | 판정 |
|---|---:|---:|---|
| `XX\tXX`, max-content | 100 / 100×10px | 100px | 두 글자 뒤 다음 80px stop |
| `\tX`, max-content | 90 / 90×10px | 90px | 선행 탭이 x=80으로 이동 |
| `XXXXXXXX\tX`, max-content | 170 / 170×10px | 170px | x=80 exact stop에서 다음 stop x=160 사용 |
| `XX\t\tX`, max-content | 170 / 170×10px | 170px | 각 탭이 차례로 x=80, x=160으로 이동 |
| `XX\tXX\nX\tX`, max-content | 100 / 100×20px | 100px, 90px | 강제 break 뒤 inline 시작점부터 재계산 |
| `XX\t`, max-content | 80 / 80×10px | 80px | 끝의 preserved tab advance 유지 |
| `XX\tXX`, min-content | 100 / 100×10px | 100px | `pre`의 soft-wrap 금지 |
| `XX\tXX`, 25px fixed track | 25 / 25×10px | 100px | track 밖으로 unwrapped line overflow |
| oracle-only `tab-size:2` | 60 / 60×10px | 60px | 기본값-only runtime profile의 음성 경계 |

## 계획된 변경

1. `spinon-text-layout::normalize`에서 `WhiteSpaceMode::Pre` 탭을 기본 8-stop으로 확장한다. 탭 앞 누적 inline advance로 다음 strict stop을 계산하고 `pre`의 `X`, 보존 U+0020, CR/LF/CRLF/form-feed forced break를 유지한다. 강제 break는 tab position을 새 줄 시작으로 재설정한다. 기존 `Pre`의 비탭 결과와 `PreWrap` tab 동작은 바꾸지 않는다.
2. profile은 고정 Ahem 10px/10px, weight 400, normal style, horizontal-tb/LTR, normal word/line breaking, normal ligature, spacing 0, inline style/ancestor 없음으로 제한한다. source characters는 이 fixture에서 검증한 ASCII `X`, U+0020, tab, forced break만 받는다. 탭이 있는 `Pre`는 profile 밖에서 `UnsupportedPreTabProfile`로 닫고 partial snapshot을 만들지 않는다. 여러 `TextSource`가 한 paragraph로 이어질 때 tab cursor는 유지하되 source node/UTF-16 provenance는 각각 보존한다.
3. tab 하나의 `TextSourceRange.paragraph_utf8`은 그 tab을 대신하는 전체 공백 범위를 가리키고 `source_utf16`은 원본 U+0009 한 code unit에 대응한다. line break의 CRLF 두 source range, tab 위치 재계산, trailing tab, exact-stop 다음 stop을 각각 단위 테스트한다. max-content/min-content 및 definite-width paragraph advance도 검증한다.
4. Stylo runtime Grid computed-style snapshot에 `tab-size` longhand 값을 노출하고, 탭 장면은 상속 후 computed `8`만 받는다. CSS `tab-size:2` 음성 scene은 정확한 unsupported-style 진단으로 거부하고 layout frame·trace·draw/present를 발행하지 않아야 한다. `tab-size` number/length 일반 cascade mapping이나 `<length>` 지원은 추가하지 않는다.
5. 기존 `RuntimeBlockFlexGridTrackSizingV1` anonymous Grid fixture에 위의 8 positive scenes를 추가한다. 고정 tab-size 음성 scene까지 총 40개 행렬에서 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 각각 실행한다. Positive scene은 root/item frame, track/contribution, normalized text/source mapping, revision, draw/present 및 retry 0을 확인한다. 음성 scene은 지정 오류와 partial result 부재를 확인한다. 설치 APK·앱 executable hash가 검증한 최신 source build와 일치해야 한다.
6. 기본 product artifact에서 새 scene marker, fixture FFI ABI와 payload가 빠지고 fixture-test positive control에서만 포함되는지 macOS ARM64 static archive 경계를 확인한다. text-layout/layout/runtime/FFI focused tests, fixture module boundary, Android/iOS builds, workspace Clippy, `cargo fmt --all -- --check`를 수행한다. 전체 workspace test의 기존 실패와 변경 관련 결과는 분리 기록한다.
7. 구현/runtime은 계획 리뷰와 다른 20개 실패 관점, 구현 PR diff는 별도 30개 관점으로 검토한다. PR 본문에 범위·테스트·Android/iOS simulator 화면을 각각 연결하며, C11.4/C11 checkbox와 공식 `72/213`은 유지한다. 저장소 정책의 Rebase and merge 전 최종 PR head가 검토 diff와 같은지 확인한다.

## 실패 기준과 명시적 제외

- `XX\tXX`가 100px이 아니거나, leading/exact-stop/consecutive/trailing tab 산술이 기대값과 다르면 실패다.
- 강제 break 뒤 tab 위치가 이전 줄 상태를 물려받거나 CRLF가 두 줄바꿈으로 세어지면 실패다.
- tab source range가 1 code unit에서 확장 공백 전체를 가리키지 않거나, source nodes가 합쳐질 때 provenance가 바뀌면 실패다.
- `min-content`에서 줄바꿈이 생기거나 25px track 입력에서 `pre`가 soft-wrap하면 실패다.
- `tab-size:2`, non-Ahem font, nonzero letter/word spacing, non-LTR/orthogonal writing mode, unsupported characters, inline ancestor 또는 offset content origin을 허용하거나 부분 결과를 발행하면 실패다.
- 새 장면이 anonymous Grid allowlist 밖, 일반 authored CSS 경로, 기본 product artifact 또는 fixture 외부 경로에서 활성화되면 실패다.
- Android/iOS 최종 source 실행이 다르거나 stale binary/hash, retry, draw/present 누락, 음성 입력의 frame 발행이 있으면 실패다.

이 제한 slice는 임의 font/script, `tab-size` 사용자 지정 number/length, offset ancestor, bidi, soft-wrapping tab, 일반 CSS admission, glyph ink pixel equality, Android 실기기/hardware GPU, iOS compositor 이후 pixel, Spinon WPT 적합성 또는 전체 C11.4/C11을 입증하지 않는다. 내부 계약 숫자 버전 `0.1.0`은 유지한다.
