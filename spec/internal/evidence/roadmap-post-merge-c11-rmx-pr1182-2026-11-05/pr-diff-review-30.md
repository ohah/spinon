# PR #1182 병합 후 상태·공개 로드맵 diff 적대 검토 30개

**검토 대상:** `spec/STATUS.md`, 내부 계약 0059, C11.1 계획, `spec/public/roadmap.html`, Tailnet 동기화 증거. 구현 PR의 코드 diff 30개 검토와 별도다.

| # | 문서 diff에서 확인한 실패 경로 | 판정 |
|---:|---|---|
| 1 | 제한 slice가 C11.1/C11 전체 완료로 표시되는가 | 통과 — 상위 체크는 열려 있고 완료 수 `78/214`가 유지된다. |
| 2 | 공식 상태 대장의 PR 번호나 merge SHA가 틀리는가 | 통과 — PR #1182, `2026-11-05T12:25:28Z`, `8dab763943215e7fc7b5250a8eb22dddba5e811f`를 GitHub 병합 결과와 대조했다. |
| 3 | 내부 계약 숫자 버전이 제한 구현 때문에 부당하게 올라가는가 | 통과 — 기존 `0.1.0`을 유지한다. |
| 4 | 계획이 병합 뒤에도 구현 전/후보 상태로 남는가 | 통과 — `제한 구현 병합`과 실제 merge id를 적었다. |
| 5 | 상태 대장과 내부 계약의 지원 범위가 다른가 | 통과 — target 한 item, finite px `0..=70`, constrained `2 / 1` 경계가 일치한다. |
| 6 | public roadmap에 PR만 있고 commit state가 빠지는가 | 통과 — Rebase and merge 시각과 SHA를 함께 적었다. |
| 7 | public roadmap이 공식 완료 수를 slice 개수만큼 올리는가 | 통과 — `<code>78/214</code>`를 유지한다. |
| 8 | roadmap 변경이 C11.1/C11 체크박스를 완료로 표시하는가 | 통과 — 상위 incomplete state를 명시한다. |
| 9 | card가 C11.4 opt-in 증거를 기본 Root HTML 결과로 오인하는가 | 통과 — 기본 `RuntimeRootHtmlTextPaintV1` direct-text route와 fixture를 식별한다. |
| 10 | min/max 허용값 범위가 잘못 표기되는가 | 통과 — finite direct px `0..=70px`만 기록했다. |
| 11 | ratio fallback 범위가 임의 numeric ratio로 넓어지는가 | 통과 — constrained ratio는 computed·typed `2 / 1`로 제한한다. |
| 12 | owner 또는 첫 sibling도 제약 대상처럼 기술되는가 | 통과 — 두 번째 direct-text target 하나만 허용한다고 적었다. |
| 13 | percentage, math, intrinsic 값이 지원 범위에 들어가는가 | 통과 — 이 값들은 계속 fail-closed라고 명시한다. |
| 14 | Chrome oracle scene 수가 과장되거나 변경되는가 | 통과 — 10-scene reference와 독립 matrix만 기록한다. |
| 15 | Android result가 실기기/hardware GPU로 표현되는가 | 통과 — API 37 ARM64 AVD와 ANGLE/SwiftShader software renderer다. |
| 16 | iOS Metal drawable이 compositor 이후 화면으로 표현되는가 | 통과 — compositor 제출 전 표본이라고 적었다. |
| 17 | PixelCopy/Metal readback 정확도 또는 scene이 다르게 기술되는가 | 통과 — overflow 표본 플랫폼별 3/3 exact를 runtime evidence와 대조했다. |
| 18 | build-off 결과를 simulator runtime test로 오해하는가 | 통과 — 문서가 앱 feature-off build와 pixel runtime을 별도 구분한다. |
| 19 | Tailnet host/backend가 오프라인인데 복사를 진행하는가 | 통과 — preflight에서 Tailscale `Running`, local preview HTTP 200을 확인했다. |
| 20 | 기존 preview process를 임의 재시작하는가 | 통과 — PID 28040을 유지했고 restart 기록은 false다. |
| 21 | 교체 중인 파일에 open handle이 있는데 덮어쓰는가 | 통과 — preflight `openHandlePids`는 빈 목록이다. |
| 22 | source와 destination이 다른 roadmap revision인가 | 통과 — source/target/local/remote bytes와 hash를 함께 비교했다. |
| 23 | 이전 preview 파일을 backup 없이 덮어쓰는가 | 통과 — 이전 roadmap을 고유 백업 경로에 먼저 복사했다. |
| 24 | 과거 backup이나 목적지 이름을 덮어쓰는가 | 통과 — 새 PR 전용 `pass01` 경로이며 스크립트는 기존 파일이면 중단한다. |
| 25 | 불필요한 파일 삭제가 동기화에 포함되는가 | 통과 — `filesToReplace`는 roadmap 하나이고 `noDeletes=true`다. |
| 26 | 파일 복사가 반쯤 쓰인 상태를 제공하는가 | 통과 — 임시 파일 작성 후 atomic rename을 사용했다. |
| 27 | local HTTP 또는 Tailnet HTTPS가 stale HTML을 주는가 | 통과 — 둘 다 200, 574409 bytes, source SHA와 일치한다. |
| 28 | desktop card가 화면 밖인데 표시 성공으로 기록되는가 | 통과 — Agent Browser DOM card rect는 1440px viewport 안에 있다. |
| 29 | mobile 페이지나 card가 가로로 넘치는가 | 통과 — viewport/document/body 390px, card 362px이다. |
| 30 | screenshot이 다른 화면을 보여 주거나 Agent Browser가 남는가 | 통과 — DOM/viewport와 같은 session에서 card screenshot을 저장했고 browser session close를 기록했다. |

**결과:** 이 30개 문서/배포 경계에서 수정 결함은 발견되지 않았다. Agent Browser는 roadmap 렌더링·반응형 폭만 검증하며 앱 동작, 전체 CSS Grid 적합성 또는 workspace tests를 증명하지 않는다.
