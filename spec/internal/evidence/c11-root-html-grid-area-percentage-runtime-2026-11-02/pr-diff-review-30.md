# C11.1 named-area percentage · PR 변경 diff 실패 관점 검토 30개

검토 대상은 [PR #1035](https://github.com/ohah/spinon/pull/1035)의 main 대비 코드, Root HTML precomparison fixture/manifest, 실행 로그·캡처, plan/status/내부 계약 변경이다. 구현/runtime 검토표를 그대로 반복하지 않고 PR diff가 추가로 만들 수 있는 계약·추적성·게시 오류를 분리해 확인했다. PR 검토 화면의 두 raw screenshot도 Agent Browser에서 실제 이미지 응답 크기까지 확인했다.

| # | 독립 관점 | 결과 |
| ---: | --- | --- |
| 1 | 기본 Root HTML 성공 경로 밖의 computed-style profile까지 바뀌는가 | admission은 named-area validator에만 추가됐고 기존 profile 변경은 없다. |
| 2 | named-area가 없는 percentage Grid 경로가 named-area 예외에 걸리는가 | `template_areas` 존재 시에만 새 gate가 쓰인다. 기존 일반 Root HTML percentage 회귀 15개가 통과했다. |
| 3 | direct track 두 개 외 list가 우회되는가 | 두 축 모두 정확히 2개 direct px/%만 새 분기를 통과한다. |
| 4 | percentage가 하나도 없는 px list가 새 분기로 우회되는가 | percentage 존재 gate가 false이고 기존 exact fixed-px 경로만 유지된다. |
| 5 | 한 축에만 percentage가 있고 다른 축은 px일 때 typed projection이 가능한가 | projection positive case에 percentage columns + fixed-px rows 조합이 있다. |
| 6 | 기존 fixed-px named-area geometry가 달라지는가 | 원래 map/track branch와 회귀 test를 유지했다. |
| 7 | 기존 named-area `1fr 2fr`와 새 `%` 조건의 우선순위가 바뀌는가 | 별도 fixed fraction branch와 regression 5건이 유지된다. |
| 8 | `%`+`fr`를 직접 px/% branch가 받아들이는가 | direct track matcher가 `Fraction`을 거부하고 runtime negative도 통과한다. |
| 9 | NaN·infinite·negative typed percentage가 track shape check를 우회하는가 | 공통 Root HTML track validation이 새 named-area matcher 전에 finite/non-negative를 검사한다. |
| 10 | percentage basis의 width/height 축을 교환하는가 | runtime snapshots는 columns width 120px, rows height 70px basis를 확인한다. |
| 11 | min/max override가 indefinite axis를 clamp해도 성공하는가 | 네 override가 fail-closed다. 잘못되게 `min-width`만 반환하던 기존 diagnostic을 수정했고 각 위반 속성을 검사한다. |
| 12 | aspect-ratio가 percentage basis를 재정의하는가 | named-area percentage runtime negative가 cascade/property diagnostic과 부분 scene 부재를 확인한다. |
| 13 | percentage gap이나 gap 분배가 track size를 바꾸는가 | area scene은 gap 0만 허용하며 percentage gap 입력은 거부된다. |
| 14 | template-area map의 크기만 맞고 모양/이름은 다른 입력이 통과하는가 | exact header/side/content rectangle topology와 dimensions를 유지한다. |
| 15 | 중복/누락 semantic area가 같은 item으로 오인되는가 | map entry 수와 각 기대 rectangle 존재를 함께 확인한다. |
| 16 | direct child와 area item source order를 혼동하는가 | fixture DOM order는 content/header/side이고 frame lookup은 semantic color/area를 따른다. |
| 17 | zero-width area item이 빠져 결과 frame 검증이 무력화되는가 | layout frame fallback으로 폭 0을 검사하고 draw box 5개는 의도된 renderer 동작으로 기록한다. |
| 18 | over-100%를 clamp하거나 root frame을 키워 overflow를 숨기는가 | 144px track, x=144 item, 120px root frame을 Rust test가 각각 대조한다. |
| 19 | `calc()`/`minmax()`/invalid negative CSS가 direct tokens로 오인되는가 | rejection matrix에서 실패와 property 진단을 확인한다. |
| 20 | 실패 scene에서 이전/부분 render snapshot을 노출하는가 | negative tests는 `completed=None`을 확인하고 기존 stale-revision regression도 유지한다. |
| 21 | document/style/layout revision이 서로 다른 snapshot에서 합쳐지는가 | positive test는 cascade 완료 key와 layout key가 같음을 검사한다. |
| 22 | source fixture include가 제품 binary/ABI에 고정 scene payload를 넣는가 | 신규 `include_str!`는 test module에만 있으며 FFI/fixture adapter/feature/ABI 변경이 없다. |
| 23 | Android/iOS 캡처가 fixture-test route에서 성공한 것처럼 보이는가 | build logs에는 fixture-test flags off, runtime logs에는 기본 Root HTML app boot marker가 있다. |
| 24 | simulator case가 같은 HTML을 실행했다고 오인하는가 | 양 플랫폼 matrix는 scene별 HTML SHA를 기록하고 paired values가 일치한다. |
| 25 | Android root screenshot/log와 Rust item geometry를 같은 증거로 합치는가 | 문서는 앱 log가 item frame을 제공하지 않는다고 구분하고 geometry authority를 Rust snapshot에 둔다. |
| 26 | iOS Simulator PNG를 Metal drawable readback이나 최종 compositor 출력으로 주장하는가 | README와 PR은 화면 캡처를 시각 증거로 한정하고 두 readback 모두 실행하지 않았다고 명시한다. |
| 27 | AVD SwiftShader를 물리 Android/hardware GPU와 혼동하는가 | build/runtime 정보는 ANGLE/Vulkan/SwiftShader software path를 명시한다. |
| 28 | 첫 Android overflow의 APK hash 누락을 감추는가 | `matrix-hashes.txt`와 README가 APK hash 미수집을 표시하고 HTML source hash는 보존한다. |
| 29 | 정리한 로그가 raw evidence인 것처럼 제시되거나 실패 harness가 숨겨지는가 | `.log`의 공백 정리와 `.raw.gz` 보존을 설명하고 zero-track 첫 harness 결과도 남겼다. |
| 30 | 이미지/roadmap/상태 링크가 빠지거나 보이지 않는가 | PR #1035 본문에 Android/iOS raw image를 넣었다. Agent Browser에서 두 이미지가 `complete=true` 및 1080×2400/1206×2622로 로드됐다. 상태 대장·두 plan·내부 계약은 같은 candidate evidence를 가리킨다. |

**수정 후 결과:** review에서 찾은 두 경계(max/min diagnostic attribution, zero-width paint box count)는 코드/test와 evidence 문서에서 바로잡았다. 최종 focused Rust tests, scoped Clippy, rustfmt, diff check 및 양 simulator의 4-scene runtime 결과를 재확인했다. PR check rollup은 비어 있고 main branch에 required status checks가 설정되어 있지 않으므로 CI 통과로 세지 않는다.

이 diff는 C11.1/C11, 일반 CSS Grid admission, WPT, Android physical GPU, iOS device 또는 pixel-perfect surface output을 완료하지 않는다.
