# C11.1 named-area 자동 최소 크기 구현 PR diff 실패 관점 검토 30개

이 검토는 실제 구현 PR 변경 파일을 대상으로 하며 [구현/runtime 검토 20개](./implementation-runtime-review-20.md)와 별도로 수행했다. 검토 범위는 Rust projection·tests, Android/iOS capture adapter, Root HTML fixture, 내부 계약 문서와 실행 증거다.

| # | 실제 diff에서 확인한 실패 경계 | 판정 |
| --- | --- | --- |
| 1 | 새 public Rust/C ABI 또는 FFI export가 생김 | 없음. 변경 파일에 `spinon-ffi`, C header, exported symbol이 없다. |
| 2 | 별도 computed-style profile/constructor가 생김 | 없음. 기존 `RuntimeRootHtmlTextPaintV1` route를 쓴다. |
| 3 | named-area auto matrix helper가 다른 caller에 연결됨 | 없음. 단일 named-area validator caller가 축을 분류한다. |
| 4 | matrix helper가 auto-repeat·line name이나 임의 track list를 같이 허용 | 없음. track 길이, order, neighbor size가 exact compare다. |
| 5 | 0-minimum breadth 비교가 음수·NaN·infinite minimum을 허용 | 없음. exact zero와 finite non-negative factor만 인정한다. |
| 6 | factor 0이 기존 rejection matrix로 인해 auto-min boundary에서 빠짐 | 없음. `auto 0fr` columns/rows control이 직접 검증된다. |
| 7 | named-area minmax 거부 문구만 바뀌고 out-of-matrix track이 승인됨 | 없음. 기존 matcher에서 새로운 exact axis shape 외 조합은 거부한다. |
| 8 | no-area explicit `minmax(auto,fr)` 경로까지 named-area exception 적용 | 수정했다. fixed implicit row 조합 guard와 19개 profile tests를 diff에 포함했다. |
| 9 | fixed implicit row guard가 기존 px/percentage/fr implicit cases를 막음 | 없음. `root_html_profile` 19/19과 runtime Grid 68/68이 통과했다. |
| 10 | root exact width/height가 content-box/border-box를 혼합 | 없음. area profile size와 border checks는 구분되어 있고 runtime frame은 border box다. |
| 11 | style projection의 item lookup에서 요소를 찾지 못했을 때 잘못된 default 적용 | 없음. `MissingComputedElement`와 `MissingComputedProperty` 오류를 유지한다. |
| 12 | item 배열이 header/side/content area 순서와 대응하지 않음 | 없음. alias index를 확정하고 `area_items`에 동일 index로 기록한다. |
| 13 | changed class/id가 supports 여부를 결정 | 없음. renamed identity positive control이 same frame을 낸다. |
| 14 | item의 min/max/aspect override가 silent clamp로 성공 | 없음. node/property diagnostics 및 negative test가 있다. |
| 15 | root border/box-sizing/aspect override를 item 조건으로 대신 검사 | 없음. root와 item 검증 함수가 분리되어 있다. |
| 16 | gap·padding·margin이 auto minimum 기대값을 변경 | 없음. fixed named-area 기존 gate와 test fixture가 zero로 유지한다. |
| 17 | visible overflow sample이 border 안쪽을 재측정 | 없음. CSS-relative sample은 `(110,30)` 또는 `(50,110)`이며 native log의 device coordinates도 보존했다. |
| 18 | screenshot 전체 색 경계를 numeric geometry oracle로 확대 | 없음. README가 full-frame geometry를 Rust assertion으로, screen capture를 visual/surface sample로 분리한다. |
| 19 | Rust test가 다른 style/layout revision frame을 참조 | 없음. runtime test가 completed layout key와 cascade key를 비교한다. |
| 20 | rejected runtime input이 이전 completed scene을 새 결과로 오인 | 없음. 실패 상태·완료 결과 부재·requested key를 따로 검증한다. |
| 21 | 런타임 테스트가 실제 기본 Root HTML entry 대신 fixture FFI route를 호출 | 없음. `RuntimeSession::new_runtime_gpu_with_root_html` 기본 route를 쓴다. |
| 22 | 런타임 테스트의 문자열 교체가 HTML 구조 marker를 잘못 바꿈 | 없음. fixture 내 id의 style 값만 바꾸고 각 장면을 정적 source/session으로 만든다. |
| 23 | HTML fixture가 다른 Root HTML/CSS meta precondition을 바꿔 결과를 오염 | 없음. fixture title/meta 및 body defaults가 유지되고 scene script는 네 inline style만 대체한다. |
| 24 | capture script가 사용자 bootstrap source를 덮고 복구하지 못함 | 없음. run 전 `cmp`, trap restoration, run 후 `cmp` 및 원본 SHA 재확인을 수행했다. |
| 25 | capture script output path override가 원래 실행 결과를 덮어씀 | 없음. 최종 source 빌드/화면은 `pass-final/`에 분리해 최초 산출물을 보존했다. |
| 26 | Android 임의 `c11na-` run ID가 유효 scene sample로 해석 | 없음. 등록 scene만 인정하고 미등록 `c11na-` prefix는 fail-closed다. |
| 27 | Android density/SurfaceView coordinate sample이 device screenshot scale에 묶임 | 없음. PixelCopy callback sample 로그와 raw surface PNG를 별도 저장하며 density 2.625를 기록한다. |
| 28 | iOS scene route가 feature-off executable에 capture argument/string을 남김 | 없음. 새 static selector도 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST_1` compile guard 안에 있고 기본 binary marker scan은 비었다. |
| 29 | iOS drawable readback을 OS compositor 후 최종 pixel로 기술 | 없음. contract, README, log 설명 모두 compositor 이전 Metal readback으로 한정한다. |
| 30 | evidence나 PR 설명이 공식 완료·WPT·실기기 상태를 과장 | 없음. `spec/STATUS.md`를 수정하지 않았고 `72/213`, C11.1/C11 incomplete, AVD software/iOS pre-compositor limits를 계속 명시한다. |

추가 확인: `git diff --check`, scoped Clippy `-D warnings`, rustfmt, named-area/style/runtime regression logs, final simulator run IDs와 opt-in feature-off artifact 결과는 이 evidence folder의 README와 manifest에 연결했다. 이 diff review는 계획 문서의 20개 검토를 재사용하지 않는다.
