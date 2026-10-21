# C11.4 anonymous text + image 계획 실패 관점 검토 20개

이 검토는 구현 전 계획의 oracle, 범위, 오류 경계와 검증 방법만 독립적으로 반박한다. 구현 코드 또는 PR diff 검토로 대체하지 않는다.

1. **기준 버전이 잘못 고정될 수 있음** — UA와 binary version을 observation에 기록하고 Chrome for Testing 155.0.8059.85로 특정했다.
2. **폰트 미로드가 기대값을 오염시킬 수 있음** — `document.fonts.check` 결과와 Ahem asset SHA-256을 고정했다.
3. **PNG 디코드 지연이 폭을 바꿀 수 있음** — `decode()`를 기다리고 24×16 natural size와 입력 hash를 검사한다.
4. **DPR 변화가 screenshot 좌표를 흔들 수 있음** — DPR 1과 viewport를 기록하고 판정은 CSS px 기준으로 한다.
5. **Grid container의 자유 공간이 tracks를 바꿀 수 있음** — 네 개 min-content track, 220px definite width, zero gap을 고정했다.
6. **span-2 image의 크기 기여가 잘못 계산될 수 있음** — 48px 높이와 3:2 intrinsic ratio에서 72px를 독립 계산했다.
7. **Chrome 값을 규범 기대값으로 잘못 사용할 수 있음** — 기대값 산식을 observation 전에 정하고 CSS Grid §11.5.1을 근거로 분리했다.
8. **익명 text item을 DOM node 자체로 오인할 수 있음** — fixture는 box/tree item과 anchor/source-node provenance를 분리 기록하도록 요구한다.
9. **Range rect를 익명 item frame으로 오인할 수 있음** — 계획이 Range 결과는 text-fragment 관찰로 한정하고 item frame을 별도 trace에서 읽는다.
10. **HTML comment가 실제 HostDocument node인 것처럼 테스트할 수 있음** — Chrome은 literal comment, 모바일 host tree는 연속 Text node로 표현한다고 구분했다.
11. **whitespace-only text가 auto-placement cell을 먹을 수 있음** — image 앞 whitespace-only run의 item 부재와 뒤 `XX`의 다음 cell을 별도 case로 둔다.
12. **숨김 subtree가 text를 잘못 flush할 수 있음** — `display:none` span을 text와 image 사이에 넣어 continuity와 숨김 제외를 함께 검사한다.
13. **image의 CSS display가 inline이라고 collector가 거부할 수 있음** — Chrome computed display를 기록하고 Grid item blockification을 규범 근거로 삼는다.
14. **image가 text item 순서를 바꿀 수 있음** — image 전후 text, image 우선, text-group 우선 세 순서를 둔다.
15. **text grouping이 단일 text node만 다룰 수 있음** — comment 전후 두 text node를 한 item으로 묶는 장면을 추가했다.
16. **tracks가 같아도 잘못된 item node/frame일 수 있음** — provenance, source order, root/item frame, trace completeness를 모두 검사한다.
17. **image intrinsic size와 text가 서로 다른 revision일 수 있음** — acceptance criteria에 같은 layout revision의 두 source와 revision metadata를 요구한다.
18. **fixture가 기본 제품에 새 동작을 누설할 수 있음** — 기존 opt-in fixture feature와 기본/opt-in artifact 대조를 검증 관문에 포함했다.
19. **Android 실기기와 시뮬레이터 근거를 혼동할 수 있음** — 사용자가 택한 Android AVD와 iOS Simulator만 계획 범위로 고정하고 물리 Android 주장을 금지했다.
20. **작은 성공이 전체 Grid 호환으로 과장될 수 있음** — 범위를 고정 다섯 scene으로 제한하고 C11.4 checkbox·완료 수를 올리지 않는다.

검토 결과 계획을 수정하지 않아도 되는 누락은 발견하지 않았다. literal comment의 Android/iOS host-tree 경계는 계획에 명시해 범위를 제한했다.
