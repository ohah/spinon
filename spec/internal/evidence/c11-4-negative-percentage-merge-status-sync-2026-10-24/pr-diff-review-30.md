# C11.4 signed percentage merge 상태 문서 PR diff 검토 30개

이 문서 PR은 이미 병합된 implementation PR #585의 merge 결과를 계획·내부 계약·상태 대장과 source manifest에 맞춘다. 각 행은 별도의 factual, link, scope 또는 evidence 연결 실패를 확인했다. 공식 완료 상태는 유지했다.

| # | 검토 관점 | 판정 |
| --- | --- | --- |
| 1 | 다른 작업의 PR 번호를 implementation PR로 기록하는가 | GitHub PR #585 title, head branch, path가 signed-percentage slice와 일치한다. |
| 2 | merge 시간을 PR 화면과 다르게 쓰거나 local timezone으로 섞는가 | GitHub `mergedAt` `2026-10-24T06:25:51Z`와 UTC 표기를 대조했다. |
| 3 | rebase merge의 최종 SHA 대신 feature branch의 원래 commit SHA를 기록하는가 | GitHub `mergeCommit.oid` `e42b1573e710c0c36e9c507790902d4a35a53121`과 문서 값을 대조했다. |
| 4 | merged PR 본문을 pending으로 남기는가 | PR API state는 `MERGED`; plan, 0059, status entry 모두 merged 문장으로 갱신했다. |
| 5 | required checks가 없는데 green CI를 주장하는가 | `gh pr checks 585 --required`는 branch에 보고된 checks가 없다고 반환했다. 문서는 “없음”으로 기록한다. |
| 6 | pre-merge CLEAN을 merge 이후 mergeStateStatus UNKNOWN과 혼동하는가 | pre-merge `CLEAN`을 merge 직전 값으로 한정하고, post-merge status는 근거로 쓰지 않는다. |
| 7 | PR #585 body image URL이 삭제된 feature branch를 참조하는가 | Chrome/AVD/iOS raw image URL을 `main`으로 바꾸고 Agent Browser에서 naturalWidth와 complete를 확인했다. |
| 8 | 이미지 URL이 HTML page 또는 오류 image를 반환하는가 | 세 URL 모두 `image/png` HTTP 200이며 browser naturalWidth 1280, 1080, 1206으로 로드됐다. |
| 9 | 계획 PR #584의 merge SHA를 #585 merge SHA로 덮어쓰는가 | plan은 #584의 `411bda24…`와 #585의 `e42b1573…`를 구별한다. |
| 10 | implementation PR merge record가 plan header에만 있고 official status에 빠지는가 | `spec/STATUS.md`에 #585, merge UTC/SHA 및 required-check 한계를 기록했다. |
| 11 | internal contract 0059가 merge된 구현을 후보로 남기는가 | 0059 signed-percentage section에 #585 merge 결과를 적었다. |
| 12 | 구현 범위를 signed percentage 전체로 과장하는가 | exact two-term typed AST subset, finite basis, single negate 제한을 유지한다. |
| 13 | negative final clamp 순서를 모호하게 만들거나 intermediate clamp를 뜻하는가 | docs는 signed terms 합산 뒤 final property range clamp라고 설명한다. |
| 14 | CSS axis basis가 logical/physical 축과 바뀌는가 | columns inline content size와 rows block content size를 명세한다. |
| 15 | error/fail-closed boundary가 status update에서 누락되는가 | unsupported AST, identity, basis, nonfinite/overflow 경계를 0059와 계획에서 유지한다. |
| 16 | 9 신규 scene과 2 controls를 11 신규 지원 scene이라고 부르는가 | 신규 supported scenes 9, regression controls 2, 합계 matrix 11을 구분한다. |
| 17 | 플랫폼 실행 scene count가 matrix evidence와 어긋나는가 | Android와 iOS 각 11/11이며 result JSON의 `sceneCount`도 11이다. |
| 18 | PixelCopy와 Metal sample 수를 platform 간 혼합하는가 | 각 플랫폼별 22/22 exact로 기록한다. |
| 19 | Android software AVD를 실기기/hardware GPU라고 부르는가 | Android API 37 ARM64 software-rendered AVD라는 제한을 명시한다. |
| 20 | iOS drawable sample을 compositor 이후 surface라고 주장하는가 | iOS 증거를 compositor 제출 전 Metal drawable로 한정한다. |
| 21 | 정적 FFI archive scan을 APK/IPA 내부 제외 검증으로 확장하는가 | docs에 최종 APK/IPA fixture exclusion 미검증을 분명히 남긴다. |
| 22 | product negative control와 test positive control의 fixture marker 수가 뒤바뀌는가 | product 0/9/no symbol, opt-in fixture-test 9/9/symbol present와 scan JSON이 맞는다. |
| 23 | 계획·contract·runtime evidence relative links가 다른 feature 문서를 가리키는가 | 세 링크를 signed-percentage path 및 C11.4 0059 section과 대조했다. |
| 24 | runtime README의 신규 review evidence link가 누락 또는 깨지는가 | implementation/runtime 20, PR diff 30 모두 runtime evidence directory에 존재한다. |
| 25 | source manifest source revision이 plan merge base와 다른가 | base commit/tree `411bda24…`/`b42e6e11…`로 설정되어 있고 source files에는 hash가 있다. |
| 26 | source manifest와 archive scan이 서로 다른 implementation diff를 보고하는가 | generator가 base revision부터 같은 source path set을 hash하고 scanner report와 equality를 검사한다. |
| 27 | source manifest의 self-hash 또는 scanner output으로 순환 hash를 만드는가 | `sourceDiffSha256`은 implementation/docs path set만 대상으로 하며 evidence outputs는 file-level hash다. |
| 28 | 공식 완료 checkbox나 `72/213`을 후속 문서 편집 중 올리는가 | C11.4 checkbox와 `72/213`을 그대로 유지한다. |
| 29 | 내부 contract version을 merge 후 올리는가 | 0059 numeric version `0.1.0`을 유지한다. |
| 30 | 전체 C11.4, WPT, 일반 author CSS 또는 미실행 platform을 완료로 표시하는가 | 문서는 전체 Grid/WPT/실기기/hardware GPU/compositor 결과를 미검증 범위로 남긴다. |

## 출처

- [PR #585 merge metadata](https://github.com/ohah/spinon/pull/585)
- [implementation runtime README](../c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/README.md)
- [source manifest](../c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/source-manifest.json)
- [FFI archive scan](../c11-4-auto-repeat-negative-percentage-neighbors-artifact-2026-10-24/scan-results-llvm22.json)
