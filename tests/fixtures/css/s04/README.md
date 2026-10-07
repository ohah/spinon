# S04 CSS 배경색 fixtures

`flex-paint.v1.json`과 `.css`는 기존 가로 분수 Flex fixture를 정의합니다. `asymmetric-y.v1.json`과 `.css`는 별도의 세로 배치 fixture입니다. 각 Chromium computed-style·geometry 기준은 `tests/fixtures/css/references/`에 고정하며, 기존 가로 fixture와 reference는 수정하지 않습니다.

비대칭 y fixture는 `301×65 CSS px`에서 부모 column Flex, 자식 높이 `12/18/24px`, `3px` 간격을 사용해 자식 y `0/15/36px`을 만듭니다. readback은 x `0/150/300`, y `0/11/12/14/15/32/33/35/36/59/60/64`의 36개 표본을 검사합니다. 태그별 네이티브 View, 일반 CSS 지원, 텍스트, 입력 이벤트, 제품 앱 runtime 또는 GPU 표시 완료를 뜻하지 않습니다.

`layout-revision-gate.v1.json`은 이 화면 fixture와 별개인 snapshot admission 입력입니다. 기준 revision tuple을 허용하고 style revision 변경, environment revision 변경, environment revision을 재사용한 viewport 변경을 전체 거부해야 합니다. [비교 기준](../../../../spec/internal/evidence/s02-layout-revision-precomparison-2026-10-04.md)과 [실행 근거](../../../../spec/internal/evidence/s02-layout-revision-gate-2026-10-04.md)를 확인합니다. 이 검증은 제품 runtime 경합이나 GPU queue 폐기를 대신하지 않습니다.

`hit-test.v1.json`은 비대칭 y 화면 fixture의 내부·경계·간격 좌표를 Chromium `document.elementFromPoint()`와 비교합니다. Chromium capture와 Rust 단위 테스트가 fixture 좌표·예상 ID·reference 결과를 비교합니다. Android·iOS 내부 화면은 터치 좌표를 같은 불변 snapshot에 넣고 진단용 NodeId를 표시합니다. 일반 CSS hit-test, 화면 표시 완료 frame, DOM 이벤트·JavaScript callback 지원을 뜻하지 않습니다. 실제 simulator 실행은 [S04.9 근거](../../../../spec/internal/evidence/s04-hit-test-platforms-2026-10-07.md)를 참고하세요.

```sh
mise exec -- bun run css:reference:s04
mise exec -- bun run css:reference:s04-y
mise exec -- bun run css:reference:s04-hit-test
```

캡처 도구는 고정 viewport·device scale factor·media preference로 로컬 Chromium을 실행합니다. 자식 순서, computed property, fixture별 고정 sample, 좌표 오차·색상 판정 조건이 fixture와 다르면 실패합니다. 새 결과 파일은 덮어쓰지 않습니다. `SPINON_REFERENCE_OUTPUT_DIR=/tmp/s04-reference mise exec -- bun run css:reference:s04-y`로 별도 경로에 재현할 수 있습니다. `width`와 `height`는 Chrome의 used value와 Stylo의 pre-layout computed value가 다르므로 문자열 대조에서 제외하고, 최종 geometry를 별도로 비교합니다.
