# PR diff 적대 검토 30개

구현/runtime 검토 20개와 다른 관점으로, PR 전체 변경의 module gate, caller 영향, oracle/증거/공식 상태와 배포 경계를 확인한다. 최종 diff와 연결된 기존 focused test, formatter, simulator 기록 및 Clippy 결과를 함께 대조한다. 검토에서 발견한 align-self 범위 누출은 별도 helper option과 negative test로 수정됐고, 테스트 파일도 500줄 이하의 역할별 두 모듈로 정리했다. 아래 판정은 수정 후 최종 diff 기준이다.

| # | diff 관점 | 최종 판정 |
|---:|---|---|
| 1 | 새 FFI fixture module과 C11.4 test modules가 opt-in fixture-test feature 아래 등록되는가 | FFI 경계는 feature-gated이며, workspace Clippy가 드러낸 기존 mixed-span test의 누락 gate도 추가했다. Default와 feature Clippy 통과. PASS |
| 2 | 신규 Rust 파일이 규정된 `fixtures/` 경계 아래 있는가 | FFI/runtime fixture 디렉터리 및 test 하위 모듈에만 존재. PASS |
| 3 | 한 작업 ID가 이전 경로에 중복 정의되는가 | module declaration search에서 신규 scene 소스는 한 경로. PASS |
| 4 | 기존 scene dispatcher 우선순위가 바뀌는가 | 별도 네 ID 분기 추가, 기존 branch 순서와 result 유지. PASS |
| 5 | fixture source가 비제한적인 입력으로 커지는가 | 네 scene allowlist, 7 KiB 상한 검증. PASS |
| 6 | generated HTML/JS가 source-order variant를 혼동하는가 | A/B/C identity와 forward/reverse DOM 순서를 명시. PASS |
| 7 | source-order control이 style/placement까지 달리하는가 | order 외 geometry/style/input 고정, Chrome와 Rust 결과 대조. PASS |
| 8 | intrinsic과 definite control의 width 설정이 동일해지는가 | scene metadata와 generated source assertion에서 분리. PASS |
| 9 | root `content-box`와 item `border-box` 기준이 바뀌는가 | CSS fixture·adapter typed style·expected box 일치. PASS |
| 10 | CSS calc 표현식이나 whitespace 차이에 예상치가 종속되는가 | fixed HTML, typed AST expectation, CRD 독립 계산을 병기. PASS |
| 11 | Ahem text 길이/공백 collapse가 Chrome과 다른가 | exact string, font bytes, whitespace mode 및 line widths 대조. PASS |
| 12 | PNG URL이 실제 network fetch로 이어지는가 | `.invalid` URL을 고정 bytes resource request와 연결; 외부 fetch 금지. PASS |
| 13 | PNG decoder가 바뀌거나 natural dimensions를 추측하는가 | 기존 fixed PNG decode 경로와 dimensions 검사 재사용. PASS |
| 14 | source order 바뀔 때 image handle을 잘못 찾는가 | resource snapshot key를 NodeId로 확인, natural metadata를 image node에서 읽음. PASS |
| 15 | 새 runtime bool가 기존 mixed scenes에 true 기본값으로 새는가 | 모든 기존 `C11_4MixedInput` 생성자는 false로 설정. PASS |
| 16 | 새 two-axis helper가 product build ABI를 늘리는가 | helper는 `c11-4-grid-track-sizing-fixture-test` cfg로만 컴파일. PASS |
| 17 | helper가 profile mismatch에서 Taffy로 계속 진행하는가 | profile을 거절하고 viewport를 확인한 뒤 projection 시작. PASS |
| 18 | 새 option을 projection 호출부 한 곳에서 미초기화하는가 | 모든 `StyleLayoutProjectionOptions` literal에 명시적 false/true 지정. PASS |
| 19 | replaced alignment exception이 non-image item에 적용되는가 | `html_image_grid_items` node membership와 per-helper opt-in 모두 요구. PASS |
| 20 | item alignment 기존 동작을 보존하는가 | 기존 fixture helper가 image `start`에서 실패하는 negative regression. PASS |
| 21 | public-style wrapper의 default behavior가 실수로 확장되는가 | 새 wrapper만 image start와 two-axis root intrinsic 옵션 사용. PASS |
| 22 | six prior scene adapter의 two-axis intrinsic behavior가 바뀌는가 | constructor false 유지, 기존 C11.4 regressions 통과 여부는 PR CI/현재 checks로 보고. PASS |
| 23 | Rust tests가 scene label만 확인하고 geometry를 생략하는가 | frame, line width, source, per-axis track과 staged contribution을 대조한다. assertion helper는 479줄 검증 모듈과 309줄 test-case 모듈로 분리. PASS |
| 24 | float 계산 noise를 equality로 오판하는가 | CSS px tolerance `1/64` 적용; 정수/sequence invariants는 exact. PASS |
| 25 | A/B span-2의 planned increase를 합산해야 하는데 잘못 비교하는가 | `[9,19,19]` stage와 뒤 span-3 C no-growth를 확인. PASS |
| 26 | Chrome 결과를 독립 기대값의 source로 오해하는가 | CRD 계산과 Chrome observation 파일을 분리, CSSOM tolerance 설명. PASS |
| 27 | Android 앱 screenshot을 physical device evidence로 표시하는가 | API 37 AVD와 `emulator-5558`로 명시, 실기기 미검증을 표기. PASS |
| 28 | iOS Metal sample을 compositor/scanout proof로 표시하는가 | compositor 제출 전 drawable 표본으로 제한. PASS |
| 29 | screenshot이 text glyph pixel proof로 오해되는가 | runtime/glyph count 0을 README와 contract에 적고 geometry로 한정. PASS |
| 30 | status/contract count, Tailnet preview나 final archive 경계를 과장하는가 | C11.4 unchecked, `77/213`, contract `0.1.0`; static archive와 final APK/IPA 구분. PASS |

이 diff는 public CSS support/완료 집계를 바꾸지 않는다. Required checks·merge state와 병합 후 Tailnet의 served bytes는 GitHub/preview의 live 결과를 별도로 확인한다.
