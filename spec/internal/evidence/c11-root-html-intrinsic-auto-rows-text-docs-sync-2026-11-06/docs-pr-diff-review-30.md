# C11.4 direct text min-content row: 문서 동기화 PR diff 검토

- 기준 변경: PR #1190 병합 SHA `950c858182810e27c39048c5bb252a4aed4810af`
- 검토 파일: `spec/STATUS.md`, 내부 계약 `0059`, C11.4 계획, 구현 전 비교 README, `spec/public/roadmap.html`
- 판정 범위: 문서 간 사실 일치, 링크·상태 표현, 공개/Tailnet 미리보기 경계

## 실패 관점

1. **구현 PR SHA 오기** — 네 문서가 PR #1190의 Rebase 병합 SHA `950c8581…`에 연결된다. **PASS**
2. **병합 날짜 오기** — PR의 실제 `mergedAt` `2026-11-05T19:13:40Z`를 사용한다. **PASS**
3. **C11 parent checkbox 조기 완료** — `spec/STATUS.md`의 C11은 미완료 상태를 유지한다. **PASS**
4. **C11.4 조기 완료** — C11.4는 미완료로 남고 문구도 이를 분명히 한다. **PASS**
5. **공식 완료 수 증가 오인** — 현재 대장 집계 `72/213`을 유지한다고 쓴다. **PASS**
6. **계획 수치와 증거 수치 불일치** — 두 줄 text, 20px contribution, `[10,20,34]` rows, y=64px overflow가 Chrome/Rust evidence와 맞는다. **PASS**
7. **기존 empty-item 기능 소실 표현** — 계약은 기존 empty direct HTML `div` 허용을 보존한다고 기록한다. **PASS**
8. **지원 대상의 과도한 확장** — text child와 exact ASCII source 한 개만 추가한 제한임을 각 문서가 유지한다. **PASS**
9. **font 지표의 플랫폼 상수화** — Android 4+2, iOS 3+3 glyph break를 따로 기록하고 두 줄/20px contribution만 공통 관찰값으로 둔다. **PASS**
10. **CSS px와 native pixel 혼동** — row와 frame을 CSS px 단위로 표시한다. **PASS**
11. **simulator와 실기기 혼동** — Android는 API 37 AVD, iOS는 Simulator라고 구체적으로 한정한다. **PASS**
12. **surface readback 과장** — screenshot은 visual smoke로만 기록하고 PixelCopy/Metal drawable readback이 없다고 밝힌다. **PASS**
13. **hardware GPU 과장** — Android 실기기/hardware GPU와 iOS compositor 이후 픽셀 결과를 주장하지 않는다. **PASS**
14. **WPT 적합성 과장** — WPT 확인이 없음을 명시한다. **PASS**
15. **Rust 결과 누락/혼합** — RuntimeSession 5/5, style/layout 1/1, workspace Clippy를 분리해 기록한다. **PASS**
16. **CI 결과 오인** — 빈 check rollup과 main required checks 부재를 CI 통과로 해석하지 않는다. **PASS**
17. **내부 계약 버전 상승** — 내부 contract `0.1.0`을 유지한다. **PASS**
18. **기준일 낡음** — 공식 상태 대장 기준일을 작업 날짜 `2026-11-06`으로 갱신한다. **PASS**
19. **계획 review 링크 누락** — 구현 PR diff review와 기존 계획/runtime review 기록을 연결한다. **PASS**
20. **문서 status 링크 잘못된 루트** — 계획처럼 Rspress에 포함되지 않는 `plan/` 파일은 GitHub blob URL로 연결한다. **PASS**
21. **C11 evidence 링크 누락** — 상태 대장과 계약은 Chrome comparison 및 Android/iOS runtime evidence를 연결한다. **PASS**
22. **Markdown 파일을 Rspress route로 오인** — 내부 계약의 evidence 링크 대상은 실제 `spec/internal/evidence` Markdown 경로다. **PASS**
23. **precomparison JSON 링크가 HTML route로 변환** — JSON/PNG/fixture 및 계획 링크를 GitHub blob 또는 raw URL로 바꿔 해당 C11 README의 Rspress dead links를 제거한다. **PASS**
24. **다른 C11.4 기록의 역사 변조** — 과거 slice의 날짜별 관측과 광범위 회귀 기록은 그대로 둔다. 현재 #1187 카드의 stale current-count 문구만 `72/213`으로 고친다. **PASS**
25. **로드맵 중복/잘못된 위치** — 새 #1190 card를 같은 C11.4 recent entries 안에서 #1187 뒤에 한 번만 넣는다. **PASS**
26. **공개 로드맵 수치가 상태 원본을 대신함** — page는 `spec/STATUS.md` read-only aggregation 설명을 유지하고 최신 card만 현재 집계를 표시한다. **PASS**
27. **Tailnet 배포를 이미 완료했다고 표시** — source `spec/public/roadmap.html` 수정과 실제 Tailnet sync를 구분한다. 배포는 docs PR 병합 후 확인한다. **PASS**
28. **미리보기 문서 링크 404** — 새 roadmap card의 plan, contract, evidence, status 링크는 GitHub `main`을 사용한다. **PASS**
29. **screenshot 링크가 변동 branch를 가리킴** — Android/iOS 이미지 링크는 실제 code/evidence가 포함된 merge SHA에 고정한다. **PASS**
30. **문서 전체 빌드 결과를 green으로 보고** — `docs:build`는 다른 기존 STATUS/internal evidence의 Rspress dead links로 실패한다. 이번 변경은 새 C11 plan/evidence 링크를 GitHub URL로 정정했지만 전체 빌드 통과를 주장하지 않는다. **PASS**

## 병합 뒤 확인

`spec/public/roadmap.html`을 Tailnet의 기존 `/spinon/roadmap.html`에 게시한 뒤 Agent Browser에서 desktop/mobile 카드, 이미지 링크, 상태 counter 및 overflow를 확인하고 브라우저를 종료한다. 확인 전에는 Tailnet 배포 완료로 표시하지 않는다.
