# C05.1 런타임 사용자 지정 속성 비교 입력

`runtime-custom-properties.v1.json`은 고정 viewport, 관찰할 CSS 노드, 문서 전이, 비교 기준의 SSOT다. `runtime-custom-properties.html`을 Chrome `154.0.8037.98`에서 실행해 computed style과 CSS px rectangle을 수집하며, reference는 fixture·HTML·앱 JavaScript·capture 도구·Chrome 실행 파일 hash에 묶는다. 캡처 도구는 기존 reference를 덮어쓰지 않는다.

픽스처는 미등록 사용자 지정 속성 상속, 대소문자 구분, 중첩 fallback, `inherit`·`unset`·`initial`·`revert`, self/mutual cycle, 잘못된 대체값, 빈 값, `!important`, `gap:var(...)`, background paint와 조상 값 변경·재부착을 다룬다. Rust 테스트는 고정 Chrome computed style을 정확 비교하고, scene의 부모·자식 geometry 및 runtime frame 전이를 확인한다.

Android·iOS fixture 앱은 C04.10에서 이미 만든 세 요소의 `style`만 변경한다. 이는 앱의 기본 시작 동작이나 공개 CSS 지원을 뜻하지 않는다. 지원 경계와 내부 호출 계약은 [C05.1 명세](../../../../spec/internal/0032-c05-runtime-custom-properties.md)를 따른다.
