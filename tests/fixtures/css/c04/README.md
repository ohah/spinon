# C04 기본 cascade 비교 입력

`cascade-input.v1.json`이 문서 트리, stylesheet 경로·순서, 비교 대상 속성, viewport와 관찰 대상의 SSOT다. 두 author stylesheet를 Chromium과 Stylo에 같은 순서로 전달해 stylesheet 간 source order도 확인한다. Chromium 결과는 `tools/css-reference/capture-c04-cascade.mjs`로 수집하며 기존 reference를 덮어쓰지 않는다.

픽스처는 UA 대 author origin, specificity, stylesheet 간 source order, `!important`, 대소문자 구분 없는 HTML inline style, 잘못된 inline 선언의 무시와 진단, 상속과 CSS-wide keyword, 고정 화면 media query를 검증한다. `font-size`, `font-weight`, `margin-top`, `display`, `color`의 계산값을 비교한다. 폰트 파일을 로드하지 않으며 실제 shaping·레이아웃·GPU 출력은 비교하지 않는다.

이 픽스처는 전체 CSS 지원 목록이 아니다. 명세 계약과 현재 완료 범위는 [C04 기본 cascade 내부 계약](../../../../spec/internal/0016-c04-basic-cascade.md)을 따른다.

## 제한 Flex margin → Taffy 입력

`margin-layout.v1.json`은 C04.6 기본 10개 case이고 `margin-logical-shorthand.v1.json`은 logical inline/block shorthand의 3개 보충 case다. 각 Chromium 캡처 도구는 별도 immutable reference에 입력·HTML·CSS·캡처 도구 hash와 Chrome 환경을 고정한다. `FlexMarginV1` 내부 profile은 C04.2의 제한 Flex 입력에 네 물리 margin computed value를 더한다. CSS px의 음수 값과 지원 가능한 shorthand를 Taffy로 전달하며 `%`, `auto`, 비-px computed value는 오류로 반환한다.

이는 내부 Rust 레이아웃 기능이다. 제품 runtime, CSSOM, 텍스트·Block formatting 전체, GPU 화면 및 Android·iOS 앱 적용은 포함하지 않는다. 상세 계약은 [C04.6 Flex margin 내부 계약](../../../../spec/internal/0027-c04-flex-margin-layout.md)을 따른다.
