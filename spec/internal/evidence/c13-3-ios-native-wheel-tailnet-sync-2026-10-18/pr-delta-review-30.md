# C13.3 iPadOS preflight Tailnet 동기화 · PR 변경 관점 검토 30개

1. **앱 ready 상태를 wheel 성공으로 오해하게 만드는가?** 계획 페이지는 native scroll 미전달, UIKit callback 미관찰, C13.3 미완료를 함께 표시한다.
2. **잠금 화면을 피해 임의 native 입력을 주입했는가?** Simulator 전경이 `loginwindow`에 가려져 입력을 보내지 않았고, 사전 점검 문서에 그 이유를 기록했다.
3. **Simulator 결과를 실기기 동작으로 확대했는가?** 장치·OS를 iPadOS 26.2 Simulator로 한정하고 실제 iPad/외부 장치 근거는 없다고 명시한다.
4. **UIKit source/translation/sign 관찰을 했다고 주장하는가?** callback 이벤트를 받지 않았으므로 해당 값과 iOS adapter는 검증되지 않은 상태다.
5. **C13.3 또는 iOS adapter 상태를 완료로 표시했는가?** 계획, 증거, 내부 인덱스 모두 부분 진행과 미통과 gate를 유지한다.
6. **공식 완료 수를 잘못 올렸는가?** `spec/STATUS.md`를 수정하지 않았고 공식 완료 수는 전과 같다.
7. **공식 roadmap과 Tailnet preview의 역할이 섞였는가?** Tailnet 미리보기의 문서 index만 갱신했으며 공식 상태 대장과 공개 roadmap 완료 항목은 바꾸지 않았다.
8. **내부 인덱스에서 새 증거를 찾을 수 있는가?** `spec/internal/README.md`의 C13.3 항목이 Tailnet 동기화 보고서를 연결한다.
9. **Markdown 원문과 생성 HTML을 같은 route로 혼동하는가?** 원문 경로와 Rspress 렌더 경로를 각각 복사·검증한다.
10. **계획의 `/spec/internal/...` raw preflight 링크가 깨지는가?** 8개 원본을 preview의 `spec/internal/...` alias에도 복사하고 hash를 기록한다.
11. **raw README가 열려도 렌더 페이지가 누락될 수 있는가?** rendered preflight HTML route도 Agent Browser로 별도 열어 화면을 확인했다.
12. **보고서에 포함한 이미지가 서버에서 빠졌는가?** 로고와 세 embedded screenshot이 complete이며 자연 크기 150×150, 1280×633, 1280×2358, 1280×633으로 확인됐다.
13. **계획 캡처를 iOS 동작 화면처럼 제시하는가?** 캡처는 Tailnet 계획/preflight 설명 화면이며 PR 설명에서 native wheel 실행 화면이 아니라고 구분한다.
14. **검색창의 동적 index 요청을 빠뜨렸는가?** 첫 Agent Browser 검사에서 `.7ebea52c.json` 404를 찾아 build artifact를 복사하고 이후 200을 확인했다.
15. **최종 번들이 다른 index hash를 참조하는가?** 최종 bundle이 참조하는 locale search index가 복사됐고 Agent Browser network log에서 200이다.
16. **이전 404와 복구 성공을 숨겼는가?** 최초 누락, 복구 검증, 최종 bundle의 요청을 서로 다른 원본 로그와 JSON으로 보존한다.
17. **동일 경로의 기존 파일을 무검사 덮어쓰는가?** final preflight에서 기존 대상은 백업 후 교체하거나 동일 hash라 skip하고 backup hash를 검증한다.
18. **동기화 범위 밖의 파일까지 복사하는가?** 최종 payload는 preflight manifest로 제한하고 추가 browser evidence만 명시 경로에 복사했다.
19. **preview 파일을 정리한다는 이유로 삭제했는가?** 모든 pass에서 삭제 수는 0이다.
20. **공유 preview listener를 재시작했는가?** 기존 preview listener와 port 8768을 유지한 채 문서 파일만 갱신했다.
21. **GitHub Pages를 배포했는가?** 수동 배포 workflow를 실행하지 않았으며 Tailnet preview 경로만 갱신했다.
22. **선택 build를 full docs 성공이라고 썼는가?** 29 HTML route selected build와 실패한 전체 docs build를 보고서에서 구분한다.
23. **기존 broad C13 selected build 오류를 감췄는가?** 기존 style-invalidation 증거의 누락 이미지로 실패한 로그와 원인을 보존했다.
24. **전체 docs build의 기존 오류가 새 변경에서 비롯된 것으로 오해되는가?** full build dead-link/MDX 오류 로그와 제한 설명을 별도 기록한다.
25. **서빙된 Tailnet 파일이 local preview와 내용이 다른가?** 최종 80 artifact의 local/Tailnet 160 response 모두 HTTP 200, byte count, SHA-256을 비교했다.
26. **최종 copy 보고서가 그 자체를 검증했다고 과장하는가?** manifest와 served verification의 self-reference 예외를 명시하고 두 control file의 URL/hash는 별도 요청으로 확인했다.
27. **공개 PR이 Tailnet 화면 증거와 연결되는가?** Agent Browser PNG를 저장소 evidence에 포함해 PR에서 immutable commit 경로로 참조할 수 있다.
28. **터미널 브라우저를 사용했거나 빌드 설정을 남겼는가?** 시각 검증은 Agent Browser로 했고 임시 Rspress 설정은 PR에 넣지 않는다.
29. **브라우저 종료가 기존 terminal-browser에 영향을 줬는가?** 종료 후 이번 Agent Browser session/Chrome은 없고 기존 terminal-browser는 건드리지 않았다.
30. **공개 근거가 개인 식별 정보를 노출하는가?** Tailnet DNS, 계정/checkout 경로, process ID는 공개본에서 가리고 HTTP status·bytes·SHA-256 검증값은 보존한다. 임시 config는 제거하고 dirty checkout과 분리된 worktree를 사용한다.

## 판정

문서와 미리보기 동기화는 검증했지만 UIKit native callback 입력은 관찰하지 않았다. 따라서 iOS adapter와 C13.3 완료 gate는 미통과이며 `spec/STATUS.md`와 공식 완료 수는 변경하지 않는다.
