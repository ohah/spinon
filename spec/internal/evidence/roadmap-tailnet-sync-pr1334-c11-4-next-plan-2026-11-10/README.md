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
- Tailnet desktop/mobile visual check, 최종 served hash 및 기존 server PID 확인은 post-merge sync 뒤 추가한다.

## 남은 한계

제품 구현은 시작 전이다. 이 로드맵 변경은 C11.4/C11 완료, CSS Grid 일반 지원, Spinon WPT 적합성, iOS/Android runtime 결과 또는 CI 성공을 주장하지 않는다. Tailnet 미리보기는 기존 사용 중인 server PID를 재사용하고 server를 재시작하지 않는다.
