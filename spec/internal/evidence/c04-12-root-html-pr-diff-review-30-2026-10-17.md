# C04.12 루트 HTML PR 변경 적대 검토

- **검토일:** 2026-10-17
- **범위:** PR 전체 코드·플랫폼 fixture flag·계약·로드맵·실행 evidence diff
- **검토 기준:** 기존 계획/구현 20개 관점 표를 반복하지 않고, PR delta의 별도 오류 경로 30개를 확인했다.
- **결과:** 고칠 결함 1건을 발견해 수정하고 영향받은 runtime fixture와 Android/iOS simulator 실행을 다시 확인했다. 이 PR은 HTML DOM 기반 단계에 한정되며 C04.12 전체 완료 근거가 아니다.

| # | 별도 실패 경로 | 확인과 처리 |
| --- | --- | --- |
| 1 | C ABI가 source byte 상한을 확인하기 전에 포인터를 읽음 | 길이 검사를 `from_raw_parts`보다 먼저 수행하고, 초과 길이 negative test에서 null 포인터도 읽지 않음을 확인했다. |
| 2 | 양수 길이에 null source 전달 | source null check가 slice 생성보다 앞서며, 경계 코드를 확인했다. |
| 3 | null source와 길이 0에서 Rust의 raw-slice precondition 위반 | 빈 입력은 정적 empty slice로 분기한다. parser는 이후 document/quirks 계약으로 처리한다. |
| 4 | 잘못된 UTF-8을 대체문자로 바꾸어 일부 DOM을 게시 | `from_utf8` 오류로 세션 생성 전에 거부하고 C ABI negative test로 확인했다. |
| 5 | output buffer가 0이거나 null | session 생성 전에 null을 반환한다. |
| 6 | output buffer가 짧아 startup 성공 뒤 session handle만 누수 | report 기록 실패면 만들어진 session을 명시적으로 drop한다. |
| 7 | UTF-8 byte 수 대신 문자 수로 입력 크기를 계산 | Rust `str::len()`은 byte 기준이며 계약의 1 MiB와 일치한다. |
| 8 | HTML doctype 없이 parser가 quirks 문서를 NoQuirks로 오인 | no-doctype negative fixture가 실패를 확인하고 startup은 NoQuirks만 허용한다. |
| 9 | 생략된 `html/head/body` 또는 잘못 중첩된 태그에서 tree가 멈춤 | 고정 html5ever document tree builder를 쓰고 자동 삽입/recovery fixture를 확인했다. |
| 10 | parser parse error가 실패로 바뀌거나 성공으로 기록되어 recovery와 혼동 | parse error count를 제한해 저장하고 tree recovery 결과와 별도로 유지한다. 제품 conformance로 확대하지 않는다. |
| 11 | `<svg>`·MathML 요소가 HTML namespace로 변환 | parser element namespace URI를 import operation까지 보존한다. |
| 12 | HostDocument batch ABI가 표현하지 못하는 namespaced attribute를 조용히 버림 | unsupported attribute namespace에서 전체 세션 생성을 거부하고 `xml:lang` negative test를 둔다. |
| 13 | HTML parser가 무시하는 duplicate attribute가 잘못 중복 import | 최종 parser attribute 목록만 import하며 fixture가 관찰된 단일 `id` 값을 확인한다. |
| 14 | attribute 상한이 element 단위로만 적용되어 문서 전체 quota 초과 | tree walk에서 누적 합을 계산하고 16,385개 초과 입력을 거부한다. |
| 15 | 들여쓰기·빈 텍스트 때문에 node/depth 계산이 문서와 불일치 | 최종 연결 tree를 root부터 순회해 node/depth 상한을 적용한다. template fragment는 import 대상에서 제외한다. |
| 16 | `documentElement`가 `html` 이름에만 고정되어 다른 첫 root element에서 표준 동작 위반 | PR 첫 검토에서 발견했다. 첫 element child 반환으로 수정하고 실제 V8 fixture가 `html` root 제거 후 `main` root getter와 `head/body === null`을 검사한다. |
| 17 | `head/body`가 문서 밖 노드나 중첩 자손에서 잘못 검색됨 | getter가 현재 root의 direct child만 순회한다. HTML root가 없거나 첫 root 이름이 `html`이 아니면 null을 반환한다. |
| 18 | accessor 반복마다 다른 JS wrapper를 생성 | 기존 WeakMap wrapper cache를 유지하고 실제 V8 fixture에서 getter identity를 확인한다. |
| 19 | doctype/comment/PI를 visible HostDocument node로 취급 | `Ignored` node로 두고 import에서 자식만 순회한다. doctype은 parser quirks 판정 전에 전달된다. |
| 20 | `<template>` fragment 내부 자식이 일반 document 자식으로 노출 | template-content fragment는 별도 저장하고 visible HostDocument import에서 제외한다. DOM `template.content` 구현으로 주장하지 않는다. |
| 21 | root HTML `<script>`가 session 시작 중 실행 | HTML parser 경로는 V8에 script source를 평가하지 않는다. V8 fixture sentinel이 설정되지 않음을 검사한다. |
| 22 | table foster-parented text가 table 뒤나 내부로 이동 | `append_based_on_parent_node`가 attached element 앞에 삽입하도록 수정했고 table text 순서 fixture가 통과했다. |
| 23 | append 전에 기존 parent에서 detach하지 않아 같은 node가 둘 이상의 자식 목록에 남음 | `append_child`가 먼저 detach한다. HTML5 reparent/move 경로를 살폈다. |
| 24 | adjacent text coalescing이 foster insertion 전후 형제 순서를 바꿈 | append와 before-sibling 텍스트의 양쪽 병합 경계를 확인했고 source-order fixture를 둔다. |
| 25 | `reparent_children`가 parent handle 또는 순서를 갱신하지 않음 | 기존 자식 목록을 take한 뒤 parent를 비우고 같은 순서로 새 parent에 append한다. |
| 26 | disconnected/template node가 parser tree quota 검사에서 빠져 보안 상한을 과장 | 상한의 계약 대상은 import 가능한 tree로 명시했다. 전체 source는 별도 1 MiB로 제한하고 template fragment는 의도적으로 가져오지 않는다. |
| 27 | 깊이/노드 초과에 일부 문서만 HostDocument에 반영된 채 V8 노출 | 파싱과 검증은 actor 초기화 중이며 오류 시 ready 실패, bridge와 worker가 게시되지 않는다. |
| 28 | 기존 JavaScript-only `RuntimeSession::new()`가 빈 문서 대신 HTML parser를 사용 | 기존 constructor는 `None`을 보내고 `HostDocumentBridge::new()` 경로를 유지한다. 회귀 test와 diff를 확인했다. |
| 29 | opt-in fixture ABI가 기본 FFI artifact에 들어가거나 플랫폼 한쪽 flag 누락 | Cargo feature, C define, Android BuildConfig/source set, iOS Swift condition을 짝지었다. host staticlib default/positive-control 경계 evidence와 양쪽 fixture 실행을 연결했다. |
| 30 | simulator PASS 화면을 실제 HTML paint/viewport/color-scheme 결과로 과대 해석 | fixture 화면과 C04.12 제품 한계를 PR/계약/evidence에 분리해 적었다. viewport, OS preference, UA Canvas, app asset loader, HTML rendering은 미구현이다. |

## 재검증

- `cargo fmt --all -- --check`
- `cargo test --locked -p spinon-runtime root_html` · 8/8
- `cargo test --locked -p spinon-ffi root_html --features c04-12-root-html-fixture-test` · 2/2
- 변경 runtime/FFI crate Clippy · 경고 0
- Android API 37 ARM64 AVD 및 iPhone 17 Pro / iOS 26.2 Simulator 실제 V8 fixture · PASS
- Simulator 캡처는 fixture 결과 화면이며 HTML 자체의 렌더 캡처가 아니다.
