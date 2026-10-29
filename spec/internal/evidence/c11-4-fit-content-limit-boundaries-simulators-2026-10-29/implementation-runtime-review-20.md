# C11.4 `fit-content()` limit 경계 구현·런타임 검토 20개

2026-10-29 · 계획 검토를 재사용하지 않고 이번 구현과 최종 Android/iOS 실행 경로를 별도로 살폈다. 기준은 pinned WPT `fit-content-track-sizing.html` 세 assertion, CSS Grid 2025 CRD §11.2/§11.5.1, 최종 AVD·Simulator 원본 로그와 결과 JSON이다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 비슷한 문자열·접두 scene ID가 새 fixture로 받아들여지는가 | FFI dispatch는 세 완전 일치 ID만 반환한다. 접미사 주입 및 미등록 ID 거부를 Rust 경계 테스트에서 확인했다. |
| 2 | 기본 runtime에서 fixture source나 진단 필드가 노출되는가 | 새 소스 모듈은 `c04-runtime-gpu-fixture-test` 경계 아래 있고 feature 없는 정적 archive marker scan은 0건이다. |
| 3 | 루트 바깥의 엉뚱한 DOM 또는 다른 root parent를 통과시키는가 | runtime validator가 HTML `div`, 정확한 root ID, `HostParent::Root`를 요구한다. |
| 4 | item/child가 재배치되거나 중간 wrapper가 들어가도 통과하는가 | 각 레벨 자식 수 1, direct parent, ID·tag·namespace와 순서를 고정한다. |
| 5 | fixture 외의 추가 computed style이 생겨도 통과하는가 | style node 집합이 root/item/child 세 개와 정확히 같아야 한다. 중복·누락·추가를 거부한다. |
| 6 | root computed CSS가 scene 입력과 달라도 typed Grid만 맞으면 통과하는가 | `display`, 고정 크기, explicit/implicit rows, auto-flow, 정렬, position, overflow, spacing, border와 typed style을 함께 확인한다. |
| 7 | `min-height:0`가 빠져 automatic minimum이 경계 결과를 바꾸는가 | computed min-height와 typed dimension 모두 0px이어야 한다. |
| 8 | 100px child의 max-content 근거가 다른 크기나 스타일에서 나오는가 | child computed 폭·높이·spacing·border를 고정하고 MaxContent contribution이 100px인지 확인한다. |
| 9 | CSS 문자열만 맞고 Taffy 입력이 다른 track sizing인 경우가 있는가 | projected `GridContainerInput`의 columns, rows, fit-content autoRows, placement mode와 repeat/area metadata 부재를 확인한다. |
| 10 | 세 limit과 기대 행 크기 매핑이 잘못됐는가 | 40→40, 30→30, 200→100px을 별도 scene table, 앱 matcher와 capture oracle에서 대조했다. |
| 11 | trace가 다른 축·다른 root이거나 sizing stage가 누락됐는가 | root trace 하나, Columns/Rows pass 두 개, Rows pass 하나와 `MaximizeTracks` stage를 요구한다. |
| 12 | raw growth limit을 argument와 같다고 오판하거나 track base를 놓치는가 | 최초 verifier 가정은 40/30px에서 틀렸다. 규범 알고리즘에 맞춰 base-size 40/30/100px과 raw finite growth limit 100px을 분리해 검증하도록 수정했다. |
| 13 | 무한 성장 가능 플래그가 잘못되거나 다른 active row가 섞이는가 | active row가 정확히 하나이고 `infinitelyGrowable=false`인지 확인한다. |
| 14 | 추가·중복 contribution이 하나의 올바른 contribution과 함께 섞이는가 | fixture item/span의 MaxContent 100px contribution이 정확히 하나인지 확인한다. |
| 15 | layout frame이 trace와 다르거나 node frame 누락·중복이 있는가 | root/item/child frame 집합을 중복 없이 정확히 세 개로 만들고 각 x/y/width/height를 대조한다. |
| 16 | style과 layout이 서로 다른 문서 revision에서 온 결과인가 | HostDocument generation/document/render-tree, style, environment revision을 runtime output key와 연결해 검증한다. |
| 17 | 진단 JSON이 frames·track stages·contribution 또는 expected 값 일부를 누락하는가 | FFI report builder가 필수 style, frame, Rows trace를 못 찾으면 오류를 반환하고 검증 값을 구조화해 기록한다. |
| 18 | Android report parser의 누락·잘못된 JSON·stale presentation을 성공 처리하는가 | 예외·필수 필드·frame·trace·revision·draw/presentation identity 검사가 fail-closed다. AVD 최종 3 scene 모두 통과했다. |
| 19 | iOS matcher가 Android와 다른 기준으로 성공 처리하거나 compositor 경계를 과장하는가 | iOS도 scene/style/revision/frame/trace/pixel 표본과 presentation identity를 검사한다. Metal 표본은 compositor 제출 전으로 제한해 기록했다. |
| 20 | fixture-only 성공을 전체 CSS 지원·전체 WPT·실기기·기본 artifact 성공으로 확대하는가 | status·plan·evidence가 세 고정 assertion으로 범위를 한정한다. 기본 archive 검사는 macOS ARM64 static archive에만 한정하고 전체 C11.4는 미완료로 유지한다. |

최종 acceptance는 Android API 37 ARM64 software AVD 3/3·PixelCopy 11/11, iPhone 17 Pro/iOS 26.2 Simulator 3/3·Metal 표본 11/11이다. 초기 Android verifier의 raw growth-limit 기대값과 최초 iOS pixel flag 누락은 최종 판정에서 제외하고 이 문서와 [실행 근거](./README.md)에 원인 및 수정 후 결과를 구분해 남겼다.
