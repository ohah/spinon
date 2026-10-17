# Tailnet sync PR 변경 실패 관점 검토 30개

검토 대상은 C15.3 WGPU glyph paint 병합 기록, 상태·계약 route 선택 렌더, Agent Browser 화면 캡처와 Tailnet copy-only 제공 검증, 그리고 C19.1 Chromium 색상 판정 문구 보정이다. 각 항목은 별도 실패 경로 또는 계약 경계를 확인했다.

1. **병합 원본 오인** — C15.3 runtime paint의 결과가 PR #252 병합 이후 상태로 연결되는지 확인한다. **통과:** 로드맵, 계약 0072와 실행 근거가 #252에 연결된다.
2. **기준 revision 혼동** — 동기화의 source base가 #252 병합 commit인지 확인한다. **통과:** README의 base와 변경 대상 경로가 동일 worktree를 가리킨다.
3. **단계 섞임** — C15.1 layout, C15.2a geometry, C15.3 shaping/raster/paint가 한 기능으로 뭉쳐 보이지 않는지 확인한다. **통과:** 각 단계와 근거가 별도 항목으로 설명된다.
4. **상태 체크 성급한 완료** — C15.3 또는 C15 parent를 완료로 바꾸는 diff가 있는지 확인한다. **통과:** 공식 STATUS 체크 상태를 바꾸지 않는다.
5. **공식 완료 수 증가** — roadmap preview의 체크 수를 public API 완료 수로 오인할 수 있는지 확인한다. **통과:** 완료 수가 증가하지 않고 preview 의미를 공식 상태와 분리한다.
6. **C15 parent 전파** — 제한 paint slice가 C15 전체 완료처럼 읽히는지 확인한다. **통과:** DOM/style admission, layout consumers, overflow와 hit-test가 남았다고 쓴다.
7. **C16/C19 범위 전파** — glyph draw를 font property나 color support로 과장하는지 확인한다. **통과:** C16/C19 미완료와 미지원 경계를 유지한다.
8. **내부 계약 버전 변경** — 제한 구현을 이유로 계약 숫자 버전이 올라가는지 확인한다. **통과:** 0072는 0.1.0을 유지한다.
9. **색인 누락** — 신규 0072 및 실행 근거가 internal index에서 고아가 되는지 확인한다. **통과:** README index에 계약·실행·review 링크를 추가한다.
10. **공개/내부 경로 뒤바뀜** — 내부 계약을 public API 문서로 노출하는지 확인한다. **통과:** 계약 0072는 internal namespace에 남는다.
11. **roadmap mirror drift** — Tailnet root roadmap과 public/roadmap route가 서로 다른 custom source를 제공하는지 확인한다. **통과:** preflight가 두 경로에 같은 source bytes를 매핑한다.
12. **Rspress route 누락** — STATUS, 계획, 계약, evidence 중 하나가 선택 output에서 빠지는지 확인한다. **통과:** selected build output과 route map에 각 문서가 포함된다.
13. **부분 빌드 과장** — dead-link/image 검사를 끈 선택 빌드를 저장소 전체 docs 성공으로 보고하는지 확인한다. **통과:** strict subset 시도의 실패와 scoped build 한계를 분리 기록한다.
14. **도구 버전 drift** — preview render에서 Rspress 또는 lockfile version이 달라지는지 확인한다. **통과:** Rspress 2.0.22를 사용한다.
15. **C19.1 판정 링크 stale** — roadmap이 보정된 Chromium source analysis가 아닌 옛 파일을 가리키는지 확인한다. **통과:** pinned source analysis route를 연결한다.
16. **revision 미고정 소스 비교** — 최신 Chromium main을 155 oracle의 소스로 대체하는지 확인한다. **통과:** Chrome 155 Chromium revision과 branch 8059를 고정한다.
17. **Blink paint entrypoint 오독** — `toSkColor4f()`의 gamut-map 호출 여부를 누락하는지 확인한다. **통과:** feature state 인자로 `ToSRGB()`를 호출한다는 흐름을 기록한다.
18. **기본값과 런타임 Finch 혼동** — `FEATURE_DISABLED_BY_DEFAULT`를 과거 캡처의 실제 활성 상태로 간주하는지 확인한다. **통과:** 과거 Finch assignment가 보존되지 않았음을 명시한다.
19. **Rec.2020 helper를 CSS 알고리즘으로 오인** — `OklabGamutMap()`을 Local MINDE 구현이라고 부르는지 확인한다. **통과:** Rec.2020 polyhedral approximation으로 구별한다.
20. **뒤쪽 sRGB 단계 생략** — feature-off helper 생략을 최종 Canvas bytes와 동일시하는지 확인한다. **통과:** `XYZD50ToSRGB()` 이후의 전체 Canvas path/readback은 별도 미확인으로 둔다.
21. **CSS Color 4 선택 폭 축소** — Local MINDE만 유효한 결과라고 기록하는지 확인한다. **통과:** Local MINDE, EdgeSeeker, Ray Trace 허용 범위를 적는다.
22. **허용 값 사이 RGB 차이를 오류로 오인** — Stylo와 다른 픽셀을 곧바로 Chrome 위반으로 판정하는지 확인한다. **통과:** mismatch 관찰과 conformance verdict를 분리한다.
23. **역사적 oracle 파괴** — 새 해석에 맞춰 Chrome의 원본 `[0,142,200,255]` 값을 덮어쓰는지 확인한다. **통과:** Chrome 관측값과 Stylo Local MINDE 값을 별도 비교값으로 보존한다.
24. **스크린샷 stale cache** — 미리보기 캡처가 업데이트 전 문서를 보여주는지 확인한다. **통과:** Tailnet host의 cache-busting URL을 Agent Browser에서 열어 캡처했다.
25. **캡처 유형 과장** — Tailnet 문서 화면을 Android PixelCopy 또는 iOS Metal readback 증거로 오인하는지 확인한다. **통과:** 문서 화면은 runtime 픽셀 증거가 아니라고 쓴다.
26. **스크린샷만으로 상태 판정** — 렌더된 화면 이미지가 공식 STATUS를 대체하는지 확인한다. **통과:** 공식 STATUS route와 내부 계약 route를 별도로 캡처하고 공식 체크를 유지한다.
27. **이미지 접근성/출처 누락** — Markdown screenshot이 대체 텍스트나 안정된 파일 경로 없이 첨부되는지 확인한다. **통과:** 각 이미지에 설명형 alt와 evidence-relative path가 있다.
28. **복사 대상 탈출/링크 공격** — 상대 경로 traversal 또는 symlink target을 따라 외부 파일을 덮는지 확인한다. **통과:** source path 검사와 target component symlink preflight를 수행한다.
29. **기존 preview 파괴** — target 교체 전 백업이 없거나 삭제가 발생하는지 확인한다. **통과:** 변경 파일은 전용 backup에 보존하고 sync는 copy-only다.
30. **served bytes stale/mismatch** — 로컬 source hash만 확인하고 실제 Tailnet 제공값을 생략하는지 확인한다. **통과:** 최종 manifest URL별 SHA-256과 HTTP 응답을 대조해 증거 파일에 남긴다.
