# C13.1 target-frame 2D hit identity PR 변경 관점 검토 · 30개 · 2026-10-14

이 검토는 이번 PR delta의 Rust test helper, 고정 Chrome observation, 수집/무결성 script, evidence, plan·contract·status 변경만 대상으로 한다. [구현 실패 관점 20개](implementation-review-20.md)와 별도로 기록한다.

| # | PR 변경 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 제품 hit-test 구현 변경 여부 | 변경은 `stacking/tests` helper와 test module 연결뿐이다. renderer 제품 경로를 변경하지 않았다. |
| 2 | 기본 앱 바이너리 영향 | 새 Rust helper는 기존 test module 하위에 있고 `#[test]` fixture 호출에만 연결된다. FFI, JNI, Swift, 앱 source는 변경하지 않았다. |
| 3 | 모듈 책임 경계 | 2D scan helper를 별도 `hit_identity_2d.rs`로 두어 1D/runtime matrix 파일에 구현 세부를 덧붙이지 않았다. |
| 4 | 모듈 상대 `include_str!` 경로 | 최초 컴파일에서 경로가 한 단계 짧은 실패를 관찰했고 수정 로그를 보존했다. 수정 뒤 현재 focused test는 8/8 통과했다. |
| 5 | 고정 oracle provenance | Chrome product/version/revision, UA, viewport, DPR, fixture 및 parent digest를 capture와 Rust assertion에 기록했다. |
| 6 | oracle 재현 경로 | README가 agent-browser capture script, session, local fixture 경로와 실행 명령을 연결한다. terminal-browser는 사용하지 않았다. |
| 7 | 고정 fixture 무결성 | Rust와 독립 validator가 원본 fixture digest를 확인한다. |
| 8 | parent evidence 계보 | 새 capture가 기존 1D identity observation digest를 포함하고 validator가 실파일 digest와 비교한다. |
| 9 | row inventory 검증 | 13개 valid runtime ID를 고정하고 invalid margin과 float reject case는 성공 matrix에서 제외했다. |
| 10 | sample inventory 검증 | Rust는 총 187,320점을 고정하고 validator가 실제 RLE 면적 합계와 일치시킨다. |
| 11 | coordinate convention | README와 helper가 target frame 안 정수 CSS 좌표를 floor/ceil 기준으로 계산한다. |
| 12 | cross-axis corner gap | 기존 1D sweep을 유지하고 별도 2D scan이 각 정수 행의 모든 x 점을 추가 검사한다. |
| 13 | identity exactness | 결과를 target 여부 boolean으로 줄이지 않고 Chrome label을 해당 scene `NodeId`와 비교한다. |
| 14 | root wrapper mapping | `case/main/body/html`을 synthetic root로 매핑하는 범위를 README에 밝히고 wrapper별 DOM identity 주장을 하지 않는다. |
| 15 | nested node mapping | nested inner NodeId를 명시 전달하며 관찰에 해당 identity가 없었다는 양성 표본 한계를 review에 기록했다. |
| 16 | fail-closed label mapping | 알 수 없는/null label은 capture 또는 Rust assertion을 실패시킨다. |
| 17 | frame drift | x/y/width/height를 관찰과 비교한 뒤 oracle 좌표에서 sweep한다. |
| 18 | row/RLE truncation | 빈 row, y gap, x gap/overlap/reversal, frame 끝 누락을 검사한다. |
| 19 | valid case 호출 경로 | 기존 13 valid row의 8개 Rust matrix test가 2D assertion까지 실행하고 focused log가 8개 통과를 보인다. |
| 20 | image/absolute descendant 회귀 | contained, escaped absolute 및 nested clip 행을 2D map과 기존 독립 test geometry assertion에 연결했다. |
| 21 | invalid CSS handling | invalid `overflow-clip-margin`을 정상 layout 표본에 넣지 않고 별도 reject 근거에 남긴다. |
| 22 | float/C26 경계 | float geometry를 성공 case로 추가하지 않았고 fail-closed/C26 소유 한계를 그대로 유지한다. |
| 23 | paint과 hit 분리 | 이 PR은 `hit_test_css_point` NodeId만 확인하며 paint surface parity를 주장하지 않는다. |
| 24 | platform evidence | source 변경은 host test/evidence 전용이어서 Android/iOS app rebuild를 하지 않았다. 기존 AVD/Simulator 결과를 새 2D scan 통과로 재사용하지 않는다. |
| 25 | 물리기기/ADB 주장 | 이번 delta에서 ADB나 실기기를 연결하지 않았으며 README에 새 simulator 결과를 주장하지 않는다. |
| 26 | screenshot 적합성 | PR #159 본문에 기존 Chrome 155 fixture 화면을 관련 동작 그림으로 붙였다. agent-browser 페이지에서 이미지 `naturalWidth=320`, `naturalHeight=120` 및 `complete=true`를 확인했다. |
| 27 | screenshot 접근성/원본 | alt text와 설명은 이 PNG가 2D identity sample 자체가 아닌 기존 Chrome runtime oracle 화면임을 밝힌다. 시뮬레이터 PixelCopy처럼 표시하지 않는다. |
| 28 | README 정확성 | README는 Chrome metadata, capture hash, JSON hash, 13/187,320 범위와 compile 경로 수정 및 test 결과를 적는다. |
| 29 | 공식 상태·계약 동기화 | plan, internal contract 0060, status와 1D/runtime README가 새 2D 근거로 연결되며 C13.1 미완료를 유지한다. |
| 30 | PR lifecycle·merge claim | #159는 `main` 기준 ready PR이며 label 3개, `CLEAN`, review approval 0 요구, branch protection의 required status check 없음과 linear history 설정을 확인했다. `gh pr checks`는 checks 미보고를 반환했다. repository 규칙에 맞춰 rebase merge를 진행할 상태다. 기존 #157/#158 merge SHA도 live PR metadata와 대조했다. |

PR delta review 결과: 현재 코드·증거·문서 연결에서 merge 차단 결함을 찾지 못했다. `inner` positive sample, frame 외 viewport, surface, event dispatch, WPT는 PR scope 밖이며 C13.1의 남은 gate다.
