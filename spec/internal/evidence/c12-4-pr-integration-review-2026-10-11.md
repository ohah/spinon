# C12.4 PR 통합 실패 관점 검토 · 2026-10-11

대상: [PR #141](https://github.com/ohah/spinon/pull/141), `0289a16` 기준 C12.4 제한 stacking-context 구현과 증거 묶음.

판정은 각기 다른 실패 조건을 한 번씩 확인한 결과다. 기존 계획·구현별 20개 검토를 다시 세지 않고, 이 문서는 서로 맞물리는 저장소 경계와 PR 상태만 검토한다.

| # | 적대적 실패 관점 | 대조한 구현·근거 | 판정 |
| --- | --- | --- | --- |
| 1 | 다른 브랜치 또는 이전 기준 커밋의 결과를 PR로 잘못 올리는가 | PR 검토 시작 시 base `main`, head `feat/c12-4-stacking-context-runtime`, head SHA `0289a16c`; 로컬 브랜치는 해당 원격 head와 일치하고 작업 트리는 깨끗했다. | 일치 |
| 2 | 진단 계측 feature가 기본 제품 빌드에서 우연히 켜지는가 | Cargo feature와 Android/iOS build script의 기본값은 `0`; 켤 때 C04 runtime GPU fixture를 요구한다. | 기본 비활성 경로 확인 |
| 3 | C ABI 헤더와 Rust export가 달라 모바일 링크에서만 실패하는가 | C12.4 eval 및 test-only surface/stack 함수의 이름·인자·feature 조건을 Rust export, C 헤더, 호출부에서 대조했다. | 대응 확인 |
| 4 | Android JNI가 다른 worker의 stack 주소를 현재 CSS worker 주소로 잘못 표본화하는가 | JNI 구현은 호출 중인 `pthread_self()`의 속성을 얻고, worker probe는 CSS 계산 worker 안에서 실행한다. 테스트 전용 값은 JSON 근거와 함께 보존한다. | 현재 worker 귀속 경로 확인; 표본은 순간 관측 |
| 5 | iOS stack 방향·guard 처리에서 범위를 반대로 계산하는가 | Objective-C++는 `pthread_get_stackaddr_np`와 `pthread_get_stacksize_np`에서 시작·끝을 계산하고 guard 0 및 page size를 전달한다. Rust가 산술 overflow와 marker 포함을 검사한다. | 범위 검사 연결 확인 |
| 6 | FFI 호출 중 surface/host가 해제되거나 잘못된 실행 queue와 경합하는가 | C12.4 FFI 주석은 host 수명, 직렬 render queue, background executor와 output buffer 계약을 명시한다. Android/iOS harness는 해당 probe를 직렬로 구동한다. | 내부 진단 호출 계약 확인; 제품 공개 API 아님 |
| 7 | 오래된 style/layout/document revision이 새 paint list로 채택되는가 | snapshot 진입에서 profile·viewport·document/style/layout revision을 검증하고, matrix 입력에는 고정 revision과 실패 비교기를 둔다. | 오래된 입력 거부 경계 확인 |
| 8 | Stylo cascade 오류나 지원하지 않는 profile이 빈 값으로 조용히 렌더되는가 | runtime painter는 지정 profile 외 입력과 cascade diagnostics를 먼저 거부한다. unsupported 값 fixture는 기대 거부로 비교된다. | fail-closed 경계 확인 |
| 9 | `position`, 정수 `z-index`, Flex item 예외를 섞을 때 context 소유자가 잘못 선택되는가 | context 생성 규칙과 negative/zero/positive stack level 분기를 내부 계약·구현·기존 구현 검토에서 대조했다. static Flex item 예외와 positioned `auto`도 별도 항목으로 기록돼 있다. | 제한 profile 규칙과 일치 |
| 10 | context root의 배경이 자손의 음수 context 뒤에 그려져 가려지는가 | flatten 단계가 context root를 먼저 내보내고 negative, flow, zero, positive 항목을 context 단위로 처리한다. nested-context fixture가 atomic isolation을 검사한다. | paint 단계와 fixture 일치 |
| 11 | Flex `order`로 absolute/fixed child까지 재배열하거나 문서 트리를 변경하는가 | painter 전용 child sequence만 생성하고 in-flow 직계 Flex item에 `(order, source order)`를 적용한다. positioned child는 제외하며 reverse 결과는 pinned Chrome pixel 관측에 한정한다고 계약에 적었다. | mutation 방지·제외 규칙 확인; 표준 설명과의 차이는 공개 |
| 12 | 같은 stack level의 순서가 입력마다 흔들리거나 NodeId 순서에 의존하는가 | stack level은 tree order와 함께 정렬하고, source-order tie fixture 및 비교 JSON에서 item sequence를 검증한다. HostDocument의 저장 순서는 변경하지 않는다. | 결정적 순서 확인 |
| 13 | `display:none` subtree, text node, 0-area frame이 paint 목록에 섞이는가 | 숨김 ancestor 전달, 제한된 text 거부, 0-area box 건너뛰기와 연속 paint rank 생성 경로를 검토했다. hidden·zero-area fixture가 있다. | 제한 동작과 오류 경계 확인 |
| 14 | 비정상 frame이나 지나치게 큰 paint index가 renderer까지 전달되는가 | frame은 finite·유효 `CssRect`로 확인하고 tree order·paint order 변환은 checked arithmetic을 쓴다. 잘못된 frame/overflow negative control이 실패를 요구한다. | 비정상 geometry 거부 확인 |
| 15 | `Vec` 예약 실패 뒤 부분 snapshot 또는 패닉을 성공으로 보고하는가 | 변경된 stacking 경로의 성장 지점은 `FallibleReserve`를 거쳐 구체 오류로 돌아가고, 여러 예약 단계 주입 및 synthetic stress 검토가 보존돼 있다. 전역 allocator OOM 보장은 하지 않는다고 명세·PR에 제한했다. | 명시적 예약 지점 확인; 전역 OOM 미보장 |
| 16 | 깊은 입력에서 Rust 재귀 overflow 또는 실패 후 worker 오염이 발생하는가 | traversal/flatten은 명시적 작업 목록을 사용한다. Android API 37 AVD와 iOS 26.2 Simulator에서 1,024 성공·1,025 거부와 같은 worker의 얕은 revision 복구를 확인했다. 기본 Rust test stack의 depth 256 실패도 별도로 보존했다. | 앱 worker 경계·복구 확인; test stack 실패 유지 |
| 17 | renderer가 계산된 paint order를 다시 정렬하거나 다른 NodeId의 색을 그리는가 | WGPU는 flat `RuntimeRenderSnapshot` 순서를 소비한다. Android `PixelCopy`와 iOS Metal surface readback의 고정 표본이 같은 Chromium oracle과 비교됐다. | 소비 경로와 surface 근거 연결 확인 |
| 18 | 캡처 타이밍·좌표·RGBA 처리 오류가 pixel 일치로 둔갑하는가 | Android 캡처는 `PixelCopy` 결과를 사용하고, iOS는 GPU surface texture의 readback을 사용한다. 두 비교기는 고정 좌표·크기·표본·채널 오차를 기록하고 negative control을 실행했다. AVD renderer는 ANGLE/SwiftShader라고 명시한다. | 실제 surface 비교; 하드웨어 GPU 주장 없음 |
| 19 | Chrome reference/WPT 후보를 Spinon 적합성 증거로 잘못 승격하는가 | 자동 WPT 후보 4개와 수동 후보 1개는 Chrome 154 결과로 표시했고, Spinon 공식 `testharness.js` runner 미통합 및 AVD/실기기 경계를 PR 본문과 후속 보고서에 구분했다. | 주장 범위 일치 |
| 20 | 추적 정보·민감 stack 주소가 공개 저장소/첨부에 노출되거나 로드맵이 PR 상태와 어긋나는가 | C12.4 변경 로그에서 주소 모양 검색 결과는 V8 `untag_mask` 상수뿐이었다. raw 주소·maps 산출물은 `.gitignore` local-only 대상이며 PR 첨부는 캡처 네 장이다. 검토 중인 PR #141을 로드맵·상태 대장에 반영했다. PR 본문에는 Tailscale 주소가 없다. | 민감 산출물 비공개 유지; 상태 문구 수정 완료 |

## 결과

이 통합 검토에서 발견한 수정 사항은 공개 로드맵과 상태 대장의 오래된 “PR 미생성” 문구였다. 두 문구를 PR #141 검토 중으로 고쳤고 C12.4 완료 체크는 해제 상태로 유지했다. 기능 구현별 검토에서 이미 다룬 세부 실패 관점을 다시 합산하지 않았다.

GitHub는 이 브랜치에 자동 check를 보고하지 않았다. 로컬 근거에는 workspace 568 통과·3 무시, CSS reference 189 통과, JS 2 통과, Clippy/rustfmt 통과와 Android/iOS Simulator matrix·surface evidence가 있다. PR 병합 전에는 이 문서 갱신을 포함한 최종 diff와 링크를 다시 검사한다.

남은 경계는 Spinon 공식 WPT runner, 실제 하드웨어 GPU 및 실기기 RSS/stack 귀속, stack-only high-water/commit, 전역 allocator OOM 복구다. 이 검토는 C12.4나 CSS 전체 지원을 완료 판정하지 않는다.
