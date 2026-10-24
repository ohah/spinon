# C11.4 auto-repeat `calc()` neighbor PR diff 실패 관점 검토 30개

PR [#583](https://github.com/ohah/spinon/pull/583)의 `main...impl/c11-4-auto-repeat-calc-neighbors-20261024` 변경을 통합·회귀·증거·상태 게시 관점으로 대조했다. 구현/runtime 검토에서 다룬 typed AST 계산, basis binding 및 fail-closed layout 경계는 반복하지 않았다.

| # | 별도 실패 경로 | 변경 diff·실행 evidence 대조와 판정 |
| --- | --- | --- |
| 1 | Taffy gutter 보정이 `auto-fill`까지 건드린다. | 변경은 `RepetitionCount::AutoFit` 조건 안에만 있다. `AutoFill` 분기는 이전대로 track과 gutter를 만든다. 통과. |
| 2 | 일반 단일 track 또는 정수 repeat의 gutter가 바뀐다. | 수정은 auto-repeat `AutoFit` 내부에 한정되고 `Single`·`Count` 경로는 수정하지 않았다. 통과. |
| 3 | auto-fit repeat가 grid 끝에 있을 때 끝 gutter가 남는다. | 기존 `is_last` trailing 처리 경로가 보존됐다. Taffy 코드와 기존 end-alignment 고정 probe를 대조했다. 통과. |
| 4 | 명시 track 앞뒤에 빈 auto-fit run이 생기며 baseline 수치가 낡는다. | Rust explicit-neighbor matrix의 columns·rows·span·suffix 기대값을 shared-gutter 결과로 갱신했다. Chrome 고정 probe의 154px·73px·28px 결과와 맞는다. 통과. |
| 5 | PR #580의 과거 200px runtime 기록을 최신 기준으로 오용한다. | 규범 audit는 이전 값을 보정 전 source 결과로 분리하고 196px 재계산을 현재 기준으로 명시한다. 통과. |
| 6 | 기존 percentage-only 기능의 Taffy 수정 회귀가 검사되지 않는다. | 보정 후 Android API 37 ARM64 AVD의 기존 13-scene matrix를 별도 재실행해 13/13 frame·track·trace·draw/present를 보존했다. iOS 재실행이 없다는 한계도 명시한다. 통과. |
| 7 | 신규 calc 후보의 gutter 동작을 이전 percentage 테스트로 대신한다. | calc candidate는 별도 Android/iOS 14-scene matrix다. 두 플랫폼 각각 14/14이며 이전 matrix와 증거 디렉터리를 분리했다. 통과. |
| 8 | repeat 양쪽에 이웃이 있을 때 앞·뒤 항의 합이 규범 기대값과 다르다. | `col-fill-both` 독립 기대값 `[22,47,47,47,47,10]`과 Chrome 관측을 validator·simulator result에 대조했다. 통과. |
| 9 | prefix-only 또는 suffix-only 장면이 반대 위치에 적용된다. | CSS fixture, 14-case JSON, 독립 기대값과 runner가 각각 `col-fill-prefix`·`row-fill-suffix`를 같은 ID로 공유한다. 통과. |
| 10 | border-box 선언 크기를 percentage basis로 착각한다. | 고정 scene은 136px border-box에서 양쪽 8px padding과 2px border를 제외해 116px content basis를 기록하며 simulator frame도 연결한다. 통과. |
| 11 | row/column을 바꿔 캡처한다. | 두 축의 positive scene ID가 Android/iOS allowlist와 JSON oracle에 분리되어 있고 expected rows/columns 및 item 좌표를 확인한다. 통과. |
| 12 | CSS-valid negative control이 oracle 또는 앱에서 빠져 unsupported 범위가 축소된다. | negative percentage, nested `clamp`, indefinite `max-content` 세 control은 독립 CSS observation과 exact runtime error 계약에 남아 있다. 통과. |
| 13 | negative final result 화면이 layout 성공처럼 보인다. | signed length negative term과 최종 0px clamp는 별도 positive scene으로 확인하고, negative percentage/nested math는 rejection path로 나눈다. 통과. |
| 14 | 기존 fixed-px slice가 새 scene별 helper에서 누락된다. | `calc-neighbor-col-fill-fixed-control`은 ID, track/frame 기대값 및 두 앱 경로에 포함되어 legacy sizing control을 대조한다. 통과. |
| 15 | fixture JSON의 scene 수·순서·oracle 대응이 어긋난다. | Rust catalog test와 JS boundary test가 14 ID 및 중복 부재를 확인하고, capture validator가 fixture→독립 계산→Chrome scene ID·순서를 일치시킨다. 통과. |
| 16 | 전체 C11.4 capture catalog를 418개로 잘라 신규 scene이 list 전용 경로에서 사라진다. | 전체 capture assertion 및 변경된 boundary assertions가 432를 사용하고 선택 시 14 ID를 돌려준다. 변경 범위에서 이전 418 assertion은 남지 않았다. 통과. |
| 17 | 임의 caller가 신규 runtime fixture를 제품 ABI로 실행한다. | scene builder JSON은 opt-in fixture module의 `include_str!`이며 dispatcher route도 `c04-runtime-gpu-fixture-test` 아래에 있다. 기본 runtime 경로를 새로 열지 않았다. 통과. |
| 18 | fixture 소스 위치만 바뀌고 feature 경계가 깨진다. | fixture-boundary test의 canonical list, module declaration과 compile-time feature gate를 확인했다. 제품/fixture-test archive 비교에서도 product 0/14·symbol absent, positive control 14/14·symbol present다. 통과. |
| 19 | 미등록 scene 이름이 빈 성공 입력으로 바뀐다. | Rust fixture builder와 C11.4 dispatcher는 미등록 ID에 각각 `None`·error를 반환하고 catalog test가 두 실패 경로를 고정한다. 통과. |
| 20 | Android 실행기가 rejection summary를 기다리지 못하고 timeout한다. | `waitForLog`는 예상 rejection summary를 완료 조건으로 포함하며 verifier는 기대 code/property/node 및 frame 부재를 별도 확인한다. 통과. |
| 21 | Android rejection이 이전 scene의 frame/pixel 성공을 재사용한다. | error branch는 positive summary 검증보다 먼저 실행되고 expected rejection이면 scene 처리에서 반환한다. 세 control의 AVD log는 frame 부재를 기록한다. 통과. |
| 22 | rejection scene의 스크린샷이 성공 화면 증거로 첨부된다. | Android `screencap`과 iOS `simctl screenshot`은 layout positive scene에만 실행한다. rejection은 결과 log로 남긴다. 통과. |
| 23 | PixelCopy fixture가 빠진 표본이나 중복 표본을 통과시킨다. | Android 검사기는 표본 수, 각 sample ID, target node, CSS coordinate를 확인하고 overflow의 root 안쪽·바깥쪽·item 점을 별도 필수화한다. 통과. |
| 24 | iOS Metal 표본 검사만 배열 길이를 확인하고 잘못된 좌표/색을 통과시킨다. | Swift 검사는 전체 tuple 개수와 ID·node·좌표·RGBA를 비교한다. overflow 경계 세 점도 별도 기대값에 들어 있다. 통과. |
| 25 | 화면에 item이 root 바깥으로 배치됐지만 화면만으로 overflow를 단정한다. | fixture frame은 x=244이고 240px root 내부 x=239, 바깥 x=240 및 item 내부 x=244의 PixelCopy/Metal 표본을 각 플랫폼에서 5/5 확인한다. 통과. |
| 26 | 14-scene 성공률을 전체 simulator matrix 또는 제품 CSS admission으로 확장한다. | runtime README는 이 결과를 고정 opt-in fixture로 제한하고 일반 CSS admission, WPT, 실기기, hardware GPU 및 iOS compositor 이후 pixels를 미검증으로 적었다. 통과. |
| 27 | raw screenshot 링크 오타 때문에 PR 본문에 화면이 표시되지 않는다. | PR 본문 Android 1080×2400·iOS 1206×2622 PNG URL을 pushed branch에서 Agent Browser로 각각 직접 열어 이미지 디코딩을 확인했다. 확인 session을 닫았다. 통과. |
| 28 | 내부 문서 링크가 로컬에만 있어 PR에서 404가 된다. | plan의 초기 capture/error-mapping 링크에 해당하는 pass01/pass02 evidence를 PR 변경 집합에 포함했다. 현재 runtime·artifact README도 함께 포함한다. 통과. |
| 29 | 새 slice를 전체 C11.4 완료 또는 공식 수 증가로 표시한다. | `STATUS.md`, 공개 roadmap과 집중 계획은 parent를 미완료로 두고 `72/213`을 유지한다. C13 `overflow·scroll`은 이 lane과 별개인 미완료·비활성으로 확인했다. 통과. |
| 30 | 공개 roadmap이 현재 문서와 다른 revision을 보여 사용자가 진행 상태를 오해한다. | PR 변경 전 Tailnet pass12에서 667/667 HTTP 200·build/local/served SHA-256 일치를 확인했고 Agent Browser DOM에서 72/213·C11.4·C13 상태를 대조했다. 이후 사용자 요청에 따른 availability 재확인에서도 preview와 Funnel이 살아 있었다. pass13 결과는 PR diff에 포함하지 않은 local sync log에 기록했다. 통과. |

## 검토 결과

- 별도 구현/runtime 실패 관점 검토와 다른 30개 통합 실패 경로를 살폈고, 병합을 막는 변경 결함은 찾지 못했다.
- 증거가 실제로 입증하는 범위는 Android API 37 ARM64 AVD, iPhone 17 Pro/iOS 26.2 Simulator, fixed fixture와 macOS ARM64 static FFI archive다. 최종 APK/IPA의 fixture 배제, 실기기, Android hardware GPU, iOS compositor 이후 표시는 포함하지 않는다.
- `cargo fmt --all -- --check`는 현재 commit에서 통과했다. 이 검토 turn에서는 새 Rust/app test를 실행하지 않았고 GitHub는 status check을 보고하지 않았다. 이전 focused test와 simulator 결과는 [현재 runtime evidence](./c11-4-auto-repeat-calc-neighbors-projection-pass03-2026-10-24/README.md)에 연결했다.
