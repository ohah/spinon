# C11.1 Root HTML anonymous Grid text PR diff 검토 30개

**검토 대상:** 현재 작업 트리의 implementation diff (계획 PR 검토와 구분)
**목표 계약:** 기본 parsed Root HTML runtime에서 제한된 direct text `AB` 하나를 anonymous Grid item으로 배치·paint
**검토 원칙:** 각 항목은 서로 다른 입력·소유권·검증·배포 경계를 확인한다. 여기의 30개는 구현 실패 관점 20개나 계획 PR 30개를 재사용하지 않는다.

| # | Diff 검토 관점 | 판정 |
|---:|---|---|
| 1 | 새 projection API가 다른 style profile을 받는가 | `RuntimeRootHtmlTextPaintV1` 검사로 차단한다. |
| 2 | viewport와 computed snapshot 불일치 | viewport 일치 검사가 기존 경로와 같은 경계에 있다. |
| 3 | 공개 API 오인 가능성 | 함수는 crate 간 내부 Rust 경계이고 공개 CSS 지원 계약은 아니다. 완료 상태 변경은 없다. |
| 4 | Root HTML/C11.4 feature 혼합 | 옵션은 분리되고 동시 허용하지 않는다. 기존 fixture positive control 유지. |
| 5 | 비-Root HTML HostDocument 경로로 새 flag 전파 | 새 생성 함수에서만 true를 설정하고 모든 기존 생성 함수는 false다. |
| 6 | paragraph owner와 grid container 불일치 | `validate_text_paragraphs`가 parent/child와 paragraph id를 검증한다. |
| 7 | source node 중복·순서 유실 | source node 집합의 중복과 text paragraph source 불일치를 검사한다. |
| 8 | source anchor가 공백 node에 고정됨 | 첫 비공백 text node를 anchor로 고른다. 인접 source 회귀 통과. |
| 9 | 빈 source 또는 whitespace-only source가 자동 배치됨 | collector가 item을 만들지 않는다. 기존 Root HTML control 통과. |
| 10 | CSS whitespace 판정을 ASCII 외 문자까지 과잉 적용 | tab/LF/FF/CR/space만 collapsible로 분류하고 다른 code point는 visible로 취급한다. 이 slice는 `AB` 외 값을 거부한다. |
| 11 | `white-space:pre` 공백-only 입력 거부 | CSS Grid §6 및 기존 테스트에 따라 whitespace-only sequence를 무시한다. 새 변경은 이 규칙을 좁히지 않는다. |
| 12 | visible element sibling 혼합 | 기본 route에서 fail-closed 진단을 반환한다. |
| 13 | hidden element sibling | hidden child는 anonymous run을 flush하지 않으며 layout에서도 제외한다. |
| 14 | inline child·nested flow child | 현재 Root HTML slice에서 지원하지 않고 거부한다. |
| 15 | 두 번째 텍스트 Grid 또는 다른 text 값 | 단일 `AB` 제한을 검사한다. 복수 owner는 코드에서 거부된다. |
| 16 | computed font-family/size/weight/style 전달 | inherited Grid style tuple을 검사하고 text layout과 raster에 넘긴다. |
| 17 | line-height, white-space, direction, word-break | adapter가 허용된 값만 통과시킨다. line height는 20px로 한정한다. |
| 18 | color adapter가 인접 text나 container 배경색을 덮음 | Grid computed text color만 anchor에 복사하고 runtime test에서 `#e5484d`를 확인한다. |
| 19 | Grid placement가 `order`·explicit placement로 오염 | anonymous item은 별도 synthetic element가 아니며 source child에 CSS grid-placement를 매핑하지 않는다. 기본 `order:0`만 허용한다. |
| 20 | Auto size exception이 일반 item에 적용 | root-specific paragraph 표식과 동일 Grid parent 조건이 예외 범위를 묶는다. |
| 21 | unsupported width/height dimension이 새 profile로 허용 | Auto 외에는 기존 validation 분기를 그대로 사용한다. |
| 22 | item layout frame이 renderer에서 제거됨 | 해당 source text node frame만 Root HTML text paint 경로에서 보존한다. 정확한 cell frame 테스트 통과. |
| 23 | 문서에 없는 text frame 허용 | snapshot의 실제 text node 목록 및 text layout 유무를 둘 다 요구한다. |
| 24 | text node 예외가 stacking/scroll 경로로 번짐 | stacking builder는 빈 허용 집합을 전달한다. 기본 renderer 경계에만 예외가 있다. |
| 25 | 일반 HTML inline text 렌더링 회귀 | Root HTML regression 54개와 style-to-render 71개 통과. |
| 26 | runtime paint owner가 Grid container와 맞지 않음 | anonymous text source를 owner style box에 연결하고 중복 mapping은 오류 처리한다. |
| 27 | draw command 누락 또는 glyph batch 중복 | 기본 runtime scene과 두 simulator 로그에서 text 1개, batch 1개를 확인했다. |
| 28 | 기존 C11.4 behavior 변경 | feature-gated 32개 anonymous text layout test와 FFI positive control 통과. |
| 29 | 실패 시 partial/stale scene 또는 완료 상태 상승 | negative runtime case는 completed scene 없이 실패한다. C11 상태와 공식 count는 변경하지 않는다. |
| 30 | 변경 증거가 코드와 다른 실행물을 가리킴 | simulator bundle HTML hash가 고정 fixture와 일치한다. build/install/launch/log/screenshot을 Android API 37 AVD와 iOS Simulator별로 분리했다. |

## 병합 전 주의 기록

- `cargo fmt --all -- --check`, scoped default all-target Clippy, Root HTML 54개, Root HTML focused 7개, renderer 71개, C11.4 anonymous text 32개와 fixture positive control은 통과했다.
- 전체 `spinon-style-to-layout`에는 기존 C07 aspect-ratio 문자열 serialization 테스트 1건의 실패가 있다. C11.4 feature all-target Clippy에는 변경하지 않은 helper lint 실패가 있다. 전체 workspace test/CI 통과로 확대하지 않는다.
- Chrome 156 사전 비교는 기준 관측이고 simulator 화면은 보조 증거다. 이 PR은 C11.1, C11.4 또는 C11 상위의 완료 체크를 추가하지 않는다.
