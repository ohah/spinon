# C11.4 열 축 fit-content Tailnet 로드맵 동기화 근거

## 범위와 공식 상태

PR [#811](https://github.com/ohah/spinon/pull/811)의 제한 opt-in runtime slice 병합 결과를 `spec/STATUS.md`, 집중 계획, 공개 `spec/public/roadmap.html`에 반영하고 기존 Tailnet 개발 미리보기를 갱신했다. PR은 2026-10-29T17:19:06Z에 Rebase and merge됐고 merge SHA는 `8b2e3088c84ae4a3df1fd56c9e75eba150ce1f59`다. 구현 PR의 GitHub check rollup과 main required status check 조회는 비어 있어 CI 통과로 기록하지 않았다.

C11.4 checkbox와 전체 공식 완료 `72/213`은 그대로 미완료/유지다. 이 slice가 추가하는 것은 고정 네 scene의 fixture-test 전용 열 축 계산이다. 일반 author CSS admission, 전체 WPT, 실기기·hardware GPU, iOS compositor 이후 pixel과 전체 C11.4/C11을 완료했다고 주장하지 않는다.

## 선택 빌드

- 고정 RSPress v2.0.22 실행 파일을 사용했다: `mise exec -- packages/docs/node_modules/.bin/rspress build --config spec/internal/evidence/c11-4-fit-content-column-roadmap-sync-2026-10-30/rspress.config.ts`.
- 첫 선택 빌드 시도는 별도 최소 root의 `public/icon.svg` 누락으로 실패했다. 해당 파일을 추가하고 새 pass02 출력 경로로 다시 빌드했다. 실패 결과는 덮어쓰지 않았다.
- 최종 선택 root: `build/spinon/c11-4-fit-content-column-roadmap-site-pass02-20261030`.
- 최종 manifest는 92개 파일, 7,176,017 bytes를 기록하고 roadmap, STATUS, 내부 계약, 계획, 비교·runtime 근거, 리뷰 페이지 및 Android/iOS 화면 경로를 확인했다.
- 첫 사전 검토에는 무관한 `404.html` replacement가 포함됐지만 복사 전에 멈췄다. pass02 preflight에서 `404.html`을 제외했다. 최종 선택 작업은 삭제를 수행하지 않았다.

## Tailnet 동기화

- 대상은 기존 `SpinonPreview` `/spinon/` 경로다. pass02 preflight는 92개 route를 분류했다: 신규 77, 교체 4, 동일 11. 교체 전에 기존 파일 4개, 총 2,022,171 bytes를 보관했다.
- 복사 기록은 신규·교체 81개, 기존 백업 4개, 삭제 0개를 남긴다. `404.html`은 교체하지 않았다.
- 검증 보고서는 92/92 로컬 대상, 4/4 백업, 92/92 원격 HTTP 200 및 SHA-256 일치를 기록한다. 삭제는 없다.
- 원시 결과는 `selected-build-verification-pass02.json`, `tailnet-preflight-pass02.json`, `tailnet-copy-pass02.json`, `tailnet-verify-pass02.json`에 있다. 실행 스크립트는 `prepare-selected-root.mjs`, `complete-selected-build.mjs`, `sync-tailnet.mjs`, `rspress.config.ts`다.

## Agent Browser 표시 확인

터미널 브라우저를 사용하지 않았다. 별도 Agent Browser 세션 `spinon-roadmap-c114-20261030`에서 Tailnet의 roadmap, STATUS, 계획, simulator README를 실제 렌더링해 확인했다.

- Roadmap에는 PR #811, merge SHA, 72/213 및 C11 미완료 한계가 보인다. [화면](tailnet-roadmap-c114-pr811.png)
- STATUS 본문은 72/213을 유지하고 C11.4 checkbox는 unchecked다. [화면](tailnet-status-c114-checkbox.png)
- C11 공식 항목의 상태 요약도 별도로 캡처했다. [화면](tailnet-status-c11.png)
- 계획 페이지에는 병합 SHA와 72/213이 있고 pending/unmerged 문구는 없다. [화면](tailnet-plan-c114.png)
- simulator README의 Android PNG는 1080×2400, iOS PNG는 1206×2622로 모두 `complete` 및 `naturalWidth > 0`이다. [화면](tailnet-evidence-screenshots-c114.png)
- URL, 본문 판정 및 image 속성은 `agent-browser-verification-pass02.json`에 기록했다. 브라우저를 닫은 뒤 `pgrep -fl agent-browser`는 결과가 비었다. [`browser-close-pass02.json`](browser-close-pass02.json)

## 범위 밖

Tailnet은 개발 미리보기로만 갱신했다. GitHub Pages나 공식 배포를 실행하지 않았다. 이 검증은 HTML 렌더링, 대상 route hash와 screenshot image 로드를 확인한 것이며 앱 runtime을 다시 실행하거나 전체 문서 사이트의 의미 정확성을 증명하지 않는다. 별도의 30개 실패 관점 검토는 [`tailnet-roadmap-sync-adversarial-review-30.md`](tailnet-roadmap-sync-adversarial-review-30.md)에 있다.
