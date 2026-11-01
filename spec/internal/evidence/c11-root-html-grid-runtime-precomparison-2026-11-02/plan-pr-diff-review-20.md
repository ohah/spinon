# C11 parsed root HTML Grid runtime 계획 PR diff 적대 검토 20관점

## 검토 범위

이번 계획 PR diff의 변경 대상은 C11 root plan, 새 parsed root HTML Grid 계획, 구현 전 HTML fixture, Chrome 156 observation/expected/screenshot/SHA manifest 및 두 개의 20관점 계획 검토다. source implementation, Rust test, build, Simulator 실행은 없다.

| # | 별도 PR 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 계획-only PR이 이미 구현됐다고 주장 | 계획 상태는 구현·test·app 실행이 미수행이라고 분명히 기록했다. PASS |
| 2 | C11 roadmap에서 새 계획을 찾을 수 없음 | `plan/c11-css-grid.md`의 다음 root HTML 통합 링크가 추가됐다. PASS |
| 3 | C11.1과 C11.4 범위 혼동 | 계획 제목/목표는 C11.1 default root HTML entry integration이며 C11.4 fixture는 배경으로 분리했다. PASS |
| 4 | 구현 결손이 인상에만 의존 | root HTML profile selection, renderer의 display rejection 및 fixture session entry code path를 문서화했다. PASS |
| 5 | Chrome oracle와 독립 기대값이 같은 값에서 생성 | 고정 track arithmetic를 별도 JSON에 고정하고 Chrome은 교차 확인으로만 쓴다. PASS |
| 6 | HTML source에서 planned style을 재현할 수 없음 | inline stylesheet와 viewport meta가 포함된 버전 관리 fixture를 포함했다. PASS |
| 7 | CSS box edge가 기대 frame에 숨겨짐 | root/item margin·padding·border를 0으로 고정했다. PASS |
| 8 | track sum과 root size가 다름 | columns 40+80=120, rows 20+50=70으로 일치한다. PASS |
| 9 | numeric placement와 semantic item 결과 불일치 | item-a (column 2,row 1), item-b (column 1,row 2)와 frame을 JSON으로 대조했다. PASS |
| 10 | CSS pixels와 device pixels를 혼동 | oracle viewport 및 DPR 1을 observation에 기록했고 tolerance는 CSS px 단위다. PASS |
| 11 | observation browser/platform version 누락 | Agent Browser 0.31.1/HeadlessChrome 156.0.0.0/macOS host를 고정했다. PASS |
| 12 | screenshot이 다른 fixture에서 생성 | Chrome Agent Browser session에서 위 버전 관리 fixture를 열고 관측한 뒤 캡처했다. PASS |
| 13 | source 또는 screenshot이 바뀌어도 알아차리지 못함 | 두 asset의 SHA-256 manifest를 포함했다. PASS |
| 14 | screen sample을 pixel equality로 과장 | plan PR은 browser screenshot만 근거로 둔다. Platform runtime pixels를 주장하지 않는다. PASS |
| 15 | implementation을 fixed node ID adapter로 완료할 위험 | positive ID/class/source-order controls와 selector-free routing acceptance를 계획했다. PASS |
| 16 | root HTML media/color-scheme regression 누락 | 기존 viewport/media/system-scheme boundary를 implementation precondition과 regression gate에 포함했다. PASS |
| 17 | invalid Grid input silently degrades to Block | fail-closed error와 fallback 금지 조건을 계획에 넣었다. PASS |
| 18 | user data/WIP가 계획 PR에 섞임 | staged scope는 계획·fixture·oracle·review files로 제한해야 하며 unrelated untracked data는 건드리지 않는다. PASS |
| 19 | official implementation count가 계획만으로 증가 | `spec/STATUS.md`, parent checkbox 및 `72/213`을 변경하지 않는다. PASS |
| 20 | 계획 검토 20개를 PR diff 검토로 중복 계산 | 계획 failure review와 이 PR file-level failure review를 별도 artifact로 유지한다. PASS |

## 판정

**PASS 20 / PARTIAL 0 / FAIL 0.** 이 PR은 계획·oracle·roadmap link만 추가한다. 구현, runtime, Android/iOS Simulator 및 product CSS support 근거는 후속 implementation change에서 새로 수집한다.
