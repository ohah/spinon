# C11.1 zero-minimum auto-repeat 구현·runtime 적대 검토 20개

**검토 시점:** 2026-11-06 · **검토 대상:** 현재 후보 작업 트리 · **상태:** 20개 관점 검토 완료, 구현 PR은 아직 없음.

계획 실패 관점 20개와 분리해 실제 변경 코드, parsed Root HTML runtime, Android/iOS test route와 artifact 경계를 확인했다. Chrome 157은 고정 fixture의 관찰 oracle로만 사용했다.

## 20개 실패 관점

| # | 공격 관점 | 확인 및 결과 |
|---:|---|---|
| 1 | 선언 원문 검색이나 fixture ID만으로 0px를 허용하는가 | cascade는 parsed Root HTML의 typed computed track 값에서만 값을 얻는다. `root_html_auto_repeat_track`의 제한을 통과한 track만 다음 단계로 간다. `spinon-style` auto-repeat 46개와 Root HTML runtime 6개가 통과했다. |
| 2 | 허용 문법이 기존 제한을 우회하는가 | auto-repeat 한 축, 1–2개 `minmax(finite non-negative px, 1fr)` track, 반대 축 fixed px 조건을 유지한다. neighbor, 두 축 repeat, named/special placement와 dense/column flow 거부 테스트가 통과했다. |
| 3 | 음수 minimum이 0으로 바뀌어 허용되는가 | typed `-0.01px`는 거부된다. CSS invalid declaration fallback은 기존 cascade 동작으로 남고, 구현은 음수 값을 clamp하지 않는다. |
| 4 | NaN·무한대·아주 큰 gap에서 float-to-integer 변환이 잘못되는가 | minimum·axis·gap 유한성을 확인하고, denominator가 유한 양수인지 검사한 뒤에만 count를 정수화한다. NaN minimum과 `f32::MAX` gap 거부 테스트가 통과했다. |
| 5 | 1px UA floor가 실제 track minimum으로 새어 들어가는가 | floor는 count 입력에만 적용한다. projection 테스트는 Taffy 입력의 minimum이 `LengthPx(0.0)`인 것을 확인하고, runtime 11px case의 첫 track은 1px로 계산된다. |
| 6 | 반복 수 하한과 단일 track의 exact boundary가 틀리는가 | `max(1, floor((A + G) / (S + P × G)))`를 사용한다. 1-track integer boundary, 최소 1회 반복, 11px·gap 0 및 gap 1 기대값이 독립 식과 일치한다. |
| 7 | 반복 사이 gutter 또는 pattern 내부 gutter가 누락되는가 | `P × G`를 denominator에 포함한다. 11px/gap 1px의 `[0]`은 6 tracks, 19px/gap 1px의 `[0,2]`는 4 repetitions/8 tracks다. Chrome, projection, layout, runtime 결과가 일치한다. |
| 8 | 두 track pattern의 순서나 최소값이 바뀌는가 | `[0,2]` pattern은 Chrome과 runtime에서 `[1,2] × 4`로 유지된다. `[0,0]`은 5 repetitions/10 tracks다. zero minimum을 양수로 덮어쓰거나 track을 정렬하지 않는다. |
| 9 | fractional px와 gap에서 중간 `f32` 나눗셈이 count를 바꾸는가 | count 산술은 입력 `f32` 값을 `f64`로 승격해 계산한다. 12.5px/gap 0.5px은 8개·1.125px씩이며 Chrome과 일치한다. |
| 10 | rows가 columns의 크기나 gap을 잘못 사용하는가 | axis 선택에 맞는 fixed size와 gap을 쓴다. 7px row axis/gap 1px은 4개다. layout의 row helper와 runtime fixture가 통과했다. |
| 11 | `auto-fill`이 비어 있는 반복 track을 제거하는가 | placement 전 track 수를 유지한다. layout test가 gap 1px에서 item origin `0,2,4,6,8,10`을 확인하고 Chrome은 6개 track을 관찰했다. |
| 12 | `auto-fit`이 점유된 track까지 collapse하는가 | empty-track collapse는 placement 뒤에 적용된다. 11px container의 item frame은 `(0,0,11,2)`이며 나머지 반복 track만 0px로 collapse한다. |
| 13 | item이 없는 `auto-fit`이 count floor를 잃는가 | Chrome 157은 empty case의 11개 track 모두 0px로 관찰했다. 같은 case가 고정 Root HTML runtime fixture 안에서 parse·layout된다. 지원 범위는 fixture와 count unit test로 한정한다. |
| 14 | single-track quota의 정확한 상한을 잘못 거부하는가 | 10,000개 track은 허용한다. projection과 layout capacity helper의 exact ceiling test가 통과한다. |
| 15 | 두 track pattern의 exact quota가 잘못 거부되는가 | `[0,0]`, width 10,000px은 5,000 repetitions/10,000 tracks로 허용한다. 이번 검토에서 경계 검사가 빠진 것을 찾아 projection/layout 테스트에 추가한 뒤 통과를 확인했다. |
| 16 | quota 초과를 조용히 clamp해 잘못된 frame을 만드는가 | count가 상한을 넘으면 layout overflow로 거부하고 `.min(GRID_MAX_TRACKS)`로 줄이지 않는다. single pattern 10,001개와 two-track pattern 10,002개 거부 테스트가 통과했다. |
| 17 | 제한 track count 뒤 cell-capacity 곱셈이 overflow하는가 | expanded track 수와 반대 축 track 수의 곱은 `checked_mul`로 검사하고 실패를 `GridTrackExpansionOverflow`로 돌린다. `spinon-layout` auto-repeat 74개 회귀에서 track-ceiling 및 capacity 경로가 통과했다. |
| 18 | 한 case의 실패가 부분 layout/scene을 publish하거나 이전 scene을 새 결과처럼 보이는가 | over-limit 및 미지원 입력은 새 frame/scene publish 전에 실패한다. `unsupported_auto_repeat_inputs_fail_without_publishing_partial_scenes`와 over-limit layout test가 통과했다. |
| 19 | 새 허용 범위가 C11.4 track-sizing 또는 다른 repeat profile로 새는가 | 새 zero-min allowance는 `RuntimeRootHtmlTextPaintV1`의 strict Root HTML auto-repeat route에 묶인다. C11.4 style-to-layout track/area 22개와 intrinsic-row runtime 15개, 전체 layout auto-repeat 74개가 통과했다. |
| 20 | 앱 fixture가 기본 artifact에 남거나 화면 로그가 다른 fixture를 가리키는가 | Android 기본 APK와 새 iOS 설치의 fixture-off route는 disabled marker를 냈고, opt-in 양쪽 앱은 원본 fixture SHA-256 `a10c8192…418c4`, `layout=ready boxes=21`, draw/present를 기록했다. 양쪽 build script의 invalid flag와 C04 prerequisite도 exit 2로 거부한다. iOS 첫 route 기록은 이전 설치 PID의 opt-in 로그였으므로 채택하지 않고 Simulator 앱을 제거 후 재설치해 다시 확인했다. |

## 검토 중 발견해 수정한 항목

- Android Gradle fixture-off 경로가 생성 파일 `delete()` 결과를 무시했다. opt-in 빌드 뒤 기본 빌드에서 stale payload가 남을 수 있어 `Files.deleteIfExists`로 바꾸고 opt-in → 기본 → opt-in 순서를 검증했다. 이때 처음 쓴 `java.nio.file.Files`는 Gradle Kotlin DSL의 `java` extension과 이름이 충돌해 빌드가 실패했다. `java.nio.file.Files`를 import하고 `Files.deleteIfExists`를 호출하도록 고친 뒤 기본·opt-in 빌드 모두 통과했다.
- 첫 runtime test filter `root_html_auto_repeat`은 모듈 이름과 함수 이름의 공백 때문에 새 zero-minimum 테스트 4개를 포함하지 않았다. 모듈 전체 filter `root_html_grid_auto_repeat`로 다시 실행해 6/6 통과를 확인했다.
- iOS Simulator에서 앱 업데이트 설치 직후 관찰한 이전 opt-in PID 로그는 기본 bundle 경계 증거로 쓰지 않았다. 앱을 제거하고 기본 build를 새로 설치한 PID에서 `fixture-asset-not-packaged`를 확인한 뒤 opt-in build도 새 설치해 성공 경로를 확인했다.

## 근거

- [실행·빌드·hash·플랫폼 제한](./README.md)
- [Chrome 157 10-case 관찰](./chrome157-runtime-observation.json)
- [최종 Android AVD runtime log](./android-avd-final-runtime.log)
- [최종 iOS Simulator runtime log](./ios-simulator-after-fix-runtime.log)
- [Android/iOS screenshot](./android-avd-final.png), [iOS Simulator screenshot](./ios-simulator-after-fix.png)

이 검토는 C11.1/C11 완료, 전체 CSS Grid 지원, WPT 적합성, Android hardware GPU 또는 iOS compositor 이후 pixel equality를 주장하지 않는다.
