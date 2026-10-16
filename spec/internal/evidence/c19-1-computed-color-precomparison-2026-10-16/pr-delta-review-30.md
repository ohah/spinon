# C19.1 computed color PR 변경 관점 적대 검토 30개

이번 검토는 계획이나 내부 carrier 구현 체크리스트를 반복하지 않고, 실제 PR diff의 호출 경계·회귀 가능성·문서와 근거 연결을 대조했다.

1. **변경 범위가 C19.1 밖으로 번짐:** Rust diff는 Stylo cascade, snapshot profile, 두 adapter의 fail-closed boundary와 그 테스트에 한정된다.
2. **제품 FFI/ABI가 조용히 확장됨:** FFI, C header, Swift/Kotlin, GPU renderer 구현에는 새 진입점이나 symbol을 추가하지 않았다.
3. **새 내부 API가 다른 색 API 이름을 덮음:** public crate export는 `compute_c19_computed_color_cascade_with_stylesheets`로 범위를 드러낸다.
4. **내부 계약 버전이 임의 상승:** contract 0068은 프로젝트 규칙에 따라 `0.1.0`을 유지한다.
5. **기존 profile snapshot에 색 carrier가 채워짐:** extraction helper는 C19 외 profile에서 두 새 필드를 `None`으로 반환한다.
6. **기존 runtime paint 값이 바뀜:** C19는 별도 `ComputedStyleProfile`이며 background paint profile arm을 재사용하지 않는다.
7. **새 profile에서 layout 속성을 layout 결과로 소비:** style-to-layout은 C19를 unsupported profile로 거부하고 테스트가 UA profile과 함께 확인한다.
8. **renderer가 C19 값을 빈 배경으로 그려 성공 처리:** runtime renderer의 allowlist는 C19를 포함하지 않으며 새 negative control이 `UnsupportedRuntimeProfile`을 확인한다.
9. **현재 `background_paint`에서 color carrier로 조용히 변환:** C19의 `computed_background_for_profile` 결과는 `(None, None)`이다.
10. **cascade에서 foreground property를 수집하지 않음:** C19 profile은 block paint property selection을 사용하고 computed `color`를 typed extraction 한다.
11. **background-color가 `currentColor`에서 오류 또는 초기색으로 바뀜:** typed extraction은 CSS `CurrentColor` 변형만 computed `color`로 해석한다.
12. **body/html rule이 mount root의 inherited color에서 사라짐:** runtime mount synthetic cascade를 회귀 테스트로 재현하고 수정했다. layout geometry check는 C19에 적용하지 않는다.
13. **synthetic context 추가로 기존 layout profile 검증이 빠짐:** html/body geometry validation은 기존 `is_runtime_layout_profile`에서만 수행한다.
14. **synthetic root font size가 비유한 값으로 설정됨:** C19도 기존 finite/non-negative guard를 지난 뒤 device root size를 갱신한다.
15. **profile 지원 목록과 incremental selector 불일치가 성공으로 오인됨:** C19 incremental reuse가 선택되지 않으면 helper 계약에 따라 `None`으로 full-cascade fallback을 요청한다.
16. **Style-to-layout exhaustive boundary에서 layout 성공:** `C19ComputedColorV1`을 명시적인 unsupported arm에 추가하고 테스트를 확장했다.
17. **Style-to-render dispatch가 `background_paint: None`을 채택:** C19의 profile allowlist 거부를 독립 renderer test로 확인했다.
18. **새 오류를 문자열만 보고 분류하기 어려움:** `UnsupportedComputedCssColor`는 node, property, reason을 구조화한다.
19. **unsupported computed color 변형이 기본값으로 숨음:** `background-color`가 absolute/currentColor 외 변형이면 property 이름과 함께 `Err`로 중단한다.
20. **invalid declaration 진단을 잘못 요구해 정상 cascade를 실패:** fixture는 이전 valid `color`와 transparent background를 검사하고 `CssParseDiagnostic` 유무를 pass 조건으로 삼지 않는다.
21. **44행 중 일부가 조용히 빠짐:** test가 fixture row와 node 수 각각 44를 단언한다.
22. **정상 색과 invalid control의 exact 기준 혼합:** 41 valid/3 invalid 입력을 각각 계산하고 invalid 3개는 이전값/transparent fallback을 확인한다.
23. **foreground field가 background 확인만으로 덮임:** 새 assertion이 모든 행의 `computed_color` 변환 bytes를 별도로 검사한다.
24. **background color의 실제 출력이 확인되지 않음:** background 변환은 비-system 37행 Chrome bytes, 네 system Stylo palette와 분리 비교한다.
25. **P3 gamut edge에서 mapping과 clipping 출력이 같다고 단정:** 단위 테스트가 `[255,100,0,255]` Local MINDE와 `[255,87,0,255]` clipped output을 함께 검증한다. 별도 guard control은 NaN와 out-of-range mapped byte를 오류로 거부한다.
26. **WPT source manifest를 실행 결과로 과장:** README는 manifest를 고정 자료로만 기록하고 이 PR의 WPT 실행은 없다고 명시한다.
27. **이미지를 앱 화면으로 오인:** 저장된 PNG는 Agent Browser에서 캡처한 Chrome reference fixture이며 Spinon Android/iOS 동작 화면이라고 표기하지 않는다.
28. **기준 환경 재현 자료 해시 누락:** fixture HTML, oracle JSON, screenshot, WPT source manifest SHA-256을 README에 기록했다.
29. **검증 중 실제 실패를 숨김:** 최초 workspace integration compile 실패, Clippy 초기 지적, body inheritance 재현 실패를 각기 보존하고 수정 후 log를 별도 둔다.
30. **공식 상태를 단일 slice 통과로 과장:** `spec/STATUS.md`의 C19 checkbox는 미완료이며 opacity, GPU, text, hit-test, contrast 경계가 그대로 남는다.

## 판정

- PR diff는 현재 C19.1 internal cascade carrier 범위와 일치한다. body inheritance 누락은 재현 후 수정했고, Clippy의 중복 분기는 단순화했다.
- 전체 workspace tests, workspace Clippy, formatter, diff check가 통과했다. Chrome reference screenshot은 oracle 화면 근거로만 제공한다.
- Tailnet roadmap 동기화와 GitHub PR 검토는 merge 뒤 별도 상태로 확인한다. 이 파일은 merge 완료를 주장하지 않는다.
