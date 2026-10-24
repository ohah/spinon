# C11.4 PR #607 최종 문서 변경과 Tailnet sync 30개 실패 관점 검토

검토일: 2026-10-25

## 범위와 기준

기준 main, origin/main, 현재 HEAD는 모두 2f00c2c95061db635facece5233327f895c99319이다. 문서 diff SHA-256은 37ae9c6ef0d2126718c9ff6d21db79a9c3576154d38b7ed551c3323c7af63fb7이며 대상은 plan/c11-4-cross-axis-fixed-auto-repeat-areas.md, spec/STATUS.md, spec/public/roadmap.html이다. 추가로 c11-4-cross-axis-fixed-auto-repeat-areas-tailnet-sync-2026-10-25의 최종 README, pass01/pass02/pass03 빌드·복사·검증 스크립트와 로그·JSON, Tailnet 캡처를 대조했다. GitHub PR metadata와 main required-check 설정은 읽기 전용으로 다시 확인했다. 이 검토에서 새 빌드나 테스트는 실행하지 않았다.

## 검토 항목

1. **비교 기준이 실제 main인가 — PASS.** 로컬 main, origin/main, 작업 HEAD가 PR #607의 merge SHA 2f00c2c95061db635facece5233327f895c99319와 같다. 최종 문서 diff는 이 기준에서 계산됐다.
2. **문서 PR 범위가 구현 코드로 섞이지 않았는가 — PASS.** 검토 요청의 세 tracked 문서 파일만 변경 대상으로 비교했고, 기존의 미추적 실행 근거들은 이 문서 diff에 섞지 않았다.
3. **PR 번호·제목·URL이 같은 변경을 가리키는가 — PASS.** plan과 public roadmap은 ohah/spinon PR #607 링크를 쓰며 GitHub 제목은 교차축 repeat 및 areas 제한 runtime fixture 변경이다.
4. **문서가 PR의 실제 상태를 반영하는가 — PASS.** GitHub에서 PR #607 state는 MERGED다.
5. **병합 시각이 일치하는가 — PASS.** mergedAt은 2026-10-24T19:06:42Z이고 세 문서의 병합 기록과 Tailnet 검증 결과가 같은 값을 사용한다.
6. **merge SHA가 정확한가 — PASS.** GitHub mergeCommit.oid와 계획·상태·로드맵·selected build marker가 모두 2f00c2c95061db635facece5233327f895c99319다.
7. **PR head/base와 병합 결과를 혼동하지 않는가 — PASS.** PR head 911c5d2534a9e2927af598df8a73d68b0f9c504a 및 base 96efeebd9e816110ac981b6c24eecd3e88b620a는 merge SHA와 별도 값이며 문서는 merge SHA를 결과로 적는다.
8. **병합 사실이 현재 main에 반영됐는가 — PASS.** 현재 local main과 origin/main은 모두 merge SHA를 가리킨다.
9. **PR checks 상태를 CI 성공으로 오독하지 않는가 — PASS.** PR metadata의 statusCheckRollup은 빈 배열이고 gh pr checks도 checks 미보고로 반환한다. 문서는 이를 빈 rollup으로만 적고 green으로 표시하지 않는다.
10. **required checks 설정을 별도 확인했는가 — PASS.** main required-status-checks API는 설정되지 않았다는 HTTP 404를 반환하며 문구가 그 경계를 정확히 전한다.
11. **문서 날짜와 UTC 병합일이 충돌하지 않는가 — PASS.** 2026-10-24T19:06:42Z는 KST 2026-10-25 04:06:42이므로 10월 25일 작업 표기와 모순되지 않는다.
12. **계획에 완료 상태가 과장되지 않는가 — PASS.** 계획은 PR 병합 사실을 추가하면서도 제한 fixture, 미완료 C11.4, 공식 72/213을 함께 유지한다.
13. **상태 대장의 C11.4 checkbox가 유지되는가 — PASS.** 새 구현 기록은 미완료 후보 설명으로 추가됐고 상위 C11.4 항목을 완료로 바꾸지 않았다.
14. **공식 완료 수가 유지되는가 — PASS.** plan, STATUS, public roadmap, selected build marker에서 72/213을 유지한다.
15. **같은 PR 기록의 중복 주장이 서로 일치하는가 — PASS.** PR 번호, 병합 시각, merge SHA, 빈 check rollup, required-check 부재와 완료 수가 세 문서에서 충돌하지 않는다.
16. **기능 범위 문구가 공개 지원 범위로 확대되지 않는가 — PASS.** 한 축 fixed-repeat pattern과 반대 축 single auto-repeat, areas를 결합한 opt-in profile로 설명한다.
17. **11개 scene 수가 비교·실행 근거와 맞는가 — PASS.** plan/status/roadmap의 11 scene 요약은 runtime 및 precomparison 근거와 일치한다.
18. **Android 대상과 scene 통과 수가 맞는가 — PASS.** Android API 37 ARM64 AVD 11/11 표기는 연결된 runtime 결과에 맞는다.
19. **iOS 대상과 scene 통과 수가 맞는가 — PASS.** iPhone 17 Pro/iOS 26.2 Simulator 11/11 표기는 연결된 runtime 결과에 맞는다.
20. **표면 pixel 표본을 scene 통과 수와 혼합하지 않는가 — PASS.** PixelCopy와 제출 전 Metal drawable의 각 35/35 exact 표본은 11/11 layout scene 결과와 별도 지표다.
21. **기본/opt-in archive 검사가 app bundle exclusion으로 과장되지 않는가 — PASS.** 0/11 대 11/11 scene marker와 host symbol 결과는 static archive 경계로 표현되며 APK/IPA exclusion이라고 하지 않는다.
22. **남은 적합성·플랫폼 한계가 보이는가 — PASS.** 전체 CSS Grid, 일반 CSS admission, WPT, 실기기 GPU 및 전체 C11.4 완료로 확대 해석하지 않도록 경계를 둔다. Metal 표본도 제출 전 drawable이라고 한정한다.
23. **수정된 public roadmap 문장이 자연스럽고 시간 관계가 분명한가 — PASS.** roadmap.html:175는 PR merge metadata를 heading에 두고, 본문은 구현 결과·11 scene 관찰·check 상태·범위 제한 순으로 읽힌다. 이전처럼 ‘11 scene을’ 목적어를 병합 절이 끊지 않는다.
24. **public/internal HTML 링크가 존재하는 route로 이어지는가 — PASS.** 새 plan·precomparison·runtime·review·Android/iOS screenshot·STATUS route는 selected build 목록에 있고 Tailnet 검증 대상에도 들어 있다. 새 PR 외부 링크는 GitHub PR #607을 연다.
25. **선택 빌드의 도구 버전과 범위가 명시되는가 — PASS.** README와 log는 Rspress 2.0.22 selected build이며 전체 문서 빌드가 아니라고 구분한다. package pin도 2.0.22다.
26. **실패 설명과 원본 build log의 whitespace 경계가 정확한가 — PASS.** README는 pass01의 실제 blocker를 public/icon.svg favicon과 runtime README가 import한 4개 PNG로 구분하고, pass02에서 public/icon.svg·public/logo.svg 및 네 screenshot을 기대 경로에 추가한 뒤 빌드가 성공했다고 기록한다. README가 설명한 대로 Rspress build log는 원본 그대로 보존되어 있다. 읽기 전용 git diff --cached --check 결과의 13 trailing-whitespace 경고는 selected-build.log 1줄(error Build errors: ), selected-build-pass02.log 6줄, selected-build-pass03.log 6줄에만 있다. 후속 두 로그의 여섯 줄은 fixed-width bundle table header/asset rows다. pass02·pass03 로그 각각에는 new blank line at EOF 알림도 하나 있다. source 문서나 다른 evidence 파일은 지적되지 않았다. 원본 log를 trim하지 않고 보존한 것은 실행 출력 provenance를 남기며, adjacent JSON이 route/hash의 기계 판독 값을 제공하므로 적절하다. README는 반복되는 fixed-width padding을 설명하고 전체 warning을 고정된 표의 문제로만 표현하지 않는다.
27. **최종 pass03 route 수와 preflight 산식이 맞는가 — PASS.** document/direct asset은 0 create + 10 replace + 8 same으로 18개이고, static asset은 12 create + 13 same으로 25개다. manifest가 경로 중복을 거부하며 최종 verifier의 distinct route 수도 43이다.
28. **교체 백업과 삭제 경계가 최종 상태에서도 맞는가 — PASS.** pass03 document preflight의 10 replacement는 10개 백업 검증과 연결되고, static asset 25개는 create/same만 포함한다. copy/verify 스크립트와 pass03 JSON은 route 삭제 없음 및 10/10 backup 확인을 기록한다.
29. **local/Tailnet 전달과 최종 Agent Browser 검증이 맞는가 — PASS.** local HTTP JSON은 43/43 status 200·byte count·SHA 일치, final Tailnet JSON은 remote HTTP 200·SHA 43/43과 backup 10/10을 기록한다. Agent Browser pass03는 roadmap·STATUS·plan의 PR/SHA/count, 갱신된 review의 final-pass/no-follow-up 상태, runtime 네 screenshot load를 확인한다. 네 이미지는 Android 1080×2400 두 장과 iOS 1206×2622 두 장이다. README의 roadmap 캡처 SHA-256 757c16c3ceb23db4b0ca519f411ad64611c3ee1dc0b170cb8f33cabb5c2b0b1a와 파일 hash가 같고 화면에 #607 행이 보인다.
30. **수정된 기존 review route가 재빌드·동기화 뒤 최신인가 — PASS.** pass03 selected-root manifest의 기존 runtime post-merge review source hash는 현재 파일과 같은 31c2bf53d31b382d0475ffe11a3bf87afb6acf60c223f0e93e63f34d8105130e다. 그 source로 생성된 HTML hash bf56c9cfc753ab5a79b58621a5585421322a1a913e307546c219c306f3a55bc4는 pass03 final verification에서 local/Tailnet HTTP 200 및 hash 일치로 확인됐다. 새 Tailnet sync review 문서는 selected manifest에 포함되지 않으며 요청 범위상 preview route로 추가할 필요가 없다.

## 판정

문서 diff의 merge 사실, 완료 수, 지원 경계와 연결 route는 일치한다. build log의 raw 보존 이유와 checker가 지적한 파일 범위도 확인했으며 30개 관점 모두 통과했다. whitespace 알림은 세 개의 Rspress build log에만 한정되고 source 문서와 다른 evidence 파일은 통과한다. README는 pass01의 실제 favicon·screenshot blocker와 pass02에서 추가한 logo asset을 구분하며, pass03는 갱신된 기존 review route를 포함한 43개 route의 local/Tailnet 응답 hash와 10개 replacement backup을 검증했다. Agent Browser final review에서도 PR metadata와 네 screenshot을 확인했다. 이는 선택된 문서 preview 및 연결 이미지의 증거이며 전체 문서 빌드나 더 넓은 CSS conformance를 뜻하지 않는다.
