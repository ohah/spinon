# C11.4 Tailnet 문서 동기화 PR 적대 검토 30개

**검토일:** 2026-10-19
**검토 대상:** 이 evidence 디렉터리의 문서·manifest·스크린샷·선택 렌더 로그 전체
**검토 목적:** 공개 저장소에 포함할 기록의 정확성, 재현 가능성, 범위 표기와 정보 노출 확인

## 독립 실패 관점

1. **PR의 목적이 상태 구현으로 오인되는가** — 제목·README·본문 초안에서 Tailnet 문서 sync로 범위를 한정한다. 앱 기능 PR로 설명하지 않는다. 통과.
2. **merge SHA가 잘못 연결됐는가** — 소스 revision을 #340 merge commit으로 고정하고 실제 branch HEAD와 대조한다. 통과.
3. **로드맵 61/72/완료 수 혼용** — 현재 화면의 표시 문자열 72/212만 인용한다. C11.4 추가로 수를 바꾸지 않는다. 통과.
4. **C11.4 전체 완료를 암시하는가** — 상태 대장과 README가 미완료를 유지하고 남은 계산 단계를 명시한다. 통과.
5. **미리보기 공개 사이트 배포와 혼동되는가** — Tailnet preview라고 부르고 GitHub Pages 배포를 주장하지 않는다. 통과.
6. **수동 Pages workflow를 실행한 것으로 읽히는가** — 이 기록은 별도 preview sync만 설명한다. Pages workflow 실행 주장이 없다. 통과.
7. **Rspress 버전 변경이 포함되는가** — 렌더 log는 2.0.22를 기록한다. package나 lockfile 변경은 없다. 통과.
8. **전체 build 성공을 허위 주장하는가** — selected render 성공만 적고 전체 문서 build와 링크 검사를 제외한다. 통과.
9. **선택 렌더에서 link check 미실행을 숨기는가** — README 한계에 전체 링크 검사가 포함되지 않았다고 썼다. 통과.
10. **15개 Markdown 경로와 출력 집합이 섞이는가** — 15는 입력 Markdown 경로 수이며 77은 전체 서비스 자산 수라고 구분했다. 통과.
11. **총량 계산 불일치** — source manifest와 copy manifest의 77개·53/19/24/34/0 계산을 대조했다. 통과.
12. **404 제외 여부 불투명** — active reader 사유와 별도 예외 경로를 기록하고 404 backup hash 동일성을 확인한다. 통과.
13. **백업이 preview 안쪽에 존재하는가** — 백업은 미리보기 외부라고 기록하며 전체 로컬 경로는 공개하지 않는다. 통과.
14. **백업 검증 수가 교체 수보다 적은가** — 백업 검증 항목 20개, mismatch 0으로 manifest를 대조했다. 통과.
15. **해시 mismatch인데 성공을 보고하는가** — copy·served verification의 mismatch가 0인 것을 확인했다. 통과.
16. **HTTP 상태만으로 내용을 증명하는가** — 200 결과와 함께 각 source/served SHA-256 equality를 기록했다. 통과.
17. **특정 HTML만 확인하고 전체 manifest를 claim하는가** — 전체 77개 파일 검증과 별개로 Agent Browser는 핵심 페이지 네 개를 확인했다고 구분했다. 통과.
18. **계획 route screenshot이 잘못된 콘텐츠인가** — 캡처에 C11.4 계획 제목·본문·미완료 하위 범위가 시각적으로 나타난다. 통과.
19. **PR review route가 깨진 링크인가** — 캡처에서 HTML review 페이지와 20개 검토 항목을 확인했다. 통과.
20. **스크린샷이 app UI로 오해되는가** — README는 캡처가 문서 페이지이며 모바일 앱 증거가 아니라고 말한다. 통과.
21. **이미지 링크 대상이 누락되는가** — README가 같은 evidence 디렉터리의 저장된 상대경로를 가리킨다. 파일 존재를 확인했다. 통과.
22. **불필요한 대용량 화면 중복** — PR에 넣을 viewport 증거는 로드맵 수와 상태 변화에 집중하며 장문의 전체 로드맵 중복 이미지는 제외한다. 통과.
23. **브라우저 종류가 잘못 표시되는가** — 증거는 Agent Browser라고 적고 Terminal Browser를 사용하지 않았음을 기록한다. 통과.
24. **브라우저 프로세스가 남아 있는가** — 종료 결과 JSON은 `active=false`, `pid=null`, idle 후 process count 0을 기록한다. 도구의 session list에 남은 값은 실행 프로세스가 아닌 비활성 이름이다. 통과.
25. **다른 브라우저 세션을 닫았는가** — 작업에서 연 이름이 붙은 세션 하나만 닫고 나머지 세션에는 손대지 않았다. 통과.
26. **미리보기 hostname이 공개 파일에 남는가** — 증거 파일에서 host를 가렸고 README는 상대 path를 쓴다. 통과.
27. **local absolute path나 계정 정보가 노출되는가** — build log의 worktree root를 `[worktree]`로 치환했고 추가 증거의 경로를 검색했다. 통과.
28. **실행하지 않은 앱 검증을 암시하는가** — simulator·physical device·앱 runtime 검증이 범위 밖이라고 여러 문서에서 분리했다. 통과.
29. **PR에 필요한 라벨과 본문 계약이 빠졌는가** — 저장소의 기존 `documentation` 유형 및 `area: docs` 영역 라벨을 확인했고 PR 본문은 양식 항목을 채운다. 통과.
30. **근거 검토 없이 무조건 merge를 전제하는가** — PR 생성 뒤 최신 checks, mergeability와 protection을 확인하고 merge 조건이 맞을 때만 저장소의 Rebase and merge 규칙을 적용한다. 통과.

## 검토 결과와 한계

이 diff에서 추가로 수정할 증거 불일치는 찾지 못했다. 종료 근거를 재확인해 inactive session handle과 실행 중인 브라우저 프로세스를 구분했다. 검토는 공개 문서 증거와 동기화 산출물에 한정된다. PR review 시점의 GitHub checks·branch protection·mergeability는 생성 후 최신 상태로 다시 확인해야 한다. 이 문서는 전체 문서 빌드, 앱 UI 또는 simulator 통과를 주장하지 않는다.
