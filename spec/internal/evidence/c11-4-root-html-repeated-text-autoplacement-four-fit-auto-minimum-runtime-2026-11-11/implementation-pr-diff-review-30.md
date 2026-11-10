# C11.4 four-fit auto-placement 구현 PR diff 실패 관점 검토 30개

검토 대상은 GitHub PR [#1344](https://github.com/ohah/spinon/pull/1344)의 원격 diff다. 검토 시 base는 `2fa78846db7490d75e4d894c59eb712a32e20576`, head는 `2b2222b2e3073789c1ccecb9f00c814fb49700cb`였고 원격 file list와 이 두 revision의 로컬 diff는 48개 경로로 일치했다. 구현/runtime 20관점 검토와 분리해 실제 PR 변경을 대상으로 검토했다.

1. **다른 Grid 경로까지 허용 범위가 넓어지는가?** 네-fit predicate는 기존 auto-placement list classifier에만 추가됐다. 공개 API/FFI나 일반 Grid parser 경로에는 변경이 없다.
2. **다섯 개가 아닌 트랙 목록이 실수로 통과하는가?** Style과 Layout 모두 member 수를 정확히 여섯 개로 제한하며 길이 5/7 단위 변형을 거부한다.
3. **fit-content 수가 member 수와 혼동되는가?** 네 개만 허용하고 0–3 및 5–6 count 변형은 네-fit predicate에서 거부한다.
4. **tail multiplicity가 틀린 목록이 허용되는가?** 각 tail을 하나씩 요구해 중복 `min-content`, `max-content`, `auto` tail을 거부한다.
5. **tail을 위치 정렬 과정에서 잃는가?** predicate는 입력 vector를 재배열하지 않는다. 15개 authored fit 위치와 두 tail 순서의 30 조합이 unit test에서 보존된다.
6. **Cap 값이 네 고정 상수로 제한되는가?** 별도 RuntimeSession control의 미리 나열하지 않은 px/percentage cap도 typed cascade에 보존되어 Ready가 되는지 확인한다.
7. **음수·비유한·계산식 cap이 새 predicate를 우회하는가?** Style·Layout predicate 테스트와 parsed Root HTML rejection matrix가 negative, NaN, infinity, `calc()` cap을 거부한다.
8. **Percentage cap에 basis가 없어도 허용되는가?** 높이를 auto로 바꾸는 runtime mutation이 거부된다. 지원 fixture에서는 definite 220px block-size가 basis다.
9. **Row-span route에 네-fit 지원이 새어 들어가는가?** 네-fit Layout classifier unit test는 span classifier 결과가 false임을 직접 확인한다. Runtime row-span 경계 회귀도 보존한다.
10. **기존 one-fit classifier가 네-fit 목록을 오인하는가?** one-fit regression은 이제 다섯-fit을 거부하고, 공유 classifier 검사는 네-fit 목록을 one-fit predicate에서 제외한다.
11. **기존 two-fit rejection test가 방금 허용한 세-fit만 검사하는가?** two-fit negative controls를 다섯-fit mutation으로 옮겼고 기존 two-fit positive matrix를 그대로 둔다.
12. **기존 three-fit rejection test가 새로 지원하는 네-fit을 거부하려는가?** 세-fit negative controls를 다섯-fit으로 이동했다. 기존 5,760-scene three-fit positive matrix와 row-span regression을 유지한다.
13. **기존 1–3-fit 기능이 전체 4-fit classifier 확장으로 바뀌는가?** 기존 one-fit, two-fit zero/auto minimum, three-fit auto-placement 및 three-fit row-span suite가 각각 별도 positive regression으로 실행되었다.
14. **Tail과 cap metadata가 fixture의 실제 authored list에서 벗어나도 통과하는가?** Runtime matrix는 여섯 serialized track member를 다시 읽어 fit 위치, tail 및 cap assignment를 관측 metadata와 비교한다.
15. **전체 scene 수만 맞고 조합 하나가 누락되는가?** 5,760 scene, 720 distinct ordered list, 15 fit 위치, 두 tail order, 24 cap permutation, 두 alignment와 두 DOM order를 각각 expected set과 대조한다.
16. **Automatic/zero minimum 비교에 짝이 빠지는가?** 두 cohort가 각 2,880 scene이며 `(track list, alignment, DOM order)` paired key 집합을 정확히 같게 요구한다.
17. **Scene마다 computed CSS track type/order가 다른가?** 각 RuntimeSession에서 typed `ComputedCssGridAutoRows` vector와 serialized computed `grid-auto-rows`를 독립 observation 값과 대조한다.
18. **Grid frame이나 direct child 개수가 틀려도 통과하는가?** 모든 scene에서 40×220 Grid frame, 9 items/rows, 여섯 authored member와 각 item frame을 Chrome observation에 비교한다.
19. **Explicit-row control에 auto minimum이 적용되는가?** 생성기는 `item-0`을 `min-height:0`으로 고정하고, 그 control만 auto로 바꾸는 mutation은 실패를 요구한다.
20. **여덟 repeated item의 minimum이 섞여 허용되는가?** auto/zero 혼합, unsupported fixed minimum 및 기존 두 minimum cohort 외 변형을 별도로 거부한다.
21. **Text content floor가 fit-content cap에 눌리는가?** 자동 최소 크기 scene에서 item height가 Chrome payload floor 이상이고 trace의 zero-cap automatic-minimum contribution도 양수인지 확인한다.
22. **Intrinsic tail을 fit row와 같은 eligible set으로 세는가?** Trace는 fit member를 배치한 item만 검사하며 고정 fixture에서 expected eligible item ID `{1,2,3,4,7,8}`과 대조한다.
23. **Trace가 다른 Grid owner나 revision에서 섞이는가?** Trace node는 현재 cascade Grid owner와 일치하고 completed layout/cascade key 및 environment revision도 함께 비교한다.
24. **Track sizing 중간값만 맞고 final track floor가 작아지는가?** 최종 row sizing pass의 automatic-minimum contribution과 final track base를 함께 확인한다.
25. **Layout frame만 맞고 text payload가 item 밖으로 넘치는가?** 각 payload frame, glyph 수, baseline line count, item 내부 overflow와 Grid block-end를 모든 scene에서 검사한다.
26. **Auto minimum이 baseline 문제 자체를 가린 채 회귀하는가?** zero-minimum baseline은 7,200 fit-row under-floor/visible-overflow occurrence와 2,880 overflow scene을 유지하고 auto-minimum candidate는 이를 0으로 줄인다.
27. **미지원 ownership/topology가 profile에서 빠지는가?** 두 번째 Grid owner, 추가 explicit column, 누락 child, nested/inline/replaced payload, explicit placement 및 row-span mutation이 거부된다.
28. **지원하지 않는 flow, clipping, alignment가 허용되는가?** RTL, column/dense flow, scrollable overflow와 `align-content:center` 변형을 거부한다. `start|stretch`는 positive matrix에 남는다.
29. **실패 scene이 stale render를 반환하거나 session 회복을 막는가?** Invalid dark revision은 `Failed`/`completed=None`이어야 하고, 같은 session의 다음 valid light revision은 새 key와 complete render snapshot을 게시한다.
30. **문서·화면 증거가 구현 범위를 과장하거나 실행 자원을 남기는가?** STATUS/0059는 C11.4 미완료와 `78/214`를 유지한다. 네 screenshot은 OS visual smoke로 한정하고 Android renderer·readback 부재를 적었다. capture 뒤 앱/source/data를 복구하고 iOS Simulator/AVD를 종료했으며 사용하지 않는 0-byte backup log를 제거했다.

**검토 결과:** 위 실패 조건에 대해 추가 결함을 발견하지 못했다. 실제 diff에는 48개 파일이 있으며 30항목 중 어느 것도 일반 CSS Grid, WPT, hardware GPU, 실제 기기 또는 C11.4/C11 전체 완료를 주장하지 않는다. 이 결론은 고정 Chrome 157 matrix, 기존 제한 profile 회귀, 현재 두 simulator 대상에만 한정된다.
