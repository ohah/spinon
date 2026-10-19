# C11.4 definite min-size + two-track auto-repeat 계획 검토 20개

**검토 대상:** `plan/c11-4-auto-repeat-two-track-min-size.md` 및 Chrome 사전 비교 fixture/model.
**판정:** 아래 실패 경로를 계획에 반영했다. Auto-fit 축별 Chrome 관찰은 규범 결과와 충돌하므로 이 plan의 acceptance에서 제외하고 미해결로 보존한다. Auto-fill matrix만 구현 관문으로 진행한다.

1. **Preferred/max-size가 minimum-only보다 우선되는 입력인가?** `width/height:max-content`와 `min-width/min-height`의 조합이 지정 size/max size로 분류되면 min-size oracle이 무효다. 계획은 실제 `auto-fill` Chrome 결과와 CSS Grid §7.2.3.2 min-size 식을 사전에 고정하고, 구현 경로의 resolved dimensions를 검사한다.
2. **`1fr` maximum fallback을 잘못 계산할 수 있다.** repeat 수에서 `1fr`를 가용공간 또는 1fr factor로 계산하면 반복 수가 바뀐다. 계획은 definite max가 없는 첫 track의 definite minimum 40px만 count input으로 사용하고 final used size는 별도 관찰한다.
3. **두 번째 repeated track을 놓칠 수 있다.** pattern 전체가 아니라 첫 track만 repeat basis로 쓰면 denominator가 40px가 된다. `40 20` sequence와 track 개수를 전부 기록하도록 acceptance를 구체화했다.
4. **Explicit prefix/suffix가 repeat 밖에 있어도 size 계산에서 빠질 수 있다.** 양쪽 이웃 20/10px를 non-repeated contribution으로 더하며 prefix-only·suffix-only case도 별도로 고정한다.
5. **마지막 gutter를 이중 계산할 수 있다.** pattern에 2개 track이 있을 때 pattern 내부/반복 사이 gutter 수는 반복당 2개지만 전체 grid 마지막 뒤 gutter는 없다. `38 + 76n` 및 한쪽 neighbor별 식으로 검산한다.
6. **경계에서 ceil/floor가 바뀔 수 있다.** min-size 190/200/342는 각각 2/3/4 반복의 경계를 고정한다. 입력 한도와 결과 기대값은 Chrome JSON 및 독립 식에서 일치해야 한다.
7. **Columns/Rows가 서로 다른 길이/스타일 경로를 탄다.** 두 축을 각각 실행하고 root width와 height를 별도 판정한다. 한 축 결과를 다른 축 증거로 대체하지 않는다.
8. **Track extent와 container min-size를 혼동할 수 있다.** 200px 최소 크기에서 3회 반복의 track+gutter 사용량 266px이 root를 확장한다. 두 값은 별도 assertion으로 기록한다.
9. **`auto-fit`을 `auto-fill` 결과에 맞춰 몰래 고칠 수 있다.** Chrome 155 fixture에서 empty/span columns와 rows가 다른 반복 수를 냈다. 규범은 빈 track 접기 전 횟수가 같다고 하므로 이 값은 unresolved interop note이며 이번 acceptance에 복사하지 않는다.
10. **Collapsed track과 collapsed gutter를 한 상태로 오인할 수 있다.** auto-fit diagnostic은 used track list와 gutter를 함께 기록하고, collapsed zero만으로 repeated count를 추론할 때 Chrome/axis 차이를 숨기지 않는다.
11. **Spanning item이 auto-fit occupancy를 잘못 바꿀 수 있다.** Chrome diagnostic의 span-two scene은 두 repeated track을 점유시키지만 acceptance에서는 auto-fill만 사용한다. Runtime 구현 후 auto-fit span behavior를 기존 definite-size case와 섞어 완료 판정을 내리지 않는다.
12. **고정 track control이 flex fallback을 증명하지 않을 수 있다.** fixed `40px 20px`는 parser/count arithmetic 대조군일 뿐이다. 실제 `minmax(40px,1fr) 20px` 결과를 별도 기대값으로 비교한다.
13. **반복 순서/인접 component가 깨질 수 있다.** prefix-only와 suffix-only는 repeat 삽입 지점과 line 순서의 서로 다른 경계를 확인한다. 둘 다를 양쪽 neighbor case로 대체하지 않는다.
14. **부분 repeated fragment 또는 잘못된 line 경계를 허용할 수 있다.** 고정 fixture는 `40 20` 두 track을 모두 보존해야 하며 parser/projection trace에 반복 range 시작·끝을 확인한다.
15. **Track count가 급증해 기존 10,000 제한을 침범할 수 있다.** 4 repetitions는 기존 bound보다 충분히 낮다. 이 계획은 10,000-track 안전성의 새 증거라고 주장하지 않고 그 경계를 남겨 둔다.
16. **Fractional serialization 허용차가 필수 차이를 숨길 수 있다.** 이 planned inputs는 expected track들이 정수 CSS px이므로 정확 비교한다. 1/64 허용은 다른 Chrome fractional output이 생기는 경우에만 쓴다.
17. **브라우저 oracle이 구현 출력으로 오염될 수 있다.** Chrome JSON과 fixture를 먼저 기록했고 Spinon 출력은 포함하지 않는다. 반복 수 기대값은 CSS 식에서 독립 산출하며 앱 결과를 보고 갱신하지 않는다.
18. **화면 screenshot이 layout oracle을 대신할 수 있다.** PNG는 재현 환경 보조물이다. acceptance는 used track list, geometry, trace JSON으로 결정하고 simulator screenshot을 Chrome evidence로 세지 않는다.
19. **미지원 grammar가 이번 계획에서 새로 승인될 수 있다.** line names/areas, non-px neighbor, multiple auto-repeat, dual-axis repeat, percentage/calc, integer-repeat 혼합은 scope에 넣지 않고 기존 fail-closed 계약을 바꾸지 않는다.
20. **부분 matrix를 C11.4 완료로 승격할 수 있다.** WPT, general placement/span/gutter, intrinsic image/text matrix, track limit 및 auto-fit discrepancy는 남는다. `spec/STATUS.md` 체크와 공식 count는 바꾸지 않는다.

이 검토는 계획 단계만 대상으로 한다. 구현 code/runtime review와 PR diff review를 대체하지 않는다.
