# C11.4 four-member intrinsic implicit-row list · PR diff 적대 검토 30개

앞선 계획 검토·계획 수정 검토·구현/runtime 검토와 다른 관점에서 최종 변경 범위, 테스트, simulator artifact, 공개 진행 문구를 대조했다. 각 항목은 `PASS`이며 PR 상태나 CI 결과를 대신하지 않는다.

| # | 실패 관점 | 최종 diff에서 확인한 방어·근거 | 결과 |
|---:|---|---|---|
| 1 | 네 member가 아닌 track variant를 허용한다 | Layout과 projection은 exact `FitContent`·`MinContent`·`MaxContent`·`Auto` 집합만 받으며 다른 variant는 matcher에서 거부한다. | PASS |
| 2 | member 중복으로 길이만 4인 목록을 통과시킨다 | 양 구현의 네 boolean slot matcher가 중복을 즉시 거부한다. Layout nearby-type negative test도 확인했다. | PASS |
| 3 | 누락 member 또는 추가 track이 들어온다 | 정확히 길이 4와 네 slot 모두 present 조건을 함께 검사한다. | PASS |
| 4 | 기존 3-member 문법을 새 경계값 허용으로 넓힌다 | 기존 3-member matcher와 새 4-member matcher가 분리되어 있고 legacy-order 회귀가 유지된다. | PASS |
| 5 | fit-content cap에 px/% 이외 단위를 전달한다 | typed cap admission은 `LengthPx` 또는 `Percentage`만 허용한다. | PASS |
| 6 | 음수·NaN·무한 cap이 enum 형태만으로 통과한다 | projection/layout은 finite non-negative 값을 검사하고 invalid typed cap test가 있다. | PASS |
| 7 | `0%` cap을 양수로 보정하거나 `120%`를 clamp한다 | 전체 24순서 Layout·projection 경계와 RuntimeSession 0%/120% 장면에서 typed 값과 zero-height frame을 확인했다. | PASS |
| 8 | percentage cap에 잘못된 basis를 사용한다 | definite 100px block basis와 20% typed identity를 확인하고 indefinite-height rejection을 둔다. | PASS |
| 9 | absolute calc와 unresolved percentage calc를 혼동한다 | 절대 calc의 computed 20px normalization은 허용하고 mixed percentage calc는 atomic rejection한다. | PASS |
| 10 | 새 matcher가 다른 placement profile을 허용한다 | Layout negative control은 같은 track list를 다른 placement mode로 제출해 거부된다. | PASS |
| 11 | Root HTML owner 또는 revision 경계를 우회한다 | Style projection은 parsed Grid owner와 하나의 document snapshot을 검사하고 RuntimeSession은 cascade/layout key 동일성을 비교한다. | PASS |
| 12 | implicit columns·dense/column placement를 우연히 허용한다 | 기존 exact row-sparse profile과 fixed column topology를 유지하고 새 코드는 `grid-auto-columns`나 flow admission을 넓히지 않는다. | PASS |
| 13 | root 크기·explicit track shape가 달라진다 | Layout·Style validator는 40×100px root, 40px column, 10px explicit row, zero gap/edge 조건을 유지한다. | PASS |
| 14 | 네-member list에서 implicit child 수가 불명확해진다 | 새 profile은 explicit item 하나와 implicit item 다섯 개의 정확한 visible shape를 요구하며 5·7 visible-child controls와 3-member/6-child mismatch를 검사한다. | PASS |
| 15 | hidden, anonymous text 또는 nested Grid가 direct item으로 섞인다 | Style validator는 visible direct HTML div만 수집하고 payload는 direct child와 non-Grid 조건을 확인한다. | PASS |
| 16 | 큰 payload를 non-fit-content track 위치에 숨겨 허용한다 | 허용 height는 해당 cycle slot이 fit-content인 경우에만 60px을 추가하며 RuntimeSession rejection은 `height`를 귀속한다. | PASS |
| 17 | cap overflow에서 track·item·payload frame이 서로 섞인다 | 48개 px/% 장면에서 20px track, 60px payload, 각 item/payload origin과 frame을 따로 검사한다. | PASS |
| 18 | cycle이 explicit row에서 시작하거나 네 번째 뒤 wrap하지 않는다 | explicit 10px row 다음 implicit row를 index 0으로 두고 다섯 번째 implicit row의 첫 member 재사용을 모든 order에서 대조한다. | PASS |
| 19 | order를 projection 또는 serialization에서 정렬한다 | 24순서 RuntimeSession matrix가 computed serialization과 typed track vector 순서를 각각 비교한다. | PASS |
| 20 | stretch가 fit/min/max track에도 free space를 준다 | 96개 RuntimeSession matrix에서 남은 22px은 auto max-sizing rows에만 분배된다. | PASS |
| 21 | start 정렬이 free space를 소모한다 | 같은 matrix의 start scene은 explicit row 포함 78px 합과 22px 잔여 공간을 유지한다. | PASS |
| 22 | unsupported alignment 오류를 다른 property에 기록한다 | Layout 및 Style projection은 `align-content`를 먼저 검증하고 center negative control로 오류 귀속을 고정한다. | PASS |
| 23 | zero-height item과 payload paint를 같은 snapshot에서 찾는다 | Runtime boundary test는 layout frame과 render payload를 각각 검사해 0% cap의 paint 동작을 분리한다. | PASS |
| 24 | 실패한 revision이 부분 결과를 남기거나 session 복구를 막는다 | unsupported list/calc는 completed layout 없이 거부되고 동일 RuntimeSession의 다음 유효 revision을 검사한다. | PASS |
| 25 | Style projection과 Layout admission의 accepted set이 어긋난다 | 두 exact matcher와 shared profile tests를 대조했으며 non-fit overflow, duplicate, unsupported alignment controls가 있다. | PASS |
| 26 | 기존 singleton/3-member intrinsic slice가 회귀한다 | 기존 fit-content/min-content/auto style·runtime regression logs를 확인했고 singleton intrinsic mode matcher는 변경하지 않았다. | PASS |
| 27 | 내부 후보가 공개 API 또는 FFI surface를 바꾼다 | diff는 layout/style/runtime 내부 Rust와 계획·근거·roadmap만 수정하고 공개 selector·FFI 파일을 건드리지 않는다. | PASS |
| 28 | 고정 payload 결과를 일반 min/max 알고리즘 적합성으로 과장한다 | 계획·0059·runtime README는 고정 높이 fixture의 geometry 한계와 임의 content/WPT 미검증을 명시한다. | PASS |
| 29 | Simulator 캡처를 pixel-readback이나 실기기 근거로 오인한다 | 같은 fixture hash의 Android AVD/iOS Simulator OS screenshots와 capture hashes를 보존하며 PixelCopy/Metal readback이 아님을 기록한다. snapshot tarball은 stage 후보에서 제외한다. | PASS |
| 30 | 공식 상태와 공개 진행 카드가 오래된 수치·중복 카드·완료 표시를 남긴다 | `spec/STATUS.md`는 계획 시점의 미착수를 과거형으로 한정하고 현재 24순서 후보를 별도 기록하며 체크박스·78/214를 유지한다. roadmap은 중복 WIP 문단을 제거하고 Chrome 96/96, Layout 12/12, Style 96 typed cases를 반영한다. | PASS |

## 판정

30/30 관점에서 이 후보의 exact grammar, typed projection, cycle sizing, rejection/recovery, evidence scope와 공개 상태 문구가 서로 맞는다. 변경 crate Clippy, focused Rust tests, formatter와 diff check는 별도 실행 근거에 연결했다. 이 후보는 limited implementation PR이며 CI·merge 상태, 임의 intrinsic contribution parity, WPT, hardware GPU 또는 전체 C11.4/C11 완료를 주장하지 않는다.

| 검증 묶음 | 최신 근거 |
|---|---|
| Layout 12/12 및 48 overflow·48 cap-boundary scene | `cap-boundary-layout-rerun-pass01.log`, `cycle-overflow-layout-rerun-pass03.log` |
| Style projection 2/2 및 96 typed cap combinations | `cap-boundary-style-rerun-pass01.log` |
| RuntimeSession 7/7 및 zero/over-100% boundary | `cap-boundary-runtime-rerun-pass02.log` |
| Changed-crate Clippy / workspace rustfmt / diff check | `cap-boundary-clippy-rerun-pass01.log`, `cap-boundary-rustfmt-rerun-pass01.log`, `cap-boundary-diff-check-rerun-pass01.log` |
