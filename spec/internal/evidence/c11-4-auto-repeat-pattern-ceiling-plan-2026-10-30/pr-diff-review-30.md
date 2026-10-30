# PR #836 계획 변경 실패 관점 검토 30개

**검토 대상:** PR #836 `docs(c11.4): auto-repeat 두-track 상한 계획`의 계획·precomparison·상위 계획 변경. 검토한 GitHub diff는 base `e17c65f69e9a8639617fad69a6288207b059e3b0`, head `d14441d49cb3a189e9bcab60ed02c211931e1c63`이다. 검토 대상은 4개 변경 파일이며 이 검토 결과 문서는 대상 diff 뒤에 별도 증거로 추가한다.

| # | 실패 관점 | 판정 및 방어 경계 |
| ---: | --- | --- |
| 1 | 이미 병합된 fit-content 열 축 계획을 새 계획처럼 반복한다 | **통과.** 이 계획은 현재 단일-track ceiling에 없는 2-track pattern과 named-line 경계만 다루며 #811과 별도다. |
| 2 | planning PR에서 기능 구현이나 완료 체크까지 했다고 오해한다 | **통과.** 제목과 본문이 계획 PR임을 밝히고 implementation/runtime은 미진행이라고 표시한다. |
| 3 | C11.4/C11 checkbox를 계획 추가만으로 완료 처리한다 | **통과.** `spec/STATUS.md`는 변경하지 않았고 parent plan은 구현 결과가 아직 없다고 쓴다. |
| 4 | 공식 프로젝트 완료 수가 올라간 것처럼 표시한다 | **통과.** `72/213`을 유지하고 계획 단계에서 집계를 바꾸지 않는다고 명시한다. |
| 5 | W3C §5.4의 implicit-grid cap을 explicit auto-repeat 절단의 근거로 잘못 쓴다 | **통과.** 두 문서 모두 §5.4의 범위를 explicit cap oracle과 분리한다. |
| 6 | definite auto-fill 계산에 neighbor track 또는 gap이 빠진다 | **통과.** neighbor 합 5px, gap 0px, repeat 3px를 고정해 계산한다. |
| 7 | exact-fit basis가 실제 10,000-track 경계에 도달하지 않는다 | **통과.** 15,002px는 1 + 9,998 + 1 track으로 정확히 10,000개를 만든다. |
| 8 | cap 초과 basis에서 실제 반복 요청이 상한을 넘지 않는다 | **통과.** 15,005px는 5,000회·10,000 repeated tracks를 요청해 앞의 track까지 포함한 총 10,002개 후보를 만든다. |
| 9 | 상한을 초과할 때 반복 횟수를 온전한 쌍으로 낮춰 partial pattern 절단을 놓친다 | **통과.** 기대값은 prefix 뒤 9,999 repeated tracks이며 마지막 2-track pattern의 첫 `1px`만 남긴다. |
| 10 | exact-fit과 cap 초과의 suffix source order 차이가 드러나지 않는다 | **통과.** exact-fit은 suffix `3px`를 보존하고 cap 초과는 suffix를 제거한다. |
| 11 | alternating track size 대신 count만 검사해 pattern 정렬을 놓친다 | **통과.** `Initialize` 전체 sequence를 `[1px, 2px]` 반복으로 비교한다. |
| 12 | repeat 바깥 prefix가 반복 track에 합쳐져 offset이 달라진다 | **통과.** prefix `2px`를 별도 첫 track으로 둔다. |
| 13 | named-line group 수가 repeated pattern track 수와 맞지 않는다 | **통과.** 두 track에 대응하는 시작·중간·끝 세 group을 고정한다. |
| 14 | 반복 시작 line name이 바깥 line과 중복되어 occurrence를 한 칸 당긴다 | **통과.** CSS 입력은 repeat 내부 line names만 사용하고 바깥 line names를 비운다. |
| 15 | 음수 named-line occurrence의 끝 기준을 반대로 해석한다 | **통과.** CSS Grid §8.3의 explicit-grid 역방향 기준과 `-1`을 사용한다. |
| 16 | exact-fit의 마지막 pair와 cap 초과의 마지막 partial track을 같은 좌표로 둔다 | **통과.** target offset을 각각 `14,996px`, `14,999px`로 분리했다. |
| 17 | named line이 잘린 pattern에서 실제 남지 않은 두 번째 track을 가리킨다 | **통과.** cap scene은 `pattern-start -1 / pattern-middle -1` child size가 `1px`인지 확인한다. |
| 18 | Rows 입력을 Columns와 같은 치수로 만들지 않아 다른 축 효과가 섞인다 | **통과.** 각 최대축 실행의 직교 축은 고정 `1px` 하나다. |
| 19 | root 크기와 viewport 크기가 달라 repeat basis가 흔들린다 | **통과.** 둘 다 동일한 target-axis basis를 사용한다. |
| 20 | border·padding·gap이 독립 계산과 runtime 입력 사이에 추가된다 | **통과.** 세 값 모두 0으로 고정한다. |
| 21 | item intrinsic contribution이 fixed repeat count 또는 track size를 바꾼다 | **통과.** child는 두 축에서 1px 고정 크기다. |
| 22 | auto-fit collapse가 auto-fill track 수 판정을 가린다 | **통과.** 이 후속은 auto-fill만 사용하고 기존 auto-fit matrix를 별도 범위로 남긴다. |
| 23 | resolved count와 stage trace가 달라도 테스트가 통과한다 | **통과.** explicit count, 단일 Initialize stage, gutter 제외 track 수를 함께 비교한다. |
| 24 | 10,000-track 입력에서 capture budget 초과를 놓친다 | **통과.** 각 실행이 100,000 captured-state 제한을 단언한다. |
| 25 | 화면 근거가 없는데 simulator 결과로 가장한다 | **통과.** 알고리즘 trace와 item frame만 사용하고 앱/simulator는 이 slice에서 제외한다. |
| 26 | W3C auto-repeat 문법 전체나 author CSS 지원으로 확대한다 | **통과.** 고정 `GridAutoRepeat` 입력과 테스트 전용 경계로 제한한다. |
| 27 | local link가 새 계획·precomparison 파일을 찾지 못한다 | **통과.** 네 변경 파일의 로컬 상대 링크를 검사했고 통과했다. 기존 `/spinon/` 생성 site 링크는 파일 경로 링크로 오판하지 않도록 별도 취급했다. |
| 28 | CRD section URL이 다른 조항으로 연결된다 | **통과.** auto-repeat, large grids, line placement 조항을 공식 W3C 문서에서 확인했다. |
| 29 | 계획 검토 20개를 구현 검토나 PR 검토 횟수에 재사용한다 | **통과.** 계획 검토 20개가 별도 파일에 있고 구현/runtime 검토는 이후 새 관점으로 남긴다. |
| 30 | PR checks가 비어 있는데 CI 성공을 주장한다 | **통과.** `gh pr view`의 status check rollup은 비어 있으며 PR 본문은 계획 검토와 문서 검사만 보고한다. |

**판정 당시:** PR 변경 실패 관점 30/30 통과. 계획 범위, independent expected values, 공식 완료 상태, 테스트/플랫폼 주장 사이에 모순을 찾지 못했다. 계획 검토 20개와 이 PR diff 검토는 구현/runtime 검토를 대신하지 않는다.

## 구현 전환 후 확인된 계획 누락 · 2026-10-30

첫 focused runtime 입력은 `InvalidGridTrackList`로 거부됐다. `validate_auto_repeat_axis`는 repeat pattern의 `GridAutoRepeat.line_names` 3개뿐 아니라 `tracks.len() - repeated_track_count + 2` 길이의 outer line-name vector도 요구한다. 계획과 precomparison에 이름 없는 네 outer group을 입력한다는 조건이 빠져 있었다. 구현 입력을 빈 vector 4개로 고치고 계획 문서·계획 검토 관점 7·8을 보완했다. 기존 PR diff 관점 7·8은 이 validator 구조를 놓쳤으며 구현/runtime 검토에서 재확인한다.
