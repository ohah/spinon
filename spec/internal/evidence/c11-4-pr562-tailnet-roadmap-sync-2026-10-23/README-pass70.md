# C11.4 PR #562 Tailnet roadmap sync · pass70

## PR과 source revision

- 구현 PR [#561](https://github.com/ohah/spinon/pull/561)은 `ff4233d0df00a632d71c28398c9a8274fb4abc16`로 병합됐다. simulator evidence source commit은 `d2b4826e4a76de5367e0671984d5e7b2c8c62ae0`이다.
- 상태·공개 roadmap PR [#562](https://github.com/ohah/spinon/pull/562)은 2026-10-23 10:57:28 UTC에 Rebase and merge되어 `22eafb6cad66b6be8facaf264299e7bd6073d0f5`에 병합됐다. `spec/STATUS.md`와 공개 roadmap은 C11.4 미완료·공식 `72/213`을 유지한다.
- PR #562 이후 공개 roadmap의 새 카드에 있던 내부 근거 경로를 기존 `/spinon/internal/...` route 형식에 맞춘 branch source commit은 `10ae04a09f9f0b1b97056693dad0438a2c022b62`이다. 이 경로 수정과 Tailnet sync 근거는 현재 docs PR에서 검토한다.
- 선택 사이트와 Tailnet sync manifest는 branch commit `10ae04a`의 public roadmap route를 담고 있으며, sync evidence를 기록한 branch commit은 `1d04fec9b174ed16a65021129134145684b01bcc`이다.
- 이 기록과 Tailnet roadmap 경로 수정은 PR [#563](https://github.com/ohah/spinon/pull/563)으로 제출했다.

## 선택 문서 빌드

- 이전 pass69 선택 root를 hardlink로 보존하고, `82f7514dc51bedd4160f27180145a13bc3df8750` 이후 main의 추적 문서 변경 429개를 새 pass70 root에 덮어썼다. 추가 경로 변경은 공개 roadmap 한 파일뿐이다. [선택 root manifest](./selected-root-preparation-pass70.json).
- 저장소에 고정된 RSPress 2.0.22로 선택 문서를 빌드했다. 종료 코드 0, 선택 root 20,500 files / 730,166,856 bytes, public assets를 hardlink한 뒤 결과 11,551 files / 657,473,256 bytes다. 로그의 표 형식 행 끝 공백은 검토 가능하도록 제거했다. `checkDeadLinks`와 `checkDeadImages`를 끈 선택 빌드이므로 저장소 전체 dead-link 검증으로 해석하지 않는다. [build summary](./selected-build-summary-pass70.json) · [build log](./selected-build-pass70.log).
- 필수 상태·plan·runtime route와 overflow 화면 경로를 빌드 결과에서 확인했다. 공개 roadmap HTML SHA-256은 branch source와 같고 새 카드의 로컬 링크가 모두 빌드 파일로 해석된다. [선택 사이트 검증](./selected-site-verification-final-pass70.json) · [roadmap route staging](./public-roadmap-staging-pass70.json).

## Tailnet 미리보기 동기화

- 기존 Tailnet preview server PID 1191과 `127.0.0.1:8768` listener를 유지했다.
- 동기화 전 대상은 62,768 files / 2,684,729,238 bytes였다. 빌드와 9,852개 동일 경로를 비교하고 새 파일 967개와 바뀐 파일 732개만 반영했다. 이전 파일 273,408,169 bytes는 같은 볼륨 hardlink로 backup했다. 교체는 unique sibling temporary file 작성 후 atomic rename으로 처리했고 삭제는 0건이다. [preflight 및 delta](./preflight-pass70.json) · [변경 경로 SHA-256](./delta-manifest-pass70.json) · [copy와 backup 결과](./copy-and-backup-pass70.json).
- 로컬 changed file 1,699개와 backup을 확인했다. 최종 대상은 63,735 files다. Tailnet HTTPS의 문서·이미지 route와 페이지에서 참조한 정적 asset 32개 모두 HTTP 200 및 빌드 source SHA-256과 일치한다. [served verification](./remote-verification-pass70.json).

## Agent Browser 확인

- `/spinon/roadmap.html`에서 PR #561 카드, merge SHA `ff4233d0…`, 공식 `72/213`을 확인했다. 체크박스 213개 중 72개가 checked이며 C11.4는 미완료다. [로드맵 DOM과 화면](./agent-browser-verification-pass70.json) · [카드 화면](./tailnet-roadmap-c114-pr561-agent-browser-pass70.png).
- 공식 상태 route의 `#css-구현-체크리스트` anchor가 존재하고 화면에 보이며, C11.4 checkbox는 unchecked다. status 화면과 본문에 `72/213` 및 #561 merge SHA가 있다. [상태 화면](./tailnet-status-c114-pass70.png).
- runtime README에서 16-scene 범위, evidence source commit과 pixel 비교 한계를 확인했다. Android API 37 ARM64 AVD PNG는 1080×2400, iOS 26.2 iPhone 17 Simulator PNG는 1206×2622로 Agent Browser에서 열렸다.
- 세션 `spinon-c11-4-zero-calc-plan-20261023`은 PR 및 Tailnet 확인 뒤 닫았다. Tailnet preview server는 그대로 유지했다. [browser close record](./browser-close-pass70.json).

## 범위

이번 변경은 Tailnet 개발 미리보기와 문서 route다. production 배포나 새 앱 runtime 검증이 아니다. 전체 C11.4는 미완료이며 공식 완료 수 `72/213`을 유지한다.
