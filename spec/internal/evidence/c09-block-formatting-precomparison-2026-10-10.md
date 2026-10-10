# C09 Block formatting 구현 전 Chromium 기준

## 범위

이 기록은 C09 runtime 구현에 앞서 독립 oracle과 재현 입력을 고정한다. `RuntimeBlockFormattingV1`, Rust layout adapter, Android/iOS 앱은 아직 구현하거나 실행하지 않았다. 이 결과만으로 C09 CSS 지원이나 runtime 정확도를 주장하지 않는다.

## 고정 oracle과 입력

- Oracle: Google Chrome `154.0.8037.98`, DevTools revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`.
- 실행 파일: `/Applications/Google Chrome.app/Contents/MacOS/Google Chrome`; SHA-256 `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`.
- 실행 환경: macOS `25.5.0`, `arm64`, Node `v26.7.0`. Chrome 캡처는 임시 profile과 고정 headless flags를 썼다.
- viewport: `320×240` CSS px, DPR 1·2. `en-US`, `UTC`, light, forced colors none, coarse pointer, hover none.
- 입력: [HTML fixture](../../tests/fixtures/css/c09/block-formatting.html), [inventory](../../tests/fixtures/css/c09/block-formatting-inventory.json), [capture 도구](../../tools/css-reference/capture-c09-block-formatting.mjs), [공통 Chromium session helper](../../tools/css-reference/chromium-session.mjs).
- HTML SHA-256: `1b2207fddcad783f1079c7267ea037ae3b502297e833604532bc2fdab6e1211a`.
- inventory SHA-256: `6f9e4004b0cd543ff0d405f2446039e372ac681a4ebd8641c909b0ea8ed4236d`.
- capture 도구 SHA-256: `03e44c16a86e0f04902acd088dfd51917a7bac514bf34708ce1e69be9e3ad0eb`.
- reference ID: `chromium-darwin-arm64-Chrome-154.0.8037.98-6f9e4004b0cd-1b2207fddcad-03e44c16a86e`.

## 관찰과 판정 경계

고정 reference JSON은 23개 독립 case와 76개 app node의 computed CSS 값·Typed OM 진단·node별 CSS px rectangle을 담는다.

| 하위 범위 | case | node | 포함 동작 |
| --- | ---: | ---: | --- |
| C09.1 | 10 | 30 | fragment root와 viewport, auto width/height, min-width clamp, percentage containing block, box-sizing, 수평·수직 auto margin, LTR overconstraint, 음수 margin, `display:none` subtree |
| C09.2 | 9 | 30 | 양수·음수·분수 signed strut, 빈/self-collapsing Block, parent-first/last-child, padding과 used border 장벽 |
| C09.3 | 4 | 16 | flow-root 내부 child margin, 중첩 부모-자식 first/last-child 외부 margin, 이전/다음 형제 margin |

- DPR 1·2의 computed 값과 CSS px rectangle이 70개 node 모두 동일했다. 이 값은 DPR 불변 확인이며 Rust/Spinon runtime 오차 측정이 아니다.
- reference에는 각 rectangle의 x/y/width/height와 비교 한계 `0.5 CSS px`를 기록한다. `0.5 CSS px`는 향후 runtime 비교 허용치이고, 이번 oracle 캡처가 runtime과 그 오차를 통과했다는 뜻이 아니다.
- fixture node는 `horizontal-tb`, LTR, static, float/clear 없음, transform 없음, Block/flow-root/none 범위로 제한한다. case wrapper는 화면 원점에 절대 배치해 서로 겹치는 fixture 전용 `flow-root`이고 Spinon node가 아니다. 실제 runtime 비교는 inventory에서 case 하나만 HostRoot subtree로 만들고 wrapper를 제외한다.
- 실행·수치 비교는 macOS Chrome만 해당한다. Android/iOS Simulator, 실기기, Rust adapter, V8 runtime, WGPU 표시는 이번 범위에서 측정하지 않았다.
- 재현: `bun run css:reference:c09-block-formatting`; 입력 provenance 검증: `bun run test:css-reference`.

## 적대적 검토와 반영 사항

| # | 독립 실패 관점 | 확인 및 처리 |
| ---: | --- | --- |
| 1 | Chrome 설치 버전이 고정 oracle와 다름 | CLI 버전 불일치에서 캡처를 거부한다. |
| 2 | CLI는 맞지만 DevTools가 다른 Chromium revision을 보고함 | product와 revision을 별도로 비교하고 불일치를 거부한다. |
| 3 | 다른 Chrome 실행 파일이 같은 표시 버전을 주장함 | 실제 실행 파일 SHA-256과 절대 경로를 reference provenance에 기록한다. |
| 4 | 날짜·언어·색상·입력 환경이 바뀜 | locale, timezone, color scheme, forced colors, pointer, hover를 강제하고 관찰값을 검사한다. |
| 5 | viewport 또는 DPR이 바뀌어 layout을 바꿈 | 두 DPR에서 viewport와 devicePixelRatio를 확인하고 관찰 전체를 대조한다. |
| 6 | CSS fixture 수정 후 예전 JSON을 최신 oracle로 오인함 | HTML·inventory digest를 저장하고 reference 테스트에서 현재 입력과 대조한다. |
| 7 | fixture HTML만 바뀌어도 reference 식별자가 그대로 남음 | reference ID에 HTML digest를 추가해 입력 변경을 식별한다. |
| 8 | 이전 기준 JSON을 실수로 덮어씀 | 생성 도구가 기존 출력에 실패하는 것을 실행해 확인했고 출력 SHA-256이 유지됐다. |
| 9 | inventory에서 case·node·harness ID가 중복됨 | capture 전 중복 ID를 검사해 불완전한 관찰을 차단한다. |
| 10 | DOM 순서가 inventory 순서와 달라짐 | wrapper별 실제 preorder를 고정 tree preorder와 비교한다. |
| 11 | node가 다른 부모 아래 놓임 | 각 node의 parent ID를 확인하고 mismatch에서 실패한다. |
| 12 | wrapper 위치가 숨김 node 좌표를 상대적으로 왜곡함 | 최초 probe에서 숨김 rect의 상대 y가 음수로 나오는 문제를 발견했다. 모든 wrapper를 원점에 겹쳐 두고 wrapper origin 0을 검증하도록 고쳤다. |
| 13 | 앱 node와 fixture wrapper가 한 레이아웃 box로 계산됨 | wrapper는 원점에 겹쳐 배치한 `flow-root`이며 case마다 독립 root 하나를 둔다. runtime 비교기는 case 하나씩 HostRoot subtree로 materialize하고 wrapper를 제외한다. |
| 14 | computed content width와 border-box rectangle을 혼합함 | auto width case에서 computed `225px`와 outer rectangle `240px`를 별도 필드로 보존하고 둘 다 검증한다. |
| 15 | percentage width가 viewport를 기준으로 계산됨 | 중첩 definite containing block과 `border-box` percentage child를 별도 case로 고정한다. |
| 16 | auto margin을 양축에서 동일하게 배분함 | 수평 auto margin의 가운데 정렬과 수직 auto margin의 사용값 `0px`를 별도 case로 고정한다. |
| 17 | auto width의 min-width clamp가 부모 너비 방정식에서 빠짐 | auto width·min-width·고정 좌우 margin이 함께 있는 case를 추가했다. |
| 18 | `display:none` subtree가 사라지거나 이전 형제 흐름을 밀어냄 | 숨긴 부모·자식의 preorder와 0 rectangle을 보존하고 visible sibling의 위치를 함께 확인한다. |
| 19 | 여러 margin이나 음수·분수 margin을 pairwise max로만 처리함 | 3개 이상이 연결된 empty block strut, 음수 전용 형제, 1/4 px 단위의 mixed signed collapse를 따로 고정한다. |
| 20 | parent-child collapse 장벽 또는 flow-root 외부 margin을 잘못 적용함 | first/last child, padding, `border-style:none`의 used width 0, solid border, flow-root 내부·외부·이전·다음 형제 관계를 서로 다른 case로 구분한다. |

검토에서 발견된 harness 좌표 문제와 reference ID 누락은 수정했다. 코드·fixture 참조 검증은 `tools/css-reference/c09-block-formatting.test.mjs` 7개 테스트 및 전체 CSS reference suite로 확인한다. 남은 필수 경계는 C08.1 병합 뒤 수행할 C09 runtime 구현과 Android/iOS 비교다.
