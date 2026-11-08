# C11.4 반복 텍스트 행 span 구현 적대적 검토 20개

검토 대상은 반복 intrinsic row의 제한 텍스트 span 구현과 실제 parsed Root HTML runtime 결과다. 이 검토는 C11.4 전체 또는 일반 CSS Grid span 지원 완료를 뜻하지 않는다.

1. **입력 원본이 같은가?** fixture sentinel과 Android/iOS 번들·runtime 로그의 SHA-256이 모두 `c1f81d9b…14668a9`로 일치한다. 통과.
2. **지원 토폴로지가 계획된 수로 제한되는가?** visible direct child 3개 또는 4개만 후보가 되며 각 span 조합은 정확히 한 번씩 확인한다. 960 scene 전부 통과.
3. **span item DOM 순서를 바꿔도 위치가 안정적인가?** 두 span-2 item의 순서와 span-2/span-3 item의 순서를 바꾼 네 topology를 Chrome oracle과 비교한다. 통과.
4. **첫 explicit-row item의 위치가 고정되는가?** 비어 있는 첫 direct child는 row `1 / 2`, column `1 / 2`여야 한다. 다른 배치는 validator가 허용하지 않는다. 통과.
5. **열 방향 자동 배치가 새 implicit column을 만들 수 있는가?** 모든 span item에 column `1 / 2`를 요구한다. `grid-column:auto`와 다른 end line을 포함한 변형 35개 거부 matrix에서 확인한다. 통과.
6. **자동·음수·역방향 row line이 지원으로 오인되는가?** row auto start, negative line, reversed pair 및 line end 형태를 거부한다. 통과.
7. **span 길이 경계가 정확한가?** span 1, 4, 5와 topology의 span 수가 다른 값을 거부한다. 통과.
8. **중복·누락 child가 부분 장면을 남기는가?** missing direct item, duplicate direct item, duplicate span start를 모두 실패시키며 `completed=None`이다. 통과.
9. **wrapper와 payload의 깊이가 제한되는가?** nested wrapper, inline child, missing payload가 거부된다. 통과.
10. **텍스트 grammar와 계산 스타일이 잘못되어도 허용되는가?** lowercase, weight·size 변경, nowrap, letter spacing, RTL text를 거부한다. vertical writing mode는 cascade 단계에서 `cascade_failed`로 멈추며 completed output이 없다. 통과.
11. **반복 auto-row list의 구성원·순서가 보존되는가?** 6-member list의 30개 고유 tail order를 전 scene에서 computed style과 비교한다. wrong multiplicity 및 extra explicit track은 거부한다. 통과.
12. **fit-content cap의 단위와 percentage basis가 깨지는가?** `20px`, `20%`, `0%`, `120%`를 네 topology 및 두 정렬과 Chrome에 비교한다. percentage 섞인 calc 변형은 거부한다. 통과.
13. **content alignment 범위가 넓어지는가?** `start`와 `stretch`만 oracle 행 크기와 비교한다. center 변형은 거부한다. 통과.
14. **container의 크기·축·간격·overflow가 다른 입력으로 확장되는가?** 40px column, 10px explicit row, 220px block size, LTR, zero gap, visible overflow 경계를 validator가 확인한다. extra explicit column, nonzero row gap, clipped overflow 및 column/dense flow를 거부한다. 통과.
15. **숨김 child나 추가 Grid owner가 소유권 판정을 우회하는가?** hidden span item과 두 번째 visible Grid owner가 거부된다. 통과.
16. **계산 style과 layout이 서로 다른 revision을 게시하는가?** 960 scene 각각에서 `RuntimeLayoutCompleted.key == cascade.key`를 확인한다. 환경 변경 뒤 새 key로 recovery 한다. 통과.
17. **trace가 누락되거나 다른 owner의 trace를 사용할 수 있는가?** row trace 없음과 Grid owner ID 불일치를 넣어 normalization이 error를 반환하고 frame/flow frame map을 보존하는 단위 테스트를 추가했다. 통과.
18. **trace의 row 수나 내부 인덱스가 모순되어도 보정되는가?** 3개 row trace와 불연속 internal index를 거부하며 frame map을 바꾸지 않는다. 통과.
19. **정규화가 실제 track/span geometry를 벗어나는가?** explicit 1 + positive implicit 3, negative/collapsed 없음, final trace 4 tracks, 각 item의 trace 기반 row interval 합과 사전 frame이 0.01px 이내여야 한다. 960 scene 통과.
20. **실패한 revision이 부분 출력 또는 session 오염을 남기는가?** 35개 거부 변형은 `completed=None`; 같은 session의 dark-scheme 거부 후 light-scheme 재평가에서 새 revision과 원래 frame을 회복한다. 통과.

실행 로그는 같은 디렉터리의 `runtime-repeated-text-span.log`, `layout-row-span-trace-failures.log`, `layout-grid-track-sizing.log`, `android-runtime.log`, `ios-runtime.log`에 있다. 캡처는 `android-root-html-span-sentinel.png`, `ios-root-html-span-sentinel.png`이다.
