# C19.1 computed color 구현 실패 관점 검토 20개

검토 범위는 `ComputedCssColor` 변환, Stylo cascade 추출, runtime mount 상속, snapshot profile 경계다. C19 계획 검토 항목을 재사용하지 않고 구현에서 깨질 수 있는 별도 불변 조건을 확인했다.

1. **Stylo 색 공간 variant 누락:** `ColorSpace`의 모든 현재 variant가 `ComputedCssColorSpace`에 명시적으로 매핑되며 wildcard 변환은 없다.
2. **missing component가 숫자 placeholder로 합쳐짐:** source component와 `missing_components`를 함께 보존하고 `none` fixture와 직접 단위 테스트가 있다.
3. **missing alpha와 실제 투명 alpha 혼동:** `missing_alpha`는 Stylo flag에서 읽고 실제 `alpha`는 별도 저장한다. missing alpha가 계산값 0인 단위 테스트로 구분한다.
4. **legacy 문법 정보 손실:** source 색 값의 `is_legacy_syntax()`를 carrier에 보존한다.
5. **원래 색 공간 대신 변환 공간이 노출됨:** source `AbsoluteColor`의 공간과 raw component를 저장하고 `converted_srgb`를 별도 필드에 둔다.
6. **두 번째 CSS parser의 해석 차이:** 문자열을 다시 parse하지 않고 Stylo `ComputedValues`와 `AbsoluteColor`를 직접 사용한다.
7. **`background-color: currentColor`가 미해결 상태로 남음:** cascade 결과가 `CurrentColor`이면 같은 element의 computed `color`를 사용하고 그 밖의 variant는 구체 오류로 실패한다.
8. **상속된 foreground가 자식에서 초기값으로 바뀜:** fragment fixture의 `color: inherit`가 ancestor의 Display-P3 source space와 변환 bytes를 보존한다.
9. **runtime mount의 body 상속 누락:** 수정 전 body 지정 색이 root에서 sRGB가 되는 회귀를 재현했다. synthetic document cascade 추가 후 Display-P3와 `[251,0,101,255]`가 통과한다. 수정 전 log는 `c19-runtime-body-inheritance-before-fix.log`다.
10. **invalid 선언이 앞선 유효 값이나 배경 기본값을 덮음:** 세 invalid control에서 `color: rgb(1, 2, 3)`과 transparent background가 유지된다.
11. **NaN/infinity가 snapshot에 도달:** source component와 직접 sRGB 변환 결과를 각각 유한성 검사하고 실패를 `UnsupportedComputedCssColor`로 반환한다. NaN source 입력 negative control도 오류를 확인한다.
12. **gamut mapping 전 변환값을 버림:** unbounded encoded sRGB를 직접 저장해 out-of-range 색을 확인하고 Chrome surface quantization에 재사용한다.
13. **CSS Color 4 mapping과 Chrome clipping 혼동:** 범위 밖 값은 Stylo `gamut_map_binary_search(Srgb)` 결과를 별도 저장한다.
14. **sRGB 범위 안 색을 불필요하게 Local MINDE 왕복:** 직접 변환값이 in-gamut이면 그대로 사용한다. HSL 무채색 50%에서 `[128,127,128]` 왕복 오차가 사라진다.
15. **Chrome-compatible 경로가 gamut mapping 결과로 오표기:** `converted_srgb_clipped_rgba8()`는 채널별 clip 후 byte 변환이며 CSS Color 4 output으로 문서화하지 않았다.
16. **8-bit 경계 반올림이 일관되지 않음:** byte 변환은 `round()`를 사용하고 fixture의 유효 비-system 색 37개를 exact 비교한다.
17. **mapped RGB가 범위 밖인데 성공 처리:** Local MINDE 출력의 RGB 모두 0..1인지 검사하고 위반 시 실패한다. 공개 carrier 값 변경 뒤 byte accessor를 호출하는 negative control도 NaN 및 `1.01`을 거부한다.
18. **alpha가 gamut 변환 후 유효 범위를 벗어남:** mapped alpha 0..1을 검사하며 byte accessor도 유한성/범위를 재검증한다.
19. **OS system palette를 일반 색 변환의 정답으로 취급:** 네 system row는 현재 Stylo Servo palette에 대해 별도 단언하고 Chrome/macOS 차이를 적합성 결과와 분리한다.
20. **C19 carrier가 layout/paint 입력으로 조용히 소비됨:** style-to-layout 및 runtime renderer에서 profile을 fail closed하고 각각 negative control test를 둔다.

## 판정

- 구현 중 확인한 body 상속 누락과 Clippy의 중복 분기 지적을 수정했다. body 상속은 수정 전 실패와 수정 후 통과 로그가 있다.
- 현재 전체 검증은 `workspace-test-final.log`, `workspace-clippy-final.log`, `cargo fmt --all -- --check`, `git diff --check`에서 통과했다.
- 이 carrier만으로 GPU paint, text, alpha compositing, hit-test, contrast 또는 system palette parity를 완료 처리하지 않는다.
