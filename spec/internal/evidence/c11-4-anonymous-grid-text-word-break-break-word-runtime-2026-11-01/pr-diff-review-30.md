# C11.4 `word-break: break-word` 구현 PR diff 적대적 검토 30개

2026-11-01, PR #974의 구현 head `b4e8b856a9d0a69e0b8e9367a6acacfb0ba46f3b`와 `origin/main` base `48c97510a4a49c06785c2ff74834b67b3ce1329d` 사이의 구현 diff 76개 파일을 검토했다. fetch 후 PR head/base SHA가 로컬 검토 범위와 같았다. 이 기록을 추가한 뒤 PR diff는 문서 1개가 늘어난 77개 파일이다. 추가 문서 diff도 별도로 다시 읽어 path 수, base/head, 검토 대상과 본문 주장이 구현 기록과 맞는지 확인했다. 계획 자체의 20관점과 구현/runtime 실패 관점 20개는 재사용하지 않고 parser 경계, cascade source identity, feature 유출, 플랫폼 어댑터와 증거 귀속을 별도로 확인했다.

| # | PR diff의 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR에 C11.4와 무관한 변경 또는 다른 미완료 작업이 섞이는가 | 76개 tracked diff path는 C11.4 `word-break` parser/cascade, 고정 Grid text fixture, 앱 verifier, 독립 비교·실행 증거로 한정된다. 다른 로컬 untracked evidence는 diff와 staging에 포함하지 않는다. **통과** |
| 2 | 제한 slice가 공식 완료 수나 상위 체크를 올리는가 | `spec/STATUS.md`는 PR diff에 없고 현재 C11/C11.4는 미체크, 공식 `72/213`이다. PR 본문과 증거 README도 이 한계를 유지한다. **통과** |
| 3 | 계획과 구현이 다른 입력·성공 조건을 사용하게 되는가 | 계획, precomparison JSON, FFI/runtime scene catalog, 양 플랫폼 생성기와 capture script가 `word-break` 2 × `overflow-wrap` 3 × track 3의 같은 18개 ID를 사용한다. **통과** |
| 4 | 내부 계약 숫자 버전이 구현 후보만으로 상승하는가 | PR diff는 versioned contract 0059를 수정하지 않는다. 계획은 `0.1.0`을 유지하고 일반 author CSS admission 및 전체 C11.4 지원을 범위 밖으로 둔다. **통과** |
| 5 | 기본 제품 CSS parse 경로에 fixture alias 동작이 들어가는가 | `c11_4_word_break_css` 선언과 stylesheet/inline parse 호출이 `c11-4-grid-track-sizing-fixture-test` cfg 뒤에 있다. 일반 feature 구성에서는 adapter가 연결되지 않는다. **통과** |
| 6 | 새 feature가 crate 사이에서 불일치하거나 기본 앱에 켜지는가 | style/runtime/FFI의 Cargo 연결과 Android/iOS fixture build flag는 기존 C11.4 fixture-test 조합을 재사용한다. 새 기본 feature나 공개 FFI 진입점은 추가되지 않았다. **통과** |
| 7 | parser rewrite가 다른 속성이나 비슷한 텍스트를 바꾸는가 | CSS token parser는 속성명이 `word-break`이고 값이 단일 literal `break-word`인 경우만 기록한다. `content`, custom property, `var()` 및 다른 keyword control이 그대로 남는다. **통과** |
| 8 | 대소문자, 주석·공백, `!important`에서 keyword token 경계가 흐트러지는가 | cssparser token을 읽고 중요도 토큰을 건너뛴 뒤 정확한 값 토큰 range를 저장한다. 대소문자 및 중요도 포함 byte-length 회귀가 있다. **통과** |
| 9 | 치환 뒤 원본 CSS source 위치가 밀려 이웃 선언을 잘못 가리키는가 | `break-word` range 안만 `normal`과 공백으로 덮어 원문 UTF-8 byte 길이를 유지한다. inline과 stylesheet 뒤쪽 선언 offset을 회귀로 확인한다. **통과** |
| 10 | 문자열·함수 인자·selector prelude 속 가짜 선언을 scanner가 수집하는가 | component-value drain은 괄호·함수·문자열 token을 건너뛰고 declaration scan은 rule body에서만 실행한다. 문자열/변수와 중첩 `@media` 단위 검사가 있다. **통과** |
| 11 | 중첩 at-rule 재귀가 제한 없이 진행되거나 일부 CSS만 바꾸고 계속되는가 | nested rule depth 64를 넘으면 scanner가 exceeded 상태를 남기고 최종 출력은 원문 전체를 반환한다. 부분 rewrite를 하지 않는다. **통과** |
| 12 | cascade winner의 원문을 다른 stylesheet나 declaration block에서 가져오는가 | raw-style map은 Stylo style rule의 block pointer와 source location을 연결한다. word-break가 든 block만 회수하며 inline은 동일 Arc identity와 element 원문으로 따로 처리한다. **통과** |
| 13 | source recovery 실패가 상속 alias를 성공값으로 오인시키는가 | winning parsed declaration의 원문 복원 실패는 `Other`로 fail closed한다. 값이 없는 경우만 `None`으로 부모 provenance를 상속한다. **통과** |
| 14 | stylesheet selector 우선순위·rule tree 순서를 다시 계산하다 specificity를 잃는가 | source recovery는 실제 Stylo winning rule path를 사용하고 declaration block 안에서는 뒤쪽 같은 중요도 선언을 찾는다. specificity/source-order 회귀가 연결돼 있다. **통과** |
| 15 | author `!important`와 normal declaration의 winner를 섞는가 | 각 declaration 중요도를 cascade node의 importance와 대조한 뒤 원문에서도 같은 importance의 최종 `word-break`를 복원한다. 양방향 inline 중요도 runtime cases가 있다. **통과** |
| 16 | `inherit`/`unset` 또는 synthetic HTML/body 경로에서 alias가 끊기거나 잘못 누출되는가 | cascade traversal은 부모 alias 상태를 html/body wrapper 및 child queue로 전달하고 명시된 inherit/unset을 부모값으로 처리한다. 미지정·명시 상속 회귀가 있다. **통과** |
| 17 | `revert`가 더 낮은 cascade origin을 무시하는가 | `CascadePriority::allows_when_reverted(Origin)`을 이용해 롤백 후보를 건너뛴다. child author revert와 lower UA normal 사례가 테스트된다. **통과** |
| 18 | `revert-layer`가 lower layer 후보나 inline source를 놓치는가 | Stylo의 layer priority rollback을 사용하고, inline `revert-layer`가 lower stylesheet alias를 드러내는 회귀가 있다. author `@layer` CSS admission은 범위 밖으로 명시했다. **통과** |
| 19 | incremental reuse가 변경되지 않은 노드의 alias provenance를 잃는가 | 재사용 output에 실제 `word-break` computed property가 있을 때만 alias flag를 복구하고 다음 자식에 전달한다. 신규 계산·재사용 경로가 같은 provenance 형태를 쓴다. **통과** |
| 20 | serializer가 원래 속성이 없는 node에 가짜 computed property를 추가하는가 | alias 문자열 override는 Stylo computed property map에 `word-break`가 이미 들어 있을 때만 수행한다. layout source도 이와 같은 cascade 계산 결과를 받는다. **통과** |
| 21 | `word-break: break-word`가 Parley에서 `break-all`처럼 동작하는가 | 모델은 `BreakWord`를 별도 보존하고 Parley에는 `WordBreak::Normal`을 준다. emergency wrapping 효과는 아래 별도 effective overflow-wrap으로 표현한다. **통과** |
| 22 | layout이 원래 계산된 `overflow-wrap` 진단값까지 `anywhere`로 덮는가 | `word-break: break-word`일 때 Parley 입력에만 effective `Anywhere`를 사용한다. paragraph computed mode와 snapshot provenance에는 실제 normal/break-word/anywhere가 남는다. **통과** |
| 23 | `overflow-wrap: break-word` 자체가 min-content 계산에 잘못 포함되는가 | 기존 overflow-wrap intrinsic 분기는 computed `BreakWord`만 Normal로 재측정한다. word-break alias 효과와 분리돼 기존 9-scene/control 회귀가 유지된다. **통과** |
| 24 | `max-content` 또는 고정 폭 wrapping이 alias의 min-content override로 오염되는가 | max-content는 40px이고 definite 10px layout은 effective anywhere로 별도 계산한다. 독립 oracle은 min/max/fixed track 결과를 따로 고정한다. **통과** |
| 25 | FFI generator가 18개 이외 입력, 임의 문구나 다른 구조를 받아들이는가 | fixture FFI catalog는 exact scene ID에서만 source를 만든다. payload는 Ahem 10px `XXXX`, direct Text child와 고정 profile로 생성한다. **통과** |
| 26 | runtime adapter가 scene ID를 CSS의 진실값으로 쓰거나 잘못된 owner/style을 publish하는가 | computed `word-break`와 `overflow-wrap`을 같은 Grid owner snapshot에서 읽고 text input과 비교한다. owner, locale, XML lang, track, duplicate child 및 unknown scene 거부 경로가 있다. **통과** |
| 27 | 성공 trace가 실제 track·line layout·revision과 연결되지 않는가 | runtime test 및 capture validator는 source owner, document/layout revisions, item frame, paragraph locale/font, line count/advances, intrinsic sizes와 final row/column tracks를 함께 검사한다. **통과** |
| 28 | Android와 iOS의 10px 고정 track 인식 또는 기대 조합이 달라지는가 | 두 adapter가 각각 2×3×3 exact matrix를 생성하고 `-10px` suffix를 fixed width로 분류한다. old `-fixed-10px` overflow-wrap 경로도 함께 보존한다. **통과** |
| 29 | capture가 다른 설치 binary를 실행하거나 iOS superseded attempt를 성공으로 숨기는가 | Android APK와 설치 APK, iOS executable과 설치 앱 hash를 비교한다. iOS superseded run은 별도 attempts/log로 남기고 한 번만 재시도하며 앱 종료는 `finally`에서 수행한다. **통과** |
| 30 | 증거·PR 설명이 glyph pixel, 실기기, 전체 CSS 또는 CI 성공을 과장하는가 | evidence는 SwiftShader/Simulator·고정 Ahem 범위를 밝히고 glyph paint가 꺼져 동작 screenshot 근거로 쓰지 않는다. PR은 checks 미보고 상태를 성공으로 부르지 않고 일반 CSS/WPT/실기기/전체 C11을 제외한다. 731줄 runtime test 파일은 같은 18-scene matrix의 공용 `Case`·snapshot builder와 다섯 검증을 한 책임으로 공유하므로 검토했으며, PR 본문에 응집 이유를 덧붙인다. **통과** |

## 결론

이 diff 검토에서 추가 제품 결함은 발견하지 않았다. 앞선 구현/runtime 검토에서 확인한 cascade `revert`/`revert-layer` 처리는 코드와 회귀 경로에서 다시 대조했다. 현재 GitHub PR check rollup은 비어 있으며 CI 성공을 주장하지 않는다. 이 slice는 제한된 opt-in fixture 후보이므로 C11/C11.4 및 공식 `72/213`은 미완료로 유지한다.
