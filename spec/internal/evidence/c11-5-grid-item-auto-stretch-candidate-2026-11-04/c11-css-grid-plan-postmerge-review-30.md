# C11 상위 계획의 auto-size/stretch 병합 상태 변경 검토 30개

검토 대상은 `plan/c11-css-grid.md`의 요약 문구 하나다. 이전에는 PR #1132의 제한 구현을 “다음 계획/미구현”으로 적었고, 변경 뒤에는 실제 병합된 제한 slice를 최신 C11.5 기록으로 둔다. 확인 기준은 PR #1132의 `MERGED` 상태, merge SHA `8e3d6b4ac3391a1d8e7601a96ab5fe870c825be0`, 실행 evidence와 `spec/STATUS.md`다.

| # | 독립 실패 관점 | 판정 |
|---:|---|---|
| 1 | C11 plan이 PR #1132를 아직 다음 미구현 계획으로 적는지 | 통과: 다음 계획 문구를 최신 제한 구현으로 고쳤다. |
| 2 | 병합 PR 번호가 틀렸는지 | 통과: #1132를 연결한다. |
| 3 | 병합 시각이 UTC 값과 다른지 | 통과: `2026-11-03T20:03:49Z`다. |
| 4 | merge SHA가 GitHub 값과 다른지 | 통과: `8e3d6b4ac3391a1d8e7601a96ab5fe870c825be0`다. |
| 5 | PR URL이 #1132 이외 PR을 가리키는지 | 통과: 링크 번호와 병합 기록이 일치한다. |
| 6 | 제한 구현을 C11.5 전체로 표현하는지 | 통과: C11.5 전체는 미완료로 적는다. |
| 7 | 제한 구현을 C11 완료로 표현하는지 | 통과: C11 전체는 미완료로 적는다. |
| 8 | 공식 완료 수를 바꾸는지 | 통과: `72/213`을 유지한다. |
| 9 | auto-size 대상이 빈 non-replaced item 범위를 벗어나는지 | 통과: 해당 제한 범위로 적는다. |
| 10 | 허용 alignment 값이 바뀌었는지 | 통과: computed `normal`/`stretch`만 언급한다. |
| 11 | accepted matrix 크기가 실행 근거와 다른지 | 통과: Rust accepted 10-scene matrix다. |
| 12 | expected negative control 수가 잘못 합쳐졌는지 | 통과: 16 atomic negative controls로 분리한다. |
| 13 | Chrome oracle 결과를 제품 결과와 혼동하는지 | 통과: Chrome 독립 기대값과 Rust matrix를 구분한다. |
| 14 | Android simulator를 실기기로 부르는지 | 통과: API 37 ARM64 AVD라고 명시한다. |
| 15 | Android software renderer 한계를 빠뜨리는지 | 통과: ANGLE/Vulkan/SwiftShader를 명시한다. |
| 16 | Android pixel 수가 실행 로그와 다른지 | 통과: PixelCopy 11/11 exact다. |
| 17 | iOS simulator를 실기기로 부르는지 | 통과: iPhone 17 Pro/iOS 26.2 Simulator다. |
| 18 | iOS pixel 수가 실행 로그와 다른지 | 통과: Metal drawable 11/11 exact다. |
| 19 | iOS readback을 compositor 이후로 과장하는지 | 통과: compositor 제출 전이라고 명시한다. |
| 20 | PR check rollup이 비었는데 CI 통과로 쓰는지 | 통과: CI 통과로 세지 않는다고 적는다. |
| 21 | dedicated C11.5 plan 링크가 빠졌는지 | 통과: 기존 전용 계획 링크를 보존한다. |
| 22 | Android/iOS 실행 evidence 링크가 빠졌는지 | 통과: C11.5 evidence README 링크를 둔다. |
| 23 | 기존 self-alignment PR #1128 기록을 덮어쓰는지 | 통과: 이전 제한 구현 기록은 유지한다. |
| 24 | 앞선 C11.1 item alignment가 최신 C11.5 작업으로 잘못 남는지 | 통과: 그 단락은 C11.1 범위 제목으로 남는다. |
| 25 | C11.4 intrinsic sizing 완료를 암시하는지 | 통과: C11.4 체크와 내용은 바꾸지 않는다. |
| 26 | 일반 CSS admission을 선언하는지 | 통과: 일반 지원을 추가로 주장하지 않는다. |
| 27 | WPT 적합성을 선언하는지 | 통과: WPT 완료를 추가로 주장하지 않는다. |
| 28 | public API 또는 ABI 추가를 암시하는지 | 통과: API/ABI에 대한 새 주장은 없다. |
| 29 | 내부 계약 버전이나 scope를 올리는지 | 통과: 계획 요약만 고치고 계약 버전은 건드리지 않는다. |
| 30 | 이 계획 상태 한 줄 밖의 C11 작업을 바꾸는지 | 통과: C11.4/C11.1 계획 및 다른 항목은 변경하지 않는다. |

**결론:** 실제 병합된 C11.5 제한 slice를 상위 계획에 반영하고, 전체 C11.5/C11과 `72/213` 경계를 유지한다.
