# C11.4 `auto <ratio>` 구현/runtime 실패 관점 검토 20개

범위는 branch `feat/c11-4-direct-text-auto-ratio`의 코드 변경과 Android/iOS simulator receipt다. 각 항목은 별개의 실패 경로를 검사했다. 기능은 fixture-test opt-in으로 한정한다. 여기서 완료 판정은 C11.4/C11 승격을 뜻하지 않는다.

| # | 실패 관점 | 확인과 근거 | 판정 |
|---:|---|---|---|
| 1 | 기본 style profile에서 auto ratio가 새로 허용됨 | cascade gate는 `RuntimeBlockFlexGridTrackSizingV1`에만 `auto <ratio>`를 허용한다. 다른 profile rejection은 유지된다. | 통과 |
| 2 | 숫자 ratio가 content-box로 잘못 분류됨 | typed value는 numeric ratio에 `BoxSizing`, auto ratio에 `ContentBox`를 보존한다. 23-scene 행렬의 numeric controls가 기존 geometry를 분리한다. | 통과 |
| 3 | `auto`만 있는 값이 양수 ratio처럼 취급됨 | degenerate ratio는 typed ratio를 만들지 않고 collector는 computed serialization `auto`만 no-ratio control로 인정한다. | 통과 |
| 4 | 0/음수/비유한 ratio가 helper에 전달됨 | ratio와 파생 geometry의 finite-positive 검증 및 거부 테스트가 있다. | 통과 |
| 5 | typed ratio와 CSS computed serialization이 서로 다른 값 | collector/projection은 serialization의 ratio 및 auto 여부를 typed 값과 대조하고 불일치 시 거부한다. | 통과 |
| 6 | incremental cascade의 예전 sizing basis 재사용 | sizing basis가 computed element snapshot 안의 typed value로 함께 저장된다. snapshot 변경은 equality 비교에 포함된다. | 통과 |
| 7 | 대상 item을 scene 이름이나 authored text에서 추론 | direct-text collector가 computed style에서 owner `NodeId`를 얻으며 scene ID로 ratio를 고르지 않는다. | 통과 |
| 8 | 누락된 content-box owner marker로 일반 projection 허용 | 전용 projection gate는 지정 owner를 요구하고 누락 marker를 거부한다. | 통과 |
| 9 | 다른 item의 `NodeId`가 전달되어 엉뚱한 node 변형 | wrong-owner projection negative control이 layout 전에 실패한다. | 통과 |
| 10 | 복수 content-box ratio owner가 선택 순서에 따라 통과 | collector는 ratio owner 최대 하나만 허용하고 duplicate marker는 gate에서 거부한다. | 통과 |
| 11 | 일반 style-to-layout API가 content-box ratio를 우회 허용 | 전용 fixture entrypoint와 일반 projection 경계를 분리했고 ordinary projection 경계 테스트가 있다. | 통과 |
| 12 | 지정 owner 외 content-box ratio까지 override | projection은 받은 정확한 `NodeId`와 `ContentBox` typed basis가 동시에 일치할 때만 box sizing을 바꾼다. | 통과 |
| 13 | CSS computed `box-sizing` property 자체를 변형 | override는 projected Taffy item의 sizing basis에만 적용하고 snapshot property 문자열을 유지한다. | 통과 |
| 14 | padding/border를 포함한 외곽 폭에 ratio를 적용 | Taffy helper는 content-box constraints를 만든 뒤 ratio를 적용하고 padding/border를 복원한다. auto-row와 definite-row trace가 검증한다. | 통과 |
| 15 | 기존 숫자 ratio 계산이 새 helper에서 바뀜 | `BoxSizing` 경로는 기존 Taffy ratio helper를 사용한다. numeric controls와 기존 16-scene 회귀가 통과한다. | 통과 |
| 16 | ratio가 없는 control에도 transfer가 남음 | no-ratio scene 세 개와 이전 회귀 controls에서 computed `auto`, contribution, frame을 검사한다. | 통과 |
| 17 | auto row contribution과 max-content column을 뒤바꿈 | 양축 contribution/track trace 및 독립 기대값을 scene별 비교하고 Android/iOS에서 모두 통과했다. | 통과 |
| 18 | definite 60px row에서 overflow가 잘못 계산되거나 표본 좌표를 놓침 | auto/numeric/no-ratio 행렬은 91/90/50px track을 분리한다. root 내부 item edge와 root 외부 blue 표본이 72개 표본에 포함된다. | 통과 |
| 19 | Android/iOS native scene registry가 불일치 | 두 플랫폼 실행 모두 23/23 scene과 72/72 sample이며 27 source hash가 일치한다. | 통과 |
| 20 | fixture route가 기본 정적 라이브러리에 포함됨 | default/opt-in FFI archive marker를 비교했다. 결과는 FFI static archive에 한정하고 app bundle로 확대하지 않는다. | 통과(범위 제한) |

## 열린 검증 항목

인접 C07.3 exact computed-style 문자열 비교는 4/5이며 `1.7777778 / 1`과 Chromium `1.77778 / 1`의 차이는 남아 있다. 이 review는 이를 C11.4 구현 통과로 세지 않는다. Android는 software-rendered AVD, iOS readback은 compositor 이전이다. 실기기, hardware GPU, release APK/IPA 전체 artifact 경계, 일반 CSS admission 및 전체 WPT는 검증하지 않았다.
