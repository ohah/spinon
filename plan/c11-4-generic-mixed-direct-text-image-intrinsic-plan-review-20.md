# C11.4 generic mixed text/image runtime · 계획 실패 관점 검토 20

검토 대상: [`c11-4-generic-mixed-direct-text-image-intrinsic.md`](./c11-4-generic-mixed-direct-text-image-intrinsic.md)

범위: 구현 전 계획, 고정 Chrome/Ahem/PNG oracle, 기존 static mixed route와 collector 경계. 코드 변경, 테스트, 빌드, simulator 실행은 하지 않았다.

| # | 서로 다른 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 제한 fixture 경로가 일반 HTML/CSS 지원 주장으로 확대됨 | opt-in feature만 변경 대상으로 고정했고 public CSS, product ABI, WPT, 전체 C11.4와 `72/213` 변경을 명시적으로 제외했다. PASS |
| 2 | 기존 static scene가 새 adapter에 가로채여 회귀할 수 있음 | 새 adapter는 기존 `SCENES` ID lookup miss에만 실행한다. fixed scene의 기존 validation·route는 유지한다. PASS |
| 3 | 새 adapter가 generic direct-text collector보다 늦게 호출됨 | 계획은 mixed adapter 반환 시점이 generic collector보다 앞이라고 명시한다. 유효 mixed 입력이 text-only collector에 들어가지 않는다. PASS |
| 4 | one-item legacy `TEXT_CASES` fallback이 변경됨 | generic text collector의 `Ok(None)` 경계를 유지하고 full dispatcher의 legacy route를 별도 보존한다고 명시했다. PASS |
| 5 | 두 text item과 `<img>` 조합의 기존 거부가 완화됨 | generic collector rejection을 제거하지 않으며 기존 거부 동작의 직접 regression control을 요구한다. PASS |
| 6 | 새 HostNode ID가 계속 scene 선택에 사용됨 | 새 긍정 scene은 고정 root/item 목록에 없는 HostNode ID를 쓰며 기대값과 source kind는 content/style에서 정하도록 고정했다. PASS |
| 7 | 서로 다른 mixed child 수 또는 HTML namespace가 수용됨 | 정확히 두 direct HTML element, 하나의 text `div`, 하나의 leaf `img`를 요구하고 다른 topology는 fail-closed한다. PASS |
| 8 | anonymous text 또는 nested text가 source처럼 처리됨 | text item은 direct element child이고 그 아래 direct text node 하나만 허용한다. Nested/non-leaf topology는 negative input이다. PASS |
| 9 | source 순서와 grid placement가 혼동됨 | forward/reversed DOM order를 별도 장면으로 유지하되 text row 1, image row 2 placement는 고정한다. DOM source order와 node identity를 검사한다. PASS |
| 10 | min-content Ahem 기대값이 틀리거나 Chrome에서 순환 생성됨 | `XX XXXXX` 중 가장 긴 단어 5 glyph × 10px = 50px로 독립 계산하고 Chrome은 관측값으로만 사용한다. PASS |
| 11 | max-content에 공백 advance가 누락됨 | 전체 8 glyph × 10px = 80px로 계산했다. PASS |
| 12 | image intrinsic 기여가 잘못됐거나 CSS 크기 override가 숨음 | 고정 PNG natural size 24×16, auto width/height, aspect-ratio override 없음, 실제 decode를 요구한다. PASS |
| 13 | used track이 합이 아닌 max contribution 규칙을 놓침 | §7.2.1/§11.5 step 2에 따라 `max(50,24)=50`, `max(80,24)=80`을 네 source-order scene에 적용한다. PASS |
| 14 | root/item frame과 column track을 혼동함 | 120×48 root, fixed 24px rows, text의 50×20/80×10 frame, image (0,24) 24×16을 track 기대값과 별도로 기록한다. PASS |
| 15 | root의 unrelated style 차이가 target scene으로 받아들여짐 | border-box, dimensions, track list, gaps, edges, alignment, direction, writing-mode와 fixture profile을 엄격한 조건으로 제한했다. PASS |
| 16 | item width/height, span 또는 self-alignment mismatch가 수용됨 | 두 item의 column 1/span 1, row 1/2, static block, auto self-alignment, zero box edges 및 source별 sizing을 요구한다. PASS |
| 17 | stale style/document revision과 이미지 resource revision이 섞임 | 동일 snapshot/style revision을 요구하고 resource decode, intrinsic input, axis trace를 같은 run에 묶도록 했다. PASS |
| 18 | 거부 시 partial text-only 또는 image-only 결과가 성공 처리됨 | signature가 있는 잘못된 mixed input은 error로 끝내고 partial `C11_4MixedInput` 생성·다른 handler fallthrough을 금지한다. PASS |
| 19 | FFI route가 HostNode ID 기반 routing을 다시 도입함 | 신규 runtime route ID와 novel root/item HostNode ID를 분리하고, Android/iOS matcher에서 node order·source·trace·frame을 검사하도록 했다. PASS |
| 20 | simulator·screenshot·merge 상태가 범위를 넘어 보고됨 | Android API 37 AVD/iPhone 17 Pro simulator를 요구하고 glyph/PNG pixels, physical device, compositor 이후 pixels를 제외한다. Agent Browser 링크 확인, 브라우저 종료, empty check rollup 한계, post-merge roadmap sync를 명시했다. PASS |

## 판정

**PASS 20 / PARTIAL 0 / FAIL 0.** 계획의 admission boundary, 독립 기대값, fallback/rejection 보호, 호출 순서와 증거 한계는 구현 계획을 시작할 만큼 구체적이다. 계획 PR diff 검토 20관점은 이 보고서와 별도로 남아 있다. 구현은 계획 PR 병합 후 진행한다.
