# PR #1334 다음 C11.4 slice 로드맵 동기화

PR [#1334](https://github.com/ohah/spinon/pull/1334)에서 계획만 병합된 다음 C11.4 slice를 계획 완료·제품 구현 미착수 상태로 표시한다. 칸반의 `할 일` 열에 세 fit row-span 계획을 다음 작업으로 추가하고, 계획 문서의 `PR 준비` 상태를 병합 완료로 바로잡는다. 구현 완료 수·C11.4/C11 checkbox·internal contract version은 바꾸지 않는다.

## 변경 범위

- `plan/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum.md`: #1334 병합과 구현 미착수 상태를 기록한다.
- `spec/STATUS.md`: #1334 병합 시각, 다음 작업 선택, 구현 미착수 상태를 기록한다. 완료 수 `78/214`, C11.4/C11 미완료, contract `0.1.0`을 유지한다.
- `spec/public/roadmap.html`: 계획을 다음 작업 칸반에 표시하고 plan/oracle 링크를 제공한다. `진행 중` 열에는 넣지 않는다.

## 확인 근거

- PR #1334 merge SHA `50678c278dc235bfb09ad2bf24e4ef174a46358a`, merge time `2026-11-10T11:45:04Z`.
- Chrome 157 precomparison: 11,520 scenes, 5,760 paired keys, 24,576 eligible occurrences; content-floor violation, source-order delta, non-auto stretch delta 모두 0. 이는 계획 입력의 browser oracle이며 제품 RuntimeSession 결과가 아니다.
- 문서 수정 후 `git diff --check`와 고정 문자열·공식 집계 검사를 기록한다.
- Local Agent Browser 0.31.1에서 1440×1000과 390×844를 확인했다. 칸반은 한 번만 제목을 표시하고, `세 fit row-span`은 `할 일` 카드로 나오며 WIP는 `0/2`다. 양쪽에서 공식 상태 동기화 메시지는 `78/214`를 표시한다. 모바일 문서 폭은 viewport 390px이며 1120px 칸반은 컨테이너 내부에서 가로 스크롤한다.
- [데스크톱 화면](./roadmap-agent-browser-desktop.png) (1440×1000, SHA-256 `b4fdae5647eb55704f09337aed5ff2086c1a6314c3ce1b9ba74f0fc1fc1e1909`), [모바일 화면](./roadmap-agent-browser-mobile.png) (390×844, SHA-256 `cb4475248d27b2650978d3011c039b13ad861dae7552ce5bf981b7a3565223b6`). Local HTML smoke server는 확인 뒤 종료했고 Agent Browser session 목록도 비었다.
- Local HTML smoke server는 `spec/public`만 제공했기 때문에 `/spinon/icon.svg` favicon 요청은 404였다. Tailnet full-site publication에서 해당 경로가 정상인지 별도로 확인한다.
- [Tailnet 데스크톱 화면](./tailnet-roadmap-agent-browser-desktop.png) (1440×1000, SHA-256 `e2ec0e6f29bec163f58f977681fe80ba24cfdc94c94137a36a018f896362209f`), [모바일 화면](./tailnet-roadmap-agent-browser-mobile.png) (390×844, SHA-256 `d3e6810a6b2bb38673c856eedcc6bbd4727a320cc2c84b72a2310a790aec1de8`). 두 viewport에서 next card와 공식 `78/214`를 확인했고, WIP는 `0/2`였다. 모바일 문서/body 폭은 390px, 칸반만 컨테이너 내부에서 가로 스크롤한다. heading은 한 개다.

## Tailnet 제공 확인

- source commit `83e5bdd7c440d6049d614e8ee3a9194c83f206ac`, tree `13af91e245fecebd077eee2e3b60da59673322eb`에서 RSPress 2.0.22 선택 build를 만들었다. Selected root는 182 files / 90,244,678 bytes, publication은 263 routes / 103,888,398 bytes다.
- Tailscale backend는 `Running`이었다. `127.0.0.1:8768`의 기존 preview server PID `90711`와 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html` route HTTP 200을 확인했다. 서버는 재시작하지 않았다.
- 사전 점검에서 70 route create, 29 replace, 164 identical이었고 교체 route open handle은 0이었다. 기존 29개 파일은 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1334-c11-4-next-plan-2026-11-10/pass01`에 복사했다.
- 동기화 뒤 263/263 local route hash, HTTPS 263/263 HTTP 200/hash, rollback backup 29/29 hash가 일치했다. 파일 삭제는 없었다. 로드맵 route는 662,675 bytes, SHA-256 `1c89b4d26c71b9ec3d3609031e91f2ca985ed03118bd0eded3b550dc97c682e0`다.
- Agent Browser 0.31.1 세션은 캡처 뒤 닫았고 `session list --json`은 빈 배열이었다. 이번 확인은 Tailnet 문서 화면과 정적 route만 대상으로 했으며, 전체 문서 dead-link 검사, 앱 실행, CI, 제품 runtime 검증을 뜻하지 않는다.
- 검증 후 이번 작업에서 생성한 `build/spinon/roadmap-tailnet-sync-pr1334-c11-4-next-plan-2026-11-10`만 `lsof` open handle 0 및 관련 build process 종료를 확인한 뒤 제거했다. 다른 build 디렉터리와 기존 rollback backup은 보존했다.

## 남은 한계

제품 구현은 시작 전이다. 이 로드맵 변경은 C11.4/C11 완료, CSS Grid 일반 지원, Spinon WPT 적합성, iOS/Android runtime 결과 또는 CI 성공을 주장하지 않는다. Tailnet 미리보기는 기존 사용 중인 server PID를 재사용하고 server를 재시작하지 않았다.
