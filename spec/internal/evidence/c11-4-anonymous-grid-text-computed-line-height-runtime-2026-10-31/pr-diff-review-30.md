# C11.4 anonymous Grid text computed `line-height` PR diff 실패 관점 30개

계획 검토 20개 및 구현/runtime 검토 20개와 분리해, 제출할 전체 source·harness·근거·상태 변경을 기준으로 검토했다. 앱 행렬 결과는 최종 코드와 변경된 capture runner로 최신 실행했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 신규 dispatch가 default product feature에서도 runtime scene을 노출한다 | **통과.** 새 FFI matrix module은 fixture-test parent 아래에서만 컴파일되며 기본 archive에서 16 ID marker 부재를 확인했다. |
| 2 | fixture-only Rust source가 `fixtures/` 경계를 벗어나거나 이전 경로 사본과 중복된다 | **통과.** 기존 fixture 모듈 파일에 child module만 연결했고 fixture module-boundary 검사 24/24를 통과했다. |
| 3 | runtime adapter 변경이 다른 C11.4 anonymous-text scene의 child contract를 느슨하게 만든다 | **통과.** 신규 16 ID에만 direct text shape를 추가했고 기존 C11.4 `c114_` regression 23/23이다. |
| 4 | 기본 release archive에 신규 scene ID 문자열이나 payload가 유입된다 | **통과.** `c04-runtime-gpu` archive에서 16 marker 모두 부재다. |
| 5 | opt-in positive control이 scene 목록을 빠뜨려 default 부재 검사가 공허해진다 | **통과.** fixture-test archive에 16 marker 모두 있고 FFI generator test 1/1이다. |
| 6 | FFI·runtime allowlist·precomparison 기대값 수가 서로 다르다 | **통과.** 모두 16개다. Rust uniqueness assertions와 최종 양 플랫폼 capture가 전 장면을 실행했다. |
| 7 | app route가 새 scene ID를 C11.4 Grid harness로 분류하지 않는다 | **통과.** Android/iOS scene dispatch에 16 ID를 등록했고 두 Simulator matrix가 각 16/16이다. |
| 8 | 고정 JS scene 생성 값이 코드 삽입으로 임의 변형될 수 있다 | **통과.** ID·style·text는 상수 목록에서만 오고 unknown ID는 `None`이다. |
| 9 | newline·single-line·soft-wrap 장면에 다른 text shape를 넣어 line count를 맞춘다 | **통과.** FFI source test와 runtime layout assertion이 normalized text와 줄 수를 각각 검사한다. |
| 10 | scene ID가 computed `line-height`를 대신 선택하고 실제 Stylo snapshot은 무시한다 | **통과.** adapter는 같은 computed-style snapshot의 실제 property 값을 읽어 검증·전달한다. unit integration이 16 computed value와 revision을 확인한다. |
| 11 | 허용 가능한 다른 serialization이 해당 fixed scene의 기대 결과처럼 통과한다 | **수정 후 통과.** scene별 serialization mismatch를 reject하고 negative integration test가 layout 발행 전에 거부를 확인한다. |
| 12 | unitless 숫자를 absolute px와 혼동하거나 font-size basis를 놓친다 | **통과.** unitless 값은 고정 10px font size와 곱하고 length는 그대로 사용한다. 결과 line metrics 및 track을 검사했다. |
| 13 | `%`, `em`, `calc()` author value를 adapter에서 중복 계산해 cascade 결과와 달라진다 | **통과.** adapter는 computed `12.5px`만 소비하고 세 author source 모두 Chrome/Stylo reference 기대값과 양 플랫폼 결과가 일치한다. |
| 14 | `normal`을 number/length처럼 취급해 normal flag를 잃는다 | **통과.** `normal` variant를 별도로 넘기고 Ahem 관찰 환경에서만 10px metric을 비교한다. |
| 15 | parser가 음수, 비유한 수, overflow, 잘못된 단위를 허용한다 | **통과.** parser negative set과 finite/range guard가 해당 입력을 거부한다. |
| 16 | 0px paragraph line-height가 validation 또는 line geometry를 깨뜨린다 | **통과.** ParagraphStyle 4/4에서 0-height 2-line 결과와 유한 metrics를 검사한다. |
| 17 | ParagraphStyle 변경이 span 전용 inline-run contract를 무심코 완화한다 | **통과.** `InlineRunStyle` 0 거부 assertion이 유지되며 C15 internal contract에 경계를 기록했다. |
| 18 | 기존 55 scene의 이전 computed `10px` invariant가 제거된다 | **통과.** 신규 목록 밖의 recognized legacy scene은 `10px` serialization을 요구하고 C11.4 regression을 실행했다. |
| 19 | unknown/unsupported scene이 generic fallback 결과를 받아 layout을 발행한다 | **통과.** adapter allowlist 밖 ID, 누락 text와 malformed profile은 fail closed한다. |
| 20 | newline과 collapsible space의 whitespace profile이 line count/inline size를 바꾼다 | **통과.** forced-break scenes와 soft-wrap scenes의 `PreLine`/`Collapse` 구분 및 normalized text를 검사했다. |
| 21 | 16 scene 중 한 장면이 auto row sizing 또는 gutter 제외 final track과 다르다 | **통과.** Rust trace는 마지막 Rows pass의 gutter 제외 track 및 max-content contribution을 16 case 전부 대조한다. |
| 22 | soft-wrap의 inline/block min/max contribution을 동일 값으로 오인한다 | **통과.** runner와 플랫폼 matcher가 min/max inline 및 block 네 값을 각각 확인한다. 1.5 soft-wrap의 `30/15px` block 차이도 검사했다. |
| 23 | text item frame이 root에 붙지 않거나 폭·원점이 잘못돼도 높이만 맞아 통과한다 | **통과.** 최종 runner가 item x/y/width/height를 모두 `[0,0,20,expected]`로 대조한다. |
| 24 | Android harness가 JSON parse 오류나 빠진 intrinsic revision을 무시한다 | **통과.** Android matcher는 source, revision, text source, owner/order와 네 intrinsic 값을 모두 요구한다. |
| 25 | Android 정수 기대 frame이 5px/25px 케이스를 잘못 반올림한다 | **통과.** 16 frame 모두 정수 CSS px이며 API 37 ARM64 AVD에서 실제 summary와 trace를 비교했다. |
| 26 | iOS harness가 Android ID 목록/geometry에서 벗어나거나 float 오차를 무시한다 | **통과.** 동일 16 IDs와 geometry를 쓰고 trace intrinsic 값에 1/64 CSS px 오차만 허용한다. |
| 27 | trace chunk 중복·누락이 합쳐져 JSON이 완전한 것처럼 보인다 | **초기 gap 수정 후 통과.** runner가 실제 조각 수와 고유 index 수 모두 total인지 검사하며 duplicate chunk를 거부한다. 수정 runner로 Android/iOS 전체 matrix를 재실행했다. |
| 28 | 다른 설치 APK/앱 executable로 성공 결과를 현재 빌드에 귀속한다 | **통과.** Android APK 및 iOS Simulator executable의 설치 hash가 빌드 hash와 각각 일치한다. |
| 29 | 다른 OS/Simulator 환경 결과를 요청한 대상의 증거처럼 표기한다 | **통과.** Android API 37 ARM64 AVD, iPhone 17 Pro/iOS 26.2 Simulator metadata를 저장했고 실기기 결과로 확장하지 않는다. |
| 30 | 약한 screenshot이나 전체 docs build 실패를 성공 근거로 과장한다 | **통과.** iOS screenshot은 route/status만, 검은 Android screenshot은 PR에서 제외한다. docs build는 저장소 dead-link 실패로 기록하고 이번 line-height link는 분리했다. C11/C11.4와 공식 `72/213`은 미완료로 둔다. |

**판정:** 별도 30개 PR diff 관점을 확인했다. 발견된 trace 중복 수용 gap을 고쳐 최신 Android AVD 16/16 및 iOS Simulator 16/16을 다시 통과했다. Rust focused/regression 검사, workspace Clippy, fixture boundary, 기본/opt-in archive 검사와 변경 whitespace 검사는 통과했다. `docs:build`는 현재 저장소의 1,131개 문서에서 보고된 dead link 때문에 실패하며 전체 통과로 표시하지 않는다.
