# C11.4 `hyphens` PR 변경분 실패 관점 검토 30개

이 검토는 계획 검토 표를 재사용하지 않고 현재 구현·test·Android/iOS app adapter·문서 diff를 대상으로 했다. 구현 semantics review와 다른 관점에서 feature 경계, 소스 연계, 플랫폼 validator, fixture 부작용, evidence 정확성을 확인했다.

| # | diff 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | `ParagraphStyle` public data model에 새 field가 생겼는데 기존 initializer가 누락 | 기존 literal을 `hyphens: None`으로 유지했고 default runtime checks/build 및 feature Clippy가 통과. 기본 동작은 기존과 동일. |
| 2 | `HyphensMode`가 layout crate에서 노출되지 않음 | `spinon-text-layout`에서 정의하고 `spinon-layout` re-export 경로까지 연결. runtime fixture가 public crate 경계를 통해 사용. |
| 3 | fixture mode가 기본 runtime 또는 일반 CSS profile에 노출 | CSS alias adapter와 runtime scene validator는 기존 `c11-4-grid-track-sizing-fixture-test` opt-in cfg 아래에 있다. default FFI archive에는 candidate strings/export가 없음. |
| 4 | Cargo/native/Android/iOS feature 정의가 서로 어긋남 | 앱 build logs에서 C04 runtime GPU와 fixture-test만 켜지고 unrelated fixture flags는 꺼진 것을 확인. Android/iOS builds 모두 성공. |
| 5 | `custom_property_allowlist` 추가가 기존 author-profile 정책을 넓힘 | 기존 정책 생성자는 `None`을 두어 원래 전체 custom-property allow/deny 로직을 유지하고 hyphens fixture profile에서만 `--shyph` allowlist를 사용. tests/build 통과. |
| 6 | `--shyph` 허용이 다른 custom property까지 허용 | profile policy는 정확히 `shyph` 하나만 받는다. stylesheet `--mode` negative test에서 전체 profile fail closed. |
| 7 | stylesheet scanner가 CSS token을 regex로 훑어 comment/string을 변환 | cssparser AST traversal을 사용하며 comment/string 대조 test 통과. |
| 8 | inline-style와 stylesheet scanner의 source-coordinate 기준 불일치 | 두 경로를 각각 같은-length byte replacement로 처리하고 parser helper tests에서 원문 offset과 `!important` 유지 확인. |
| 9 | escaped property name을 정규 `hyphens`로 잘못 처리 | escaped property spelling은 fixed profile 밖 입력으로 거부. |
| 10 | 대문자 property/value의 CSS case-insensitive 처리 누락 | property token/name 비교는 ASCII case-insensitive; value alias는 Stylo가 computed custom-property winner에서 읽는다. `initial`도 case-insensitive scanner로 정규화. |
| 11 | 문자열 안의 `@property`가 registration으로 오인 | at-rule token 기준 검사로 literal text를 건너뛴다. |
| 12 | `@property --shyph` 대소문자를 잘못 합침 | custom property 이름은 case-sensitive 비교를 사용하며 예약된 정확한 alias만 registration gate가 검사. |
| 13 | nested CSS parser failure 뒤 partial rewrite를 반환 | scanner error/depth flag를 최종 치환 전에 검사해 전체 입력을 거부한다. |
| 14 | CSS 원문 byte 길이와 replacement byte 길이 불일치가 조용히 넘음 | range bounds 및 exact byte length를 매 replacement에서 검증하고 UTF-8 재구성 실패도 반환 error. |
| 15 | Stylo adapter가 author winner 대신 원문 마지막 선언을 읽음 | winner는 `ComputedValues.custom_properties().get_for_cssom(--shyph)`에서 읽는다. cascade tests에서 specificity/importance/order를 대조. |
| 16 | cascade output `hyphens` 키가 이전 computed output에서 stale | current element computed style로부터 매번 property map을 만든 뒤 alias를 삽입한다. style test와 simulator traces의 `None|Manual` 값이 일치. |
| 17 | 다른 Runtime profile에 alias property를 삽입 | `ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1`와 fixture feature 두 조건에서만 alias extraction. |
| 18 | custom property allowlist가 다른 profile에서 회귀 | 기존 constructor를 `None`으로 유지하고 기본 FFI/runtime checks 통과. |
| 19 | scene table과 app validators의 scene ID 집합이 다름 | FFI table은 고유 ID 21개 test, 각 platform app은 같은 21개 latest-source matrix를 모두 처리. Android/iOS 19 positive + 2 reject. |
| 20 | inherited scenes에서 root/grid/text NodeId가 한 칸씩 밀림 | parent wrapper를 추가한 두 scene에서 node 1/2/3, 두 box frame 및 Grid trace owner를 검사. 양쪽 matrix 통과. |
| 21 | inherited scene의 expected root width/track width를 혼동 | wrapper root 320px와 inner fixed Grid 30px를 각각 검증하고 item frame/track trace는 30px. |
| 22 | min-content / max-content / fixed 30px scene dimensions 재사용 오류 | independent expected-values 계산과 scene table에 각 track kind를 별도 보유; 두 matrix의 columns/rows 및 overflow traces 일치. |
| 23 | `None`에서 layout text와 authored source text를 같은 값으로 오검증 | trace는 original U+00AD source와 normalized U+2060 layout text를 따로 검증. |
| 24 | SHY UTF-8 byte offset과 UTF-16 source offset 혼동 | trace validator가 source UTF-16 range, normalized UTF-8 range와 source node mapping을 따로 검사. |
| 25 | line advance를 row/frame width로 간주해 overflow를 숨김 | fixed 30px track과 40px unbroken line width를 별도로 검증한다. `layout.width`와 `trackWidth`도 adapter 기준을 구분. |
| 26 | 기대 거부가 error code만 맞고 property/node가 틀려도 통과 | platform validator는 code `unsupported_c114_hyphens_value`, `computed hyphens` reason과 node ID를 검사하고 partial frame 부재도 확인. |
| 27 | Android route guard가 expected-reject scene을 fixture off에서도 실행 | reject route는 `BuildConfig.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST`로 제한. fixture flags는 build log에서 확인. |
| 28 | iOS route/expected rejection이 generic grid handler에서 누락 | `hyphens-*` scenes와 두 expected rejects를 app selector 및 C114 rejection mapping에 등록; 최신 21/21 Simulator run 통과. |
| 29 | simulator evidence가 다른 설치 binary 또는 재실행 실패와 섞임 | runner가 build/install SHA-256 동일성을 assert하고 run UUID별 log/trace를 저장하며 `finally`에서 app을 종료. 현재 APK/executable SHA가 installed hash와 동일. |
| 30 | 제한 구현이 CSS/WPT/실기기 완료를 암시하거나 공식 상태를 올림 | 계획·내부 계약·runtime evidence는 고정 profile과 한계를 적고 C11.4/C11 미완료 및 `72/213` 유지. status entry도 candidate progress로 기록한다. |

첫 Clippy 실행의 두 지적은 final diff 전에 수정했다. CSS parser 조건식은 let-chain으로 바꿨고 11-argument `Scene` helper는 값/ID를 바꾸지 않는 macro literal로 바꿨다. 최신 fixture Clippy와 focused tests 및 양 플랫폼 재실행 통과 후 검토했다. 변경 diff review에서 별도의 미해결 결함은 찾지 못했다.

이 기록은 PR #977에 병합된 제한 diff를 검토한다. GitHub status check rollup은 비어 있어 CI 통과로 세지 않으며, 일반 CSS admission, APK/IPA 전체 artifact 제외, WPT, 실기기/hardware GPU 또는 실제 glyph pixels를 뜻하지 않는다.

## 전체 패키지 회귀 후 확인

전체 `spinon-runtime` fixture-test suite 실행에서 기존 `hyphens` inline declaration test가 Stylo unknown-property 결과를 요구해 현재 후보 계약과 어긋난 점을 수정했다. computed `hyphens:none` 노출과 `UnknownProperty` 진단 부재를 검사한다. 같은 실행에서 기존 `word_break_overflow_wrap` matrix의 line-count assertion이 `MaxContent` 장면에도 min-content/fixed-track 조건을 적용하던 오류를 찾아 track별 정의에 맞게 고쳤다. 이후 `cargo test -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test`는 unit 394 passed/3 ignored와 integration 19 passed/0 failed로 통과했다. 두 수정은 테스트 기대값만 바꾸며 production 경로는 변경하지 않는다.
