# C11.4 four-fit 구현·RuntimeSession 독립 검토 20개

계획 검토 및 계획 PR diff 검토와 분리해, four-fit 구현, 기존 profile 경계, parsed Root HTML RuntimeSession 관측을 서로 다른 실패 조건으로 재검토했다. 기준 scene과 기대값은 같은 폴더의 [Chrome 사전 비교](../c11-4-root-html-repeated-text-autoplacement-four-fit-auto-min-precomparison-2026-11-11/README.md)에 고정했다.

1. **목록 길이와 fit 수가 겹치는가?** Layout 및 style admission은 정확히 여섯 member를 요구하고 fit-content 네 개를 센다. 0–3 및 5–6 fit 변형은 거부한다.
2. **tail 종류/중복이 느슨해지는가?** 나머지 두 member의 개수가 `min-content` 1개와 `max-content` 1개인지 요구한다. `auto` tail과 중복 tail은 거부한다.
3. **음수·비유한 cap 또는 계산식을 통과시키는가?** 두 계층이 공용 직접 cap 검사와 같은 경계를 적용한다. 음수, NaN, 무한대, `calc()` cap 변형이 거부된다.
4. **Fit 위치를 정렬·정규화하며 authored order를 잃는가?** Style snapshot의 typed track vector와 각 scene의 authored member 순서를 직접 대조한다. 네 fit 위치의 15개 조합이 유지된다.
5. **Style projection과 layout admission이 다른 언어를 쓰는가?** 양쪽에 별도 four-fit predicate와 positive/negative test가 있고 RuntimeSession에서 projected CSS 문자열과 typed layout list를 같은 scene마다 대조한다.
6. **기존 one-/two-/three-fit profile이 four-fit 확장 때문에 바뀌는가?** 기존 1–3 fit matrix 회귀가 계속 positive다. 단일 fit classifier 자체가 four-fit list를 자기 profile로 받지 않는 검사도 추가됐다.
7. **Row-span 지원이 의도치 않게 열리는가?** Four-fit predicate는 auto-placement classifier에만 추가된다. Layout profile test에서 같은 four-fit list가 row-span classifier에 대해 false임을 확인한다.
8. **네 fit 위치와 min/max tail 순서를 충분히 순회하는가?** `C(6,4)=15` 위치 조합과 두 tail 순서를 모두 만들고 30 조합을 assert한다. Runtime corpus의 고유 위치와 tail 집합도 정확한 expected set과 비교한다.
9. **Cap의 위치별 permutation이 빠지는가?** 네 cap 값의 24개 순열을 모두 생성하고 720개의 distinct authored list가 존재함을 검사한다. 각 scene metadata도 실제 track list를 다시 읽어 검증한다.
10. **Automatic minimum과 zero minimum이 서로 다른 조건으로 비교되는가?** 5,760 scenes를 2,880개씩 나누고 `(track list, alignment, DOM order)` paired key 집합이 동일함을 assert한다.
11. **Chrome metadata가 payload나 track list와 어긋나도 통과하는가?** Cap, 위치, tail metadata를 authored track member에서 재구성해 scene metadata와 매번 비교한다. 직접 child와 row count도 모두 확인한다.
12. **Alignment/DOM order 축이 빠지는가?** `start|stretch`와 `forward|reverse`를 corpus set으로 검사하며 각 scene의 Grid와 item frame을 기준 좌표에 대조한다.
13. **빈 explicit-row control이 automatic minimum 적용 대상으로 섞이는가?** Item 0은 HTML generator가 항상 `min-height:0`으로 명시한다. Mutation test는 여기에 `auto`를 넣으면 거부되는지 확인한다.
14. **한 Grid에 혼합 최소 크기가 admission 되는가?** `auto`와 `0`을 섞고 지원하지 않는 고정 최소값을 넣은 변형을 별도로 거부한다.
15. **Fit-row minimum이 cap으로 잘리는가?** Matrix에서 item frame height가 Chrome row와 같고 text payload floor 이상인지 확인한다. Trace test는 zero cap에서도 양수 automatic-minimum contribution이 남는지 검사한다.
16. **Min/max tail을 fit row와 같은 eligibility로 세는가?** Trace 검사는 fit row item ID만 eligible set에 넣고 `min-content`/`max-content` tail은 따로 제외한다. 예상 fit row ID 집합은 `{1,2,3,4,7,8}`이다.
17. **Track-sizing trace가 다른 Grid나 revision에서 오는가?** Trace는 단일 Grid owner ID 및 cascade/layout revision을 연결하고, final row pass의 `AutomaticMinimum` source, 단일 track span, 비-flexible 조건을 확인한다.
18. **Text paint가 layout frame만 맞추고 glyph 결과는 틀려도 통과하는가?** 8개 payload별 frame, glyph 수, baseline line count와 Grid block 끝 위치를 검사한다. Candidate에서 item/payload overflow가 없어야 한다.
19. **잘못된 Root HTML topology가 profile에 섞이는가?** 추가 owner/column, 누락 child, nested/inline/replaced payload, explicit placement, row span, dense/column flow, scroll overflow와 unsupported alignment를 거부한다.
20. **거부 중 stale output이 게시되거나 같은 session이 복구되지 않는가?** 각 mutation은 Failed 및 `completed=None`을 확인한다. 별도 dark-mode revision rejection 뒤 새 light revision이 같은 session에서 Ready가 되고 새 key와 complete render snapshot을 게시하는지 검사한다.

**검토 결과:** 이 범위에서 추가 결함은 발견하지 못했다. 확인된 경계는 5,760개 고정 Chrome scene과 제한 Root HTML profile이다. 이 결과를 WPT 전체 적합성, 일반 CSS Grid, surface pixel parity, hardware GPU, 실기기 확인 또는 C11.4/C11 완료로 확대하지 않는다.
