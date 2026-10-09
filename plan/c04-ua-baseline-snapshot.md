# C04.5 · 지원 HTML UA 계산 스냅샷

## 목표

컴파일 시 포함한 `supported-elements-v0.css`를 실제 Stylo cascade의 UA origin에 등록하고, 호출자가 선택한 HostRoot 직속 요소 subtree에서 지원 요소의 구조 기본값을 계산하는 내부 Rust 진입점을 제공한다. C04.1의 fixture 전용 `compute_basic_cascade`와 분리된 `SupportedElementsUaV1` profile을 추가한다.

진입점 입력은 불변 `StyloDocumentView`, author stylesheet 목록, 명시적 `CssViewport`, `StyleRevision`이다. 결과는 `display`, `list-style-type`, block/inline 방향 margin, inline 시작 padding을 포함한 계산 style snapshot이며 문서·표시·스타일·환경 revision과 stylesheet 진단을 보존한다.

## 비교 모델과 완료 기준

- Chromium `154.0.8037.95`, revision `@05d469856e75794131cc2e5d9b2f6b6f10a70388`의 기존 C01 UA reference를 고정한다.
- `C01-UAv0-supported-html-elements`의 HTML namespace 요소 9개와 inventory의 19개 feature ID를 사용한다.
- 고정 입력은 `800×600` CSS px, scale `1`, screen/light/en-US/UTC이며 reference snapshot의 `observations.chromiumDefault` computed CSS 값을 oracle로 한다.
- C04.5의 UA stylesheet에는 환경 의존 media query가 없고 C01 Chromium 기준 author sheet는 비어 있다. 이 함수는 viewport 크기·배율 외의 완전한 media environment를 입력받지 않는다. Stylo `Device`는 screen·light scheme·고정 font metrics를 쓰고 pointer capability default는 Stylo target cfg를 따른다(Android/iOS coarse, 데스크톱 fine/hover). 그러므로 target-independent author `@media` 동작을 계약하지 않는다.
- inventory의 19개 feature ID가 각각 예상 요소 하나와 계산 property 하나에 연결되고 값이 Chromium과 문자열 정확히 같아야 한다. profile은 whitelist 7개 속성을 모든 요소에 계산하므로, inventory 밖의 snapshot 값을 inventory 누락으로 오인하지 않는다.
- author origin 덮어쓰기, 비-author 입력 거부, 잘못된 viewport, 잘못된 root, namespace 불일치, stylesheet 진단, revision 보존을 부정·경계 사례로 확인한다.
- 네트워크 로딩은 수행하지 않는다. author stylesheet는 메모리 입력이며 상대 URL은 계약된 절대 base URL에서만 파싱한다.

## 범위 경계

이 작업은 Rust 스타일 엔진에 호출 가능한 computed-style profile을 더한다. V8/JS API·제품 런타임 scheduler 연결, Taffy 투영, inline formatting, list marker 생성, text shaping, GPU 표시, 동적 invalidation, Android/iOS 앱 실행은 완료 범위가 아니다. `inline`, `inline-block`, `list-item` 계산값을 산출해도 그 요소가 화면에 올바르게 배치되었다고 주장하지 않는다.

제품 runtime에서 author `@media`를 지원하려면 media type, primary/any pointer·hover, color scheme, motion/contrast 선호, 기본 font metrics 등 환경 입력과 revision 관리가 별도 계획·구현 대상이다. 현재 `environment_revision`은 snapshot에 보존하는 번호이며 실제 OS 환경을 읽거나 이 값 하나로 환경을 복원하지 않는다.

외부 CSS URL 자원 로더는 이 작업에서 구현하지 않는다. 제품 UA stylesheet 전체 적용 상태도 미완료로 유지한다.

## 구현 순서

1. `ComputedStyleProfile::SupportedElementsUaV1`과 전용 computed property 목록을 추가한다.
2. 기존 공통 cascade에서 새 profile을 계산하는 공개 Rust 함수로 노출하고 기존 fixture entrypoint 동작은 바꾸지 않는다.
3. C01 reference의 9개 요소·19개 property를 비교하는 Rust 테스트와 author override/오류 경계 테스트를 추가한다.
4. 버전 있는 내부 계약, 실행 근거, 공식 상태 대장과 Tailscale CSS/로드맵 미리보기를 함께 갱신한다.
5. 포맷·대상 crate 테스트·workspace lint/build를 실행하고, 구현 코드와 결과를 계획 검토와 별개의 20개 실패 관점으로 재검토한다.

## 계획 적대 검토

초기 계획 검토는 [C04.5 계획 검토 근거](../spec/internal/evidence/c04-ua-baseline-plan-review-2026-10-09.md), 구현 검토에서 발견한 media environment 경계의 계획 수정 검토는 [C04.5 계획 수정 검토](../spec/internal/evidence/c04-ua-baseline-plan-amendment-review-2026-10-09.md)에 둔다. 계획은 두 검토의 범위와 합격 기준으로 고정한다.
