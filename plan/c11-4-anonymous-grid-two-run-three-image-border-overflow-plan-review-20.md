# C11.4 두 anonymous text run + 세 image bordered Grid 계획 적대 검토 20개

**검토 대상:** 고정 fixture, 표준 예상값, Simulator gates, image overflow/border sample, artifact boundary와 한계. 구현 전 계획을 서로 다른 실패 관점으로 점검했다.

| # | 독립 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | 사용자 선택과 다른 기기에서 계획을 수행하는가? | Android API 37 AVD와 iPhone 17 Pro Simulator만 gate로 지정했다. 통과. |
| 2 | fixture 입력이 바뀌었는데 고정 browser evidence를 재사용하는가? | fixture/JSON/screenshot 각 SHA-256을 기록해 입력-관측 연결을 보존했다. 통과. |
| 3 | Ahem 또는 PNG가 준비되지 않은 상태를 성공으로 세는가? | font ready 및 이미지 여섯 개 decode/natural size를 precomparison gate로 확인했다. 통과. |
| 4 | text run이 둘이 아닌 하나로 그룹화될 수 있는가? | direct text sequence마다 image element가 경계가 되며 양 scene 모두 두 `XX` run이 보인다. 통과. |
| 5 | anonymous item placement를 DOM order에서 임의로 지정하는가? | 세 image의 explicit placement 후 두 text item을 row1 column3/4로 계산했다. 통과. |
| 6 | reverse scene에서 placement나 style도 함께 달라지는가? | 두 scene의 CSS placement/asset/size는 같고 source order만 뒤집는다. 통과. |
| 7 | intrinsic ratio 입력이 두 run 세 image 출력과 맞지 않는가? | 고정 PNG 24×16과 heights 16/32/48에서 24/48/72를 독립 계산했다. 통과. |
| 8 | 네 번째 track을 anonymous run 두 번째의 20px 기여로 설명하는가? | 두 run 모두 20px min-content이며 column3/4를 각각 채운다. 통과. |
| 9 | track3의 20px limit이 image B/C 단계에서 잘못 무시되는가? | span-1 text의 base/growth limit을 20px로 고정하고 up-to-limit에서 보존한다. 통과. |
| 10 | image A의 span이나 분배 입력이 잘못됐는가? | columns1–2, contribution24px, expected 12px씩을 명시했다. 통과. |
| 11 | image B의 잔여 공간 계산이 잘못됐는가? | columns2–3 base합 `12+20=32`, 잔여 `48-32=16`을 확인했다. 통과. |
| 12 | image B의 16px이 track3에 limit 초과로 들어가는가? | track3의 20px limit을 지키고 남은 공간을 무한 limit인 column2에 배분한다. 통과. |
| 13 | image C span3 expected가 첫 세 track 밖까지 섞이는가? | contribution72와 first-three base합60으로 12px 잔여를 분리했다. 통과. |
| 14 | 앞선 span group이 track1/2 growth limit을 이미 finite로 만들었는가? | infinite-limit substitute 후 span-2 growth-limit pass 잔여가 0임을 plan에 적었다. 통과. |
| 15 | span3 up-to-limit distribution에 column3가 끼어 20px을 넘기는가? | columns1/2만 무한 capacity라 각 6px 증가, column3는 20px 유지다. 통과. |
| 16 | beyond-limit을 먼저 적용해 과거 Chrome expectation을 반복하는가? | up-to-limit에서 12px 전부 소진되므로 beyond-limit 단계는 도달하지 않는다. 통과. |
| 17 | 두 source order의 독립 expected track이 다른가? | 둘 다 `[18,34,20,20]`으로 고정했다. 통과. |
| 18 | Chrome 156 CSSOM 값을 표준 expected로 승격하는가? | `[16,32,24,20]` 관측과 `[18,34,20,20]` 계산을 분리했다. 통과. |
| 19 | 34px overflow와 root-local sample 변환이 틀렸는가? | image frame x=2..74, outer border edge x=40이므로 overflow34; sample x62는 edge보다22px 밖이다. 통과. |
| 20 | 픽셀/지원 범위가 실제 계획 gate를 넘어서는가? | 단일 `#00ff40` sample, pre-compositor iOS readback, no-WPT/no-physical/no-full-C11.4 제한을 명시했다. 통과. |

이 검토는 계획 전용이다. 구현 코드, runtime 결과와 구현 PR diff는 각각 별도 검토한다.
