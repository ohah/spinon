# C11.4 양 축 auto-repeat + areas 계획 PR diff 적대 검토 30개

**검토 대상:** `plan/c11-4-two-axis-template-areas.md`, `plan/c11-4-two-axis-template-areas-plan-review-20.md`, 9-scene HTML fixture, CSS Grid 규범 기대값, Chrome 156 observation, screenshot, hash manifest.

**검토 기준:** 계획 자체 20관점 표를 재사용하지 않고 PR diff의 정확성·범위·근거·재현성·운영 영향만 별도로 대조했다. 30관점 모두에서 계획 PR을 막는 결함은 발견하지 못했다. 이 결과는 구현/runtime 완료를 뜻하지 않는다.

| # | PR diff 실패 관점 | 확인 결과 | 판정 |
| --- | --- | --- | --- |
| 1 | 계획 문서의 상위 항목 링크가 다른 C11 단계로 연결된다. | C11.4 전체 계획, 기존 two-axis repeat·area 계획, 내부 계약과 STATUS anchor를 확인했다. | 통과 |
| 2 | 계획 PR이 공식 상태를 완료로 승격한다. | 계획은 `77/213`과 C11.4 미완료 checkbox를 유지한다고 명시한다. | 통과 |
| 3 | 2025-03-26 W3C 문서가 Recommendation으로 잘못 표기된다. | Candidate Recommendation Draft로 표시하고 고정 URL을 연결했다. | 통과 |
| 4 | Auto-repeat 계산이 gap 또는 content box를 무시한다. | CSS Grid §7.2.3.2 근거와 100/76/40px의 수식·반복 수를 고정했다. | 통과 |
| 5 | 두 track list의 auto-repeat 한계가 한 축 내부 반복 횟수 제한을 넓힌다. | 각 list에 하나씩만 허용하고 같은 list의 추가 repeat은 fail-closed한다. | 통과 |
| 6 | Area matrix 크기와 expanded template 크기를 혼동한다. | 큰 dimension이 explicit grid를 확장하는 규범과 독립 기대를 분리했다. | 통과 |
| 7 | Area가 추가한 explicit track의 sizing source가 불분명하다. | 고정 추가 track에만 `grid-auto-columns:16px`/`grid-auto-rows:12px`를 사용한다고 적었다. | 통과 |
| 8 | Fixture의 area strings가 비직사각형이나 행 길이 불일치를 포함한다. | 9 scene의 각 matrix가 valid rectangular CSS syntax인지 fixture에서 확인했다. | 통과 |
| 9 | Area가 expanded grid보다 짧은 control이 빠진다. | `area-short`는 2×2 area와 4×3 repeat list를 대조한다. | 통과 |
| 10 | Area가 columns만 확장하는 경계가 빠진다. | `area-added-columns`와 40×76px root 및 세 track 결과를 고정했다. | 통과 |
| 11 | Area가 rows만 확장하는 경계가 빠진다. | `area-added-rows`와 76×40px root 및 세 row 결과를 고정했다. | 통과 |
| 12 | Area가 양 축을 확장할 때 한 축 metadata만 처리한다. | `area-added-both`에서 두 axis의 auto-size extra tracks와 item frame을 고정했다. | 통과 |
| 13 | auto-fill과 auto-fit의 차이를 합쳐 기록한다. | 각 repeat mode를 독립 fixture scene과 per-axis track expectations로 나눴다. | 통과 |
| 14 | Empty named area가 실제 item 없이 track을 점유한다. | `fit-fit-empty`는 item이 없고 모든 repeated tracks가 0px인 기대값이다. | 통과 |
| 15 | Spanning area의 빈·occupied track 판정이 틀린다. | `fit-fit-span`의 item은 2×2 area에 stretch해 정확히 44×44px가 된다. | 통과 |
| 16 | Collapse 뒤 gutter가 유지되어 line alias 위치가 틀린다. | CSS Grid collapse 결과를 track arrays와 root-relative frame에 함께 대조한다. | 통과 |
| 17 | Area-generated line names가 한 axis에서만 검사된다. | `area-longhand`는 두 축의 `hero-start`/`hero-end`를 별도로 사용한다. | 통과 |
| 18 | Named area shorthand와 longhand placement가 같은 경로라고 과장한다. | shorthand/area frame scene과 longhand scene을 별 ID로 유지한다. | 통과 |
| 19 | Runtime fixture가 CSS oracle의 frame으로부터 expected를 역산한다. | `expected-values.json`은 규범 수식으로 먼저 작성하고 Chrome 결과는 별도 JSON이다. | 통과 |
| 20 | Chrome observation이 계획 중 새 수정으로 낡아진다. | 최종 CSS를 다시 열어 9/9 observation을 저장하고 screenshot을 다시 캡처했다. | 통과 |
| 21 | User-agent가 실제로 Chrome 156인지 알 수 없다. | Agent Browser doctor의 full build `156.0.8078.48`과 navigator UA를 함께 기록했다. | 통과 |
| 22 | Screenshot이 simulator 화면 또는 Spinon 동작이라고 오인된다. | README와 plan은 Chrome screenshot을 CSS oracle로만 분류한다. | 통과 |
| 23 | Overflow sample이 grid 경계 안쪽만 확인한다. | 40×40 scene item은 x=44..52, y=40..48이고 native sample points가 경계 밖을 포함한다. | 통과 |
| 24 | Native surface clear를 Chrome page background로 가정한다. | 계획은 native capture의 host clear를 별도 관찰하도록 한다. | 통과 |
| 25 | PixelCopy/Metal 결과가 border stroke나 compositor-after pixels로 확대된다. | AVD software path와 iOS pre-compositor readback 한계를 계획에 적었다. | 통과 |
| 26 | 실기기 검증이 시뮬레이터 선택을 우회한다. | Android `emulator-*`와 iOS Simulator를 지정하고 physical device는 제외했다. | 통과 |
| 27 | Product FFI에 fixture payload가 실수로 들어간다. | Default archive negative와 opt-in fixture-test positive control을 요구한다. | 통과 |
| 28 | Invalid area/oversized metadata가 partial frame을 남긴다. | checked conversion, byte/track bounds, allocation/error atomicity를 게이트에 넣었다. | 통과 |
| 29 | Tailnet sync가 선택 파일 검증 또는 Agent Browser 종료를 빠뜨린다. | hash manifest, HTTPS bytes/hash, rendered route와 process cleanup을 별도 관문으로 둔다. | 통과 |
| 30 | 계획·구현·sync PR review를 합산하거나 하나를 생략한다. | 계획/구현/sync PR diff 각각 30관점, 구현 runtime은 새 20관점으로 분리한다. | 통과 |

## 종료 상태

- Chrome 156과 독립 규범 기대값: 9/9 일치.
- 계획 자체 실패 검토: 독립 20관점 완료.
- 계획 PR diff 실패 검토: 독립 30관점 완료.
- 구현, Rust 회귀, Android/iOS Simulator, FFI artifact 검증: 미실행.
- Agent Browser session을 닫았고 뒤이은 doctor 확인에서 daemon이 없었다. 별도 process scan 결과도 함께 저장한다.
