# C11.4 anonymous Grid border overflow 계획 실패 관점 검토 20개

이 검토는 구현 전 계획의 oracle, 독립 sizing 계산, border/overflow 검증 경계와 artifact 범위만 반박한다. 구현 코드나 PR 변경 검토로 대신하지 않는다.

1. **Chrome oracle version이 바뀔 수 있음** — Chrome for Testing `155.0.8059.85` full version, Agent Browser, macOS architecture, viewport와 DPR을 observation에 기록했다.
2. **Ahem fallback이 text sizing을 바꿀 수 있음** — `document.fonts.check('10px Ahem')`이 true인 시점에만 관찰하고 text min-content `20px`을 저장했다.
3. **PNG decode 전에 natural size를 읽을 수 있음** — 네 image의 decode promise가 모두 완료되고 natural size가 `24×16`인지 확인한다.
4. **직접 text가 하나의 anonymous item으로 합쳐질 수 있음** — 각 `XX` run 사이에 direct grid-item `img`를 두고 HTML child-node 순서와 Range를 각각 기록한다.
5. **element child 순서를 reverse했다고 text item 순서까지 뒤집힌다고 착각할 수 있음** — 역순 fixture에서도 text node 두 개의 순서는 유지하며, 각 text와 이미지 ID를 JSON에 별도로 보존한다.
6. **explicit image가 auto-placement cursor를 소비하지 않을 수 있음** — 이미지의 numeric row/column 점유를 먼저 둔 뒤 남은 anonymous item의 첫 빈 cell을 §8.5에서 계산하고 Chrome geometry와 대조한다.
7. **auto-placement가 dense나 column mode로 달라질 수 있음** — `grid-auto-flow:row` 기본값만 사용하고 다른 flow는 allowlist에 넣지 않는다.
8. **anonymous text Range rect를 anonymous item box로 오인할 수 있음** — 관찰 JSON에서 `textFragment`로 이름 붙이고 item frame 기대값과 구분한다.
9. **독립 track 기대가 Chrome CSSOM에서 역산될 수 있음** — image 24px span, text single-span 20px, image 48px span의 contribution을 §11.4–§11.5.1 순서로 별도 유도한다.
10. **첫 image 기여를 무시해 column 1·2가 0으로 남을 수 있음** — 24px spanning minimum을 양 track에 12px씩 배분한다고 계획에 기록한다.
11. **text column 3·4의 base가 image span 계산 뒤 적용될 수 있음** — 20px single-span text contribution을 먼저 배치해 second image의 initial 32px sum에 포함한다.
12. **48px span contribution을 단순 균등 분배할 수 있음** — column 3의 20px growth limit과 column 2의 remaining 16px planned increase를 독립적으로 적었다.
13. **두 span item 순서가 결과를 바꿀 수 있음** — forward/reverse scene에서 used track 배열과 image frame을 모두 검사한다.
14. **border-box 폭과 content-box 폭을 혼동할 수 있음** — 40px border-box, 각 2px border, 36px content width를 각각 기록한다.
15. **overflow 계산이 content edge를 기준으로 잘못될 수 있음** — `22px`은 root border-box 오른쪽 x=40과 image frame 끝 x=62의 차이다.
16. **grid track sum이 definite width에 맞춰 축소된다고 가정할 수 있음** — `min-content` track 합 80px과 36px content width를 비교하고 used sizes를 판정값으로 고정한다.
17. **CSS border가 있는데 renderer가 그리지 않아도 성공 처리할 수 있음** — 구현 gate는 image와 겹치지 않는 top border `#f0c674` surface sample을 별도로 검사한다.
18. **overflow item이 border 뒤에 잘려도 frame만으로 통과할 수 있음** — 이미지 소스의 해당 좌표가 `#00ff40`임을 확인하고 border-box edge 밖 PixelCopy/Metal sample에서 같은 색을 비교한다.
19. **한 image pixel을 PNG 전체 texel 정확도로 확대 해석할 수 있음** — 이 장면은 overflow visibility를 위한 고정 단일 point만 확인하고 full texture matrix는 제외한다고 적었다.
20. **좁은 fixture를 C11.4 완료로 오인할 수 있음** — 실패 범위, simulator 한계, WPT 미검증을 적고 checkbox와 `72/213`을 유지한다.

계획 검토에서 implementation 전에 보완해야 할 blocker는 발견하지 못했다. Browser 관측, 독립 계산, 구현 관문은 서로 다른 자료로 남긴다.
