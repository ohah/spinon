# C14.1 live V8 image measurement 계획 검토 · 20개 실패 관점

**계획:** `plan/c14-1-live-v8-image-measurement.md`
**검토 기준:** 코드 작성 전 계획·비교 기준·실패 경로
**결과:** P1/P2 미해결 없음. 아래 결함을 계획에 반영한 뒤 구현을 진행한다.

| # | 독립 실패 관점 | 판정 및 계획 조치 |
|---:|---|---|
| 1 | synthetic fixture를 실제 V8 문서 결과라고 잘못 주장하는가 | 실제 `RuntimeSession::eval`이 만든 snapshot으로 경로를 정의하고 이전 synthetic evidence와 분리한다. 통과. |
| 2 | 외부 thread가 actor 소유 HostDocument에 직접 접근하는가 | snapshot 복사를 owner actor 명령으로 제한한다. 통과. |
| 3 | snapshot 응답이 eval보다 먼저 실행되어 이전 DOM을 보는가 | V8 eval 응답이 commit/publish 뒤 돌아온 다음 snapshot 요청을 enqueue한다. snapshot contract를 actor 처리 시점 최신 문서로 고정한다. 통과. |
| 4 | queue full/stopped/closed가 빈 snapshot이나 성공으로 바뀌는가 | 각각 오류로 반환하고 fixture report를 실패시킨다. 통과. |
| 5 | busy actor·종료 중 pending snapshot에서 caller가 무한 대기하거나 종료 reply가 유실되는가 | 계획 검토에서 timeout 누락을 찾았다. 5초 응답 timeout과 종료 reject 오류를 계획에 추가했다. timeout 뒤 실행될 수 있는 명령은 읽기 전용이고 늦은 reply를 버린다. 통과. |
| 6 | HostDocumentSnapshot이 worker 경계를 넘어 안전하게 전달되는지 확인하지 않는가 | feature-gated compile와 Rust actor fixture 실행으로 Send/ownership 경계를 확인한다. 통과. |
| 7 | root가 없거나 여러 개여도 첫 항목을 임의 선택하는가 | 고유 root 조건을 사전 판정으로 둔다. 통과. |
| 8 | 문서에 HTML `img`가 없거나 여러 개일 때 대상이 모호해지는가 | 연결된 HTML image가 정확히 하나여야 통과한다. 통과. |
| 9 | SVG namespace의 `image`나 detached `img`가 허용되는가 | namespace와 연결 상태를 검증하고 거부한다. 통과. |
| 10 | 고정 PNG bytes가 snapshot `src`와 무관하게 다른 image에 붙는가 | source URL exact match 후에만 bytes를 decoder에 넘긴다. 통과. |
| 11 | 요청 생성 뒤 src 변경을 stale admission이 놓치는가 | `ImageResourceSnapshot::new`의 현재 source/src 검사를 유지하고 negative control을 포함한다. 통과. |
| 12 | 다른 document generation/revision에서 만든 completion이 현재 문서에 섞이는가 | request/resource/layout source stamp 일치가 통과 조건이며 stale case는 실패다. 통과. |
| 13 | image NodeId가 아닌 frame을 Chrome 기준에 대조하는가 | frame key는 고유 image handle에서 얻은 `NodeId`로 확인한다. 통과. |
| 14 | DPR/density가 자연 CSS 크기에 중복 적용되는가 | oracle과 runtime 모두 CSS px 단위를 사용하고 density를 곱하지 않는다. 통과. |
| 15 | CSS style을 V8이 파싱했다고 과장하는가 | 고정 author stylesheet는 외부 fixture 입력이며 V8 `<style>` parsing은 범위 밖이라고 기록한다. 통과. |
| 16 | corrupt PNG 또는 같은 크기지만 바뀐 PNG bytes가 고정 oracle 입력으로 오인되는가 | reference byte length·SHA-256을 decoder 전에 대조하고 decode error도 fail-closed 처리한다. 통과. |
| 17 | measurement revision overflow나 resource source mismatch가 조용히 재사용되는가 | checked revision 증가와 source equality를 조건으로 둔다. 통과. |
| 18 | native output buffer 부족, UTF-8 변환, null route가 false positive를 만드는가 | 기존 report route의 status/UTF-8/buffer 오류를 검사하고 실패로 기록한다. 통과. |
| 19 | feature-off artifact에 새 actor command·fixture payload가 남는가 | runtime method/command를 fixture Cargo feature로 gate하고 default/opt-in artifact를 비교한다. 통과. |
| 20 | Simulator 계산 report가 이미지 paint·hardware·제품 로더 증거로 과장되는가 | report-only 범위를 유지하고 두 Simulator 결과를 별도로 기록하며 pixels·실기기를 주장하지 않는다. 통과. |

이 검토는 계획과 adapter 경계만 다룬다. 아직 구현·앱 실행·제품 resource loader는 검증하지 않았다.
