# C11.3 · `repeat()`, named grid lines, template areas

**상태:** 계획안·검토 완료 · C11.3.1 Rust 구현과 단독 후보 source gate 완료 · Android 16 SM-S731N 실기기/iPhone 17 Pro iOS 26.2 Simulator 표본 각 5/5 exact · C11.3.2/.3 미착수
**상위:** [C11 CSS Grid](c11-css-grid.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)
**규범 기준:** [CSS Grid Layout Module Level 1 · 2025-03-26 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/) §7.2–7.3, §8.3
**비교 기준:** Chrome for Testing `154.0.8037.92` (mac-arm64), CDP revision `@334b65d254ccc35df4fca82706d1753227b01039`, CfT stable revision `r1689415`; WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`
**계약 버전:** 내부 계약 `0059`와 crate 버전 `0.1.0` 유지
**계획 검토:** [기능 계획 실패 관점 20개](../spec/internal/evidence/c11-3-template-repeat-named-lines-areas-plan-review-2026-10-13.md) · [Chromium 기준 교체 실패 관점 20개](../spec/internal/evidence/c11-3-chromium-baseline-review-2026-10-13.md)
**구현 검토:** [C11.3.1 구현·플랫폼 표면 픽셀 실패 경계](../spec/internal/evidence/c11-3-1-grid-repeat-implementation-review-2026-10-14.md) · [초기 구현 검토](../spec/internal/evidence/c11-3-1-grid-repeat-pixels-implementation-review-2026-10-13.md)

### Chromium 기준 버전 고정

2026-10-13 확인 당시 계획에 적힌 Google Chrome `154.0.8037.98`은 공식 [Chrome for Testing 배포 목록](https://googlechromelabs.github.io/chrome-for-testing/)에 없었다. 공식 목록의 Chrome 154 stable 바이너리 `154.0.8037.92`를 내려받아 실행하고 `Browser.getVersion`에서 제품 버전·CDP revision을 확인했다. 이후 C11.3 기준 캡처는 이 정확한 바이너리와 실행 파일 SHA-256을 기록한다. 기존 C11.1–C11.2.3 기준 자료(Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`)와 이 신규 기준 결과를 같은 실행 결과처럼 섞지 않는다. WPT 원문 revision과 각 파일 hash는 그대로 고정한다.

### C11.3.1 정수 `repeat()` 사전 비교 완료

고정 HTML fixture는 12 scene·29 node이며 Chrome for Testing `154.0.8037.92`에서 viewport `320×240 CSS px`, LTR·`horizontal-tb`, DPR 1·2로 캡처했다. 실행 ID는 `c1131-chrome-20261013T035742923Z-f59a108e`다. CSSOM resolved value, line-name group, node별 geometry를 분리해 저장했고 fixture의 geometry 비교 허용치는 축별 `0.5 CSS px`다. [비교 결과·스크린샷·원본 로그](../spec/internal/evidence/c11-3-1-repeat-precomparison-2026-10-13/README.md) · [Chromium 기준 자료](../tests/fixtures/css/references/c11-3-1-grid-repeat-precomparison-v1.json).

Chrome에서는 반복 패턴이 7 track으로 확장되고, 반복 seam은 하나의 line-name group으로 합쳐졌다. 같은 line에 중복된 이름은 CSSOM reference에 중복으로 남는다. Chrome은 10,000·10,001 explicit track 모두 계산했지만 Spinon profile의 10,000 상한과 10,001 거부 계획은 유지한다. auto-repeat·`minmax()`는 Chrome 기준값을 수집했을 뿐 이번 지원 profile에는 포함하지 않는다. 잘못된 nested/zero repeat 선언은 Chrome CSSOM에서 cascade fallback 결과를 보였다. 이 fallback을 repeat 지원 또는 Spinon의 오류 동작으로 해석하지 않고 adapter/cascade 경계를 별도로 비교한다.

2026-10-13 14:01 UTC 이후 앱 검증 fixture와 runtime GPU 내부 모듈을 분리한 뒤 C11.3.1 앱을 현재 작업 트리에서 재실행했다. Android API 37 AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable은 같은 Chrome 기준 표본 5/5 RGBA exact였고, 변조 표본 좌표는 표면 readback 전에 거부됐다. [Android AVD run](../spec/internal/evidence/c11-3-1-grid-repeat-pixels-2026-10-13/android-avd-c1131-android-avd-20261013T140115888Z-551001c8/README.md) · [iOS Simulator run](../spec/internal/evidence/c11-3-1-grid-repeat-pixels-2026-10-13/ios-simulator-c1131-ios-simulator-20261013T140129522Z-48646be4/README.md). 두 앱은 C11.2 후속·C12.4 변경이 함께 있는 작업 트리에서 빌드됐으므로 이 run은 재현 진단이며 단독 feature source gate가 아니다. 이전 C11.3.1 후보의 iOS 차단 원인이었던 C11.2.3 경로 결함은 [PR #146](https://github.com/ohah/spinon/pull/146)으로 병합·복구했고 C11.2.3 clean-main 후속 실행은 [별도 근거](../spec/internal/evidence/c11-2-3-dense-column-placement-ios-simulator-runtime-matrix-2026-10-13/c1123-i-mv6poes2-6eaa4460/README.md)에 기록했다.

단독 후보 소스 gate는 최신 기준 commit `a40504d762cdfa19f6af3048336b563e6366bb9e`와 C11.3.1 변경을 합친 source tree `96f2c8c2a75bd2ce43be735a27b212271bd0c51a`에서 완료했다. 첫 iOS 실행은 C11.3.1 인수가 C11.1 장면 검사로 빠지는 라우팅 누락과 Metal 비교기의 `C1131` marker 거부를 드러냈다. iOS 전용 host 생성·scene 평가·표면 표본 경로와 비교기 schema/test를 추가하고 같은 후보에서 재실행했다. Android 16 실기기 PixelCopy와 iOS 26.2 Simulator Metal drawable은 각각 5/5 RGBA exact, 최대 채널 오차 0이었고, 변조한 표본 좌표는 PixelCopy/Metal readback을 시작하기 전에 거부됐다. [source gate manifest와 두 실행](../spec/internal/evidence/c11-3-1-grid-repeat-pixels-2026-10-13/source-gate.json).

2026-10-14에는 fixture 경로 후속 정리 PR #149를 반영한 `origin/main`에서 C11.3.1 변경을 다시 검증했다. 새 후보에서 CSS reference 387/387, Rust workspace 656 passed·4 ignored, 기본 FFI 18/18, fixture feature FFI 53 passed·2 ignored, workspace/fixture Clippy와 rustfmt가 통과했다. Android 16 SM-S731N 실기기 PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable은 각각 5/5 RGBA exact, 최대 채널 오차 0이었다. 새 후보와 해당 두 앱 실행을 만든 후보의 `crates/`·`platforms/` 소스는 바이트 단위로 같음을 확인했다. [최신 재검증 로그·화면·바이너리 feature 대조](../spec/internal/evidence/c11-3-1-grid-repeat-pixels-2026-10-14/README.md). C11.3.1 runtime은 여전히 `multi-track-repeat` 한 장면이며 named line 배치·template areas는 미구현이다.

고정 WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 정수 repeat 후보 5개는 Chrome runner에서 5/5 PASS, unexpected 0이었다. 이는 Chrome oracle 실행이며 Spinon WPT 적합성 결과가 아니다. WPT runner 결과 JSON은 `headless: false`로 보고하므로 이 실행을 headless 실행이라고 주장하지 않는다. 비교 캡처 도구의 별도 Chrome 실행은 기록된 `--headless=new`와 `HeadlessChrome` user agent를 사용했다. 이후 Rust cascade·projection·Taffy profile과 단일 고정 runtime scene을 추가했고, Android 17/API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 geometry가 Chrome 기준과 일치하는 것을 확인했다. 각 플랫폼 실행은 HostDocument 4개 node인 `multi-track-repeat` 한 장면이다. 고정 Chrome 154 DPR 1 화면에서 선택한 표면 표본은 Android AVD PixelCopy 5/5, iOS Metal drawable 5/5가 RGBA 완전 일치했다. 잘못된 표본 좌표 negative control은 양쪽 모두 PixelCopy/Metal readback 전에 거부됐다. AVD는 ANGLE/Vulkan/SwiftShader 소프트웨어 backend이며 실기기 GPU 근거가 아니다. iOS readback은 OS compositor 제출 전 drawable이고 화면 캡처는 픽셀 비교 입력이 아니다. 통합 로그·이미지는 [표면 픽셀 근거](../spec/internal/evidence/c11-3-1-grid-repeat-pixels-2026-10-13/README.md)에 둔다. Android SM-S731N 실기기 PixelCopy는 5/5 RGBA exact였지만, 해당 APK는 C11.2 후속·C12.4 변경이 함께 있는 작업 트리에서 빌드했다. 혼합 run은 진단으로 남기고, 단독 후보 gate 결과는 source-gate manifest에 별도로 기록한다.

## 목표와 완료 상태

C11.2.3의 제한된 dense/column auto-placement 뒤에 CSS Grid 명시 트랙의 정수 `repeat()`, named line, `grid-template-areas`와 해당 배치 해석을 추가한다. Stylo의 typed computed 값을 소유된 Grid 입력으로 변환하고, 엔진 중립 계층에서 line/area를 해석한 뒤 Taffy에 숫자 트랙과 배치 위치를 전달한다. Taffy의 CSS 문자열 파서나 named-line 해석에 CSS 의미를 위임하지 않는다.

C11.3은 다음 세 단계로 분할한다. 각 단계는 독립된 기능 PR 하나로 제출하며, 각 계획과 각 구현 PR은 서로 다른 20개 실패 관점 검토를 갖는다. 앞 단계가 병합되고 회귀 검증된 뒤 다음 단계로 간다. C11.3 상위 완료 표시는 세 단계가 모두 끝난 뒤에만 바꾼다.

| 단계 | 한 PR에 넣을 기능 | 명시적으로 포함하는 경계 | 완료 관문 |
| --- | --- | --- | --- |
| C11.3.1 | 정수 `repeat()` track 확장 | 양의 고정 count, 현재 지원 track-size subset, 각 expanded line의 line-name metadata 및 repeat seam 병합 | Chrome `getComputedStyle()` resolved output, track 수·line-name sequence, allocation·limit 실패, Android/iOS runtime |
| C11.3.2 | explicit named line 정의·배치 조회 | 이름과 양·음 occurrence, 이름을 세는 span, 기존 numeric placement와의 조합 | 중복 이름·양/음 방향·span 방향·없는 이름을 Chrome과 대조 |
| C11.3.3 | `grid-template-areas`와 named area 배치 | 직사각형 area grid, 생성된 `*-start`/`*-end` line alias, named area/line 해석 및 `grid-area` shorthand | parser/cascade·area map·배치·geometry·모바일 표면 비교 |

기능 단위 PR은 해당 단계 코드·fixture·내부 계약·구현 검토·상태표 갱신만 포함한다. 무관한 미커밋 파일이나 다음 단계의 코드는 넣지 않는다. 계획을 바꾸면 변경된 설계도 별도 20개 실패 관점 검토를 거친다.

## 공통 입력·출력 계약

1. 입력은 같은 HostDocument, mount root, style/environment revision을 가진 Stylo computed snapshot이다. C11.3.1에서 누적 `RuntimeBlockFlexGridTemplateV1` profile을 추가하고 C11.3.2/.3에서 이 profile만 확장한다. 선언 문자열을 다시 파싱하거나 C11.2.1–2.3 profile의 성공 의미를 바꾸지 않는다.
2. C11.2.1–C11.2.3의 track-size subset만 허용한다: definite-axis의 기존 `px`·`%`·유한 양수 `fr`, 기존 gap/placement/auto-placement 규칙. `auto`, `minmax()`, `fit-content()`, `calc()` track, intrinsic sizing은 이 계획에서 추가하지 않는다.
3. 소유된 엔진 중립 track 입력은 track 크기 N개와 경계 line-name 목록 N+1개를 보존한다. 각 CSS Grid line의 이름은 순서·반복 횟수·축을 잃지 않고 저장한다. 중복 이름을 임의로 제거하지 않는다. 동일 line에 복수 표기가 있을 때의 조회 결과는 규범과 고정 Chrome fixture로 확정한다.
4. 정수 repeat는 Taffy에 넘기기 전에 Spinon이 확장한다. count와 기존 track 수를 checked arithmetic으로 계산하고 `GRID_MAX_TRACKS`(10,000)를 넘으면 전체 계산을 구체 오류로 거부한다. 축별 expanded line-name metadata 예상 크기는 line-group/vector header와 이름 payload를 합산해 1 MiB까지 허용한다. 초과 입력, 산술 overflow, `try_reserve` 실패는 전체 계산을 거부하고 부분 layout을 내보내지 않는다. 이 크기는 메모리 사전 차단용 추정치이며 프로세스 전체 메모리 상한은 아니다. 조용한 clamp나 축소는 금지한다.
5. named line/area 해석은 고정된 CSS Grid 규칙에 따라 Spinon 엔진 중립 계층에서 수행한다. Taffy 입력에는 검증된 explicit numeric line을 전달한다. 지원 밖 해석은 구체 진단으로 실패시키며 다른 이름·line으로 대체하지 않는다.
6. C11.3.3 area matrix는 모든 행의 cell 수가 같고 각 area 이름이 하나의 채워진 직사각형이어야 한다. 하나의 이름이 여러 인접 cell을 차지하는 것은 유효하며, L자형·분리 영역은 무효다. `.` 연속 표기는 하나의 null cell token이다. area matrix의 행·열 수는 명시 rows/columns track 수와 같아야 한다. 현재 단계는 area 선언만으로 자동 크기 explicit track을 생성하는 CSS 동작을 지원하지 않는다.
7. 입력은 `grid-template-columns`, `grid-template-rows`, `grid-template-areas`, `grid-row`/`grid-column` longhand와 shorthand, `grid-area`를 포함한다. `grid-template`과 `grid` shorthand도 authored 문법을 구별해 차단하지 않는다. Stylo computed 결과가 이 계획의 typed-value subset에 들어오면 처리하고, shorthand가 만든 값 중 하나라도 지원 밖이면 해당 computed property를 진단과 함께 거부한다. explicit grid 밖 implicit line/track 생성은 지원하지 않는다.
8. CSS parser가 잘못된 declaration을 버리고 cascade fallback을 적용하는 것과 layout adapter의 unsupported computed value 오류를 구분한다. malformed CSS를 adapter에서 재파싱하거나 모든 잘못된 authored string을 runtime error라고 부르지 않는다.
9. Chrome `getComputedStyle()`은 브라우저 oracle로만 사용한다. `grid-template-rows/columns`의 CSSOM resolved value(used track size serialization), Stylo typed computed value, normalized line/area metadata, node frame을 각각 다른 관측값으로 비교한다. 이를 Spinon 공개 CSSOM/`getComputedStyle` 구현으로 표시하지 않는다.
10. DOM/source/accessibility/paint order, NodeId, mount root, document/style/environment revision은 placement 결과만으로 재정렬·변경하지 않는다. 모든 runtime profile과 새 metadata는 같은 snapshot revision을 유지한다.

## 단계별 세부 계약

### C11.3.1 · 정수 `repeat()` 확장

- `repeat(<positive-integer>, <track-list>)`만 추가한다. auto-repeat (`auto-fill`, `auto-fit`), nested repeat, count 0/음수/실수, 반복 안의 현재 unsupported track-size는 성공시키지 않는다.
- track-size subset은 기존 C11 입력과 같다. repeat 안팎의 line-name group을 포함하며 결과 track 개수와 N+1 line group을 모두 보존한다.
- 반복이 맞닿는 두 경계 line-name group은 CSS 규칙에 맞춰 하나의 line metadata 위치로 합친다. repeat count 1, 양쪽 경계의 빈/비어 있지 않은 group, 같은 이름 중복을 분리 fixture로 둔다.
- 확장 결과는 전체 explicit track 상한 검사를 통과한 뒤 Taffy에 전달한다. overflow, 10,001개 초과, `try_reserve` 실패는 실패 원자성과 같은 worker 후속 정상 계산으로 검증한다.
- `grid-template`와 `grid` shorthand는 Stylo가 만든 longhand computed 값을 통해 정수 track repeat를 처리한다. `grid-template-areas`가 `none`이 아니면 아직 areas 배치가 구현되지 않았으므로 구체 오류로 거부한다. shorthand가 만든 지원 밖 computed track/auto-flow 값도 성공시키지 않는다.
- 반복으로 불어난 line-name metadata는 축별 1 MiB 예상 크기를 넘으면 layout 전에 거부한다. 초과 및 각 실제 reserve 지점의 실패를 따로 검증한다.
- 이 단계에서는 item placement가 이름을 조회하지 않는다. expanded names는 다음 단계가 사용할 metadata로 보존하고 기존 numeric placement 결과와 회귀 비교한다.

### C11.3.2 · named line 조회와 explicit placement

- track list의 `[<custom-ident>+]`를 axis별 line metadata로 받는다. CSS-wide keyword, `auto`, `span`을 line name으로 받아들이지 않는다.
- `grid-row/column-start/end`의 기존 numeric/auto/span과 named ident, grammar가 허용하는 ident + occurrence 정수 조합을 처리한다. positive occurrence는 explicit grid 시작에서, negative occurrence는 explicit grid 끝에서 찾는다. 이름 비교는 CSS의 case-sensitive 식별자 비교를 따른다.
- `span [<positive-integer>] [<custom-ident>]`은 양쪽 문법 순서를 처리한다. ident가 있으면 반대쪽 definite line에서 그 이름의 grid-line occurrence를 세며, 숫자가 생략되면 1을 사용한다. row/column start/end 방향을 각각 검증한다. 한 물리 line에 같은 ident가 여러 번 있어도 line occurrence는 line index 기준으로 한 번만 센다.
- line name이 없거나 지정 occurrence가 explicit grid 밖 implicit line을 요구하면 `UnsupportedNamedGridLine` 계열 오류를 반환한다. implicit line materialization은 다음 계획 단계가 아니라 별도 C11 후속 범위다.
- 같은 이름이 한 line group에 반복되어도 index는 그 line을 한 번 센다. 원본 name list는 metadata에서 보존하고, resolver만 line index를 중복 계산하지 않는다. repeat seam에서 같은 이름이 합쳐지는 사례를 Chrome fixture로 확인한다.

### C11.3.3 · template areas와 area 배치

- Stylo의 typed `GridTemplateAreas`를 owned row/cell/name 구조로 변환한다. raw string 재파싱은 하지 않는다.
- empty/null cell, 반복 점, unequal row width, 하나의 유효한 반복 영역, L자형·분리 area, `none`, 비 ASCII/escaped custom-ident, area alias와 authored line name 충돌을 분리한다. explicit `foo-start`/`foo-end` line names가 만드는 implicitly-named area도 CSS Grid §7.3.3 규칙으로 조회한다. 이름으로 배치할 때 explicit/generated `foo-start`/`foo-end` 후보의 우선 순서는 CSS Grid §8.3의 첫 매칭 규칙을 따른다.
- area 이름은 각 축에 `name-start`/`name-end` alias를 제공한다. authored explicit line name과 생성 alias의 합성 및 동일 이름의 occurrence는 CSS 규칙 및 Chrome 비교로 검증한다.
- area 이름을 placement로 지정하면 해당 rectangle의 네 경계로 정규화한다. `grid-area` shorthand의 영역 해석, 네 longhand에서 이름을 지정하는 경우, 이름이 area와 line 모두와 관계되는 경우를 구분한다.
- area 행/열과 명시 track 수가 다르거나 auto track 생성이 필요하면 실패한다. invalid authored declaration이 Stylo cascade에서 무시되어 earlier declaration/initial value가 선택되면 그 computed 결과를 그대로 사용한다.

## 단계별 fixture 최소 목록

아래 입력은 각 PR의 Chromium 사전 비교에서 모두 포함한다. 같은 장면에서 기대 성공/기대 거부를 섞지 않고 scene별 상태를 고정한다. WPT 기반 scene은 원문 WPT 전체를 Spinon에서 실행했다고 표시하지 않는다.

| 단계 | 최소 장면 | 예상 결과 |
| --- | --- | --- |
| 3.1 | 양 축 `repeat(1, 1fr)`와 repeat 없는 negative control | 성공, 기존 단일 트랙 frame과 일치 |
| 3.1 | multi-track repeat, repeat 앞·뒤 단일 track | expanded track count·frame 일치 |
| 3.1 | repeat 양쪽 line names, 내부 반복 name, 반복 seam 충돌 | expanded N+1 line groups와 lookup index 일치 |
| 3.1 | repeat 안의 `px`, definite `%`, `fr` 및 gap 조합 | 기존 track subset과 같은 used frame |
| 3.1 | 10,000 track 정확 경계와 10,001 초과 | 경계 내 성공, 초과는 전체 오류/no-draw |
| 3.1 | 0/negative/auto-fill/nested/unsupported track-size | parser fallback 또는 정확한 unsupported 진단; partial output 없음 |
| 3.2 | 유일 name, 같은 name 여러 line, named line의 positive·negative occurrence | resolved numeric line과 Chrome item frame 일치 |
| 3.2 | row/column 시작·끝, ident/integer 순서, CSS name case | 축·방향·case-sensitive lookup 일치 |
| 3.2 | `span name`, `span integer name`, opposite definite edge 조합 | span count·numeric normalization 일치 |
| 3.2 | 한 line group에 동일 name 중복, repeat seam의 중복 name | metadata 보존, line index는 한 번만 계산 |
| 3.2 | 없는 name, occurrence 부족, `auto`/`span` reserved name | implicit line을 만들지 않고 명시적 거부 |
| 3.2 | named line이 numeric endpoint와 충돌·역전되는 placement | C11.2.1 conflict normalization 회귀 |
| 3.3 | 단일 cell/직사각형 spanning area와 null cell | area rectangle·generated aliases·frame 일치 |
| 3.3 | `.`와 `....` null-cell token, 모든 cell null | cell count와 빈 named-area 결과 일치 |
| 3.3 | row cell count 불일치, trash token, empty string, L자/분리 영역 | declaration invalid 및 CSS cascade fallback 확인 |
| 3.3 | area rows/columns와 explicit track list 크기 불일치 | auto-sized track 추정 없이 명시적 오류 |
| 3.3 | area와 authored `foo-start`/`foo-end` 충돌·동일 이름 | CSS §8.3 first-match 순서와 area edge 일치 |
| 3.3 | explicit `foo-start`/`foo-end`만으로 만든 implicit named area | `grid-area: foo` line resolution 일치 |
| 3.3 | `grid-area` 1–4 값, longhand·grid-row/grid-column·template/grid shorthand | shorthand expansion과 같은 resolved placement; 값이 subset 밖이면 거부 |
| 3.3 | nested container, hidden child, 이전 numeric Grid scene | child identity·revision·source/paint order 및 C11.2 회귀 |

## Chromium·WPT 비교 기준

고정 WPT commit은 `c999c58338ee1d223df5ad62e48be1202ced0d37`다. 아래 SHA-256은 해당 commit의 raw source다. C11.3.1의 다섯 정수 repeat 원문은 Chrome 154 runner에서 사전 실행해 5/5 PASS를 기록했다. 나머지 단계 후보는 아직 사전 실행하지 않았다. WPT 원문은 고정 Chrome 154 runner에서 먼저 실행하며 PASS는 Chrome oracle 결과이지 Spinon WPT 적합성이 아니다. 구현 전에 각 원문 test/subtest를 `Spinon fixture 후보`, `Chrome oracle-only`, `현재 미지원`으로 분류하고 그 사유를 inventory에 남긴다. 각 implementation PR은 파생 Spinon fixture의 source hash와 WPT 원문 실행 결과를 분리 보관한다.

| 단계 | WPT 경로 | SHA-256 |
| --- | --- | --- |
| 3.1 repeat | `css/css-grid/grid-definition/grid-repeat-integer-001.html` | `f48604bbd0316467f1b004052db9425d132c56632b034f99eb7aaf34b530ad1c` |
| 3.1 repeat | `css/css-grid/grid-definition/grid-repeat-integer-002.html` | `27b78abd184512f3f5371dd13fe8bd49e7aee41dc0ae7a7cc1689d54ab571a10` |
| 3.1 repeat | `css/css-grid/grid-definition/grid-repeat-integer-003.html` | `baba3dbdfc7476bd2937cdffd2cc7e8c8c2b73ed8c80849d15e9e846e40544ac` |
| 3.1 repeat | `css/css-grid/grid-definition/grid-repeat-integer-004.html` | `d87c350e49934251cbbf346086fd89f6dd815b5ecde0dec31ed2ba843263a333` |
| 3.1 repeat | `css/css-grid/grid-definition/grid-repeat-integer-005.html` | `34a7548251bb2ad1a0d45677d6a8bbf99684f71157d6aa5ef587d25b42c154d4` |
| 3.2 named lines | `css/css-grid/placement/grid-placement-using-named-grid-lines-001.html` | `d31717c0e82d8030f0a99070a5e98333be71c2cc834e36950ae5b5d303fd180e` |
| 3.2 named lines | `css/css-grid/placement/grid-placement-using-named-grid-lines-009.html` | `beff24e4a4afdc8a78113df44c35b7f70c22c9067c935ae3340a4432f2b2e6eb` |
| 3.2 missing name | `css/css-grid/placement/grid-placement-unknown-named-grid-line-001.html` | `27f9afb4ca2c856718ca81ef750b511242bce5be7386f80fa335c37329e9d049` |
| 3.3 areas | `css/css-grid/grid-definition/grid-support-grid-template-areas-001.html` | `fdc219b1887a5aaf7e81c4db2cea39e7fb9878274736b06a629adabf06e77b4a` |
| 3.3 areas | `css/css-grid/placement/grid-placement-named-grid-area-resolution-001.html` | `573b98353618c00c3eefc27ccc56c176dc269fe12ece7bbcd8dd6375dcce6ae7` |
| 3.3 areas + lines | `css/css-grid/placement/grid-placement-named-lines-and-areas-001.html` | `6e55fadc66203e6d570c61009c5d6a42d2f2db7eb77c7864aced34f86219aaa9` |
| 3.3 missing area | `css/css-grid/placement/grid-placement-unknown-named-area-001.html` | `7fa795ad262e74399487fa817da1e99a75bbcce2e46b5f4547d55959e2e9ebd4` |

CSS Grid `grid-template-rows/columns`의 `getComputedStyle()`은 resolved value를 반환하므로, 그 CSSOM 문자열을 Stylo의 typed computed 값과 같은 종류라고 취급하지 않는다. 각 단계의 사전 비교는 viewport `320×240 CSS px`, DPR 1·2, `horizontal-tb`/LTR, 고정 font·locale로 실행 ID, browser version, raw fixture hash를 남긴다. 지원 scene은 node별 frame, CSSOM 결과 및 typed track/line/area metadata를 각각 비교하며 frame 허용치는 각 축 `0.5 CSS px`다. 기대 거부 scene은 error code/property/node, 결과 부재, 빈 draw를 확인한다. DPR별 geometry가 바뀌지 않는지도 확인한다.

## 각 기능 PR의 검증 관문

1. 계획 문서와 해당 계획 검토 20개 관점을 코드 변경 전에 고정한다. 구현 뒤에는 별개의 20개 구현 실패 관점 검토를 수행하고 발견한 문제를 수정한다.
2. CSS reference fixture는 HTML input, 기대 JSON, Chromium raw source hash를 함께 고정한다. CSSOM 결과만 보거나 screenshot만 보는 것을 성공 기준으로 삼지 않는다.
3. Rust: style projection, layout normalization, profile/runtime, failure atomicity, old C11.2.1–2.3 regression 및 workspace 검사 결과를 기록한다. allocation은 repeat/metadata의 실제 reserve 지점에 실패를 주입한다.
4. Android: 연결된 SM-S731N에서 동일 fixture의 runtime geometry, error/no-partial-draw 및 surface PixelCopy를 측정한다. API 37 AVD는 고정 Chromium pixel 비교를 보완하지만 실기기 gate를 대신하지 않는다. 실기기 연결이 없으면 그 gate를 미완료로 남긴다.
5. iOS: iPhone 17 Pro/iOS 26.2 Simulator에서 같은 fixture를 실행하고 Metal drawable sample을 확인한다. 이 결과는 iOS 실기기나 OS compositor 이후 픽셀을 뜻하지 않는다.
6. Profile 외 입력, track 상한, malformed/cascade fallback, duplicate names, missing names, row/column axis swap, nested container, stale revision, revision overflow, allocation failure 및 다음 정상 요청 복구를 검사한다.
7. 변경 후 `spec/internal/0059-c11-grid-layout.md`, `spec/STATUS.md`, `spec/internal/README.md`, public compatibility support table과 `spec/public/roadmap.html`을 실제 결과에 맞춘다. Tailnet 미리보기도 동기화 후 HTTP status, 변경 SHA-256, 링크 및 모바일 layout을 확인한다. GitHub Pages는 배포하지 않는다.
8. PR은 C11.3.1, C11.3.2, C11.3.3 각각 하나씩만 올린다. commit subject는 영어 conventional prefix 뒤 설명을 한국어로 적고, PR 본문·주석·문서는 한국어로 쓴다.

## 이 계획의 미포함

auto-repeat, nested repeat, automatic/intrinsic track sizing, `minmax()`, `fit-content()`, implicit named lines/tracks, area matrix로 인한 implicit auto tracks, auto-sized shorthand result, subgrid, masonry, writing mode/RTL, baseline/alignment, text/replaced content, scroll/overflow, full WPT runner·적합성 또는 성능 우위는 이 세 단계의 완료를 주장할 근거가 아니다. 해당 항목은 C11.4–C11.6 또는 별도 후속 계획으로 남긴다.

C11.3 전체는 세 단계의 제한 profile과 지정 검증이 모두 통과해도 공개 CSS 지원 선언이나 전체 CSS Grid/WPT 적합성을 뜻하지 않는다. 공개 호환성 표는 각 단계에서 실험/내부 상태와 사용자 지원 상태를 분리해 갱신한다.
