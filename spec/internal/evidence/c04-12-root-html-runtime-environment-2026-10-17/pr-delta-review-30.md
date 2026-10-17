# C04.12 root HTML 환경 cascade PR 변경 적대 검토

- 대상: 이 PR의 최종 변경 delta 전체(구현, 내부 ABI, 플랫폼 연결, 명세·상태 대장, 실행 근거)
- 검토 시점: PR 생성 전 최종 diff
- 검토 방식: 구현 검토 20개와 중복되지 않도록 리뷰 포인트를 변경 파일과 배포/소비 경계로 나눴다. 발견 사항을 수정한 뒤 최종 diff에서 다시 확인했다.
- 근거: [실행·빌드·테스트 README](README.md), [구현 실패 관점 검토](implementation-review-20.md), `git diff --check` 로그.

## PR delta 30개

1. **세션 API의 범위** — `set_root_html_environment`는 root HTML session의 내부 API로 추가됐고, 기존 `new()` 동작은 바뀌지 않는다.
2. **surface 단위 이름** — 입력 필드 이름에 `_px`가 있고 계약은 physical pixel/DPR을 명시한다. CSS viewport 출력은 `_css_px`로 구별된다.
3. **scale 의미** — `device_scale_factor`와 `page_scale`이 구조체에 별도 필드이며 DPR 합산을 하지 않는다.
4. **scheme ABI 값** — C 헤더에 input/output `0=light`, `1=dark`를 명시했다.
5. **초기 document scheme 설명** — `document_color_scheme`은 OS media input과 별개이며 UA 초기값 표본으로만 설명한다.
6. **ABI 오류값** — 성공, 인자/root session, 종료, CSS worker 오류를 API 주석과 Rust error mapping에 맞췄다.
7. **FFI 포인터 수명** — output writable 범위와 session free 동시 호출 금지를 주석에 적었다.
8. **출력 atomicity** — Rust FFI는 성공 snapshot에서만 구조체를 쓴다. 실패 경로의 부분 결과를 공개하지 않는다.
9. **session 생성 실패 경계** — HTML parse/import 오류는 V8 session을 게시하기 전 반환하는 기존 root document 계약을 유지한다.
10. **parsed tree 소유권** — parse 결과를 actor에 넘겨 같은 ParsedRootHtml에서 metadata와 HostDocument를 만든다. 두 번 파싱하지 않는다.
11. **CSS profile 분리** — root HTML 전용 profile만 viewport dimension media query와 `color-scheme` computed property를 받는다.
12. **기존 media 정책 회귀** — 기존 `supported_media_query_surface`는 viewport dimension 허용을 끈 경로로 유지되고 profile test가 경계를 확인한다.
13. **media token grammar** — root profile parser가 list, media type, `not`/`only`, `and`, block 조건의 순서를 확인한다. 알려진 malformed 입력은 negative test에 포함됐다.
14. **dimension 값 type** — viewport query는 CSS px와 numeric zero만 받고 음수·비 px 단위는 거부한다.
15. **미지원 property 거부** — root media allowlist를 열어도 CSS property allowlist는 별도로 검사한다.
16. **computed style projection 누락** — profile enum이 stylesheet admission, cascade output, layout extraction 모두에 연결됐는지 확인했다. aspect ratio 누락 분기는 수정되어 test를 통과했다.
17. **paint boundary** — root HTML profile은 paint 결과를 만들지 않는다. `has_paint`와 문서는 Canvas/HTML paint 미지원으로 일치한다.
18. **revision semantics** — viewport/DPR/media input revision과 동일 입력 no-op 의미를 API·테스트·로그에서 확인했다.
19. **stale snapshot 의미** — 환경 setter는 cascade 완료를 동기 보장하지 않는다. 문서가 report만으로 계산 완료를 주장하지 않는다.
20. **Android pixel source** — surface physical metrics와 density를 JNI에 전달한다. AVD report에서 CSS viewport와 DPR 계산값을 얻었다.
21. **Android theme source** — Android night-mode configuration을 native scheme 입력으로 매핑하고 로그에서 light/dark 복귀를 확인했다.
22. **Android startup race** — 비동기 bootstrap 이후 session 게시 뒤 현재 config를 UI thread에서 다시 읽어 초기 캡처값 경쟁을 보정한다.
23. **Android session lifetime** — 실패 시 새 session을 해제하고 activity 종료 뒤 session을 보관하지 않는 흐름이 기존 lock 경계와 연결된다.
24. **iOS pixel source** — host view bounds와 window screen scale을 surface pixel로 변환한다. iPhone Simulator report는 `402×874 CSS px`, DPR 3이다.
25. **iOS trait/resize source** — trait 및 layout 변화에서 최신 환경을 다시 전달하고 동일 환경은 publish하지 않는다.
26. **iOS bootstrap race** — 초기 session 게시 뒤 main queue에서 현재 환경을 강제 재적용한다. Simulator log에 최초/변경 revision이 남는다.
27. **iOS teardown** — `applicationWillTerminate`가 lock 아래 handle을 비우고 free한다. update 경로는 종료 flag를 보고한다.
28. **build/test 근거 연결** — Cargo package test, workspace Clippy, Android/iOS Simulator build 원본 로그를 evidence 폴더에 저장했다.
29. **화면 근거의 정확성** — HTML paint를 제공하지 않는 빈 native host에는 screenshot을 붙이지 않고, README가 이를 명시한다.
30. **공식 상태/계획 연결** — 0077, STATUS, 내부 index, 작업 계획이 새 evidence와 미완료 범위를 링크한다. C04.12 checkbox를 그대로 둔다.

## 검토 결론

최종 diff에서 병합을 막는 미해결 결함은 발견하지 못했다. 구현 중 발견한 profile output 누락 두 건, malformed media query 오수락, Android bootstrap 환경 race는 모두 수정된 변경 delta에 포함된다. 이 PR은 환경 입력·제한 cascade 계산 slice를 추가한다. page-scale transform, Canvas/native theme, HTML layout/paint 및 C04.12 전체 완료는 증명하지 않는다.
