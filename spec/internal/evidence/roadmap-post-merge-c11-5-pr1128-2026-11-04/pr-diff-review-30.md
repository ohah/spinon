# C11.5 병합 후 로드맵 동기화 PR 변경분 적대 검토 30개

PR #1129의 상태·계획·내부 계약·공개 로드맵·Tailnet 근거를 서로 다른 실패 관점으로 대조했다. 이 검토는 구현 PR #1128의 코드 검토를 재사용하지 않는다.

| # | 실패 관점 | 판정과 확인 근거 |
|---:|---|---|
| 1 | 로드맵에서 구현 PR과 문서 PR을 혼동할 수 있는가 | **통과** — 로드맵 카드와 상태 대장은 구현 PR #1128을 연결하고, 이 문서 PR은 #1129로 구분했다. GitHub PR #1129 제목·head/base도 확인했다. |
| 2 | 병합 시각이 문서마다 달라지는가 | **통과** — `2026-11-03T17:52:20Z`가 상태 대장·계획·내부 계약·로드맵의 기록과 GitHub #1128 결과에 일치한다. |
| 3 | merge SHA가 틀리거나 축약되어 추적을 잃는가 | **통과** — `0f704151cfd805ad2fef2e12535739f0d3a109dd`가 상태 대장·계획·내부 계약·로드맵에 동일하게 기록됐다. |
| 4 | 비어 있는 check rollup을 CI 성공으로 읽는가 | **통과** — GitHub #1128 조회에서 `statusCheckRollup: []`; 문서는 “비어 있음”으로 기록하고 성공 주장을 하지 않는다. |
| 5 | required check 부재를 통과한 CI로 오인하는가 | **통과** — 기존 상태 근거는 main의 required status check 부재와 실행된 check 부재를 구분한다. 문서도 두 사실을 CI 통과로 합치지 않는다. |
| 6 | 제한 병합 때문에 전체 완료 수가 증가하는가 | **통과** — 변경 전후 공식 완료 `72/213`을 유지한다고 STATUS와 공개 로드맵이 기록한다. |
| 7 | 상위 C11이 실수로 체크되는가 | **통과** — `spec/STATUS.md`의 C11 CSS Grid는 unchecked이고 변경 diff에서도 상태 체크를 바꾸지 않았다. |
| 8 | C11.5의 한 slice가 C11.5 전체로 승격되는가 | **통과** — 제한 구현 병합과 전체 C11.5 미완료를 모든 사용자 노출 기록에서 함께 명시한다. |
| 9 | Grid eligibility를 일반 Grid 입력으로 넓혀 읽는가 | **통과** — eligible fixed-track Grid, direct empty `div`, 고정 크기 item의 제한을 계획과 내부 계약에 유지한다. |
| 10 | 구현된 속성이 과장되는가 | **통과** — 범위를 computed `align-self`·`justify-self` 위치 정렬로 한정하고 `place-self` computed longhand 경계를 내부 계약과 일치시켰다. |
| 11 | 일반 CSS author admission을 지원한다고 암시하는가 | **통과** — 일반 CSS admission 미완료를 공개 로드맵과 계획에 남겼다. |
| 12 | Chrome oracle을 Spinon WPT 적합성으로 오인하는가 | **통과** — Chrome 156 19/19를 사전 비교로 표현하고 Spinon WPT 적합성 미완료를 명시한다. |
| 13 | Android AVD 결과를 실기기·hardware GPU로 읽는가 | **통과** — API 37 ARM64 AVD/SwiftShader로 표기하고 실기기·hardware GPU 완료 주장을 배제했다. |
| 14 | iOS Metal 표본을 compositor 이후 화면 결과로 읽는가 | **통과** — Metal drawable 7/7은 compositor 제출 전이라고 기록한다. |
| 15 | Chrome scene 수를 앱 플랫폼 결과와 합산하는가 | **통과** — Chrome 156 19 scene 19/19를 별도 사전 비교 항목으로 기록한다. |
| 16 | Android direct surface 표본과 screenshot을 혼동하는가 | **통과** — Android SurfaceView PixelCopy 7/7과 OS 합성 PNG를 별도 근거로 구분했다. |
| 17 | iOS drawable 표본의 표면 경계를 누락하는가 | **통과** — iOS Metal drawable 표본 7/7 및 compositor 전 경계를 계획·실행 근거·로드맵에서 유지한다. |
| 18 | fixture payload가 기본 artifact에 포함되는가 | **통과** — Android default DEX와 iOS default executable의 제외, opt-in positive artifact의 포함을 대조해 기록했다. |
| 19 | `spinon-layout`의 실패를 감추는가 | **통과** — `423 passed/1 failed`와 C15 Grid text error-kind assertion을 기록했다. |
| 20 | `spinon-style-to-layout`의 실패를 감추는가 | **통과** — `372 passed/1 failed`와 C07 aspect-ratio serialization 비교 실패를 기록했다. |
| 21 | `spinon-style`의 실패를 감추는가 | **통과** — `283 passed/1 failed`와 다른 Grid profile의 typed track-list assertion을 기록했다. |
| 22 | `spinon-runtime` 실패·ignored 수를 누락하는가 | **통과** — `261 passed/2 failed/3 ignored` 및 두 rejection-code assertion의 기존 dense auto-placement 경계를 기록했다. |
| 23 | 상태·계획·내부 계약·로드맵이 서로 모순되는가 | **통과** — PR #1128 결과, bounded scope, package counts, 미완료 경계를 네 문서 간 대조했다. |
| 24 | 제한 구현 때문에 내부 계약 숫자 버전이 바뀌는가 | **통과** — 내부 계약 버전은 `0.1.0` 그대로라고 기록했다. |
| 25 | 최신 공지가 이전 로드맵 상태를 덮어쓰는가 | **통과** — C11.5 공지를 최신 hero 위치에 추가하고 기존 roadmap 체크리스트 본문을 변경하지 않았다. |
| 26 | Tailnet에서 없는 내부 경로를 열도록 링크하는가 | **통과** — 새 카드 링크는 GitHub `main`의 문서 경로를 사용하도록 수정했고 Agent Browser DOM에서 각 href를 확인했다. |
| 27 | 링크 대상 파일이 main에 아직 없는가 | **통과** — plan, contract, precomparison, simulator evidence, implementation review, PR review, STATUS 대상이 `origin/main`에 존재한다. |
| 28 | 데스크톱 화면에서 카드가 viewport 밖으로 밀리는가 | **통과** — 로컬 1280px viewport는 문서 1280px/카드 908px, Tailnet 1440px viewport는 문서 1440px/카드 928px로 Agent Browser JSON에 기록됐다. |
| 29 | 모바일에서 가로 overflow가 생기는가 | **통과** — 로컬·Tailnet 390px viewport에서 body/document 390px, 카드 362px이며 가로 overflow가 없다. |
| 30 | Tailnet 복사본이 stale이거나 원본을 파괴하는가 | **통과** — pass02 source/local/remote가 550,857 bytes 및 SHA-256 `e953474d1b547abb8d2a8c82ccdabb7fab2e3b3bd117955d126ef2c5812c2c37`로 일치한다. pass01 파일 백업, atomic rename, no-delete, PID 28040 무재시작이 JSON 근거에 있다. |

## 판정 한계

문서 diff whitespace와 evidence checksum을 검사했고, Agent Browser로 로컬·Tailnet 표시를 확인했다. 이 문서 PR에 대해 앱 테스트나 C11 구현 테스트를 새로 실행하지 않았다. 화면 검증은 로드맵 렌더링·반응형 폭에 한하며 앱 동작, 전체 Grid 호환성 또는 package tests의 통과를 증명하지 않는다.
