# C11.3.3 양 축 negative named-line 계획 실패 관점 검토 20개

| # | 실패 관점 | 계획의 방어 또는 남은 확인 |
|---:|---|---|
| 1 | 기존 one-axis mode가 양 축 성공으로 조용히 넓어진다 | 신규 typed mode를 요구하고 기존 #967 path를 regression control로 보존한다. |
| 2 | 한 축의 endpoint resolution을 다른 축에 재사용한다 | row/column custom-ident resolution과 signed extent를 독립 계산한다. |
| 3 | authored name precedence보다 implicit fallback을 먼저 택한다 | authored matching line control과 precedence negative case를 고정한다. |
| 4 | negative occurrence에서 leading implicit extent를 한 track 덜 만든다 | `-4/-1`의 네 leading tracks와 세-track item span을 CSSOM/independent expected로 분리한다. |
| 5 | equal endpoints가 0-track span으로 처리된다 | §8.3.1 end-line removal 뒤 기본 1-track span, `[18,20]`/`[12,15]`를 고정한다. |
| 6 | reversed endpoints가 swap되지 않거나 pattern을 잘못 바꾼다 | 양 축 동시 및 축별 reversed controls를 고정한다. |
| 7 | leading fixed-pattern phase가 explicit edge에서 틀린다 | 독립 기대 track order `[5,7,5,7,20]` 및 `[3,4,3,4,15]`를 둔다. |
| 8 | item contribution이 한 축에서 다른 축으로 오염된다 | 18px Columns와 12px Rows를 독립 계산하고 auto/fixed controls를 둔다. |
| 9 | span 밖 leading auto track이 item contribution을 나눠 가진다 | 네 번째 leading track은 0px로 명시하고 per-axis vectors를 전부 비교한다. |
| 10 | frame만 일치하고 resolved tracks가 틀린다 | CSSOM track vector와 item frame을 모두 acceptance로 둔다. |
| 11 | row/column validator가 서로 바뀐다 | 비대칭 18×12 item과 20px/15px explicit tracks로 axis swap을 감지한다. |
| 12 | stale snapshot의 placement가 새 layout에 남는다 | HostDocument, style snapshot과 layout revision identity를 함께 검사한다. |
| 13 | extra participating child가 sizing contribution에 영향을 준다 | child count를 exact 1로 제한하고 0·2개 negative controls를 둔다. |
| 14 | auto-sized item으로 다른 intrinsic contract가 열릴 수 있다 | width와 height가 모두 fixed인지 확인하고 auto dimension은 거부한다. |
| 15 | percentages, `fr`, `minmax`, intrinsic tracks가 우연히 허용된다 | auto 및 fixed-px list 외 grammar는 publication 전에 명시 거부한다. |
| 16 | 양 축 중 하나만 valid할 때 partial frame이 publish된다 | both-axis mode에서 한 축 오류는 atomic failure와 frame 부재로 검사한다. |
| 17 | axis extent overflow/10,000 limit에서 wrap/clamp가 생긴다 | checked signed arithmetic와 shared ceiling, over-limit negative test를 요구한다. |
| 18 | app fixture 성공이 default artifact에 노출된다 | default/product negative control과 fixture-test positive control을 모두 빌드해 확인한다. |
| 19 | AVD/Simulator 증거를 실기기나 compositor 결과로 과장한다 | AVD software renderer, iOS pre-compositor drawable과 실기기 미검증을 따로 기록한다. |
| 20 | 제한 slice를 C11.3.3/C11 또는 WPT 완료로 승격한다 | 공식 checkboxes, contract version `0.1.0`, `72/213` 및 public support 한계를 명시적으로 유지한다. |

검토는 Chrome 사전 비교 자료와 계획 본문을 기준으로 각 실패 경로를 분리해 수행했다. 계획은 리뷰 완료이며 구현/runtime 승인은 별도 20개 검토를 요구한다.
