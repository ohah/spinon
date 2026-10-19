# C11.4 텍스트 고유 크기 PR diff 적대 검토 30개

2026-10-19 PR 후보 diff를 구현 리뷰와 별개의 실패 관점 30개로 검토했다. 각 항목은 변경된 코드, 계약, 비교 결과 또는 artifact/evidence 링크를 대상으로 한다.

## diff 관점별 결과

1. **변경 범위** — C11.4 opt-in fixture, app 검증 경로, FFI 진입점 및 그 증거에 한정된다. 일반 문서·제품 CSS allowlist를 넓히지 않는다. 통과.
2. **기존 fixture 보존** — 전체 C11.4 scene 수는 기존 7개와 text 10개를 합한 17개다. 이전 fixed-item 장면 7개가 유지된다. 통과.
3. **플랫폼 목록 동기화** — Rust FFI, Android, iOS의 17개 scene ID 집합이 같다. 미지원·오타 ID는 route 경계에서 거부한다. 통과.
4. **scene별 DOM 구조** — 각 text fixture가 root, item, text node의 세 단계이고 direct-leaf만 입력한다. 추가 element/형제는 허용하지 않는다. 통과.
5. **Ahem text 측정** — `XX XXXXXXX` 입력의 min/max width와 height는 70×20/100×10이다. AVD, iOS 및 Chrome 결과가 기록돼 있다. 통과.
6. **`auto` track** — `auto-text-max-content`가 100px column, 100×10 item을 낸다. trace와 oracle row가 이를 확인한다. 통과.
7. **`min-content` track** — normal wrap 장면이 70px column과 두 line item을 낸다. 통과.
8. **`max-content` track** — normal wrap 장면이 100px column과 단일 line item을 낸다. 통과.
9. **`fit-content(40px)`** — 제한값 아래에서 minimum contribution 70px이 보존되는 70×20 결과를 확인했다. 통과.
10. **`fit-content(calc())`** — 100px grid에서 `10% + 5px`가 15px 제한으로 계산되고 8자 `break-all` item은 15×80이다. 통과.
11. **content-box 변환** — inline padding 4px와 좌우 border 1px로 content 70px이 item 80px이 되는 edge scene을 확인했다. 통과.
12. **`min-width`** — 90px 지정이 min-content contribution보다 커도 item 폭 90px로 반영된다. 통과.
13. **`max-width`** — 40px 지정이 긴 word의 preferred contribution을 제한해 item 폭 40px을 만든다. 통과.
14. **`word-break:break-all`** — break opportunity가 열려 8개 글자의 min-content width 10px, height 80px이 된다. 통과.
15. **`nowrap`** — 공백이 있어도 줄바꿈 없이 width 100px, height 10px이다. 통과.
16. **computed alignment profile** — Chrome precomparison의 `align-content:start`와 app runtime `normal` 차이를 같은 runtime-equivalent profile로 맞춘다. 서로 다른 computed style을 비교하지 않는다. 통과.
17. **전체 frame 비교** — root/item의 x/y/width/height가 Chrome, Android, iOS에서 10/10 exact다. 통과.
18. **node·paint order** — node ID 순서와 paint order가 세 경로에서 10/10 exact다. 통과.
19. **text input provenance** — min/max-content width·height 네 값, `kind=text`, node/grid/sourceOrder 및 `fontRegistryRevision=1`이 10/10에서 일치한다. 통과.
20. **trace 재조립** — 모든 Android/iOS part 번호가 채워져 있고 JSON을 완전히 파싱했다. chunk 손실로 인한 비교는 없다. 통과.
21. **trace 정규화 범위** — cross-platform canonical equality는 `AutomaticMinimum` Debug 문자열 내부 공백만 normalize한다. 숫자와 stage data는 원형 그대로 비교한다. 통과.
22. **byte 차이 보고** — 8/10 trace는 byte-identical이다. 두 나머지는 `auto-text-max-content`, `min-content-text-nowrap`이며 차이는 Rust Debug whitespace뿐이다. 통과.
23. **Android PixelCopy ID** — 3개 표본 로그가 최신 `c114-android-jsoninput-min-content-text-break-all-20261019` run에 속한다. 이전 `heightcheck` ID를 최신 비교로 표시하지 않는다. 통과.
24. **iOS Metal ID** — 3개 표본이 현재 `c114-ios-jsoninput-min-content-text-break-all-20261019` run에 속하며 drawable readback과 화면 캡처를 구분한다. 통과.
25. **root 밖 overflow 조건** — y=45 CSS px의 item 표본은 40px root 바깥이고 root control 표본은 별도 색으로 확인한다. 표본은 각 플랫폼 3/3 exact다. 통과.
26. **screenshot 해석** — PNG는 shell과 box geometry를 확인하며 fixture renderer text glyph, border stroke 또는 iOS compositor 이후 출력을 증명한다고 주장하지 않는다. 통과.
27. **C ABI 안전 조건** — pixel 표본 count, null, overflow, fallible allocation과 직렬 renderer 경계를 검사하고 header declaration의 feature guard를 맞췄다. 통과.
28. **제품/fixture 경계** — default release artifact 부재와 opt-in artifact 포함 결과를 따로 기록했다. 제품 기본 artifact에 fixture payload가 섞이지 않는다. 통과.
29. **회귀·재현 자료** — workspace test/Clippy/rustfmt, focused Rust/FFI checks, Android/iOS build와 실행 로그가 각각 별도 파일로 남는다. 통과.
30. **상태·제한·링크 정합성** — STATUS, plan, 내부 계약과 README는 C11.4를 미완료로 유지하고 미검증 항목을 구체적으로 나열한다. README screenshot/oracle/trace/hash와 review 링크가 실제 evidence 파일을 가리키는지 확인한다. 통과.

## 검토 결론

이 diff는 고정된 direct-leaf Ahem text contribution matrix의 구현·검증 근거를 추가한다. 리뷰에서 수정한 앱 input assertion의 report 전역 문자열 false positive는 현재 코드와 simulator log에서 제거됐다. 이전 iOS 수집 predicate 누락은 evidence 수집 오류였고 현재 Metal log의 JSON-input run 3/3 기록으로 교체했다. 구현이 일반 text/image contribution, glyph paint, WPT 또는 전체 C11.4 지원을 뜻하지 않는다는 제한은 유지한다.
