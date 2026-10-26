# C11.4 · `align-content: normal` 런타임 계획 PR diff 적대 검토 30관점

**검토 대상:** plan/evidence payload 10개 파일(런타임 계획, 독립 계획 검토 20관점, 상위 계획 갱신, Agent Browser 사전 비교 fixture/evidence/checksums)과 이 30관점 리뷰 artifact 1개, 현재 PR diff 총 11개 파일.<br>
**기준 소스:** `origin/main` `18bb75f295e472c567e554c41e30ec4534940cb6`<br>
**검토 방식:** 최종 staged diff와 연결된 저장소 문서를 정적으로 대조했다. PR #668의 live 상태·본문·라벨은 GitHub CLI와 Agent Browser로 확인하고 screenshot 이미지가 화면에 로드되는지 확인했다. staged diff whitespace 검사와 SHA256SUMS 6개 대상의 실제 bytes를 확인했다. build/test는 실행하지 않았다.<br>
**결론:** **30 PASS / 0 PARTIAL / 0 FAIL**. 최신 plan/evidence 수정 후 #5와 #25를 재검토해 해소했고, PR #668 생성 후 #30의 body/labels/image/state 확인을 완료했다.

## 30개의 독립 실패 관점

| # | 관점 | 판정 | 근거 및 판단 |
|---:|---|---|---|
| 1 | plan PR에 제품 코드가 섞였는가 | PASS | live PR diff 11개는 plan/evidence payload 10개와 이 리뷰 artifact 1개다. 모두 계획·검토·상위 계획·CSS 비교 fixture/evidence 또는 리뷰 기록이며 Rust, FFI, Android/iOS host 제품 코드 변경은 없다. 구현은 계획 병합 뒤로 둔다. |
| 2 | 20관점 계획 검토와 PR diff 검토를 같은 절차로 셌는가 | PASS | 독립 계획 검토 문서는 20개의 계획 실패 경로를 기록한다. 이 30관점 표는 staged 파일 교차관계, 증거·메타데이터·PR handoff를 따로 본다. 계획 20관점 결론을 PR diff PASS로 재사용하지 않는다. |
| 3 | 사전 비교를 Spinon 구현 결과로 표시하는가 | PASS | 계획과 README는 Chrome 비교 단계에서 Spinon runtime 및 WPT harness를 실행하지 않았다고 구분한다. 계획 검토나 Chrome 화면은 구현 통과 근거로 표시되지 않는다. |
| 4 | 계획 전용 PR이 공식 상태를 앞당기는가 | PASS | `spec/STATUS.md`는 staged되지 않았다. 신규 계획·상위 계획 갱신 모두 C11.4 미완료와 공식 `72/213` 유지로 제한한다. |
| 5 | 계획 20관점 PASS의 확인 링크가 새 계획에서 빠지는가 | PASS | 신규 plan 상태 문구가 `[계획 자체 20관점 검토](./c11-4-align-content-normal-runtime-plan-review-20.md)`로 staged된 검토 파일을 직접 연결한다. 상대 경로가 `plan/` 아래의 해당 파일을 가리킨다. |
| 6 | 상위 계획 링크가 잘못된 상대 경로를 가리키는가 | PASS | `plan/c11-4-grid-track-sizing.md`에서 신규 plan과 evidence로 가는 상대 경로가 해당 디렉터리 구조와 일치한다. 신규 plan에서 0059·STATUS·precomparison·fixture·expected JSON·기존 oracle evidence 경로도 저장소 상대 경로로 일관된다. |
| 7 | C11.4 상위 항목이 완료로 바뀌는가 | PASS | 상위 기록은 신규 행을 계획으로만 기록하며 C11.4 체크 및 완료 집계를 변경하지 않는다. |
| 8 | 새로운 slice가 #664의 동일 주장 반복인가 | PASS | 계획은 이미 런타임에서 관찰한 `normal`/auto-track stretch와 새로 주장할 두 auto row의 균등 분배 및 fixed 세 번째 row 보존을 구분한다. #664의 `[10,10] → [10,290]` 입력과 새 `[10,20,5] → [32.5,42.5,5]` 입력을 혼동하지 않는다. |
| 9 | 신규 동작을 제품 CSS 지원으로 과장하는가 | PASS | 계획은 test-only opt-in fixture, 일반 CSS admission 미지원, public API 미추가, C11.4 미완료를 반복해서 제한한다. |
| 10 | 규범 근거가 판본/단계 없이 모호한가 | PASS | CSS Grid Layout Module Level 1 2025 CRD §11.8의 고정 URL과 알고리즘 단계가 plan, README, JSON에 명시돼 있다. |
| 11 | 기대값이 구현 출력에서 역산되는가 | PASS | `expected.json`은 독립 산술 입력과 예상 stage/frame을 담고, 계획은 Taffy 출력에서 값을 역산하지 않는다고 한다. Chrome observation도 별도 자료로 유지된다. |
| 12 | free-space 산술이 gap을 중복·누락하는가 | PASS | `100 - (10+20+5) - (10+10) = 45px`로 두 내부 gutter를 한 번씩 뺀다. `22.5px`씩 증가한 뒤 최종 행 합과 gutter를 더하면 root의 100px과 맞는다. |
| 13 | `auto`가 아닌 track까지 늘리는가 | PASS | 기대값은 index 0,1만 stretch 대상에 두고 fixed index 2의 `5px`을 보존한다. plan·expected·Chrome JSON의 최종 rows가 모두 `[32.5,42.5,5]`다. |
| 14 | 실제 stage 중간값/순서를 빠뜨리는가 | PASS | 예상 stage JSON과 plan은 intrinsic, maximize, flex expansion, stretch의 순서 및 앞 세 stage의 `[10,20,5]` bases를 고정한다. 마지막 stage `applied:true`, remaining space 0도 명시한다. |
| 15 | frame 기대값이 row starts와 item 높이에 맞지 않는가 | PASS | `0`, `32.5+10`, `32.5+10+42.5+10` 계산으로 child y `[0,42.5,95]`를 얻고 각 고정 높이 및 100px root 경계와 일치한다. |
| 16 | CSS fixture와 input JSON이 authored style에서 어긋나는가 | PASS | 둘 다 `160px` column, `auto auto 5px` rows, `row-gap:10px`, `column-gap:0`, normal/flex-start alignment, row flow, 세 child 높이 `10/20/5px`로 일치한다. |
| 17 | JSON의 행 index가 CSS에서 author가 지정한 placement인 것처럼 읽히는가 | PASS | `input.json`의 `row:0/1/2`는 auto-flow 결과 위치를 표현하고 HTML은 `grid-auto-flow:row`와 DOM 순서만 지정한다. 계획도 direct items가 각 행에 자동 배치된다고 적어 의미를 뒷받침한다. |
| 18 | root-size 정책 차이를 감춘 채 parity를 주장하는가 | PASS | plan과 README는 pure Rust `Match`와 앱 `ResolveWithin` 차이를 명시하고, 고정된 viewport/root 크기의 geometry만 비교한다고 한다. sizing policy 일반 동등성으로 넓히지 않는다. |
| 19 | inline-axis 폭 변경이 row-axis 비교를 오염시키는가 | PASS | `20→160px` 변경을 plan/README에 적고 root·column·모든 child 폭을 같은 값으로 고정한다. 전체 x-frame parity는 주장하지 않는다. |
| 20 | viewport 좌표 원점이 DOMRect 수치와 맞지 않는가 | PASS | HTML은 html/body margin 및 body padding을 0으로 초기화한다. 관찰 JSON은 raw viewport-relative `getBoundingClientRect()` 결과를 기록하며 root 원점을 `(0,0)`으로 제시한다. |
| 21 | CSSOM과 사용 track 값이 서로 맞지 않는가 | PASS | CSS fixture의 `align-content:normal` 및 `justify-content:flex-start` 선언과 observation JSON의 computed 값이 맞고, used `gridTemplateRows`와 root/child rectangles가 독립 기대값과 일치한다고 기록한다. |
| 22 | PNG가 JSON에서 말하는 장면이 아닌가 | PASS | 정적 PNG metadata는 RGB 160×100이며 이미지 확인에서 root 배경, 두 내부 gutter와 높이 10/20/5의 세 색 row를 확인했다. 이미지가 예상 크기와 장면을 나타낸다. |
| 23 | screenshot을 전체 RGBA equality 증거로 과장하는가 | PASS | plan은 pixel sample을 보조 자료로만 두고, 명시적인 RGBA 비교 없이는 pixel equality를 주장하지 않는다. README도 이미지가 Chrome 관측 screenshot임을 한정한다. |
| 24 | upstream WPT 실행을 Spinon 적합성으로 오인하는가 | PASS | observation의 `wpt_runner:false`와 README/plan의 설명이 일치한다. upstream WPT나 Spinon WPT pass claim은 없다. |
| 25 | browser 관측을 재현할 환경 메타데이터가 충분한가 | PASS | observation과 README는 Agent Browser `0.31.1`, Chrome `156.0.8078.48` 및 revision `d0c02bfad08ec92a6ffcf989e53cd995e6381a4c`, macOS `26.5.1` build `25F80` arm64, viewport `160×100 CSS px`, DPR `1`, PNG pixel dimensions를 기록한다. |
| 26 | `align-content:start` 대조군도 Chrome에서 관찰한 것처럼 오해되는가 | PASS | `expected.json`과 README는 start 값을 기존 Rust oracle control test의 결과로 연결하고 이번 Chrome 관찰에서 실행하지 않았다고 명시한다. 링크 `#L251`은 해당 test의 `#[test]` 선언이며 이어지는 test 본문은 바로 다음 행이다. |
| 27 | checksum이 실제 staged artifact 전체를 덮지 않는가 | PASS | manifest는 HTML fixture, input/expected/observation JSON, PNG, README 여섯 파일을 포함하고 변경 대상 모두를 찾는다. manifest 자신을 제외한 관례적 구조다. |
| 28 | checksum 선언이 파일 bytes와 맞지 않는가 | PASS | `shasum -a 256 -c`에서 SHA256SUMS 6/6이 OK였고, staged diff whitespace 검사도 통과했다. |
| 29 | Agent Browser 정리 요구가 계획/증거에서 누락되는가 | PASS | Chrome observation JSON은 닫은 Agent Browser session ID, 종료 뒤 `session_info_active_after_close:false`, `active_session_count_after_close:0`을 기록하고 README도 같은 cleanup 결과를 연결한다. 이 근거는 해당 Agent Browser session에 한정한다. |
| 30 | PR presentation 요구와 이미지 URL의 불변성이 성립하는가 | PASS | live PR [#668](https://github.com/ohah/spinon/pull/668)은 OPEN, non-draft, mergeStateStatus=CLEAN이고 미병합이다. labels는 `planning`, `area: layout`, `area: docs`이며 본문에 변경 항목, C11.4/72·213 상태, 근거·API 미변경 사유·남은 범위와 완료된 검토 결과가 있다. 본문의 PNG URL은 head SHA `a5921deb873d3598a487535b0bfed2ef1dfc3078`에 고정된 raw URL이다. Agent Browser에서 해당 `<img>`의 `complete=true`, `naturalWidth=160`, `naturalHeight=100`, `visible=true`를 확인했다. 본문은 검토 횟수 대신 검토한 범위와 결론을 서술하고, 30관점 artifact를 PR branch의 최신 파일로 연결한다. 현재 PR은 checks 0이며 CI 통과 증거로 취급하지 않는다. |

## 재검토 후 상태

- PR #668 body를 계획·PR diff 검토의 실질 결과, C11.4 및 완료 집계 경계, 근거, 남은 범위와 동기화했고 최신 30관점 report 링크를 확인했다.
- 수정한 report를 포함한 staged diff에 `git diff --cached --check`를 실행했다. SHA256SUMS 여섯 항목과 JSON 문서 세 개는 기존 precomparison 검토에서 검증된 그대로이며 이번 live-PR 재검토에서는 다시 빌드·테스트하지 않았다. Agent Browser PR 확인 세션을 닫았고 `session list`에는 활성 세션이 없다.
