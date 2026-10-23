# C11.4 현재 코드·로드맵 동기화 · 2026-10-23

## 판정 기준

검토 기준은 `origin/main` `ba04577148e2c7396691e6fa9b1d547bc5cbdbad`이며 문서 수정 직전 checkout과 tree가 같다(`c60273288c48b5f0ea60a2d823a05a1fd42d88a1`). 구현 상태의 원본은 [`spec/STATUS.md`](../../STATUS.md)이고 공개 요약은 [`roadmap.html`](../../public/roadmap.html)이다. 공개 페이지는 상태 대장의 완료 항목을 읽어 현재 완료 수를 표시한다.

## 현재 상태

- C11.4는 전체 track sizing·CSS Grid 지원이 끝나지 않아 공식 체크가 미완료다. 현재 진행 lane은 C11.4다.
- C13도 미완료지만 현재 활성 lane은 아니다. C13.2 ink-only `box-shadow` overflow는 fail-closed이고, public DOM `scroll` event/CSSOM은 구현되지 않았으며, OS-native wheel/trackpad은 부분 검증 상태다. C13.3 policy simulator 후속 PR #185는 병합됐다.
- 공개 공식 완료 수는 `72/213`이다. 각 체크박스 수를 직접 바꾸지 않았다.
- 단순 Markdown checkbox 수는 공식 집계 범위가 아니다. Agent Browser에서 상태 페이지의 raw checkbox input 226개/checked 87개를 관찰했지만, 공개 roadmap은 상위 task ID 78개, 공식 세부 ID 133개, `미구현 하위 항목` 형식의 ID 없는 하위 task 2개만 집계한다. 이 필터 결과가 213개 중 72개 완료다. 구현은 `spec/public/roadmap.html`의 `baseTasks`, `detailTasks`, `childPattern` 및 `boardTasks` 집계를 따른다.

## 코드·병합 근거 대조

GitHub에서 다음 구현 및 상태 PR의 병합 상태와 SHA를 다시 조회했다. 관련 runtime 근거와 현재 제한은 [C11.4 상위 계획](../../../../plan/c11-4-grid-track-sizing.md), [각 slice 계획](../../../../plan/c11-4-auto-repeat-template-areas.md), [내부 Grid 계약](../../0059-c11-grid-layout.md), [runtime evidence](../c11-4-template-area-auto-repeat-runtime-2026-10-23/README.md) 및 각 링크된 근거 파일에 있다.

| PR | 병합 SHA | 코드·문서 범위 | 로드맵 판정 |
| --- | --- | --- | --- |
| [#539](https://github.com/ohah/spinon/pull/539) | `0f46437c299df6f0c069b694a3f2184ac464ad3d` | auto-repeat + `grid-template-areas`; Android/iOS Simulator 각 13/13 scene, surface 66/66 | 제한 slice 병합, C11.4 미완료 |
| [#540](https://github.com/ohah/spinon/pull/540) | `92b447f687e751cfc2c333bd17e22c4271c36536` | #539 상태·로드맵 반영 | 문서 상태 PR 병합 |
| [#545](https://github.com/ohah/spinon/pull/545) | `d7a6b35966f7ba9ad53fc5c450bfd1e6ec9ead34` | 양축 auto-repeat + area 조합 후속 | 별도 제한 slice 병합 |
| [#549](https://github.com/ohah/spinon/pull/549) | `fdf5a3a24f3f5f3f1e2f1bc2f5e871f29dbf7ca8` | auto-fit collapse + area 확장 후속 | 별도 제한 slice 병합 |
| [#553](https://github.com/ohah/spinon/pull/553) | `7151605419e4777e2bd0c8ced5cb6a926d4bddab` | absolute `calc()` auto-repeat minimum; 각 Simulator 6/6 | 제한 slice 병합, PixelCopy/Metal pixel equality 검사는 아님 |
| [#561](https://github.com/ohah/spinon/pull/561) | `ff4233d0df00a632d71c28398c9a8274fb4abc16` | definite positive percentage minimum; 각 Simulator 16/16 | 계획 문구가 미구현처럼 남아 있던 부분 정정 |
| [#568](https://github.com/ohah/spinon/pull/568) | `c436a38388fe8a191fd374495c516c88d1e3cd54` | definite percentage maximum; 각 Simulator 28/28, surface 표본 78/78 | 제한 slice 병합, Android ANGLE/SwiftShader |
| [#569](https://github.com/ohah/spinon/pull/569) | `ba04577148e2c7396691e6fa9b1d547bc5cbdbad` | #568 merge 상태 문서 동기화 | 최신 main 기준 |

현재 구현 경계는 `crates/spinon-layout/src/grid.rs`의 auto-repeat admission·percentage/calc 지원 검사와 `crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs`의 typed style projection에 있다. 회귀 조건은 `crates/spinon-layout/src/grid_track_sizing_tests/positive_percentage_auto_repeat.rs`, `crates/spinon-layout/src/grid_auto_track_sizing_tests/auto_repeat/area_expansion.rs`, `crates/spinon-style-to-layout/src/c11_4_track_sizing_areas_tests/`에서 확인했다. 앱 장면 adapter는 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/`, `.../c11_4_auto_repeat_calc_minimum.rs`, `.../c11_4_auto_repeat_positive_percent_minimum.rs`, `.../c11_4_template_area_auto_repeat.rs`에 있다.

PR 조회에서 위 merge state는 모두 `MERGED`였다. 현재 GitHub 응답의 status-check rollup은 각 비어 있고 `mergeStateStatus=UNKNOWN`으로 돌아와, 이 문서는 PR 병합 사실을 CI 통과 주장으로 확대하지 않는다.

## 발견한 문서 차이와 수정

- #553 공개 roadmap 카드, `spec/STATUS.md`, 내부 계약 `0059`가 당시 `74/207`을 최신 공식 집계처럼 표현했다. `74/207`은 실행 당시 기록으로 보존하고 현재 `72/213`과 구분했다. 기존 C13 실행 기록의 같은 숫자도 날짜별 스냅샷으로 표시했다.
- #561은 이미 병합됐는데 positive-percentage plan에는 구현 전이라고 남아 있었다. 병합 SHA, Android/iOS Simulator 16/16, 실제 지원 경계와 미완료 parent 상태를 추가했다.
- #539 계획에는 merge/Tailnet 동기화가 남았다고 적혀 있었고 merge SHA도 GitHub 값과 달랐다. 유효하지 않은 SHA를 `0f46437c299df6f0c069b694a3f2184ac464ad3d`로 바로잡고 #539/#540/#545/#549 병합 상태와 현재 Tailnet 확인을 반영했다.
- C11.4 및 C13 공식 체크박스는 바꾸지 않았다. 제한된 scene fixture 결과를 전체 Grid/scroll 기능 완료로 승격하지 않았다.

## 변경 범위와 검증

코드·명세 계약·체크박스는 변경하지 않았고, 계획·상태 설명·로드맵·기존 실행 기록의 시점 표기만 정리했다. PR diff 별도 검토는 [`pr-diff-review-20.md`](./pr-diff-review-20.md)에 기록한다.

### 문서 빌드

- 저장소 전체 `mise exec -- bun run docs:build`는 exit code 1로 끝났다. RSPress가 기존 내부 증거 문서의 누락 이미지·로그 링크를 보고했다. 첫 사례는 `c04-runtime-css-to-gpu-resize-simulators-2026-10-09.md`가 존재하지 않는 PNG와 로그를 참조하는 항목이었다. 이번 변경 페이지에서는 오류가 발견되지 않았다.
- 영향 페이지를 분리한 RSPress 2.0.22 선택 빌드는 exit code 0이었다. 기존 pass70 선택 root를 hardlink로 재사용하고 현재 페이지를 overlay한 뒤 `checkDeadLinks=false`, `checkDeadImages=false`로 빌드했다. 따라서 이 결과는 전체 저장소 링크 검사 통과를 뜻하지 않는다.
- 변경 roadmap HTML은 source와 build output 모두 SHA-256 `01659634af042c1708e29681073a815c6b248cfbebcda65efb8dd22b7132f538`이다. 상태·계획·근거 페이지 12개를 포함해 필수 route 13개가 생성된 것을 확인했다. manifest는 `selected-root-pass01.json`, `selected-build-verification-pass01.json`이다.

### Tailnet과 Agent Browser

- 기존 Tailnet preview server PID `1191`과 `127.0.0.1:8768` listener를 유지했다. preview root의 변경된 HTML 9개를 교체하고 새 HTML 2개를 추가했다. 교체 전 9개 파일, 합계 `4,096,773` bytes를 별도 backup root에 보존했다. 삭제는 0건이다.
- 계획의 상대 링크가 `/spinon/spec/internal/evidence/...`로 해석되는 것도 확인해 sync README·review 별칭 route 2개를 추가했다. 결과적으로 선택 HTML route 13개 모두 HTTP 200이며 build source, 로컬 preview, HTTPS 제공 응답의 SHA-256이 일치한다. 관련 machine-readable 기록은 `delta-manifest-pass01.json`, `copy-and-backup-pass01.json`, `remote-verification-pass01.json`, `alias-route-verification-pass01.json`이다.
- Agent Browser에서 공식 로드맵 카운터 `213개 표시 · 공식 완료 72/213`, breakdown `상위 78개 + 세부 ID 133개 + ID 없는 하위 체크 2개`를 확인했다. 상태 route에서 C11.4와 C13의 상위 체크는 모두 미완료였다. 공개 roadmap은 C11.4를 활성 lane으로, C13은 미완료·비활성으로 표시했다. #553 카드의 `74/207`은 당시 기록, 현재 값은 `72/213`으로 함께 보인다.
- Agent Browser에서 #539 계획이 실제 PR merge SHA `0f46437c299df6f0c069b694a3f2184ac464ad3d`를 표시하고, 잘못된 이전 SHA를 포함하지 않는 것을 확인했다. #561 계획은 merge SHA·16/16 Simulator 결과·상위 C11.4 미완료 상태를 표시한다. 새 근거 route도 Agent Browser에서 열렸다.
- Agent Browser 화면 캡처는 `tailnet-roadmap-agent-browser-pass01.png`(현재 lane과 공식 카운터), `tailnet-roadmap-c11-4-count-agent-browser-pass01.png`(#553 과거·현재 집계 카드)다. 캡처 검증 세션은 종료했고 `active=false`, `pid=null`을 확인했다. 결과는 `agent-browser-verification-pass01.json`, `browser-close-pass01.json`에 기록했다.

GitHub Pages workflow는 실행하지 않았다.
