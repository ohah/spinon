# 계획 PR diff 적대 검토 30개

**대상:** 계획 및 사전 비교 PR의 실제 변경 파일. 구현 코드, 시뮬레이터 결과, WPT 통과 여부는 이 검토의 대상이 아니다.
**기준:** PR diff와 파일 참조를 항목별로 다시 열어 확인했다. 아래 점검은 계획 단계 PR만 다룬다.

1. **변경 경계:** dirty 원본 checkout과 무관한 새 plan worktree에서 변경했는가? 확인: branch는 `origin/main` 1c6b46f4에서 분기했고 plan/evidence/fixture만 바뀐다.
2. **공식 작업 연결:** 새 제목이 임의의 상위 roadmap item을 만들지 않는가? 확인: C11.4 explicit-neighbor 계획의 명시된 min-only + two-track gap을 연결한다.
3. **계획과 지원 완료 구분:** 계획 작성 자체가 구현 지원을 뜻하지 않도록 했는가? 확인: 문서 첫 부분에 app/simulator/WPT 실행 전이며 완료 수를 바꾸지 않는다고 적었다.
4. **실행 수치의 출처:** 표의 used tracks와 root가 구현 결과에서 역산되지 않았는가? 확인: Chrome observation JSON과 CSS 식을 함께 참조한다.
5. **근거 파일 누락:** fixture 원본이 PR에 실제 포함되는가? 확인: `tests/fixtures/css/c11/` 아래 정적 HTML로 추가한다.
6. **fixture 실행 의존성:** 사전 비교 fixture가 앱 번들, 네트워크 또는 생성된 프로젝트 artifact에 의존하지 않는가? 확인: 표준 HTML/CSS/inline JS만 사용한다.
7. **oracle 자체 증명 과장:** Chrome screenshot이 Spinon layout이나 simulator pixels라고 오인될 수 있는가? 확인: screenshot은 Chrome DOM oracle로 명시하고 구현 근거가 아님을 적었다.
8. **원시값 보존:** serialized track list 외에 browser/viewport 조건이 남아 있는가? 확인: JSON에 UA, viewport, DPR, gaps, min size, root, item을 보존한다.
9. **두 파일의 hash 불일치:** fixture hash가 evidence README·SHA256SUMS와 같은가? 확인: diff 확정 후 세 파일 모두 재계산해 일치시킨다.
10. **이미지 누락:** 링크된 PNG가 실제 변경에 존재하고 올바른 format인가? 확인: 1600×1500 PNG이며 SHA256SUMS 대상이다.
11. **Chromium source link:** Chromium 코드 citation이 auto-repeat 함수의 exact source 위치를 가리키는가? 확인: `ng_grid_layout_algorithm.cc`의 `ComputeAutomaticRepetitions` anchor를 사용한다.
12. **Normative link:** CSS Grid 링크가 두 규칙(반복 수와 auto-fit collapse)을 담는가? 확인: fixed 2025 CRD §7.2.3.2로 고정한다.
13. **고정버전과 UA 구별:** UA의 HeadlessChrome major version을 설치된 full Chrome patch version으로 잘못 부르지 않는가? 확인: Agent Browser doctor의 installed Chrome 155.0.8059.85와 UA 155.0.0.0을 구분해 기록한다.
14. **Chrome과 규범 충돌:** Chrome auto-fit 축 차이를 기대값으로 조용히 승격하지 않는가? 확인: README에서 unresolved interop으로 분리하고 계획 acceptance에서 제외한다.
15. **진단행렬 누락:** auto-fit empty와 span two가 결과에 있는가? 확인: diagnostic HTML 4 case와 JSON에 모두 남아 있다.
16. **auto-fill acceptance 누락:** 한쪽 neighbor, 양쪽 neighbor, 두 축, 반복 경계를 실제 app acceptance에 포함하는가? 계획의 구현 관문에 각각 적었다.
17. **Fixed control 의미:** fixed `40px 20px` control이 flex-min fallback을 증명한다고 주장하는가? 확인: control과 `minmax(...,1fr)` 실제 pattern을 분리한다.
18. **CSS height/width cross-talk:** axis fixture가 다른 축 크기로 overflow를 만들거나 percentage cycle을 도입하는가? 확인: orthogonal track은 fixed 16px이며 percent/calc은 사용하지 않는다.
19. **Page-level content 오염:** fixture label이나 diagnostic strips가 대상 grid item count에 들어가는가? 확인: label/strip은 figure sibling이고 grid 안에는 span-scene의 명시 item 외 콘텐츠가 없다.
20. **Item 좌표 frame:** span diagnostic의 item x/y가 viewport absolute 좌표로 혼동되는가? 확인: JSON은 grid rect를 뺀 local 좌표를 저장한다.
21. **Screenshot viewport 일치:** screenshot dimensions와 observation viewport가 다른가? 확인: browser resize 후 reload하고 둘 다 1600×1500으로 캡처한다.
22. **Run 과정 cleanup:** PR diff에 Agent Browser session이나 process를 남기는가? 확인: browser 세션을 닫고 session list와 pgrep으로 확인한 뒤 문서만 커밋한다.
23. **개인정보/로컬경로:** artifact에 user home, cookie, machine identifier가 들어가는가? 확인: fixture/UA/geometry만 기록하며 local file URL은 저장 관찰에 넣지 않는다.
24. **시뮬레이터 claim:** 계획 문서가 AVD/iOS Simulator 미실행을 누락하는가? 확인: 미실행 명시, acceptance는 구현 PR 이후다.
25. **WPT claim:** WPT file을 언급하는데 실행했다는 인상을 주는가? 확인: 이번 precomparison에서 WPT를 실행하지 않았다고 명시한다.
26. **지원 범위 확장:** `fixed px` neighbor 제한이 CSS grammar 전체 지원으로 표현되는가? 확인: line names, percentages, dual-axis, integer repeat 등을 남은 범위로 열거한다.
27. **API 명세 필요성:** 계획용 fixture가 public API contract를 추가했다고 오인되는가? 확인: 공개 API를 새로 만들지 않고 C11.4 내부 runtime test/evidence 경계로 설명한다.
28. **Status와 API checklist:** 계획 PR에서 공식 `spec/STATUS.md` 완료 상태를 임의 변경하는가? 확인: status와 API count 변경이 없다.
29. **Tailnet/Pages side effect:** 계획 PR이 별도 문서 배포나 Pages workflow 실행을 요구하는가? 확인: PR은 source change만 포함하며 Pages를 실행하지 않는다.
30. **PR merge 검증:** 이 기록만으로 checks/approval/merge를 추정하는가? 확인: PR 생성 후 live required checks, branch protection, mergeStateStatus를 다시 조회하고 Rebase and merge 결과를 확인한다.

이 문서는 계획 PR diff 검토 기록이다. 계획 20개 검토, 구현 review 20개, 구현 PR의 독립 diff review 30개와 서로 대체되지 않는다.
