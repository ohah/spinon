# C14.1 live V8 PR 변경점 적대 검토 · 30개 관점

- PR: [#213](https://github.com/ohah/spinon/pull/213)
- 검토일: 2026-10-16
- 검토 대상: C14.1 live V8 fixture diff와 Android/iOS 실행 근거, PR 화면 이미지 및 남은 범위
- 판정: 발견한 보고 필드 의미 불일치를 수정하고 영향받은 Rust 검사를 다시 실행했다. 미해결 P1/P2 없음.

| # | 독립 실패 관점 | 대조·판정 |
|---:|---|---|
| 1 | C14.1 변경에 무관한 작업 트리 파일이 함께 들어오는가 | 격리 worktree의 PR diff 26개 파일을 확인했다. 변경은 fixture/runtime 연결, C14.1 계획·상태·플랫폼 문서와 실행 근거에 한정된다. 통과. |
| 2 | 내부 검증 fixture가 공개 CSS/API 완료로 서술되는가 | 공개 API 추가는 없고 C14.1·C14 상태 체크가 미완료다. 문서와 PR 본문도 product loader·image paint·overflow pixel을 제외한다. 통과. |
| 3 | 새 runtime 메서드가 기본 build에서 노출되는가 | 메서드, command variant, actor arm과 scheduler drain arm 모두 `c14-1-live-v8-fixture-test`로 묶었다. 기본 앱의 native symbol 부재와 opt-in positive control을 각각 확인했다. 통과. |
| 4 | FFI feature가 runtime feature와 잘못 연결되거나 기본 의존성을 끌어오는가 | `c14-1-app-fixture-test`가 전용 runtime feature와 필요한 optional dependency만 활성화한다. Android/iOS 기본·opt-in artifact 경계 자료와 Cargo feature 선언을 대조했다. 통과. |
| 5 | actor 밖 스레드가 mutable HostDocument를 직접 읽는가 | 요청은 scheduler queue를 거치고 실제 복사는 actor arm에서 `callbacks.document.snapshot()`으로 수행한다. 양 플랫폼 app route에서 script 뒤 snapshot을 받았다. 통과. |
| 6 | eval 응답보다 snapshot 요청이 먼저 처리되어 빈 문서를 읽는가 | fixture가 `RuntimeSession::eval` 성공 응답을 확인한 뒤 snapshot 요청을 보낸다. 두 플랫폼 report는 generation 1, document revision 8, render-tree revision 2를 기록했다. 통과. |
| 7 | snapshot submit과 session close가 교차해 종료 뒤 요청이 접수되는가 | submission lock 안에서 closing을 읽고 enqueue한다. actor 전용 테스트는 shutdown 뒤 요청이 오류가 되는지 확인한다. 통과. |
| 8 | queue가 가득 차거나 중지됐을 때 빈 snapshot을 성공으로 반환하는가 | `Full`과 `Stopped`를 별도 `Err`로 돌린다. 포화 분기는 코드로 확인했고 앱 성공 report에 빈 snapshot을 사용하지 않는다. 통과. |
| 9 | shutdown이 pending snapshot 응답을 버려 caller가 무한 대기하는가 | `reject_pending`은 fixture 명령에 오류 응답을 보낸다. shutdown 뒤 호출 거부 focused test 1/1 통과. 통과. |
| 10 | actor reply 채널이 끊긴 경우 성공으로 해석되는가 | `recv_timeout`의 `Disconnected`를 오류로 반환한다. 반환형이 `Result<HostDocumentSnapshot, String>`이라 실패 snapshot은 없다. 통과. |
| 11 | 정체된 actor가 호출자를 무한 대기시키거나 timeout 뒤 문서를 바꾸는가 | 대기는 5초로 제한한다. 늦게 실행돼도 queued command는 snapshot 복사만 하며 DOM 변경을 하지 않는다. 통과. |
| 12 | 요청 시점과 실제 actor 처리 시점이 다르다는 계약이 감춰져 있는가 | 문서 주석과 계획은 actor 처리 시점의 최신 snapshot이라고 경계를 적는다. document/source revision을 report에 남긴다. 통과. |
| 13 | 고정 JS fixture가 비동기 작업 또는 외부 network 성공에 기대는가 | JS 입력은 동기 DOM 생성·속성 변경만 수행한다. `.invalid` URL을 실제 fetch하지 않고 고정 PNG bytes를 로컬 decode한다. 통과. |
| 14 | root가 없거나 여럿일 때 임의 root를 선택하는가 | root child 수가 정확히 하나인지 확인하고 HTML `div#root`인지 검증한다. 실패 시 구체 오류로 중단한다. 통과. |
| 15 | image가 누락·복수·비 HTML·detached 상태여도 대상으로 사용하는가 | root child 수, HTML namespace, `img` tag, `id=image`, document 연결 상태를 확인한다. 다른 구조를 성공으로 바꾸지 않는다. 통과. |
| 16 | 고정 PNG가 문서의 다른 src에 잘못 연결되는가 | `ImageRequest::from_document` 다음 exact fixture URL을 비교하고 불일치 시 decode 전에 오류를 낸다. 통과. |
| 17 | reference PNG가 같은 길이의 다른 bytes로 교체돼도 oracle을 통과하는가 | reference id·파일명·byte length·SHA-256을 확인한다. 바뀐 digest negative unit test 포함 2개 FFI focused test가 통과했다. 통과. |
| 18 | decoder 실패 또는 resource map 누락이 자연 크기 0으로 감춰지는가 | decode와 `ImageResourceSnapshot` 생성 오류를 전파하고 image NodeId가 map에 없으면 실패한다. 통과. |
| 19 | request/resource/layout이 서로 다른 문서 source stamp를 섞는가 | request source와 resource source, intrinsic source와 resource source를 비교하고 layout에는 같은 immutable document snapshot을 넘긴다. 두 simulator report에서 source/revision 결과가 일치한다. 통과. |
| 20 | src 교체 뒤 이전 completion이 새 문서에 승인되는가 | live V8 global이 가리키는 같은 HTML image의 src를 바꾸고 revision 8→9를 확인한 뒤 이전 completion을 현재 snapshot에 제출한다. 통과. |
| 21 | stale negative control이 엉뚱한 오류를 성공으로 세는가 | `SourceChanged` variant와 대상 `NodeId`를 모두 맞춰야 성공한다. 그 외 오류와 admission 성공은 별도 실패다. 통과. |
| 22 | `intrinsic_source_matches`가 request source 불일치까지 혼합해 잘못 보고하는가 | 검토 중 필드가 두 비교의 conjunction을 보고하는 것을 발견했다. intrinsic/resource 비교를 독립 bool로 분리하고 전체 gate만 두 bool의 conjunction으로 유지했다. 아래 재검증 결과 참조. 수정 후 통과. |
| 23 | style-layout이 새 snapshot 또는 다른 root를 사용해 image frame을 찾는가 | `StyloDocumentView`와 Taffy adapter에 검증한 동일 document snapshot과 root를 전달하고, 결과 frame은 image NodeId로 조회한다. 통과. |
| 24 | Chrome 비교의 stylesheet를 V8이 파싱했다고 과장하는가 | 기존 fixture의 author CSS를 별도 고정 stylesheet로 사용한다. 계획은 V8 `<style>` parsing이 범위 밖임을 기록한다. 통과. |
| 25 | Android density 또는 iOS scale이 CSS px 값에 중복 적용되는가 | viewport는 CSS px 및 scale 1로 고정한다. AVD와 Simulator report에서 `24×16`과 `(0,0) 24×16` CSS px가 같다. 통과. |
| 26 | 잘못되거나 무한한 frame 값이 tolerance 비교에 통과하는가 | 각 실제 frame 값은 finite여야 하고 네 좌표·크기 필드별 차이가 `0.5 CSS px` 이하인지 비교한다. 자연 크기는 고정 oracle와 정확히 비교한다. 통과. |
| 27 | 조건 일부가 실패해도 report의 `status`가 `ok`가 되는가 | source·revision·frame·natural size·stale rejection을 모두 `matches`에 결합한다. PNG metadata 오류는 report 생성 전에 중단된다. 통과. |
| 28 | C ABI null/zero/짧은 output buffer 오류가 성공 결과를 노출하는가 | null·zero는 즉시 거부하고 report writer 실패는 `-3`으로 반환한다. 현재 host E2E harness에 플랫폼 V8 provider가 없어 짧은 버퍼 동적 실행은 하지 않았고, Android/iOS의 실제 C ABI route는 정상 용량으로 실행했다. 미실행 경계를 명시했다. |
| 29 | PR 화면이 변경된 screenshot 경로를 실제로 읽지 못하는가 | Agent Browser에서 PR #213을 열어 두 raw 이미지의 `complete=true`, natural size 1080×2400 및 1206×2622, 양수 rendered width를 확인했다. 링크 오류 없음. |
| 30 | simulator report가 paint/overflow 증거로 보이거나 공식 완료 수가 바뀌는가 | 두 화면은 report text만 표시하고 미지원 범위를 기록한다. C14.1/C14는 미완료다. Tailnet roadmap Agent Browser에서 공식 상태 checkbox 65/197을 확인했다. 통과. |

## 수정 후 재검증

- `mise exec -- cargo fmt --all -- --check`: 통과.
- `mise exec -- cargo test --locked -p spinon-ffi --features c14-1-app-fixture-test c14_1_app::tests --quiet`: 2/2 통과.
- `mise exec -- cargo test --locked -p spinon-runtime --features c14-1-live-v8-fixture-test c14_fixture_snapshot_is_actor_owned_and_closed_sessions_fail --quiet`: 1/1 통과.
- `mise exec -- cargo clippy --locked -p spinon-ffi --features c14-1-app-fixture-test --all-targets -- -D warnings`: 통과.
- `git diff --check`: 통과.
- PR checks API는 빈 목록, PR merge state는 `CLEAN`/`MERGEABLE`이다. 기본 branch protection은 선형 이력을 요구하고 승인 0개, 필수 status check 0개다.
