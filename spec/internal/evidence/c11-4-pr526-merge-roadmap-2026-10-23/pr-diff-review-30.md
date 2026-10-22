# PR #526 병합 상태 문서 PR #527 diff 검토 30개

문서 PR [#527](https://github.com/ohah/spinon/pull/527)의 상태 대장·계획·공개 roadmap·실행 evidence 변경을 서로 다른 실패 경로로 검토했다. PR #526의 C11.4 구현/runtime 검토 및 PR diff 검토는 이 표에 재사용하지 않았다.

| # | 독립 실패 관점 | 확인 및 판정 |
| ---: | --- | --- |
| 1 | PR #526 상태를 아직 OPEN으로 기록하는가 | GitHub PR API에서 `MERGED`를 확인했고 docs는 Rebase and merge 완료로 기록한다. 통과. |
| 2 | 병합 UTC 시각을 한국 시각으로 잘못 변환하는가 | GitHub `mergedAt=2026-10-22T19:48:48Z`에 9시간을 더해 `2026-10-23 04:48:48 KST`로 변환했다. 통과. |
| 3 | merge SHA를 PR head SHA와 혼동하는가 | GitHub merge commit `b87c2a7d5e68434335fed4418c9f95a3a0353c6f`가 fetch한 `origin/main`과 일치한다. 통과. |
| 4 | 기준 plan PR #525 merge 기록이 #526 기록에 덮이는가 | 집중 계획과 상위 계획에서 #525의 `bd665f...`와 #526의 `b87c2a...`을 별도로 남긴다. 통과. |
| 5 | merge 상태 요약이 plan과 STATUS 사이에서 다른가 | 두 문서 모두 PR #526 merge 일시와 SHA를 같은 값으로 기록한다. 통과. |
| 6 | 공개 roadmap 카드가 merge 대기 상태로 남는가 | 카드 제목을 `PR #526 병합` 상태로 바꾸고 merge SHA를 표시한다. 통과. |
| 7 | 공개 roadmap이 PR을 잘못 연결하는가 | 구현 PR 링크가 정확히 `https://github.com/ohah/spinon/pull/526`을 가리킨다. 통과. |
| 8 | status-check rollup 부재를 성공한 CI로 잘못 표현하는가 | status rollup이 비었다고 기록하고 passing CI가 있다고 주장하지 않는다. 통과. |
| 9 | main required status checks 상태를 추측하는가 | branch-protection API가 required status checks 미설정을 반환했고 문서에 그 상태만 적었다. 통과. |
| 10 | PR merge 자체를 별도 code review 승인과 혼동하는가 | GitHub `reviewDecision`은 비어 있고 required review count는 0이다. 계획에는 이를 별도 code review pass라고 주장하지 않는다. 통과. |
| 11 | C11.4의 상위 완료 checkbox를 체크하는가 | `spec/STATUS.md`의 C11.4는 미완료 그대로다. 통과. |
| 12 | 공식 완료 수를 `77/213`에서 올리는가 | 계획·STATUS·roadmap 모두 `77/213` 미완료 상태를 보존한다. 통과. |
| 13 | fixture 추가를 공개 API 지원으로 설명하는가 | 새 API·versioned internal interface가 없으며 fixture-only임을 plan과 PR body가 명시한다. 통과. |
| 14 | 네 scene의 지원 범위를 일반 CSS admission으로 확대하는가 | 결과는 고정 opt-in scene으로 표현되고 general CSS Grid 지원을 주장하지 않는다. 통과. |
| 15 | 계획에 없는 scene count나 geometry를 상태 요약에 더하는가 | 두 intrinsic과 두 definite scene, 해당 track 값은 실행 README와 동일하다. 통과. |
| 16 | border/image/clear sample `6/6` 범위를 전체 border 적합성으로 오인하는가 | 세 지정 표본만 검증했다고 runtime README와 plan에 적었다. 통과. |
| 17 | Android simulator 결과를 물리 기기 결과로 승격하는가 | Android API 37 ARM64 AVD라고 명시하고 physical device/hardware GPU를 미검증으로 남겼다. 통과. |
| 18 | iOS surface sample을 compositor 이후 화면으로 주장하는가 | iOS 26.2 Simulator 및 compositor 제출 전 Metal drawable로 경계를 표시한다. 통과. |
| 19 | line geometry를 glyph raster evidence로 확대하는가 | runtime text count가 0이며 glyph paint 미검증이라고 유지한다. 통과. |
| 20 | simulator scene 결과를 WPT 적합성으로 오인하는가 | WPT를 실행하거나 통과했다고 쓰지 않고 범위 밖으로 기록한다. 통과. |
| 21 | static archive 비교를 final APK/IPA 제외로 과장하는가 | archive feature 경계만 증거라고 하고 release APK/IPA 제외는 주장하지 않는다. 통과. |
| 22 | 실제 실행 evidence link가 precomparison만 가리키는가 | runtime README, simulator screenshots, 20/30 검토 문서와 archive 경계를 직접 연결한다. 통과. |
| 23 | 공개 roadmap screenshot 링크가 잘못된 scene/platform을 가리키는가 | Android와 iOS 모두 successful `max-content-forward.png` 경로를 지정하며 파일이 PR #526 merge tree에 있다. 통과. |
| 24 | Rspress URL 변환이 문서 링크를 깨뜨리는가 | 새 Markdown 문서는 기존 `.html` route 규칙, PNG는 정적 evidence 경로를 따른다. 파일 경로와 href를 대조했다. 통과. |
| 25 | 화면 PR에서 raw screenshot 링크가 404인가 | PR #527 본문의 Android/iOS raw URL은 각각 HTTP 200이다. 통과. |
| 26 | PR 본문에 기존 화면이 아닌 다른 산출물이 표시되는가 | Agent Browser DOM alt·src가 PR 본문의 Android definite-forward와 iOS definite-forward 파일에 대응한다. 통과. |
| 27 | GitHub가 이미지를 내려받지 않았는데 성공으로 판단하는가 | Agent Browser에서 두 이미지 모두 `complete=true`, `rendered=true`; dimensions는 1080×2400 및 1206×2622다. 통과. |
| 28 | Agent Browser나 terminal-browser가 확인 뒤 남아 있는가 | Agent Browser 탭은 0으로 닫았고 세션 daemon은 종료 확인 대상이다. `terminal-browser` 프로세스는 없으며 Tailnet 서버 PID 1191은 보존한다. 통과. |
| 29 | PR diff review evidence 자체가 계획/runtime 검토를 재탕하는가 | 이 문서는 merge metadata, docs consistency, routes, limits, rendered links만 검토한다. 기존 20·30개 구현 검토를 다시 세지 않는다. 통과. |
| 30 | Tailnet 미리보기 동기화를 완료 전에 주장하는가 | README와 PR body는 sync가 PR #527 병합 후라고 명시하며 아직 성공한 preview sync를 주장하지 않는다. 통과. |

## 판정

문서와 PR 표시에서 남은 차단 결함을 찾지 못했다. PR #527은 문서 변경이며 C11.4의 기술 범위·공식 완료 수·simulator 증거 경계를 바꾸지 않는다. Tailnet 동기화는 이 PR merge 이후 현재 `main`을 기준으로 별도 수행한다.
