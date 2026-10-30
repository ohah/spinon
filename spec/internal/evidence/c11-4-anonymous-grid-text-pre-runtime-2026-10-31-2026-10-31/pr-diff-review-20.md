# C11.4 anonymous Grid text `white-space:pre` PR diff 실패 관점 검토 20개

검토 대상은 현재 브랜치에서 staged 된 제품 코드, fixture adapter, 계획·명세·상태 문서 및 실행 근거다. 계획 검토 20개와 구현/runtime 검토 20개를 재사용하지 않고 PR 통합 경계를 별도로 확인했다.

1. **CSS longhand 조합의 과잉 허용 — PASS.** Root HTML mapping은 `preserve + nowrap`만 `Pre`로 바꾸고 `preserve + wrap`은 거부한다. `NoWrap`과 `PreLine` mapping도 별도 유지한다. [mapping test](../../../../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/root_html_default_render/style.rs)
2. **기존 white-space 모드 회귀 — PASS.** Text engine은 `Pre`를 collapse=`preserve`, wrap=`nowrap`으로 추가하고 기존 `PreWrap`, `BreakSpaces`, `NoWrap`, `PreLine` 분기를 그대로 둔다. 두 플랫폼 회귀 행렬은 기존 26 scene을 포함한다. [Android matrix](android-matrix-status-sync.log) · [iOS matrix](ios-matrix-status-sync.log)
3. **반복 공백 collapse — PASS.** Normal control은 `XX  XX`를 50px, Pre 장면은 보존된 두 공백을 포함해 60px로 검증한다. 독립 expected value와 HostDocument intrinsic assertion이 모두 기록돼 있다. [scene 결과](README.md)
4. **min-content에서 보존 공백을 break opportunity로 오인 — PASS.** `XX XX` Pre 장면의 min/max contribution은 50px이고 한 줄이다. 동일 source의 normal control은 25px에서 두 줄이다. [focused test log](focused-tests.log)
5. **forced segment break를 nowrap 때문에 제거 — PASS.** Pre의 `XX\nXX`는 두 줄이며 column 20px·row 20px로 host, runtime 및 simulator 결과를 검사한다. [scene 결과](README.md)
6. **CRLF/form-feed 정규화 중 소스 provenance 손실 — PASS.** Text unit test는 CRLF와 form-feed의 두 줄 변환 및 UTF-16/paragraph UTF-8 source range를 각각 검증한다. [pre tests](../../../../crates/spinon-text-layout/src/tests/c11_4_pre.rs)
7. **탭 입력의 부분 결과 노출 — PASS.** normalization은 Pre tab을 구체적인 `UnsupportedPreTabProfile`로 반환하며 intrinsic 측정과 최종 paragraph layout 두 경로가 같은 오류를 기대한다.
8. **fixed-width overflow를 Grid item 크기와 혼동 — PASS.** Pre fixed-width scene은 root/item 25px, text line advance 50px, single line을 독립적으로 확인한다. glyph pixel clipping 여부는 주장하지 않는다. [Android/iOS simulator 결과](README.md)
9. **HostDocument validation이 enum만 허용하고 실행 경로에서 거부 — PASS.** `Pre`가 paragraph validation과 collection allowlist 양쪽에 추가됐고 C11.4 HostDocument scene tests가 intrinsic 값과 frame을 확인한다.
10. **root HTML style gate와 fixture profile이 불일치 — PASS.** Runtime adapter는 새 조합을 허용하고 anonymous-text validator는 신규 scene별 white-space, wrap, width, direct text child를 고정한다.
11. **fixture allowlist가 일반 author CSS로 확대 — PASS.** 고정 scene ID와 source/style profile 외의 입력은 기존 validator를 통과하지 않는다. 계약 문서는 이를 일반 CSS cascade/admission이 아니라고 명시한다. [계약 0059](../../0059-c11-grid-layout.md)
12. **scene ID 중복 또는 플랫폼별 장면 누락 — PASS.** FFI와 Rust runtime 목록은 각각 31개의 unique ID를 요구하며 Android/iOS 고정 matrix에도 신규 다섯 ID가 들어 있다. [FFI source matrix](../../../../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_text_items.rs)
13. **expectation이 Taffy 계산값을 되읽어 통과 — PASS.** Ahem 폭, CSS 규칙, 독립 산술을 작업 계획에 사전 고정하고 Chrome은 교차 관찰로만 사용한다. [계획](../../../../plan/c11-4-anonymous-grid-text-pre.md)
14. **scene dimensions와 intrinsic track kind 혼동 — PASS.** `max-content`, `min-content`, fixed 25px track의 종류와 row/column 크기를 runtime validator와 플랫폼별 assertion에서 각각 확인한다.
15. **앱 플랫폼 결과가 서로 다른 source를 실행 — PASS.** Android/iOS 실행 근거는 base commit과 tracked diff SHA를 묶고 각 설치된 APK/app hash가 로컬 build hash와 같다고 기록한다. [runtime 근거](README.md)
16. **AVD 재시도 실패를 숨기거나 정상 테스트로 계산 — PASS.** 첫 SurfaceFlinger/startup 실패가 별도 로그로 남고 cold boot 후 새 31-scene 전체 행렬이 통과한 사실을 분리해 기록한다. [실패 실행](android-matrix-final.log) · [최종 실행](android-matrix-status-sync.log)
17. **fixture ABI가 product artifact로 유출 — PASS.** 기본 macOS ARM64 archive에서 marker 0/5·host export 0, opt-in positive control에서 marker 5/5·export 2를 확인한다. 이 결과를 최종 앱 bundle 검사로 확대하지 않는다. [artifact 경계](artifact-boundary/README.md)
18. **simulator 캡처를 glyph 동작 증거로 오해 — FIXED.** 현재 경로의 glyph paint가 꺼져 있으므로 PNG는 PR에 포함하지 않는다. 계획 문구를 PR의 Android/iOS 실행 log 및 geometry 결과 링크로 고쳤다. 실제 glyph pixels는 검증 한계로 남긴다.
19. **전체 workspace 실패와 좁은 성공을 합쳐 보고 — PASS.** 262개 통과 뒤 C07.3 aspect-ratio 문자열 비교 한 건의 실패를 별도 명시하며 workspace 전체 통과라고 쓰지 않는다. [workspace log](workspace-test.log)
20. **좁은 slice를 C11.4/C11 완료 또는 공식 집계 증가로 과장 — PASS.** `spec/STATUS.md`, 0059 계약, 계획, 실행 근거 모두 C11/C11.4 미완료와 `72/213`을 유지한다.

최종 상태: 발견한 문서상 화면 증거 불일치를 수정했다. PR 화면 캡처는 glyph 동작을 입증하지 못해 제외하고, 두 simulator 실행 근거와 제한을 PR 설명에 연결한다.
