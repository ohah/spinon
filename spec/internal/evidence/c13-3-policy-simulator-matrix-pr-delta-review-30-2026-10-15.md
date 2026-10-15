# C13.3 computed overscroll simulator matrix PR 변경 검토 30개

대상 PR: [#185](https://github.com/ohah/spinon/pull/185), 초기 검토 head `1f57290879f012b95d497560831047b87ae755d6`. 구현 검토 20개 및 계획 검토 20개와 별도의 변경 범위 검토다.

1. **PR 제목/범위** — computed overscroll simulator matrix만 제목과 상태 ID에 남기고 C13.3 전체 완료처럼 표시하지 않는다.
2. **공식 상태 대장** — `spec/STATUS.md` 결과와 남은 gate가 계획·증거와 일치하는지 확인한다.
3. **계획의 pass 조건** — 시나리오, 방향별 oracle, simulator 한계가 새 실행 결과와 모순되지 않는다.
4. **versioned contract** — 내부 계약 버전을 `0.1.0-draft`로 유지하고 구현 진척만으로 숫자를 올리지 않는다.
5. **scenario 0 기본 동작** — ID 생략과 명시적 0이 같은 zero-state를 만들고 첫 draw revision 0을 기록한다.
6. **scenario 1 대조군** — `auto/auto` 경계에서 잔여 축 입력이 outer로 전달된 두 플랫폼 결과를 연결한다.
7. **scenario 2 정책** — computed inner `contain/auto`와 expected offsets가 정적 설명·로그·pixel 결과에서 맞는다.
8. **인수 파싱의 적대 입력** — Android wrong type, iOS duplicate/invalid, fixture route 누락을 오류 경로로 검토한다.
9. **FFI/헤더 일치** — C declaration, Rust export, JNI/Objective-C call shape와 feature define이 같은 scenario ID ABI를 쓴다.
10. **default artifact 격리** — Android/iOS default static archive에서 새 scenario symbol·문자열이 빠진다.
11. **positive control** — 동일 대상의 opt-in archive에는 symbol과 세 scenario marker가 들어간다.
12. **기존 fixture route 회귀** — 기존 기본 C13.3 route와 zero-offset 장면의 인수 기본값 및 초기 상태가 바뀌지 않는다.
13. **computed style 경로** — scenario script가 선택값만 전달하고 scroll owner policy는 cascade 결과로부터 온다.
14. **scroll range 경계** — root/outer/inner range가 README·host report·pixel bounds에서 일치한다.
15. **hit-test target** — swipe point가 inner content target을 골라 owner chain inner→outer→root를 기록한다.
16. **X-only auto 경로** — inner X 최대에서 남은 X가 outer X로 소비되고 Y가 바뀌지 않는다.
17. **Y-only auto 경로** — inner Y 최대에서 Y만 outer로 소비되고 X가 바뀌지 않는다.
18. **diagonal auto 경로** — 두 축 remainder가 동일 방향으로 전달되며 root range까지 과장하지 않는다.
19. **X-only contain 경로** — outer/root X가 고정되고 boundary seed가 유지된다.
20. **Y-only contain/auto 경로** — Y chaining이 살아 있고 X policy가 Y를 막지 않는다.
21. **diagonal contain 경로** — X는 고정되고 Y만 움직이며 Android와 iOS가 축별로 일치한다.
22. **첫 draw 동기화** — `READY` 직전의 drawn revision/offset assertion이 두 adapter에 존재한다.
23. **presentation invalidation** — iOS 환경 갱신으로 sequence만 바뀌고 snapshot이 사라지는 회귀 수정이 기록·재검증됐다.
24. **resize 오류** — renderer resize 실패는 입력 준비 상태를 통과시키지 않고 원래 오류 status를 반환한다.
25. **bounded queue** — regular/terminal capacity와 terminal reservation이 adapter drain 변경에 의해 축소되지 않는다.
26. **실제 입력 대 shim 분리** — matrix의 Android/iOS swipe는 native callback이고 seed만 fixture router가 적용한 점을 분리해 기록한다.
27. **pixel 증거의 정확성** — threshold, top inset 제외, root 경계, no-diff contain-horizontal을 JSON·README에서 일치시킨다.
28. **이미지 경로** — PR 본문에 사용할 app screenshot 경로가 repo tree에 실제 존재하고 GitHub에서 PNG로 렌더되는지 확인한다.
29. **도구·테스트 진술** — workspace test, Clippy, feature test, app build와 simulator 실행을 실제 통과 범위 그대로 적는다.
30. **문서 한계와 후속 계획** — 실기기·hardware GPU·scanout, public events, tap/click, wheel, WPT 등 미확인 범위를 남기고 Tailnet preview는 merge 뒤 별도로 갱신한다.

## 결과

30개 관점에서 수정이 필요한 미해결 PR blocker를 찾지 못했다. GitHub PR body를 Agent Browser로 열어 Android 1080×2400 및 iOS 1206×2622 PNG가 실제 로드되는 것을 확인했다. 2026-10-15 조회 시 PR check는 게시되지 않았고 `main` required-status-check endpoint는 미설정(HTTP 404), branch rules는 빈 배열이었다. Rebase and merge 조건과 repository state를 확인했다. PR 링크·체크 구성 메모를 더한 후속 문서 변경도 현재 PR diff에서 다시 확인한다.
