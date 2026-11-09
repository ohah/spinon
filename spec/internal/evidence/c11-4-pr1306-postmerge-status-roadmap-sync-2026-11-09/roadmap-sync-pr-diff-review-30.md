# PR #1306 병합 후 상태·명세·로드맵 diff 검토 30개

대상은 `spec/STATUS.md`, `spec/internal/0059-c11-grid-layout.md`, C11.4 집중 계획, 공개 `spec/public/roadmap.html`이다. 각 항목은 별도의 상태·범위·근거·렌더링 실패 경로를 검토한다.

| # | 실패 관점 | 결과 |
|---|---|---|
| 1 | C11.4 공식 체크를 bounded PR 병합만으로 완료 처리한다. | PASS — C11.4 checkbox가 미체크다. |
| 2 | 상위 C11 체크를 함께 완료 처리한다. | PASS — 상위 checkbox를 변경하지 않았다. |
| 3 | 제한 slice를 공식 완료 수에 더한다. | PASS — `78/214`를 유지한다. |
| 4 | 내부 숫자 계약 버전을 구현 범위 증가로 올린다. | PASS — `0.1.0`을 유지한다. |
| 5 | PR 링크와 merge SHA가 서로 다른 변경을 가리킨다. | PASS — 모든 문서가 PR #1306과 `884b07962375566350476c565f2456af1d65e922`를 기록한다. |
| 6 | 병합 시각이 시간대 없이 다른 시각으로 해석된다. | PASS — UTC 표기 `2026-11-09T09:30:44Z`를 사용한다. |
| 7 | 링크가 잘못된 PR 번호나 PR 상태를 표시한다. | PASS — GitHub PR #1306의 병합 정보와 일치한다. |
| 8 | 위치 4–6 지원을 모든 `fit-content()` 위치에 일반화한다. | PASS — 세 authored 위치 4·5·6으로 제한한다. |
| 9 | track list의 member multiplicity를 바꿔 적는다. | PASS — `min-content ×2`, `max-content ×2`, `auto ×1`를 유지한다. |
| 10 | arbitrary row-span topology를 지원한다고 읽힌다. | PASS — 기존 두 여섯 implicit-row topology만 허용한다고 명시한다. |
| 11 | percentage cap basis를 누락하거나 다른 기준으로 적는다. | PASS — definite 220px block size를 계약에 남겼다. |
| 12 | Chrome/RuntimeSession scene 분모를 과장한다. | PASS — 4,320 scene 및 각 회귀 분모를 분리해 기록한다. |
| 13 | 이전 row-span 위치 1–3 회귀가 누락된다. | PASS — 1,920-scene 회귀를 기록한다. |
| 14 | auto-placement 위치 1–6 회귀가 누락된다. | PASS — 1,440-scene 회귀를 기록한다. |
| 15 | iOS 화면 smoke를 전체 native parity로 부풀린다. | PASS — iPhone 17 Pro/iOS 26.2 여섯 장면과 visual-smoke 한계를 적었다. |
| 16 | Android 화면 smoke의 API/기기를 혼동한다. | PASS — API 37.2 ARM64 AVD 여섯 장면으로 표기한다. |
| 17 | screenshot을 PixelCopy·Metal pixel-readback으로 주장한다. | PASS — 두 종류의 readback과 native 전체 matrix가 아님을 명시한다. |
| 18 | 전체 layout crate 결과를 전부 통과로 기록한다. | PASS — `506 pass / 1 C15 fail`로 정확히 적었다. |
| 19 | baseline 단일 실패 재현을 전체 baseline-suite 통과로 오해한다. | PASS — PR base의 동일 filtered assertion만 재현했음을 밝힌다. |
| 20 | PR check 부재를 CI green으로 표현한다. | PASS — check rollup 미보고 및 CI green 주장을 하지 않는다고 적었다. |
| 21 | 공개 API 또는 FFI가 추가된 것처럼 읽힌다. | PASS — 기존 bounded 내부 구현이며 API/FFI 확장이 없음을 유지한다. |
| 22 | 내부 계약 링크가 다른 Grid 명세를 연다. | PASS — 0059 링크를 연결한다. |
| 23 | 집중 계획 상태가 아직 병합 전이라고 남는다. | PASS — 상태 줄이 실제 merge timestamp와 SHA를 표시한다. |
| 24 | 계획 이력이 현재 구현 전 상태처럼 보인다. | PASS — 계획 선택·검토는 계획 당시의 이력으로 표현한다. |
| 25 | 공식 상태 대장에서 최신 slice가 누락되거나 과거 항목을 덮어쓴다. | PASS — 기존 C11.4 이력을 보존하고 #1306을 별도 최신 항목으로 추가한다. |
| 26 | 공개 roadmap 최신 카드가 PR #1302에서 멈춘다. | PASS — #1306 카드를 #1302 카드 뒤에 추가한다. |
| 27 | 칸반 JavaScript가 이미 병합된 #1306을 NEXT로 다시 표시한다. | PASS — `detailNext`를 비운다. |
| 28 | 정적 다음 계획 문구가 완료된 #1306을 가리킨다. | PASS — 후속 bounded slice 미선정으로 갱신한다. |
| 29 | screenshot 링크가 변동 가능한 branch 대신 틀린 SHA를 사용한다. | PASS — merge SHA 경로를 사용하고 해당 화면 파일이 병합 tree에 있음을 확인했다. |
| 30 | 문서 전체 빌드 실패를 성공으로 보고한다. | PASS — Rspress build의 기존 evidence dead-link 오류를 별도로 남기며 build 성공을 주장하지 않는다. |

전체 문서 빌드 오류는 여러 기존 evidence Markdown의 `.log`, `.json`, 소스 파일 상대 링크에서 발생했다. 이 변경에서는 광범위한 기존 링크 정리를 하지 않았으며, Tailnet 정적 로드맵 파일과 선택 링크는 Agent Browser로 별도 확인한다.
