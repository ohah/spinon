# PR #1179 상태·로드맵·Tailnet 동기화 diff 검토 30개

검토 대상은 `spec/public/roadmap.html`, `spec/STATUS.md`, `spec/internal/0059-c11-grid-layout.md`, `plan/c11-root-html-grid-auto-repeat.md`와 이 디렉터리의 Tailnet 동기화 근거다. 각 항목은 서로 다른 오류 경로를 확인했다. 문서 빌드·Agent Browser·route hash와 동기화 JSON을 대조했고 수정이 필요한 결함은 발견하지 못했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR 링크가 다른 저장소나 PR을 가리키는 경우 | `ohah/spinon/pull/1179`이며 제목·상태가 구현 PR과 일치한다. |
| 2 | merge SHA가 PR의 실제 merge commit과 다른 경우 | `7c042f4ac5080609927fb26760112c5a88a56d3c`로 PR API와 일치한다. |
| 3 | merge 시간이 부정확하거나 timezone이 빠지는 경우 | `2026-11-05T10:16:16Z`를 PR API와 대조했다. |
| 4 | PR이 병합 전/후 상태를 혼동하는 경우 | 로드맵·계획·상태 문구는 병합 상태로 맞췄다. |
| 5 | 공식 완료 수가 roadmap과 STATUS에서 달라지는 경우 | 두 곳 모두 `78/214`를 표시한다. |
| 6 | C11 상위 체크가 제한 slice 때문에 완료되는 경우 | STATUS의 C11 체크는 미완료다. |
| 7 | C11.1 완료를 과장하는 경우 | C11.1 미완료를 명시한다. |
| 8 | C11.4 완료를 과장하는 경우 | C11.4 미완료를 명시한다. |
| 9 | 제한 profile을 일반 CSS 지원으로 오해하는 경우 | 기본 parsed Root HTML route에 한정한다고 쓴다. |
| 10 | 반복 축 개수가 두 축으로 늘어나는 경우 | 한 축만 허용한다고 기록했다. |
| 11 | `auto-fill` 외 반복 keyword를 허용하는 것처럼 읽히는 경우 | `auto-fill`/`auto-fit`만 표시한다. |
| 12 | track minimum 또는 maximum grammar를 넓혀 읽는 경우 | `minmax(positive px, 1fr)`로 표시한다. |
| 13 | pattern track 수 경계를 놓치는 경우 | 1–2개 track으로 기록했다. |
| 14 | 반대 축도 반복될 수 있다고 오해하는 경우 | 반대 축은 fixed px track 하나라고 명시한다. |
| 15 | 미지원 item/box/placement 조합이 지원된다고 오해하는 경우 | 계획에 제한 밖 입력의 fail-closed 경계가 남아 있다. |
| 16 | UA 1px floor가 실제 track sizing에도 반영되는 경우 | 계획은 count-only floor와 실제 minimum을 구분한다. |
| 17 | expanded-track resource cap이 누락되는 경우 | 계획은 Taffy의 10,000-track 제한을 기록한다. |
| 18 | oracle 장면 수가 잘못 전달되는 경우 | Chrome 156 사전 비교 README의 13 scenes와 일치한다. |
| 19 | focused test 결과를 전체 suite 통과로 읽는 경우 | 9/9, 4/4, 3/3을 각각 명시하고 전체 적합성 주장은 하지 않는다. |
| 20 | 지원 밖 입력 26개가 성공 케이스로 집계되는 경우 | runtime README의 원자 거부 case로 적었다. |
| 21 | Android AVD를 실기기로 오인하는 경우 | Android 17/API 37 ARM64 AVD로 한정한다. |
| 22 | iOS Simulator를 실기기로 오인하는 경우 | iPhone 17 Pro/iOS 26.2 Simulator로 한정한다. |
| 23 | draw/present를 pixel parity로 확대 해석하는 경우 | PixelCopy·Metal readback·compositor-after pixels를 하지 않았다고 명시한다. |
| 24 | simulator evidence를 WPT 적합성으로 확대하는 경우 | WPT 확인이 아니라고 적었다. |
| 25 | 비어 있는 PR check rollup을 CI 성공으로 부르는 경우 | rollup과 required-check 부재를 적고 CI 통과라 기록하지 않는다. |
| 26 | 로드맵이 문서 내부 링크를 잘못 연결하는 경우 | 새 카드의 local href 7개가 publication manifest route에 있다. |
| 27 | PR 이미지/fixture 파일 경로가 누락되는 경우 | 카드 링크 대상 screenshot과 문서 route가 실제 파일·hash와 대응한다. |
| 28 | 모바일 줄바꿈으로 카드가 화면 밖으로 밀리는 경우 | Agent Browser 390×844에서 카드 폭 362px, 좌우 여백 14px, 가로 넘침 없음이다. |
| 29 | 선택 publication이 기존 시작·404 route를 덮어쓰거나 전체 사이트를 대체하는 경우 | `index.html`·`404.html`을 제외하고 기존 파일 삭제 없이 118개 route만 반영했다. |
| 30 | 열린 파일 덮어쓰기, backup 손상 또는 Tailnet 응답 불일치를 놓치는 경우 | preflight 열린 handle 0개, backup hash, local/HTTPS 118개 HTTP 200·SHA-256, 서버 PID 유지가 기록됐다. |

## 검증 경계

RSPress 선택 문서 build와 Agent Browser 화면 검증은 통과했다. 전체 문서 link/image sweep과 CI는 수행하지 않았으며, GitHub Pages는 배포하지 않았다. Tailnet preview server는 PID `28040`을 유지했다. Agent Browser 종료 후 default session inactive와 브라우저 프로세스 부재를 확인했다.
