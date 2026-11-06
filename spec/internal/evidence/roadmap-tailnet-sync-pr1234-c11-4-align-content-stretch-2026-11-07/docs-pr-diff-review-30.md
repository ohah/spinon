# PR #1234 merge 상태·로드맵 동기화 PR diff 검토 30개

검토 대상은 `spec/STATUS.md`, `spec/public/roadmap.html`, C11.4 plan 후속 결과, Tailnet sync·browser evidence다. 코드 완료 범위와 roadmap 공개 문구를 일치시키고, user preview의 side effect와 기존 파일 보존 경계를 별도로 확인했다.

| # | 독립 실패 경로 | 판정 |
|---:|---|---|
| 1 | 잘못된 구현 PR 번호 기록 | `#1234`와 GitHub merged state를 대조했다. |
| 2 | rebase merge SHA 누락/오타 | 대장·roadmap 모두 `0983a94cc80424b0f00f0e401286982125573e48`다. |
| 3 | merge 시각 오기 | GitHub merged timestamp `2026-11-06T19:04:55Z`를 썼다. |
| 4 | 제한 slice가 C11.4 전체 완료로 승격 | `C11.4`와 `C11` checkbox는 미완료다. |
| 5 | 공식 count 증가/불일치 | 모든 update가 `78/214`를 유지한다. |
| 6 | internal contract version bump | `0.1.0`을 그대로 둔다. |
| 7 | 범위를 일반 CSS support로 오해 | `RootHtmlAutoRowStretchV1` fixed shape와 computed `normal|stretch`만 명시한다. |
| 8 | row-size oracle 변경 | Chrome/독립/Rust에서 `[10,15,15]px`를 기록한다. |
| 9 | frame/source ordering 누락 | fixed item과 `A`/`B` direct item frames를 문서화한다. |
| 10 | unsupported `align-content` 누락 | non-candidate values와 altered shape fail-closed를 기록한다. |
| 11 | legacy intrinsic profile regression 누락 | legacy auto-row rejection이 유지됨을 기록한다. |
| 12 | focused test 결과 과장 | 8/8과 실제 명령/근거 문서 링크를 연결한다. |
| 13 | all-target Clippy 결과 과장 | workspace all-target 경고 금지 Clippy 성공을 정확히 적는다. |
| 14 | release artifact fixture boundary 누락 | default release marker absence와 fixture-test positive control을 구분한다. |
| 15 | simulator를 실기기라고 표기 | Android API 37 ARM64 AVD와 iPhone 17 Pro Simulator를 명시한다. |
| 16 | Android render backend parity 과장 | Screenshot은 visual smoke로만 보고하며 pixel parity를 주장하지 않는다. |
| 17 | iOS compositor/readback 과장 | drawable 이후 surface readback 근거가 없음을 유지한다. |
| 18 | iOS build warning 숨김 | 기존 `RawWindowMetalLayer` duplicate-symbol linker warning을 남긴다. |
| 19 | CI green 주장 | PR workflow run/status가 없음을 적고 통과로 주장하지 않는다. |
| 20 | code/docs link drift | GitHub links, repo-relative links와 merge SHA를 대조했다. |
| 21 | roadmap card 시간순서 역전 | 화면·DOM에서 `#1228 → #1231 → #1234` 순서를 확인했다. |
| 22 | 중복 PR #1234 card | static source에 카드가 정확히 한 번 추가됐고 HTML parser가 예외 없이 처리했다. |
| 23 | public count marker 제거 | roadmap source와 Tailnet response에 `78/214`가 유지된다. |
| 24 | mobile card viewport overflow | 390px document/body, x=14, right=376이며 horizontal overflow가 없다. |
| 25 | desktop card overflow | 1440px document/body, x=90, right=1018로 viewport 안이다. |
| 26 | 잘못된 제공본 교체 대상 | `/spinon/roadmap.html` preview file 한 개만 바꾼다. |
| 27 | 변경 전 제공본 소실 | 이전 605,373-byte file과 hash를 별도 backup에 보존했다. |
| 28 | copy 중 부분 파일 제공 | same-directory temporary file 후 atomic rename을 사용한다. |
| 29 | Tailscale server interruption | Tailscale Running 및 PID 28040/8768을 대조했고 restart하지 않았다. |
| 30 | user 요청 밖 배포·browser 잔류 | GitHub Pages를 실행하지 않았다. Agent Browser session/process를 종료했고 server는 유지했다. |

최종 검토에서 공식 상태 숫자·체크, merge identity, 제공 파일 hash와 browser viewport 기록의 불일치는 없다. [로컬/Tailnet 200 응답](./served-verification.json)과 [Agent Browser cleanup](./agent-browser-lifecycle.json)을 연결한다.
