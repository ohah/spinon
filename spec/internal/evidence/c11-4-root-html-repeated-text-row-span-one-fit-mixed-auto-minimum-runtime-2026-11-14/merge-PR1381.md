# C11.4 one-fit row-span mixed minimum 구현 PR #1381 병합 기록

- PR: [#1381](https://github.com/ohah/spinon/pull/1381), `feat(css): one-fit row-span item별 mixed 자동 최소 크기`
- 상태: Merged · Rebase and merge
- 병합 시각: `2026-11-14T00:42:53Z`
- PR base SHA: `929c4475517ec9a981deccffafa3af62c84238c1`
- PR source head: `886e1b586c405f061f9cc10946a2818e244086b8`
- main 반영 SHA: `416bd54519269851f07192bfc15a8c539154d866`
- PR source diff SHA-256: `2d6e4d1d2f4aa55f86604e61b6768845e26dfe7bd9f3e55ec1224420a4672f20`
- labels: `enhancement`, `area:layout`, `area:testing`
- GitHub workflow run과 status check는 각각 0건이었다. `main` 보호 설정에도 required status check가 없다. CI green으로 기록하지 않는다.

## 구현 범위와 검증

- 기본 Root HTML RuntimeSession mixed matrix: `120,960/120,960` scenes · `900/900` unique track-list batches · eligible automatic-minimum item occurrence `169,344`.
- 기존 endpoint/회귀: one-fit uniform positions 1–3 `2,880` pairs, positions 4–6 `4,320` pairs, two-fit row-span `3,840` scenes, one-fit auto-placement `1,440` scenes, two-fit auto-placement 및 repeated-text `810` corpus.
- Atomic rejection/recovery와 invalid-trace controls, layout `56/56`, projection `5/5`, focused RuntimeSession `1/1`, formatting과 변경 crate scoped Clippy를 통과했다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 light/dark 캡처는 고정 Root HTML route visual smoke다. 앱 matrix, 실기기, hardware GPU, surface readback 또는 pixel parity가 아니다.
- PR 본문의 iOS light `<img src>`에는 commit SHA 8자가 빠져 있었다. PNG와 `<a href>`는 정상이었고 본문 주소를 고친 뒤 Agent Browser에서 네 이미지 모두 로드됨을 확인했다: Android 각 `1080×2400`, iOS 각 `1206×2622`.

## 작업 리소스

- Android API 37 AVD와 iOS 26.2 Simulator는 종료했다. task staging directory `build/spinon/c11-4-one-fit-mixed-smoke`는 열려 있는 파일이 없음을 확인한 뒤 제거했다.
- PR 확인용 Agent Browser 세션을 종료했다. 재사용 가능한 빌드 캐시와 simulator profile은 남겨 두었다.
- 별도로 오래된 roadmap 확인용 로컬 서버 `127.0.0.1:8769` (PID `99970`)와 `:8770` (PID `5175`)는 11일간 실행 중이었고 established client가 없으며 backing generated directory도 존재하지 않아 종료했다. 선택된 Tailscale preview 서버 PID `90711` (`127.0.0.1:8768`)은 계속 사용 중이어서 유지했다.

공식 `78/214`, C11.4/C11 미완료, 내부 numeric contract `0.1.0`은 그대로다. 이 bounded slice는 일반 CSS Grid author admission이나 WPT 전체 적합성이 아니다.
