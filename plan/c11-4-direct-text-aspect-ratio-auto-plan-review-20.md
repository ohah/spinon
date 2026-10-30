# C11.4 direct-text `auto <ratio>` 계획 실패 관점 검토 20개

**대상:** [content-box ratio 계획](./c11-4-direct-text-aspect-ratio-auto.md)과 [Chrome 156 사전 비교](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-auto-precomparison-2026-10-30/README.md)

**기준:** `main` / `origin/main` `38359a4212550fab9e29544367a16e4eba45e06a`에서 분기한 같은 체크아웃의 계획 변경 전 source

**판정:** 계획 입력·실패 범위 20/20 PASS. 구현을 평가하거나 C11.4 완료를 주장하지 않는다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | `auto <ratio>`가 숫자 `<ratio>`와 같은 `box-sizing` 기준으로 계산 | PASS — 표준 basis를 content-box로, 숫자 ratio는 CSS `box-sizing` 기준으로 분리하고 Chrome 91px/90px 열 대조가 있다. |
| 2 | ratio 방향을 height/width로 뒤집음 | PASS — `width / height`와 auto-row `40 / 1.5` 및 definite-row `54 × 1.5` 계산을 모두 고정한다. |
| 3 | `auto` 키워드가 `<ratio>` 의미를 무효화한다고 해석 | PASS — non-replaced item에 지정 ratio가 preferred ratio로 남고 content-box basis가 별도 typed 값으로 보존된다. |
| 4 | `box-sizing:border-box`의 padding/border를 auto ratio 식에 포함 | PASS — 40px content width와 6px vertical edge를 분리해 독립 row 기대값을 계산한다. |
| 5 | `box-sizing:content-box`에서도 auto ratio에 edge가 두 번 더해짐 | PASS — 같은 padding/border의 content-box scene을 별도 대조군으로 요구한다. |
| 6 | Padding과 border의 축별 합을 뒤바꾸거나 누락 | PASS — horizontal 10px, vertical 6px을 입력과 독립 계산에 구분한다. |
| 7 | Ahem webfont 미로드 또는 fallback 상태로 폭을 고정 | PASS — `document.fonts.ready`, `fontCheck`, HTTP 200과 pinned Ahem hash를 기록했다. |
| 8 | Chrome의 1/64 CSS px used-value 양자화를 oracle 오차로 오판 | PASS — 32⅔·33⅓ 독립 값과 Chrome 양자값을 나눠 기록한다. |
| 9 | Auto row contribution에서 ratio item을 `align-items:normal` stretch control과 혼동 | PASS — preferred ratio, `auto` control과 각각의 row size를 세 장면으로 분리한다. |
| 10 | Definite row에서 ratio item frame 대신 grid-area 폭을 검사 | PASS — item `91×60`/`90×60`와 `91px`/`90px` track을 별도로 요구한다. |
| 11 | Max-content track과 item outer size를 같은 값으로 간주 | PASS — 40px Ahem content, 10px edge, 91/90px ratio track 관계를 독립 계산한다. |
| 12 | 100px root에 111px 이상 tracks가 배치돼 발생한 overflow를 누락 | PASS — root `100px`과 used track 합/frame/overflow를 분리하고 clipping을 허용하지 않는다. |
| 13 | No-ratio computed `auto`와 degenerate `auto 0/0`을 같은 positive로 수락 | PASS — no-ratio는 serialization `auto`만 허용하고 degenerate candidate를 fail-closed한다. |
| 14 | Typed ratio가 computed serialization의 숫자나 `auto` basis와 다름 | PASS — 같은 revision의 typed ratio·basis와 property serialization의 일치를 collector 계약으로 둔다. |
| 15 | Ratio를 short item, 두 item 또는 hidden/replaced item에 적용 | PASS — 하나의 `AAAA` owner만 허용하고 `A`, duplicate, replaced/extra structure를 negative controls로 둔다. |
| 16 | Auto ratio 지원이 C07.3·Flex·Block 등 다른 profile로 새어 나감 | PASS — 허용 cascade profile을 `RuntimeBlockFlexGridTrackSizingV1`로 제한하고 현행 rejection을 보존한다. |
| 17 | Taffy에 content-box basis 전달 시 일반 border-box size/min/max의 meaning이 바뀜 | PASS — fixture의 size/min/max auto 제한을 검사한 뒤 그 opt-in item에만 basis를 투영하고 computed property snapshot을 유지한다. |
| 18 | Unsupported edge/percentage/calc/dimension input을 일부만 계산해 success 반환 | PASS — fixed nonnegative edge와 auto dimension만 positive로 두고 나머지는 layout 전 거부한다. |
| 19 | Simulator evidence를 product CSS 지원·실기기·compositor-after pixels로 확대 | PASS — AVD/iOS Simulator, software renderer 및 compositor 전 readback 한계를 계획에 명시한다. |
| 20 | 새 slice를 C11.4/C11 완료나 공식 집계 증가로 처리 | PASS — C11 checkbox와 공식 `72/213` 유지, fixture-only 범위와 API 0.1.0 고정을 완료 gate로 둔다. |

## 검토 후 확인

W3C 2026-09-04 문서는 Working Draft이자 exploratory draft라 Level 3를 기준으로 사용하라고 적으며, §4.1에 content-box rule을 둔다. 따라서 계획은 이를 진행 중인 문서로 라벨링하고 단독 규범 근거로 게시하지 않는다. HeadlessChrome 156 관측에서 `fontCheck:true`, auto row `32.65625px` 대 `33.328125px`, definite row columns `91px` 대 `90px`을 확인했다. 이는 이 고정 입력의 browser oracle이지 표준 전체·Spinon 결과 검증은 아니다.
