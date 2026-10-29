# C11.3.3 PR #807 병합 후 상태·공개 로드맵 변경 적대 검토

## 검토 범위

`origin/main`의 PR #807 merge commit `ee824922ecff07f2dc0b79ba02cf865a51d45bb4`를 기준으로, 이 후속 변경의 `spec/STATUS.md`, `spec/0001-conformance.md`, 내부 계약 0059, C11.3.3 계획, 공개 `spec/public/roadmap.html`, 이 디렉터리의 캡처·기록을 서로 다른 실패 관점에서 확인했다. 원격 PR #807 조회는 `MERGED`, `mergedAt=2026-10-29T13:54:24Z`, `statusCheckRollup=[]`였다. 이 문서가 갱신하는 Tailnet 페이지의 배포 결과는 아직 증명하지 않으며, 동기화 뒤 별도 검증한다.

## 30개 독립 실패 관점

1. **병합 여부** — GitHub PR #807은 `MERGED`; 열린 상태로 문서화하지 않는다. **PASS**
2. **병합 시각** — GitHub `mergedAt`과 문서의 `2026-10-29T13:54:24Z`가 같다. **PASS**
3. **merge SHA** — GitHub mergeCommit, `origin/main`, 문서의 `ee824922ecff07f2dc0b79ba02cf865a51d45bb4`가 같다. **PASS**
4. **원격 기준점** — `git fetch origin main` 뒤 `HEAD`와 `origin/main`이 해당 SHA다. **PASS**
5. **CI 표현** — PR check rollup은 비어 있다. 문서는 CI 통과를 주장하지 않는다. **PASS**
6. **공식 상위 상태** — `spec/STATUS.md`의 C11 checkbox는 unchecked 상태다. **PASS**
7. **C11.3.3 상태** — template areas candidate checkbox는 unchecked 상태다. **PASS**
8. **공식 완료 집계** — 관련 변경은 `72/213`을 유지하고 숫자를 상향하지 않는다. **PASS**
9. **공개 로드맵 집계** — 새 로드맵 카드에도 `72/213` 유지와 미완료 문구가 보인다. **PASS**
10. **중복 카드** — 수정한 공개 로드맵의 Agent Browser DOM에서 PR #807 카드는 한 번만 보인다. **PASS**
11. **카드 시간 순서** — 13:54 UTC의 PR #807 카드가 같은 날 11:08 UTC의 PR #804 카드 앞에 있다. **PASS**
12. **구현 범위** — 카드는 부족한 area-dimension track과 authored track 보존만 요약하며 일반 Grid 구현으로 확장하지 않는다. **PASS**
13. **track sizing 한계** — 단일 `grid-auto-rows`/`grid-auto-columns`의 `auto` 또는 px 값으로 범위를 한정한다. **PASS**
14. **Chrome matrix 산술** — Chrome 154 26 scene·65 node와 Chrome 156 보충 5 scene·25 node를 합쳐 31 scene·90 node다. **PASS**
15. **matrix 분류** — 25 success, 5 cascade fallback, 1 expected rejection의 합은 31이다. **PASS**
16. **Android 환경 경계** — Android 증거는 API 37 ARM64 AVD와 SwiftShader로 구체화하며 실기기라고 부르지 않는다. **PASS**
17. **Android scene 수** — AVD runtime README가 31 scene/90 node와 31/31 통과를 뒷받침한다. **PASS**
18. **geometry 비교 범위** — 양 플랫폼의 비교 대상은 visible 88 frame이고 최대 오차는 0 CSS px다. **PASS**
19. **PixelCopy 기본 표본** — Android 기본 장면 표본 2/2 exact를 분리해 적는다. **PASS**
20. **Metal 기본 표본** — iOS 기본 장면 표본 2/2 exact를 분리해 적는다. **PASS**
21. **overflow 대조** — 별도 unknown-area overflow 표본 3/3 exact를 기본 2/2와 혼동하지 않는다. **PASS**
22. **leading-line 대조** — leading named-line 7 scene/14 node와 표본 7/7 exact를 별도 근거로 표기한다. **PASS**
23. **iOS 환경 경계** — iPhone 17 Pro/iOS 26.2 Simulator 실행으로 구체화한다. **PASS**
24. **compositor 경계** — Metal readback이 compositor 제출 전임을 표시하고 post-compositor pixel로 확장하지 않는다. **PASS**
25. **실기기 주장** — Android 실기기 검증을 완료했다고 쓰지 않는다. **PASS**
26. **GPU 주장** — SwiftShader 결과를 hardware GPU 결과라고 쓰지 않는다. **PASS**
27. **적합성 주장** — 이 fixture 실행을 Spinon WPT 적합성 또는 전체 CSS Grid 지원으로 올리지 않는다. **PASS**
28. **PR 링크 대상** — 공개 카드의 GitHub URL은 #807을 가리키고 계획·실행 링크의 source 파일들이 존재한다. **PASS**
29. **기존 내용 보존** — `spec/public/roadmap.html` diff는 새 카드 삽입 한 건이며 #804·#790의 기존 근거는 그대로다. **PASS**
30. **화면·프로세스 경계** — Agent Browser가 로컬 정적 roadmap source를 열어 PR #807·merge SHA·72/213을 확인하고 1280×633 캡처를 저장했다. 브라우저 종료 뒤 agent-browser/HeadlessChrome 프로세스가 남지 않았으며, Tailnet 응답 검증은 완료한 것처럼 기록하지 않는다. **PASS**

## 판정

30개 관점은 로컬 PR diff와 근거 경계를 통과했다. Tailnet 배포와 served hash 검증은 병합 뒤 별도로 수행한다. 이 검토는 공식 완료 상태를 변경하지 않는다.
