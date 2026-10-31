# PR diff 실패 관점 검토 30개

범위: 현재 staged implementation candidate 전체 diff를 계획, source, tests, simulator logs, artifacts와 대조했다. 이것은 구현/runtime 검토 20개와 분리된 30개 관점이며 GitHub PR을 만들기 전 local review다. PR을 연 뒤 GitHub head diff가 이 검토한 patch와 같은지 확인하고 merge 전에 최종 state를 별도로 검증한다.

1. **계획 범위 누출:** 다섯 positive computed sizes와 두 intrinsic track 함수의 10개 scene만 추가한다. 공개 CSS API와 일반 author admission은 없다.
2. **독립 기대값 훼손:** expected JSON은 기존 schema, Ahem SHA, root profile, min/max widths를 유지하고 결과별 `lineCount`만 더했다.
3. **FFI generator 별도 책임:** scene source는 dedicated fixture module에 있고 `runtime_source`의 기존 dispatcher에서 먼저 확인한다. 사용자 product path를 바꾸지 않는다.
4. **값·ID drift:** Rust, FFI, Android, iOS와 expected JSON의 10 IDs/5 values를 대조했다. 양 플랫폼 실행기가 JSON case를 순회한다.
5. **실수로 ID를 허용:** FFI generator와 runtime allowlist가 모두 미등록 ID를 거부한다. unit control `font-size-6-min-content`도 없음/거부다.
6. **CSS serializer drift:** 허용 px serialization을 정확히 검사하며 `7.50px`처럼 같은 숫자의 다른 표기는 닫는다.
7. **typed snapshot mismatch:** CSS property와 typed float를 따로 비교한다. 값이 서로 다르면 paragraph style 생성 전에 실패한다.
8. **unknown-scene default:** review에서 발견한 기본 10px fallback 허점을 제거했다. legacy IDs만 명시한 allowlist로 10px를 받고 모르는 scene은 즉시 거부한다.
9. **유효하지만 비정수 크기:** 7.5/12.5px는 정수로 round하지 않는다. JSON trace는 37.5/62.5px track을 검사한다.
10. **invalid float:** NaN, infinity, zero, negative 및 unlisted size를 단위 test에서 거부한다.
11. **computed style 우회:** `ParagraphStyle.font_size`는 validated `font_size_css_px`를 사용하며 scene switch에서 임의 size를 다시 유추하지 않는다.
12. **line-height 상호작용:** computed line-height를 바꾸지 않고 absolute 10px를 유지한다. 줄 수만 변하며 줄 advance는 em에 맞는다.
13. **font metric oracle:** 기대 폭은 Ahem 400의 pinned `hmtx`에서 계산하며 Chrome 측정 폭을 독립 기대값으로 복사하지 않는다.
14. **text shape drift:** source는 정확한 direct Text `XX XX`이고 Ahem, white-space collapse/wrap, LTR, no inline run을 고정한다.
15. **min-content 계산:** 두 글자 단어의 `2em` contribution과 두 줄 결과를 다섯 font size 각각 별도 TextLayoutEngine에서 확인한다.
16. **max-content 계산:** 공백 포함 `5em`과 한 줄 결과를 다섯 size 각각 확인한다.
17. **track function swap:** 새 Rust test는 각 scene의 반대 intrinsic function을 실패시키고 올바른 function은 통과시킨다.
18. **row auto로 가장:** Rust test와 두 runtime matrices가 auto row의 20px/10px 결과 및 잘못된 fixed-row profile 거부를 확인한다.
19. **root/item frame 혼동:** capture runner는 root `20×rowHeight`와 item `trackWidth×rowHeight`를 분리 비교한다. max-content item은 root보다 넓게 기록된다.
20. **source provenance:** traces의 direct text source, node identity, source order, font registry revision 및 paragraph layout 연결을 양 플랫폼 adapter에서 검사한다.
21. **Android fixture flag 누락:** Android scene predicate와 expectation helper는 `SPINON_C04_RUNTIME_GPU_FIXTURE_TEST` 없이 scene을 열지 않는다.
22. **iOS fixture flag 누락:** Swift helper는 `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST_1` 아래에서만 값과 scene을 제공한다.
23. **Android log chunk truncation:** capture runner는 unique part count와 sequence completeness를 검사하고 손실 시 실패한다.
24. **iOS unified log escaping:** JSON `\\134` escape 복원 후 parser를 실행하며 scene별 trace chunk를 확인한다.
25. **불충분한 runtime assertions:** 화면 summary 외 computed paragraph font, intrinsic inputs, line advance, all tracks, rows, frames, revision을 검사한다.
26. **최종 코드가 아닌 app:** Android APK와 iOS executable hash가 설치 경로에서 각각 build hash와 같음을 확인한다.
27. **제품 archive fixture 누출:** default static archive scene scan은 0/10이며 positive-control archive는 10/10이다. 검사 범위를 static archive로 한정한다.
28. **화면 근거 과장:** 두 app screenshot에서 renderer surface가 검다. glyph paint 증거로 PR에 첨부하지 않고 README에 한계를 설명한다.
29. **증거 재현성:** 각 scene log, matrix, simulator metadata, archive JSON, build/test/clippy logs와 initial compile failure를 보존한다. trailing log whitespace 정규화는 README에 명시했다.
30. **제품 완료 상태 왜곡:** internal contract와 STATUS는 C11.4/C11 checkbox 및 `72/213`을 그대로 유지한다. 변경은 일반 WPT/전체 C11 완료를 주장하지 않는다.

리뷰 중 잘못된 track test의 첫 컴파일에서 local helper shadowing 오류가 발견되어 수정한 뒤 최종 테스트/Clippy를 통과했다. 초안 expected JSON에서 빠졌던 font/root metadata도 review 중 복구하고 기존 정보 위에 line-count oracle만 추가했다. PR 생성 후 remote diff/hash 일치 확인이 남아 있다.
