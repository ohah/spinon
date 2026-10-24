# C11.4 PR #607 병합 후 상태·로드맵 변경 검토 30개

검토 대상은 main 2f00c2c95061db635facece5233327f895c99319을 기준으로 한 문서 변경이다. 대상 파일은 plan/c11-4-cross-axis-fixed-auto-repeat-areas.md, spec/STATUS.md, spec/public/roadmap.html 세 개이며 최종 diff SHA-256은 37ae9c6ef0d2126718c9ff6d21db79a9c3576154d38b7ed551c3323c7af63fb7이다. GitHub PR metadata와 main required-status-check 설정도 읽기 전용으로 대조했다.

## 검토 항목

1. **PR 번호와 링크가 서로 일치하는가 — PASS.** 세 문서 모두 PR #607을 가리키며 GitHub URL의 대상 제목도 이 cross-axis slice와 일치한다.
2. **기록이 PR의 현재 상태를 반영하는가 — PASS.** GitHub API에서 PR #607은 MERGED로 반환됐다.
3. **병합 시각이 정확한가 — PASS.** mergedAt은 2026-10-24T19:06:42Z이며 계획·상태·공개 로드맵의 시각과 같다.
4. **merge SHA가 정확한가 — PASS.** GitHub의 mergeCommit.oid는 2f00c2c95061db635facece5233327f895c99319다.
5. **PR head SHA가 기록된 병합 결과와 혼동되지 않는가 — PASS.** PR head 911c5d2534a9e2927af598df8a73d68b0f9c504a와 Rebase merge commit 2f00…은 구분돼 있다.
6. **PR base가 검토한 main 이전 상태와 맞는가 — PASS.** PR의 base OID는 96efeebd9e816110ac981b6c24eecd3e88b620a8이다.
7. **현재 main에 병합 결과가 반영됐는가 — PASS.** 로컬 main OID가 merge SHA 2f00…와 일치한다.
8. **PR check rollup 상태가 정확한가 — PASS.** PR metadata의 statusCheckRollup은 빈 배열이고 문서는 rollup이 비었다고 기록한다.
9. **필수 status check 설정을 별도로 확인했는가 — PASS.** main의 required-status-checks endpoint는 설정되지 않았다는 404를 반환하며 문서의 설명과 일치한다.
10. **CI 통과를 과장하지 않는가 — PASS.** 문서는 결과를 CI green 또는 required gate 통과라고 부르지 않는다.
11. **테스트 증거와 GitHub CI 상태가 혼합되지 않는가 — PASS.** simulator·fixture 결과는 별도 execution evidence로 연결돼 있다.
12. **C11.4 상위 checkbox가 완료로 승격되지 않는가 — PASS.** 상태 대장은 기존 상위 C11.4 미완료 경계를 유지한다.
13. **공식 집계가 바뀌지 않는가 — PASS.** 72/213이 계획·상태·로드맵에서 유지된다.
14. **상위 C11 전체 완료를 주장하지 않는가 — PASS.** 각 문서는 제한 fixture만으로 C11.4 또는 C11을 완료하지 않는다고 경계 짓는다.
15. **일반 CSS admission 제한이 남아 있는가 — PASS.** public roadmap과 C11.4 slice plan이 일반 CSS admission을 지원 범위로 확대하지 않는다.
16. **WPT 범위가 명확한가 — PASS.** public roadmap의 제한 문구에 WPT 미완료가 남아 있다.
17. **runtime profile의 opt-in 성격이 유지되는가 — PASS.** 계획은 RuntimeBlockFlexGridTrackSizingV1 제한 profile에 한정한다.
18. **지원 grammar가 과장되지 않는가 — PASS.** 한 fixed-repeat pattern과 다른 축의 단일 auto-repeat 조합이라는 범위가 계획과 public summary에서 일치한다.
19. **축 방향 전치 범위가 빠지지 않는가 — PASS.** Android/iOS 11-scene 증거와 계획은 fixed columns 및 fixed rows 방향을 모두 포함한다.
20. **scene 개수가 계획·실행 근거와 일치하는가 — PASS.** 계획 및 status는 11개 고정 장면을 가리키며 runtime README에 같은 집합이 기록돼 있다.
21. **Android 결과가 올바른 target으로 표기되는가 — PASS.** API 37 ARM64 AVD 11/11은 Android 실행 근거와 일치한다.
22. **iOS 결과가 올바른 target으로 표기되는가 — PASS.** iPhone 17 Pro/iOS 26.2 Simulator 11/11은 iOS 실행 근거와 일치한다.
23. **surface sample 수를 geometry 통과 수와 혼동하지 않는가 — PASS.** 각 플랫폼의 PixelCopy/Metal drawable 표본 35/35가 별도 수치로 표시된다.
24. **Metal 출력 단계 한계를 보존하는가 — PASS.** 공개 로드맵과 runtime 근거는 표본을 compositor 전 drawable로 한정한다.
25. **Android renderer 한계를 보존하는가 — PASS.** public scope는 실기기 hardware GPU 결과를 주장하지 않는다.
26. **static archive 경계가 앱 번들 경계로 확대되지 않는가 — PASS.** status/runtime evidence는 static archive 결과를 APK/IPA fixture exclusion으로 표현하지 않는다.
27. **fixed-axis area dimension 제한이 남아 있는가 — PASS.** 집중 계획은 fixed-axis area dimension mismatch를 실패 경로로 기록하고, public summary는 전체 CSS 지원을 주장하지 않는다.
28. **상대 문서 링크가 실제 파일을 가리키는가 — PASS.** 계획, runtime README, 구현/runtime review 20개와 PR diff review 30개 파일이 모두 존재한다.
29. **공개 화면 링크가 실제 캡처를 가리키는가 — PASS.** Android와 iOS empty-area PNG 두 파일이 각각 존재하며 roadmap의 경로와 일치한다.
30. **공개 roadmap 병합 문구의 문법과 시간 순서가 명료한가 — PASS.** spec/public/roadmap.html:175는 PR #607 Rebase and merge, UTC 병합 시각과 merge SHA를 제목에 둔다. 이어지는 본문은 11 scene의 simulator 결과, check 상태, 범위 제한 순으로 서술하며 이전 문장처럼 ‘11 scene을’ 뒤에 병합 상태 절이 끼어들지 않는다. 2026-10-25 표기와 2026-10-24T19:06:42Z는 KST로 2026-10-25 04:06:42이므로 서로 모순되지 않는다.

## 판정

최종 문서 diff의 PR·merge metadata, 공식 완료 수, 지원 범위와 증거 링크는 일치한다. 이전 30번의 문장 문제는 수정되어 재검토에서 통과했다. 이 검토는 문서 diff에 한정하며 Tailnet 배포는 새 동기화 검토 문서에서 별도로 다룬다.
