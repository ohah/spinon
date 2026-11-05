# C11.4 multiple direct text item 구현 PR diff 적대 검토 30개

검토 대상은 `origin/main`의 `11d3e73f9899adc326963af051c2fdcd37ad4e6f`에 대한 `feat/c11-4-root-html-multiple-text-items`의 staged diff다. 실제 구현/runtime 실패 관점 20개와 별도로 PR 코드·계약·근거의 누락 및 범위 오류를 검사했다.

| # | 실패 관점 | diff 검토 결과 |
|---:|---|---|
| 1 | 무관한 사용자 파일이나 기존 untracked evidence가 PR에 포함되는가 | PASS — 지정한 구현, 계획/계약, 새 runtime evidence만 stage했고 이전 untracked evidence는 포함하지 않았다. |
| 2 | C11/C11.4 완료 표시 또는 공식 완료 수가 올라가는가 | PASS — `spec/STATUS.md`와 상위 checkbox는 바뀌지 않으며 후보 계약은 `78/214`를 유지한다고 명시한다. |
| 3 | 여러 text item 허용이 현재 C11.4 전체 지원으로 과장되는가 | PASS — 계약과 계획 모두 제한된 Root HTML 후보이며 전체 C11.4/C11 미완료를 유지한다. |
| 4 | 내부 계약 숫자 버전이 구현 확장만으로 변경되는가 | PASS — contract `0059`의 `0.1.0`을 유지한다. |
| 5 | plan, internal contract, runtime evidence가 서로 다른 source grammar를 설명하는가 | PASS — direct text child 하나와 ASCII `A` token / 단일 U+0020 separator 조건이 일치한다. |
| 6 | 새 public API, parser, runtime profile 또는 FFI가 실제로 추가됐다고 오기하는가 | PASS — 코드와 문서는 기존 default route 재사용으로 한정한다. |
| 7 | 지원 CSS 범위에 계획되지 않은 Grid grammar가 포함되는가 | PASS — 단일 explicit row, explicit columns, sparse auto-placement와 singleton `min-content` auto-row 범위만 유지한다. |
| 8 | 빈 source, 선행/후행 공백, 반복 공백, newline, 유사문자 입력이 통과하는가 | PASS — literal ASCII grammar helper와 negative assertions가 각각 거부한다. |
| 9 | NBSP 등 CSS whitespace 아닌 문자가 formatting text로 잘못 무시되는가 | PASS — CSS whitespace만 공백 direct node로 생략하며 NBSP는 visible invalid input이다. |
| 10 | hidden item이 row preflight와 admission에서 다르게 세어지는가 | PASS — 두 경로 모두 hidden node를 제외한다. |
| 11 | stale/uninspectable child handle이 정상 item으로 layout되는가 | PASS — preflight는 보수적으로 세고 후속 inspectability gate에서 fail-closed한다. |
| 12 | explicit column이 없을 때 0으로 나누거나 panicking하는가 | PASS — checked row helper가 `None`을 내고 contextual `grid-auto-rows` error로 끝난다. |
| 13 | 필요한 row 수 반올림이 마지막 partially-filled row를 누락하는가 | PASS — `div_ceil` 후 explicit row 수와 max를 취한다. |
| 14 | 10,001번째 row가 expensive frame 계산 뒤에야 발견되는가 | PASS — style/layout adapter 진입 전에 track limit을 확인하며 실제 10,001 item negative test가 오류 property를 확인한다. |
| 15 | 10,000개 정상 boundary에서 trace 제한 때문에 frame이 사라지는가 | PASS — 실제 10,000개 layout의 마지막 frame을 확인하고 Grid trace만 생략한다. |
| 16 | computed style을 source order 인덱스로 결합해 다른 item style이 붙는가 | PASS — 한 번 만든 `BTreeMap<NodeId, ...>`로 같은 snapshot의 identity를 조회한다. |
| 17 | 빈 item과 text item의 size/style admission이 섞이는가 | PASS — width/height와 text style 분기가 각각 기존 제한을 보존한다. |
| 18 | text item의 nested markup 또는 extra child가 두 번째 item에서 허용되는가 | PASS — one-child topology gate가 각 child에 동일하게 적용되고 runtime negative controls가 있다. |
| 19 | contribution을 문자열 길이/고정 값으로 추정하는가 | PASS — 기존 TextLayoutEngine 결과와 Taffy min-content query를 재사용한다. |
| 20 | source order를 뒤집으면 text contribution이 원래 owner에 남는가 | PASS — style/runtime test가 short/long 순서를 바꾸고 frame·trace item identity를 검사한다. |
| 21 | text paint가 다른 document/style/layout revision에서 재사용되는가 | PASS — RuntimeSession test가 cascade/layout key와 각 text paint source revision을 비교한다. |
| 22 | 두 번째 item 실패 때 첫 item의 frame/paint 일부가 게시되는가 | PASS — invalid source/style/markup runtime cases는 failed layout과 `completed=None`을 확인한다. |
| 23 | paint box와 glyph draw command 순서가 DOM 순서에서 벗어나는가 | PASS — runtime test가 각 box/text command의 상대 순서와 box sequence를 확인한다. |
| 24 | TraceLimitExceeded 외의 trace 오류까지 무시되는가 | PASS — trace allocation failure 오류 branch는 그대로 두고 limit 초과만 생략한다. |
| 25 | 문서의 Taffy trace 상한이 pinned dependency source와 어긋나는가 | PASS — vendored Taffy `0.15.0`의 Grid별 100,000 track-state와 100,000 contribution 상수를 대조했다. |
| 26 | combined intrinsic-columns/text-rows 기존 경로가 회귀되는가 | PASS — 별도 RuntimeSession regression 1/1 로그를 포함한다. |
| 27 | independent oracle fixture 또는 해시가 evidence와 다르게 고정되는가 | PASS — Chrome 156 fixture SHA-256 `3232e75c…657970`와 expected JSON을 연결한다. |
| 28 | Android/iOS가 서로 다른 fixture asset이나 fixture-only product feature로 실행되는가 | PASS — APK와 `.app`의 installed HTML/root-app source hash가 기준 입력과 맞고 fixture flags는 0이다. |
| 29 | simulator evidence가 실기기·GPU readback·pixel parity로 확대 해석되는가 | PASS — API 37 ARM64 AVD/iPhone 17 Pro Simulator visual smoke로 한정하고 glyph-wrap 차이와 readback 한계를 적었다. |
| 30 | PR에서 screenshot 링크가 깨지거나 화면이 scope를 과장하는가 | PARTIAL — 로컬 Android/iOS 캡처를 직접 열어 `[10,20,10,34]` 행과 34px overflow를 확인했다. PR 본문에는 업로드된 raw 이미지 URL을 넣고 실제 PR 생성 후 Agent Browser에서 렌더링/로드 여부를 확인해야 한다. |

## 검토 결과

**PASS 29 / PARTIAL 1 / FAIL 0.** 검토 중 발견해 해결한 항목은 Taffy trace 예산 초과 시 frame 보존, Android asset overlay build 순서, nested test include 경로, 증거 로그의 끝 공백이다. 구현 수정 뒤 focused style/runtime/layout tests, scoped Clippy, rustfmt 및 simulator asset hash를 재확인했다. PR 이미지 URL 검토는 PR 생성 뒤 이어서 완료하며, 이 검토는 GitHub merge/CI 성공을 뜻하지 않는다.
