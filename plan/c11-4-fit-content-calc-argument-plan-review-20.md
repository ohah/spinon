# C11.4 definite calc `fit-content()` 계획 실패 관점 20개

검토 기준: [definite calc WPT 계획](./c11-4-fit-content-calc-argument.md), pinned WPT 입력 및 Agent Browser 사전 관측, CSS Grid 2025 CRD §§6.6, 7.2.1, 11.2.

| # | 실패 관점 | 판정 및 계획의 통제 |
|---:|---|---|
| 1 | percentage basis를 root의 가용 viewport 높이로 잡는가 | root의 definite inner block-size 300px을 고정하고 10% = 30px로 산출한다. |
| 2 | 10%만 적용하고 calc의 5px 길이 항을 버리는가 | typed calc의 percentage와 absolute length 항을 각각 관찰하고 합 35px을 assert한다. |
| 3 | definite basis 장면을 cyclic percentage로 분류하는가 | root height 300px을 definite로 고정하고 intrinsic auto fallback을 적용하지 않는다. |
| 4 | 계산 결과 35px을 content max-content 400px과 혼동하는가 | argument limit과 max-content contribution을 별도 field 및 trace로 검증한다. |
| 5 | item automatic minimum이 400px로 cap을 넘기는가 | WPT의 min-height 0을 computed style과 typed minimum source/value에서 확인한다. |
| 6 | 자식의 400px height를 item contribution으로 사용하지 못하는가 | nested child frame과 Minimum/MinContent/MaxContent 값 0/400/400을 분리 assert한다. |
| 7 | border-box 300px을 content percentage basis로 잘못 읽는가 | root의 border·padding을 0으로 고정해 inner block-size와 CSS height가 300px로 일치하게 한다. |
| 8 | WPT의 auto inline width 차이가 block-axis 결과에 섞이는가 | runtime root width만 60px로 고정하고 column을 60px로 두며 row percentage basis는 300px로 유지한다. |
| 9 | explicit row가 만들어져 grid-auto-rows가 우회되는가 | grid-template-rows:none과 단일 auto-placed item을 exact gate한다. |
| 10 | 여러 implicit row가 생겨 첫 row 결과만 검증하는가 | direct item 하나와 정확히 한 generated row만 허용한다. |
| 11 | fit-content track이 align-content normal에서 늘어나는가 | definite root의 265px 남은 공간에서도 used row가 35px이며 auto stretch 대상이 아님을 확인한다. |
| 12 | max-content contribution 400px을 final row size로 오독하는가 | stage별 base와 growth limit 및 final frame을 분리 기록한다. |
| 13 | calc computed AST가 stale registry identity를 참조하는가 | 같은 style snapshot/revision의 typed track 및 calc identity를 검사하고 stale handle negative를 거부한다. |
| 14 | 35px에 가까운 반올림 값만 비교해 오류를 놓치는가 | CSS pixel geometry와 typed calc result를 독립 JSON의 exact 기대값 또는 명시 tolerance로 비교한다. |
| 15 | overflow visible을 author declaration으로 만들어 초기값 경로를 바꾸는가 | overflow author style은 생략하고 computed initial visible만 확인한다. |
| 16 | item 내부와 root 밖 child paint sample을 섞는가 | row 내부 child, row 내부 item, root 내부 배경, root 외부 child의 좌표와 색을 별도 고정한다. |
| 17 | presentation viewport 400px이 300px row basis로 쓰이는가 | app viewport 320×400은 paint 관찰에만 쓰며 root inner block basis는 300px로 고정한다. |
| 18 | 양 축이나 writing-mode 변경으로 다른 알고리즘을 검증하는가 | 단일 60px fixed column과 기본 horizontal writing mode만 허용한다. |
| 19 | unlisted DOM/style 입력이 부분 결과를 내는가 | scene ID, div identity, 3-node topology, style, track list가 다르면 전체 fixture를 거부한다. |
| 20 | AVD/Metal 샘플을 실기기·compositor 이후 증거로 확대하는가 | Android AVD software surface와 iOS compositor 제출 전 drawable을 구분해 기록한다. |

## 판정

20개의 독립 실패 경로를 계획의 입력, 규범 계산, fixture 경계, trace, surface 및 artifact 판정에 배치했다. 단일 definite WPT assertion만 다루며 일반 CSS admission이나 C11.4 완료를 주장하지 않는다. 구현은 계획 PR 병합 뒤 시작한다.
