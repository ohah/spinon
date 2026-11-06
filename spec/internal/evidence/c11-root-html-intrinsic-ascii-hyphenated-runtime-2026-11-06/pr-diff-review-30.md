# C11.4 ASCII 하이픈 단어 PR diff 적대 검토 30개

검토 대상은 구현 commit `3478d9bad89dfc8bb82a063af838b3b9af277756`과 시뮬레이터 결과·상태 문서까지 포함한 후보 PR diff다. 구현/runtime 20관점과 구분해 문법·source mapping·테스트 경계·실행 증거·공식 상태 승격 여부를 다시 확인했다.

| # | 공격 관점 | 확인한 근거 | 판정 |
|---:|---|---|---|
| 1 | token predicate가 ASCII alphanumeric 조건을 느슨하게 해 임의 punctuation을 통과 | `root_html_intrinsic_auto_rows.rs`의 byte predicate | U+002D 외 punctuation은 거부 |
| 2 | 시작 하이픈으로 음수를 단어 구분자로 오인 | `-A`, `-13` grammar/layout controls | direct-text gate 거부; 일반 text layout control은 한 줄 유지 |
| 3 | trailing hyphen의 빈 suffix를 허용 | `A-` control | 거부 |
| 4 | 연속 하이픈으로 빈 중간 segment를 허용 | `A--B`, `A1--B` controls | 거부 |
| 5 | Unicode hyphen/minus를 ASCII U+002D로 혼합 | U+2010, U+2212 negative controls | 거부 |
| 6 | slash·underscore·기타 구두점을 허용 | `A/B`, `A_B`, `A!`, `1.2` controls | 거부 |
| 7 | whitespace 허용 범위를 NBSP·NEL·Unicode space로 넓힘 | 기존 normal-whitespace negative matrix | 기존 네 ASCII collapsible whitespace만 유지 |
| 8 | 하이픈 후보의 기대값이 구현 결과를 참조 | Chrome observation과 Canvas 기반 `derive_expected.py` | 독립 계산; used Grid tracks를 oracle 입력으로 읽지 않음 |
| 9 | 9-scene expected matrix에서 scene 누락 | expected count 9 assertion과 `expected.json` | 9 scene 실행 |
| 10 | min/max/auto/fixed sizing mode에서 같은 값만 확인 | 독립 scene matrix와 row/track assertions | mode별 frame·row 기여를 확인 |
| 11 | reverse source order에서 item contribution 귀속이 바뀜 | reverse-order scene | source-order별 기대 frame 통과 |
| 12 | marker가 digit 전후가 아닌 모든 하이픈에 삽입 | 다음 byte ASCII digit 및 앞 byte ASCII alphanumeric predicate | 좁은 경계에서만 삽입 |
| 13 | 기존 hyphen-letter wrap에 marker를 중복 삽입 | `A-A`, `AA-BB` matrix | digit successor가 아니므로 기존 동작 유지 |
| 14 | 다중 marker 삽입이 뒤쪽 byte index를 무효화 | 뒤에서 앞으로 삽입하는 loop와 multi-hyphen cases | range 유지 |
| 15 | UTF-8 byte index를 Unicode 텍스트에 그대로 적용 | source와 normalized text의 ASCII gate | ASCII 밖이면 marker path 비활성 |
| 16 | CSS `white-space`·`word-break`·`overflow-wrap` 변형을 같은 profile로 수용 | normal mode gates와 style rejection cases | 기본 normal profile로 제한 |
| 17 | vertical writing 또는 RTL에 LTR 가정을 적용 | direction·writing-mode gates | profile 밖에서는 비활성 |
| 18 | text transform이 바꾼 buffer의 byte 경계를 authored source로 오인 | `TextTransformMode::None` gate | transform profile과 분리 |
| 19 | `hyphens:auto` 경로와 수동 U+200B marker를 중첩 | `hyphens` gate, 기존 fixture regression 3/3 | auto 경로 유지 |
| 20 | Root HTML `line-break:auto`의 내부 `Strict` enum을 오해해 marker를 누락 | `Loose`/`Normal`/`Strict` line-break test 및 기본 RuntimeSession route | 세 mode에서 같은 wrap |
| 21 | marker를 DOM/HostDocument text에 써서 node identity를 바꿈 | RuntimeSession source/node assertions | authored source와 identity 유지 |
| 22 | marker를 빈 UTF-16 source range로 매핑해 raster validation 실패 | hyphen range가 U+200B까지 normalized UTF-8 span을 소유하는 test | RuntimeSession render 통과 |
| 23 | 다른 source range와 marker range가 겹치거나 stale | offset update, range sort, `InvalidGlyphLayout` fail-closed | malformed mapping은 publish 전에 거부 |
| 24 | 새 positive syntax 때문에 기존 invalid-item atomic rejection이 사라짐 | stale `B-1` control을 `B--1`로 변경한 style/runtime tests | positive/negative 경계 유지 |
| 25 | unsupported item이 일부 layout/draw를 publication | RuntimeSession atomicity negatives | fail-closed 회귀 통과 |
| 26 | platform 앱이 다른 fixture revision을 실행 | Android/iOS `installed_fixture_sha256`와 same fixture SHA | 양쪽 SHA 일치 |
| 27 | Android emulator 결과를 실기기/hardware GPU로 과장 | API 37, ARM64, ANGLE/Vulkan/SwiftShader SurfaceFlinger evidence | AVD software 경로로만 기술 |
| 28 | Android test가 사용자의 기존 앱을 교체하거나 중단 | isolated package ID, original PID/activity pre/post, restore logs | 기존 PID `29922` 유지; 후보 package 제거 |
| 29 | iOS pixel sample을 compositor 이후 또는 전체 화면 픽셀 동일성으로 기술 | Metal drawable log, screenshot와 README 범위 | compositor 전 한 표본 exact로 제한 |
| 30 | 후보 evidence를 C11 완료·CI 통과로 잘못 승격 | `STATUS.md`, README, PR status 확인 경계 | C11.4/C11 미완료, `78/214`; Clippy 기존 lint와 CI 미확인 명시 |

## Residual limits

Fixture-feature RuntimeSession all-target Clippy는 변경하지 않은 두 helper의 `too_many_arguments` lint 때문에 실패한다; fixture-feature library-only Clippy는 통과했다. GitHub PR check rollup은 아직 조회하지 않았으므로 CI 결과는 기록하지 않는다. WPT, arbitrary CSS admission, physical Android device, Android hardware GPU, iOS post-compositor pixels, text selection/copy와 전체 C11.4/C11 적합성은 이 diff에서 증명하지 않는다.
