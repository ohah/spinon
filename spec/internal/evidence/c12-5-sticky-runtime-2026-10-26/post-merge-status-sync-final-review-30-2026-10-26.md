# C12.5 병합 후 STATUS 동기화 최종 PR diff 검토 · 30관점 · 2026-10-26

## 범위와 fingerprint

검토 대상은 docs/c12-5-post-merge-status-sync-20261026 worktree에서 origin/main / HEAD 761ed5e085c9aaea4da8831d62044fe04bfd6d66 위에 남은 두 tracked 경로만이다: spec/STATUS.md, spec/internal/evidence/c12-5-sticky-runtime-2026-10-26/README.md. HEAD와 origin/main이 같으므로 committed branch range는 0개 경로이며 binary diff SHA-256은 빈 입력 hash e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855다. 실제 검토 후보는 두 경로의 unstaged worktree diff다.

- 2 paths; STATUS 2 additions / 2 deletions, evidence README 1 addition / 1 deletion.
- git diff --binary -- spec/STATUS.md spec/internal/evidence/c12-5-sticky-runtime-2026-10-26/README.md SHA-256 before: 86cdf91f02bc205537c01d874db9a3eb8287f942d82aba10e9d9568a38af99ae
- 대상 범위에서 product/runtime source, fixture, DerivedData는 제외했다. 변경된 두 문서 외 tracked diff가 없는지 시작·종료에 확인했다.
- GitHub PR #639 metadata: merged=true, merged_at 2026-10-25T21:02:24Z, merge_commit_sha 761ed5e085c9aaea4da8831d62044fe04bfd6d66; PR base d7034a8c8fabd0ab836fb5f747a72d0680fc2cc8, source head d5d0027645b98d2fcb67310bfa167b6da336fb73.
- PR checks 조회는 no checks reported, merge SHA combined status는 statuses 빈 배열, main required-status-checks endpoint는 HTTP 404 Required status checks not enabled였다.
- 읽기 전용 문서 검토다. 빌드, 테스트, 시뮬레이터, 브라우저는 실행하지 않았다. 새 보고서만 evidence 디렉터리에 작성한다.

## 독립 실패 관점 30개

1. **문서 외 구현/fixture diff가 섞이는가?** spec/STATUS.md와 runtime README 두 경로만 변경됐다. product/runtime 변경을 이 문서 리뷰에 끼워넣지 않았다.
2. **DerivedData 또는 생성 산출물이 검토 범위를 오염시키는가?** 두 경로 명시 diff와 tracked name-only 결과 — DerivedData와 untracked 산출물은 후보 fingerprint에 포함되지 않는다.
3. **상단 기준 날짜가 오래됐거나 미래 날짜인가?** spec/STATUS.md:3 — 2026-10-26은 현재 문서 기준일과 일치한다.
4. **기준일 수정이 출시 상태를 바꾸는가?** spec/STATUS.md:3 — 출시 상태는 미출시로 유지한다.
5. **기준일 수정이 내부 계약 버전을 바꾸는가?** spec/STATUS.md:3 — 0.1.0 고정이 유지된다.
6. **현재 제품 지원 완료를 과장하는가?** spec/STATUS.md:3 — 제품 지원 완료 없음이 유지되어 C12.5 제한 fixture 병합을 제품 지원으로 오인시키지 않는다.
7. **잘못된 PR 번호/저장소 URL을 연결하는가?** spec/STATUS.md:377 — ohah/spinon PR #639 링크는 원격 metadata의 같은 PR을 가리킨다.
8. **단순 closed PR을 merged로 오기하는가?** spec/STATUS.md:377 — metadata는 state=closed 및 merged=true이므로 “병합됐다”가 사실이다.
9. **merge timestamp가 실제 merged_at과 다른가?** spec/STATUS.md:377 — 2026-10-25 21:02:24 UTC가 원격 2026-10-25T21:02:24Z와 같다.
10. **UTC/KST 변환이 날짜나 시간을 하루/9시간 틀리게 하는가?** spec/STATUS.md:377 — UTC를 명시하고 API의 Z 시각을 그대로 기록한다.
11. **merge SHA가 누락·축약·오타인가?** spec/STATUS.md:377 — 40자리 SHA가 PR merge_commit_sha와 일치한다.
12. **source head를 merge SHA로 혼동하는가?** spec/STATUS.md:377 — 문서 SHA 761ed5e…는 merge commit이며 PR head d5d0027…와 분리돼 있다.
13. **Rebase and merge의 base나 결과를 잘못 설명하는가?** spec/STATUS.md:377; PR metadata — base SHA d7034a8…와 merge SHA가 맞고 결과는 단일 부모 main commit이다.
14. **빈 check rollup을 nonempty로 기록하는가?** spec/STATUS.md:377; evidence README.md:3 — 둘 다 rollup이 비었다고 표현하며 PR checks와 combined status 결과가 빈 값이다.
15. **main에 required status checks가 설정됐는데 미설정이라고 쓰는가?** spec/STATUS.md:377; evidence README.md:3 — 두 문서 모두 main 한정으로 서술한다. 조회 응답은 Required status checks not enabled다.
16. **빈 checks를 CI pass로 오해하게 되는가?** spec/STATUS.md:377; evidence README.md:3 — 두 문서가 CI 통과가 아니라고 명시해 빈 결과를 성공으로 확대하지 않는다.
17. **지난 리뷰에서 발견한 stale pending 문구가 남아 있는가?** evidence README.md:3 — “PR creation pending” 문구가 제거됐고 PR diff review 완료 및 PR #639 병합 시각/SHA로 바뀌었다. 이전 검토 blocker가 해결됐다.
18. **README 수정이 C12.5 미완료 상태와 모순되는가?** evidence README.md:3 — C12.5 remains incomplete를 유지하면서 PR review와 구현 PR 병합을 구분한다.
19. **C12.5 구현 병합만으로 하위 체크가 완료되는가?** spec/STATUS.md:376-377 — C12.5 checkbox는 미완료 상태이며 변경 이력도 그 checkbox를 바꾸지 않는다.
20. **C12 상위 항목을 잘못 완료 처리하는가?** spec/STATUS.md:371 — C12는 미완료 상태다.
21. **C13 선행 상위 항목을 잘못 완료 처리하는가?** spec/STATUS.md:378 — C13은 미완료 상태다.
22. **C12.4 등 남은 stacking 범위가 감춰지는가?** spec/STATUS.md:375-376 — C12.4와 C12.5가 모두 미완료다.
23. **공식 완료 수가 72/213에서 증가하는가?** spec/STATUS.md:377; evidence README.md:3 — 두 문서가 72/213 유지로 일치하고 checkbox diff도 없다.
24. **72/213이 공개 API 지원 수로 잘못 서술되는가?** spec/STATUS.md:3,377; evidence README.md:3 — 상단 제품 지원 상태와 공식 구현 상태 집계를 구분한다.
25. **병합된 구현 profile 범위를 일반 sticky 지원으로 넓히는가?** spec/STATUS.md:377; evidence README.md:3 — fixture-test 전용, top-only leaf Block, general author CSS admission 미지원으로 한정한다.
26. **content-edge 수치가 prior blocker 재현 결과와 다르게 바뀌는가?** spec/STATUS.md:377; final-source-implementation-review-20-2026-10-26.md:22 — border bottom 82, content bottom 68, sticky top 40과 과거 계산과의 14px 차이가 같다.
27. **Android PixelCopy를 잘못된 source hash에 귀속하는가?** spec/STATUS.md:377; android-content-edge-pixelgate-20261026/README.md:8 — capture-time whole-source 5dbd898c…와 Android subset d72a3963…를 분리하고 이후 Android source 변경이 없다고 한정한다.
28. **iOS Metal gate를 이전 source 결과로 과장하는가?** spec/STATUS.md:377; final-source-implementation-review-20-2026-10-26.md:46 — iOS 결과는 final whole-source c31f9f53… 및 canonical manifest와 연결된다.
29. **플랫폼·pixel 한계를 넘어선 완료 주장이 있는가?** spec/STATUS.md:377; evidence README.md:11-29; PR body — Android software AVD PixelCopy와 iOS Simulator의 제출 전 Metal drawable로 한정한다. 실기기/hardware GPU, compositor 이후 표면, 전체 CSS/WPT 완료는 주장하지 않는다.
30. **상태와 연결 evidence 경로가 끊기거나 서로 다른 결론을 내는가?** spec/STATUS.md:377; evidence README.md:3-36 — STATUS 행에서 연결한 plan, contract, precomparison, implementation review, PR diff review, runtime README는 존재한다. README 안의 Chrome/Android/iOS 로그·manifest·화면·artifact link도 모두 로컬 파일에 연결되고, 첫 단락의 PR 상태는 STATUS와 일치한다.

## 판정

기존 blocker인 evidence README의 PR 생성 대기 문구는 병합 metadata로 교체되어 해결됐다. 두 문서의 PR #639 merge timestamp/SHA와 empty-check/no-required-main-check/no-CI-pass 설명이 일치하며 C12.5/C12/C13 미완료 및 공식 72/213도 보존된다. 링크와 제한 표현에서 새 blocker나 과장 주장은 찾지 못했다. 이 판정은 위 두 문서의 현재 unstaged diff에만 적용한다.

보고서 추가 전후 두 경로 candidate binary diff fingerprint는 모두 86cdf91f02bc205537c01d874db9a3eb8287f942d82aba10e9d9568a38af99ae다. committed origin/main...HEAD range는 검토 전후 0개 경로이며 빈 diff SHA-256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855로 유지됐다. HEAD와 origin/main은 전후 모두 761ed5e085c9aaea4da8831d62044fe04bfd6d66이다. 보고서만 untracked evidence로 추가했고 두 후보 문서 외 unstaged tracked diff는 없었다.
