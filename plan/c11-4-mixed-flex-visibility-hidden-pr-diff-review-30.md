# C11.4 `visibility:hidden` 계획 PR diff · 적대 검토 30개

**대상 diff:** 사전 비교 fixture/evidence, [mixed-track `visibility:hidden` 계획](./c11-4-mixed-flex-visibility-hidden.md), [20관점 계획 검토](./c11-4-mixed-flex-visibility-hidden-plan-review-20.md), 상위 [C11.4 plan 색인](./c11-4-grid-track-sizing.md).

**검토 경계:** PR diff의 문서·입력·oracle 연결을 검토했다. 제품 구현, 앱 runtime, Simulator, PR CI 또는 merge 결과를 검증하는 검토가 아니다.

| # | PR 변경분 실패 관점 | 판정 및 확인 |
| ---: | --- | --- |
| 1 | 신규 파일이 display:none의 기존 fixture/evidence를 덮어씀 | **통과.** 신규 파일명은 `visibility-hidden` 전용이며 기존 #734 파일의 경로는 변경하지 않는다. |
| 2 | 상위 계획이 display:none을 아직 계획 대기로 남김 | **통과.** 상단 색인 문단을 visibility-hidden 다음 계획으로 교체하고 기존 완료 계획 링크를 유지한다. |
| 3 | 새 계획에서 자체 plan-review 링크가 누락됨 | **통과.** 독립 계획 검토 20개 파일을 직접 링크한다. |
| 4 | 계획 PR diff 검토 파일을 plan 본문에서 찾을 수 없음 | **통과.** 이 review 파일을 집중 계획과 parent 색인에서 연결한다. |
| 5 | 기준 입력 HTML이 계획에 링크되지 않음 | **통과.** precomparison README에서 정확한 C11 fixture 경로를 링크한다. |
| 6 | Chrome screenshot이 로드 전 capture였음 | **통과.** Agent Browser는 `window.__ready` 뒤 observation과 screenshot을 저장했다. |
| 7 | Chrome version 또는 viewport가 사라짐 | **통과.** Observation JSON에 HeadlessChrome 156 UA, `1280×633`, DPR 1이 있다. |
| 8 | Observation이 visibility state를 기록하지 않음 | **통과.** 각 scene의 computed `display`, `visibility`, client rect count를 기록한다. |
| 9 | hidden scene에 실제 layout box가 있는지 불명확 | **통과.** `visibility:hidden`, `display:block`, client rect 1개와 item/root rect를 기록한다. |
| 10 | 기대값이 Chrome computed track만 복사 | **통과.** CSS 규범·Ahem text widths와 freeze 산술에서 별도 기대 JSON을 만든다. |
| 11 | `derive_expected.py`가 저장 JSON을 재생성하지 못함 | **통과.** Python Fraction의 `240/3`, `150` freeze, `90` remainder로 JSON을 생성한다. |
| 12 | 이상한 정수 변환으로 fraction 오차가 생김 | **통과.** 초기 fr size를 `"80"` rational string으로 보존한다. |
| 13 | visible control을 hidden scene과 다른 input으로 구성 | **통과.** Same fixture markup에서 visibility만 control ID별로 전환한다. |
| 14 | CSS source order가 overlay sample의 의미와 반대 | **통과.** Visible A 뒤에 span이 와서 visible control이면 magenta가 위에 그려진다. |
| 15 | `grid-column`/`grid-row` shorthand가 auto-placement를 허용 | **통과.** Hidden span에 두 축 `1 / 2` explicit placement를 준다. |
| 16 | Chrome screenshot이 pixel sample 위치를 정당화하지 못함 | **통과.** Root local positions, screenshot coordinates, RGB/hex와 scene semantics를 JSON에 묶었다. |
| 17 | CSS 색상 기대와 sampled 색상 불일치 | **통과.** `#2864dc`는 visible A, `#8e44ad`는 span background와 RGB sampling이 일치한다. |
| 18 | sample이 hidden item의 비-paint를 구별하지 못함 | **통과.** Hidden/visible paired controls의 같은 root-local pixel은 파랑/보라로 다르다. |
| 19 | sample JSON이 다른 screenshot revision을 참조 | **통과.** JSON은 최종 `chrome156-visibility-hidden.png`를 지정한다. |
| 20 | screenshot dimension과 browser DPR이 맞지 않음 | **통과.** Screenshot은 1280×633이며 DPR 1; sample points가 viewport 안이다. |
| 21 | row 기대값을 column geometry에서 잘못 전용 | **통과.** Row scene은 `120px` column과 `[150,40,90]` rows, image y=160px를 별도로 기록한다. |
| 22 | orthogonal writing-mode contribution을 명시하지 않음 | **통과.** Row input은 `vertical-rl` Ahem item을 고정하고 Chrome row track을 관측한다. |
| 23 | hidden item contribution trace 요구사항이 불명확 | **통과.** Owner NodeId, axis, span, `Minimum`/`MinContent`/`MaxContent`, value를 분리 assert한다. |
| 24 | hidden frame presence가 draw presence와 혼동 | **통과.** 계획은 full-size frame을 요구하면서 draw-list absence/sample을 별도 판정한다. |
| 25 | `visibility:collapse`가 지원 범위로 새어 들어옴 | **통과.** Collapse는 명시 rejection boundary에 남긴다. |
| 26 | 이전 display:none positive가 회귀 test에서 누락 | **통과.** PR #734의 기존 두 scene을 regression matrix에 포함한다. |
| 27 | simulator coverage를 physical-device proof로 과장 | **통과.** AVD/iOS Simulator만 언급하고 physical/hardware/compositor-after를 제외한다. |
| 28 | static archive 경계를 APK/IPA exclusion으로 확대 | **통과.** Product archive negative 및 fixture positive 의미를 분리하고 번들 exclusion을 주장하지 않는다. |
| 29 | 문서 변경이 공식 지원 상태를 올림 | **통과.** C11.4 checkbox와 `72/213`을 그대로 둔다. |
| 30 | Agent Browser가 실행 후 남거나 Terminal Browser를 사용 | **통과.** Browser lifecycle JSON은 session/process 0, Terminal Browser 미사용을 기록한다. 입력·기대값·화면의 SHA-256은 `SHA256SUMS.txt`에서 재검증했다. |

**판정:** PASS 30/30, 계획 PR diff blocker 0개. 구현 PR과 runtime은 이 PR의 검토 범위가 아니며 구현 이후 새 20관점·30관점 리뷰가 필요하다.
