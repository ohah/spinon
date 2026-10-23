# C11.4 code·roadmap 병합 상태 동기화 · 2026-10-24

## 기준과 결론

구현 PR [#580](https://github.com/ohah/spinon/pull/580)은 2026-10-23T20:46:27Z Rebase and merge됐다. merge commit은 `33b00d077147ad9f2102af3220daa2c6cf74e2f9`이며 현재 `origin/main`과 checkout HEAD가 같다. 병합 직전 `mergeStateStatus=CLEAN`이었다. `gh pr checks --required`는 보고된 check이 없다고 반환했고 GitHub branch protection 조회는 required status checks 미설정(HTTP 404)이었다. 이 결과를 CI 통과로 기록하지 않는다.

C11.4 구현·runtime 제한 slice는 main에 병합됐지만 C11.4 상위 항목은 미완료다. 공식 roadmap 집계는 계속 `72/213`이며 체크박스는 바꾸지 않았다. 13개 scene은 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 13/13 통과했고, PixelCopy 및 제출 전 Metal drawable은 선언 표본 각각 15/15 exact다. 실기기, Android hardware GPU, iOS compositor 이후 pixels, WPT와 일반 CSS admission은 검증하지 않았다.

C13 overflow·scroll은 미완료 상태지만 현재 활성 lane이 아니다. 현재 lane은 C11.4다. C13.2 ink-only `box-shadow` overflow는 fail-closed, public DOM `scroll` event/CSSOM은 미구현, OS-native wheel/trackpad은 부분 검증이다. C13.4 scrollbar/gutter, C13.5 scroll snap, C13.6 logical axis는 완료되지 않았다.

## 동기화 대상

- [`spec/STATUS.md`](../../../STATUS.md): PR #580 merge SHA, merge 직전 상태 확인, 검증 한계와 C11.4 미완료·72/213을 기록한다.
- [`roadmap.html`](../../../public/roadmap.html): stale pre-runtime 후보 문구를 merged implementation 결과로 교체한다. C13을 미완료·비활성으로 표시한다.
- [`percentage neighbor 계획`](../../../../plan/c11-4-auto-repeat-percentage-neighbors.md): 구현 PR 및 merge SHA를 연결한다.
- [`runtime evidence`](../c11-4-auto-repeat-percentage-neighbors-runtime-2026-10-24/README.md): 실행 기록과 최종 merge 상태를 구분한다.

## 선택 문서 빌드

RSPress 2.0.22 최종 선택 build는 종료 코드 0으로 끝났다. 이전 pass의 선택 root를 hardlink로 재사용하고 상태·계획·roadmap·근거 페이지를 별도 overlay했다. build log는 `selected-build-pass02-final.log`, 입력 hash는 `selected-root-manifest-pass02-final.json`, route/hash 검사는 `selected-assets-manifest-pass02-final.json`에 보존한다. 이 build는 `checkDeadLinks=false`, `checkDeadImages=false`를 사용하므로 저장소 전체 링크 검사 통과를 뜻하지 않는다.

roadmap·STATUS·percentage-neighbor 계획·runtime evidence·동기화 README·PR diff review의 여섯 HTML route와 변경 페이지가 참조하는 정적 asset을 확인한다. 공개 roadmap이 `72/213`과 C13 미완료·비활성/C11.4 활성 상태를 보이고, C11.4 percentage-neighbor 카드가 PR #580 merge SHA와 13/13 Android/iOS 결과를 가리키는지 검사한다.

PR #580 본문의 Android/iOS 이미지는 Agent Browser에서 실제 로드됨을 확인했다. Tailnet preview는 이 상태 문서 PR이 병합된 뒤 선택한 정적 route를 백업·hash manifest 방식으로 갱신하고 HTTPS 제공 결과를 다시 확인한다. GitHub Pages workflow는 실행하지 않는다.

## Tailnet 동기화 전 관찰

Agent Browser로 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`을 열었다. 화면 카운터와 GitHub STATUS 응답은 `72/213`으로 일치했고, C13 문구도 “미완료지만 현재 활성 작업 아님; 진행 중 lane은 C11.4”로 정확했다. 다만 C11.4 percentage-neighbor 카드에는 이전 2/2 로컬-only 결과, Android/iOS runtime 미검증 및 미커밋 상태가 계속 표시됐다. 현재 main의 PR #580은 Android/iOS 13/13 runtime 검증을 포함하므로 공개 preview만 stale한 상태다. 화면은 [동기화 전 캡처](tailnet-roadmap-before-sync.png)에 보존했다.

## 변경 검토

이 문서 변경은 [서로 다른 실패 관점 30개 검토](./pr-diff-review-30.md)를 기록한다. C11.4/C13 완료 checkbox나 공식 집계는 바꾸지 않는다.
