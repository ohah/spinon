# C18.4 계획 PR 변경 적대 검토 20개 관점

**대상 변경:** `plan/c18-4-async-image-replacement.md`, `spec/STATUS.md`, `spec/internal/README.md`, Chrome oracle, 계획 검토 기록
**검토 시점:** 계획 PR 생성 전 · 2026-10-16
**판정:** 20개 관점 완료 · 차단 발견 없음.

| # | 독립 실패 관점 | 확인 결과 | 판정 |
|---:|---|---|---|
| 1 | 상위 로드맵을 완료 처리하는가 | C18 상위와 C18.4가 모두 unchecked이고 계획이 구현 전임을 유지한다. | PASS |
| 2 | 상태표 ID가 실제 plan 파일과 다른가 | `C18.4` plan filename과 문서 제목/상태 설명이 일치한다. | PASS |
| 3 | plan의 선행 항목이 이미 main에 있는가 | STATUS에서 C14.1–C14.3, C18.1–C18.3, U05.1 근거를 확인했다. | PASS |
| 4 | plan-only 변경이 없는 계약 0067을 만들어낸 것처럼 보이는가 | 새 내부 버전 계약을 추가하지 않고 기반 코드와 미완료 범위를 설명한다. | PASS |
| 5 | status 계획 링크가 Rspress 가상 route를 가리켜 build를 더 깨뜨리는가 | STATUS와 internal README의 신규 plan 링크는 `github.com/.../blob/main` 외부 링크로 지정했다. | PASS |
| 6 | Chrome 비교 실행 환경이 재현 가능한가 | Chrome 155 UA, 로컬 threaded server, 지연 값과 화면 viewport/DPR를 oracle README에 기록했다. | PASS |
| 7 | 느린 A/빠른 B 결과를 응답 순서와 혼동하는가 | observation JSON의 event time, src, natural dimensions와 pixel이 B-only load를 구분한다. | PASS |
| 8 | 늦게 도착한 A bytes가 실제로 수신되지 않았다고 잘못 주장하는가 | server log에서 A 응답도 반환됐고 이후 B image 상태가 유지됐음을 보존한다. | PASS |
| 9 | C pending 중 기존 source pixel 보존 여부를 확인하지 않았는가 | 별도 screenshot은 성공 image C를 source A pending으로 교체한 직후 빈 화면을 보존한다. C auto-size pending dimensions는 observation JSON에서 따로 확인한다. | PASS |
| 10 | natural-size relayout 관문이 고정 frame paint 시험에 묻히는가 | 별도 `auto/auto` 경로의 0×0→2×2→4×2 geometry를 구분해 적었다. | PASS |
| 11 | decode 오류와 network 404를 동일한 oracle로 오인하는가 | 고정 frame 손상 PNG는 200 `image/png`와 잘못된 bytes로 `EncodingError`를 관찰했고, auto-size 404는 별도 fallback 제외 근거로만 기록했다. | PASS |
| 12 | Chrome의 UA broken-image icon을 Spinon 계약으로 복사하는가 | 실패 screenshot을 보존하되 platform UA icon을 pixel contract에서 제외한다. | PASS |
| 13 | 실패 결과가 어느 node/request에 귀속되는지 빠지는가 | 계획은 실패 outcome도 시작 당시 request identity/source revision에 묶도록 한다. | PASS |
| 14 | 이전 decoded dimensions 전달이 이전 RGBA texture 재사용을 허용하는가 | pending geometry를 size-only current-source measurement로 제한하고 RGBA/request/GPU key 재바인딩을 금지한다. | PASS |
| 15 | decoder/failure API가 취소·panic 뒤 worker를 막는가 | plan tests에 task panic/cancel·revision overflow와 다른 node 격리를 포함한다. | PASS |
| 16 | AVD·iOS 결과의 범위를 과장하는가 | 구현 관문은 AVD software path와 iOS pre-compositor drawable로 명시하며 device/hardware GPU 증거를 요구하지 않는다. | PASS |
| 17 | 기존 C18 pixel profile 회귀를 생략하는가 | C18.1·C18.2·C18.3 회귀를 양쪽 simulator 관문에 포함한다. | PASS |
| 18 | fixture FFI/PNG payload가 제품 artifact에 들어갈 수 있는가 | 전용 `c18-4-image-fixture-test` opt-in, default artifact negative scan과 opt-in positive control을 요구한다. | PASS |
| 19 | docs build 실패를 새 plan link 탓으로 숨기는가 | 현재 Rspress v2.0.22 build는 462개 dead-link block으로 실패한다. C18.4 변경 파일 자체는 Rspress dead-link block 목록에 없으며, 실패 건수는 프로젝트 전체 baseline 비교 없이 baseline 확정으로 표현하지 않는다. | PASS |
| 20 | 브라우저 프로세스가 작업 뒤 남는가 | 세 Agent Browser 세션을 각각 닫았고 active session list가 비었다. browser close 뒤 남은 Chrome crashpad process도 task-owned PID로 확인해 종료했다. 두 local oracle server listener도 닫혔다. | PASS |

## 남은 범위

계획 검토는 구현 코드 검토가 아니다. Rspress 전체 build는 변경 파일 밖의 dead-link 문제로 실패하며 이 PR에서 기존 route inventory를 일괄 수정하지 않는다. 구현 단계에서 결과 model, retained intrinsic measurement와 platform fixture를 실제 코드·실행으로 검증해야 한다.
