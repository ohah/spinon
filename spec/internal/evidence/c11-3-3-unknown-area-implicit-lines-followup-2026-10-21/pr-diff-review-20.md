# C11.3.3 unknown-area implicit-line PR diff 검토 · 실패 관점 20개

**대상:** C11.3.3 제한 구현, Android/iOS 앱 fixture 경로, 물리 기기 캡처기, 상태·로드맵 문서. 각 관점은 다른 계약 경계나 실패 경로를 검사한다. 공식 C11.3.3 완료 판정은 이 PR 범위에 포함하지 않는다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 제한 fixture 결과가 공식 C11.3.3 완료 체크나 완료 집계를 올린다 | **PASS.** `spec/STATUS.md`의 C11/C11.3.3 체크와 완료 수는 그대로 미완료이며, 문서는 이를 명시한다. |
| 2 | 기본 Rust/API 경로가 C11.3.3 전용 resolver를 암묵적으로 호출한다 | **PASS.** 새 runtime profile과 style-to-layout 경로는 fixture-test feature에서만 컴파일되고 기존 wrapper는 기존 resolver를 유지한다. |
| 3 | 존재하지 않는 bare ident가 아닌 authored occurrence까지 생성형 fallback으로 바뀐다 | **PASS.** occurrence 지정 placement는 기존 explicit line metadata만 조회한다. |
| 4 | explicit area/name 또는 `-start`/`-end` alias가 bare-ident fallback에 가려진다 | **PASS.** area와 authored line 조회 후 남은 이름에만 implicit fallback을 적용한다. |
| 5 | named span이 opposite definite line이 아닌 축 방향으로 확장된다 | **PASS.** 공통 resolver가 row/column `SearchDirection`을 받고 trailing 방향의 shortage만 생성한다. |
| 6 | invalid template-area rectangle이나 track-count mismatch를 implicit track이 유효 입력으로 바꾼다 | **PASS.** 기존 cascade fallback과 정확한 `grid-template-areas` rejection이 유지된다. |
| 7 | 일반 numeric positive line이 새 implicit placement mode를 빌려 track을 늘린다 | **PASS.** 해당 axis에 generated named-line 결과가 없으면 implicit mode에서 거부한다. |
| 8 | leading 또는 negative extent가 trailing-only 구현 경계를 우회한다 | **PASS.** leading span/negative placement guard와 회귀가 계속 적용된다. |
| 9 | `grid-auto-rows:auto`가 row implicit sizing으로 조용히 인정된다 | **PASS.** `UnsupportedGridAutoRows`를 root node·property로 반환한다. |
| 10 | `grid-auto-columns:auto`가 column implicit sizing으로 조용히 인정된다 | **PASS.** `UnsupportedGridAutoColumns`를 독립 검사하며 positive fixed-px control도 있다. |
| 11 | 한 축만 fixed-auto-track 조건을 만족하는 조합이 mode 검증을 통과한다 | **PASS.** layout regression은 rows와 columns를 차례로 고정 px로 바꾸며 두 실패와 성공 제어를 검사한다. |
| 12 | implicit line arithmetic나 확장 track 수가 정수/Taffy 상한을 넘는다 | **PASS.** checked conversion, 기존 `GRID_MAX_TRACKS`/line validation과 사전 count guard를 유지한다. |
| 13 | implicit line을 만든 item이 sibling의 일반 numeric extent까지 허용한다 | **PASS.** 생성 결과는 axis별로 전달되고 unrelated sibling의 unmarked extent는 따로 검증한다. |
| 14 | definite template item의 auto-size/minimum 정책이 다른 mode까지 바뀐다 | **PASS.** 분기는 새 `TemplateGridImplicitTracks` variant에 한정하며 `TemplateGrid`·기존 profile 조건을 보존한다. |
| 15 | nested/hidden item이나 DOM source/paint 순서가 추가 track에서 이동한다 | **PASS.** 26-scene matrix의 nested-hidden scene, frame ID, paint order와 draw count를 비교한다. |
| 16 | Android나 iOS intent/argument allowlist가 고정 overflow scene을 빠뜨리거나 엉뚱한 route에 보낸다 | **PASS.** 양쪽 앱의 C11.3.3 scene allowlist와 run ID/scene 검증에 새 fixture가 포함된다. |
| 17 | PixelCopy/Metal 표본 ID가 scene, node, 좌표, RGB allowlist를 벗어난다 | **PASS.** 기본 scene의 기존 2개 표본과 overflow scene의 root/sibling/outside-item 3개 표본을 별도 allowlist로 확인한다. |
| 18 | 기존 자동 implicit-track rejection을 fixed-px 신규 성공 fixture와 혼동한다 | **PASS.** 기존 `unknown-area-implicit-tracks`는 `grid-auto-rows` error로 남고 새 `unknown-area-fixed-implicit-tracks`는 독립 positive scene이다. |
| 19 | Android physical matrix가 emulator·다른 모델·다른 API를 성공 대상으로 잘못 선택한다 | **PASS.** serial을 명시하고 model `SM-S731N`, API 36, `qemu=false`를 검사한다. 실행 `c1133-matrix-20261020T152039508Z-5886c27a`에서 26 scene geometry 오차 0 CSS px, base PixelCopy 2/2, overflow 3/3, Xclipse 940/Vulkan renderer를 기록했다. |
| 20 | 실기기 capture가 개인 알림/화면을 fixture evidence나 PR 이미지로 저장한다 | **FIXED/PASS.** 최초 전체 화면 캡처에 Android 알림 패널이 잡힌 것을 확인해 해당 PNG를 제거했다. 캡처기는 전체 기기 화면을 AVD에서만 저장하고, 실기기는 앱 SurfaceView PixelCopy만 보존한다. 회귀 검사가 이 구분을 고정한다. |

확인한 개선점은 모두 수정하거나 실패 경계를 문서화했다. 이전 implementation review 20개와 계획 review 20개는 별도 문서이며 이 PR diff 검토에 재사용하지 않았다.
