# C11.4 mixed direct text/image 계획 PR diff 실패 관점 20개

이 검토는 계획 PR의 최종 staged diff를 대상으로 한다. 계획 자체의 실패 검토 20개와 별도이며, 코드·Spinon runtime·Simulator 결과를 검증했다고 주장하지 않는다. 1차 검토에서 발견한 세 문서 결함을 수정하고 영향 관점을 재검토했다.

| # | 독립 실패 관점 | 최종 판정 |
|---:|---|---|
| 1 | PR #583의 병합 상태·시각·SHA가 상위 계획과 실제 GitHub 상태에서 다를 수 있음 | **PASS.** PR이 MERGED이고 2026-10-24T05:04:59Z, `4faabe77...` 정보가 일치한다. |
| 2 | PR #585의 병합 상태·시각·SHA가 계획과 실제 GitHub 상태에서 다를 수 있음 | **PASS.** PR이 MERGED이고 2026-10-24T06:25:51Z, `e42b1573...` 정보가 일치한다. |
| 3 | 기존 PR 링크가 다른 auto-repeat 변경을 가리킬 수 있음 | **PASS.** #583은 positive percentage calc neighbor, #585는 signed percentage 후속을 가리킨다. |
| 4 | 계획 PR이 테스트·CI 통과나 product runtime을 주장할 수 있음 | **PASS.** 변경은 계획-only이며 새 build/test/runtime claim이 없다. |
| 5 | 상위 계획 날짜가 현재 STATUS 기준일과 어긋날 수 있음 | **PASS.** 초기에 `2026-10-24`였으나 `2026-10-25`로 갱신하고 재확인했다. |
| 6 | 신규 slice가 구현 완료 상태로 보일 수 있음 | **PASS.** 계획과 STATUS 모두 계획-only/구현 미검증으로 표시한다. |
| 7 | Spinon runtime·Android·iOS 결과를 선취할 수 있음 | **PASS.** 미실행이라고 명시하고 simulator를 구현 관문으로 남겼다. |
| 8 | 미검증 범위가 숨겨질 수 있음 | **PASS.** 큰 span, nested/anonymous, intrinsic rows, CSS admission, WPT, device/GPU/pixel 한계를 적었다. |
| 9 | PNG 자산 링크가 존재하지 않거나 다른 파일을 가리킬 수 있음 | **PASS.** 최초 `tests/resources` 링크를 실제 `tests/fixtures/resources/c14-1-png-basic.png`로 수정하고 파일을 확인했다. |
| 10 | Chrome 관측이 규범 기대값 산출을 대신할 수 있음 | **PASS.** 기대값은 Ahem/PNG와 Grid §11.5로 별도 계산하고 Chrome은 observation으로만 둔다. |
| 11 | Ahem 텍스트 min/max 계산이 글리프 수와 맞지 않을 수 있음 | **PASS.** `XX XXXXX`의 min-content 5 glyph, max-content 8 glyph, 각 10px advance로 50/80px이다. |
| 12 | Replaced PNG의 natural inline size가 fixture 자산과 다를 수 있음 | **PASS.** 고정 자산은 24×16이며 Chrome decode/natural-size 관찰과 일치한다. |
| 13 | §11.5 span-1 aggregate가 기대값에 적용되지 않을 수 있음 | **PASS.** 같은 track의 text/image contribution 최댓값을 사용해 50/80px을 도출한다. |
| 14 | root width, row size, gap 또는 box edges가 계산을 바꿀 수 있음 | **PASS.** 120×48 root, 24px 두 row, gap/padding/border 0을 명시했다. |
| 15 | fixture의 상대 Ahem/PNG URL이 실제 자산과 끊길 수 있음 | **PASS.** 두 자산 모두 저장소 `tests/fixtures/resources`에 있으며 Chrome에서 로드됐다. |
| 16 | observation JSON이 다른 HTML fixture로 바뀌어도 validator가 통과할 수 있음 | **PASS.** JSON의 fixture path와 SHA-256을 HTML source와 비교한다. |
| 17 | JSON의 input 또는 scene order가 fixture와 어긋날 수 있음 | **PASS.** 고정 input object, 4개의 고유 scene ID와 forward/reversed source-order pair를 검사한다. |
| 18 | saved result가 기대값과 어긋나거나 scene이 빠져도 통과할 수 있음 | **PASS.** validator 재실행에서 4/4 scene, 50px/80px 및 item/root frame 조건이 통과했다. |
| 19 | 1280×633 viewport 이미지를 전체 페이지 캡처라고 과장할 수 있음 | **PASS.** 표현을 viewport capture로 좁히고 전체 observation data는 JSON이라고 분리했다. |
| 20 | 신규 plan/evidence 링크 중 하나가 끊기거나 집계가 오를 수 있음 | **PASS.** PR 링크 외 Markdown 상대 링크를 대조했고 C11.4 checkbox·공식 `72/213`을 유지한다. |

## 최종 판정

1차 검토의 날짜, PNG 링크, screenshot 범위 표현 세 결함을 고치고 재검토해 **20/20 PASS**다. 현재 diff에는 병합 차단 결함이 없다. Agent Browser session은 닫혀 있고 이번 작업의 browser process도 남아 있지 않다.
