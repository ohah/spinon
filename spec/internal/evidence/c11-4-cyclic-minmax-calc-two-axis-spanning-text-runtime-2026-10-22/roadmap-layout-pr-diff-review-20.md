# PR #508 공개 로드맵 배치 변경 적대 검토 20개

대상: PR #506 진행 행에 기존 문서 CSS의 hero-progress 배치를 적용하는 변경이다. 결과 행을 Agent Browser로 확인하고 각 항목을 별도 계약·표시 경계로 검토했다.

1. **기존 전체 폭 규칙 누락:** spec/public/roadmap.html의 .hero > .hero-progress 규칙이 grid-column 1 / -1로 span함을 확인했다.
2. **잘못된 행 수정:** C11.4 PR #506 행에만 hero-progress 클래스를 붙인다.
3. **기존 note 스타일 손실:** kanban-note 클래스를 함께 유지한다.
4. **사이드바 침범:** hero-note 요소와 배치는 변경하지 않는다.
5. **잘못된 grid 대상:** 대상 행은 .hero의 direct child라 기존 선택자와 일치한다.
6. **데스크톱 좁은 칼럼:** Agent Browser 1280px 화면에서 PR #506 행이 본문 전체 폭을 사용함을 확인한다([화면](./roadmap-layout-agent-browser-pr508.png)).
7. **900px breakpoint:** viewport·document 폭 900px, hero·행 폭 860px이며 단일 열 grid에서 가로 overflow가 없다.
8. **600px breakpoint:** viewport·document 폭 600px, hero·행 폭 572px이고 row screenshot에서도 shell padding 안에 표시된다([화면](./roadmap-layout-agent-browser-pr508-600-row-final.png)).
9. **긴 링크 줄바꿈:** 기존 kanban-note 글꼴·line-height·anchor 스타일을 유지한다.
10. **본문 순서:** 새 행은 PR #503 다음이자 검증 관문 도식 앞에 남긴다.
11. **태그 구조 손상:** p 시작·종료 태그와 섹션 경계가 각각 하나씩 유지된다.
12. **로드맵 링크 단절:** 계획, 실행 근거, 세 검토 문서, AVD/iOS PNG, PR #506, 공식 상태 경로를 확인한다.
13. **리뷰 route 누락:** 새 레이아웃 PR diff 검토 markdown의 Rspress HTML route 생성을 확인한다.
14. **PR #506 상태 오류:** 행에는 이미 병합된 PR #506과 실제 merge SHA를 유지한다.
15. **공식 수 변경:** C11.4 체크박스를 미완료로 두고 77/213을 유지한다.
16. **플랫폼 범위 확대:** Android AVD와 iOS Simulator 결과를 실기기 결과로 확장하지 않는다.
17. **픽셀 검증 과장:** Metal 표본의 compositor 이전 한계와 PixelCopy/Metal의 24/24 범위를 유지한다.
18. **glyph·border 과장:** box geometry 근거를 glyph raster 또는 CSS border stroke 결과로 표현하지 않는다.
19. **스크린샷 출처:** 두 Agent Browser PNG는 격리한 선택형 Rspress preview를 캡처하며 Tailnet 배포 자체의 증거와 구분한다.
20. **변경 범위 확대:** source change는 한 행의 배치 class와 검토 링크만 추가하며 제품 코드나 runtime contract는 바꾸지 않는다.

근거는 [PR #507 변경 검토](./roadmap-pr-diff-review-20.md), [PR #506 runtime README](./README.md), [공개 로드맵 CSS](../../../../spec/public/roadmap.html)와 Agent Browser screenshot이다.
