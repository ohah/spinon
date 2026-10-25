# C11.4 행 축 로드맵 항목 구현 검토 20개

검토 대상은 공개 로드맵에 추가한 PR #620 결과 카드다. 구현 PR #620과 runtime evidence를 다시 검증하거나 C11.4 완료를 판정하는 문서가 아니라, 로드맵 설명이 현재 근거의 범위와 일치하는지 확인한다.

1. **병합 SHA와 구현 source SHA를 혼동할 수 있음** — 카드에서 `a985b753...`는 병합 SHA, `0af34026...`는 runtime 검증 source로 별도 표기했다.
2. **PR 상태가 열림으로 보일 수 있음** — PR #620 링크와 Rebase and merge 문구를 사용하고 실제 merge 시각은 `2026-10-25T02:29:00Z`로 고정했다.
3. **기존 #611과 새 행 축 slice를 같은 변경으로 오해할 수 있음** — 카드 제목과 내용은 intrinsic row-axis를 지정하고 #611은 선행 column fixture로만 부른다.
4. **anonymous raw text coverage로 오해할 수 있음** — 직접 Ahem text item과 natural-size PNG를 사용했다고 적어 raw text node 범위와 구별했다.
5. **이미지 크기를 화면 픽셀로 오해할 수 있음** — natural-size PNG라는 fixture 입력만 말하며 raster 결과를 주장하지 않는다.
6. **min/max-content 차이를 검증했다고 오해할 수 있음** — 두 키워드의 결과 차이를 구별하지 못한다고 제한을 명시했다.
7. **scene 수가 부풀려질 수 있음** — 두 row keyword와 두 source order의 조합으로 네 scene이라고 한정했다.
8. **forward/reversed가 layout 방향으로 오해될 수 있음** — source-order 변형으로 표현했다.
9. **Android 결과에 실기기 의미를 부여할 수 있음** — API 37 ARM64 16 KB AVD라고 명시했다.
10. **iOS 결과에 실기기 의미를 부여할 수 있음** — iPhone 17 Pro/iOS 26.2 Simulator라고 명시했다.
11. **Android 결과와 iOS 결과를 합쳐 한 번의 4/4로 셀 수 있음** — 플랫폼별로 각각 4/4라고 기록했다.
12. **기존 column regression을 새 row 4 scene에 포함할 수 있음** — column regression은 별도 재실행이며 플랫폼별 각각 4/4라고 구분했다.
13. **공유 matcher 회귀 확인을 전체 #611 적합성으로 확대할 수 있음** — 기존 #611 column regression 확인이라고 범위를 제한했다.
14. **캡처를 pixel equality로 오해할 수 있음** — box 배치 근거이며 pixel equality가 아니라고 적었다.
15. **Android emulator를 hardware GPU proof로 오해할 수 있음** — 실기기/hardware GPU proof가 아니라고 명시했다.
16. **iOS 캡처를 compositor 이후 proof로 오해할 수 있음** — compositor 이후 픽셀을 주장하지 않는다.
17. **한정 profile을 일반 CSS 지원으로 오해할 수 있음** — 일반 CSS admission을 검증하지 않았다고 명시했다.
18. **scene 성공을 WPT 통과로 오해할 수 있음** — WPT를 검증하지 않았다고 명시했다.
19. **후보 통과를 C11.4 완료로 오해할 수 있음** — 상위 C11.4는 미완료이며 checkbox는 유지한다고 적었다.
20. **로드맵 카드가 공식 집계를 임의 변경할 수 있음** — 공식 `72/213`을 유지한다고 명시했고 `spec/STATUS.md` 체크 상태를 수정하지 않았다.

확인 결과: 20개 관점에서 공개 로드맵 카드가 PR #620의 제한된 runtime 근거와 남은 범위를 넘겨 주장하지 않는다. 연결 경로와 렌더링 결과는 `selected-build-verification-pass04.json` 및 Agent Browser 사후 확인으로 검증한다.
