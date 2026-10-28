# C11.4 `visibility:hidden` 병합 후 문서 동기화 PR diff 검토 30개

구현 PR #738 병합 뒤 상태 대장, 집중·상위 계획, 공개 roadmap의 동기화를 검토한다. 구현/runtime 20관점과 구현 PR diff 30관점과 구분해, 제품 완료 상태·실행 출처·링크·배포 경계가 제출 변경에서 과장되거나 어긋나는 서로 다른 경로를 대조했다.

| # | 별도 문서 실패 가설 | 확인과 결과 |
| ---: | --- | --- |
| 1 | C11 CSS Grid parent checkbox가 제한 slice를 이유로 완료된다 | `spec/STATUS.md`의 C11 parent는 기존 `[ ]` 그대로다. 추가된 줄은 C11.4 하위 실행 기록뿐이다. |
| 2 | 공식 완료 총계가 72/213에서 증가한다 | status와 public roadmap에 `72/213`이 남고 diff에는 집계 변경이 없다. |
| 3 | 추가된 하위 status entry가 전체 C11.4 완료를 뜻한다 | entry에 exact four fixture scenes와 미지원 범위를 적고 parent checkbox는 미완료라고 명시한다. |
| 4 | PR #738의 merge SHA가 PR head commit 또는 다른 PR의 SHA와 혼동된다 | GitHub PR metadata와 main HEAD가 `bf4ab21eebc2f3739a7ae61de6511b92733b9464`로 일치한다. |
| 5 | PR merge 시각이 로컬 시간대로 잘못 기록된다 | GitHub `mergedAt` `2026-10-28T06:06:39Z`를 status, 계획과 roadmap에서 UTC 그대로 사용한다. |
| 6 | 계획 PR #737이 implementation PR #738의 merge로 잘못 표기된다 | PR #737은 별도 계획 PR이고 `7c2d939557408673f02dfdba7d4e31a2d6e53df2`로 먼저 병합된 것을 GitHub에서 확인했다. |
| 7 | 집중 계획 결과 링크가 저장소의 다른 계획 파일을 가리킨다 | `c11-4-mixed-flex-visibility-hidden.md`의 relative link와 status/public route를 실제 source path에서 확인했다. |
| 8 | 집중 계획이 일반 CSS `visibility` 지원을 선언한다 | plan은 네 fixed opt-in scene과 숨김 상속·`collapse`·arbitrary DOM 제외를 유지한다. |
| 9 | 네 visibility scene이 전체 matrix 14개로 오기된다 | expected inventory는 visibility hidden/visible control 4개, README와 manifests는 전체 14개로 서로 일치한다. |
| 10 | hidden span의 intrinsic contribution이 visible control과 다르게 기록된다 | 두 axis 모두 150px min-content와 `[150,40,90]px` tracks로 기록됐다. |
| 11 | hidden item frame record를 painted box 수로 오해한다 | plan/status는 hidden 4 visible boxes와 5 frame records를 분리하고 visible control은 5 painted boxes로 설명한다. |
| 12 | 화면 표본에서 hidden span이 실제로 paint되는 값을 hidden success로 오인한다 | README는 hidden overlap 파랑, visible control 보라를 서로 다른 exact 표본으로 구분한다. |
| 13 | Android 실행기가 실기기 또는 hardware GPU로 확대 해석된다 | 기기명/API·16 KB AVD·PixelCopy를 기록하고 real-device/hardware 범위를 미검증으로 남겼다. |
| 14 | iOS 표면 표본이 실기기 또는 compositor-after 확인으로 확대된다 | iPhone 17 Pro iOS 26.2 Simulator·Metal readback과 compositor 제출 전 한계를 구분한다. |
| 15 | 두 플랫폼 `14/14`가 하나의 합산 성공 수로 오해된다 | Android와 iOS 각자 14/14라고 표기한다. |
| 16 | 전체-screen pixel parity를 exact sample 결과에서 추론한다 | exact라는 표현을 선택된 PixelCopy/Metal sample로 제한하고 full-screen parity를 주장하지 않는다. |
| 17 | WPT 또는 일반 CSS admission이 검증 완료로 읽힌다 | focused plan, status, parent plan과 roadmap에 WPT/general CSS 미검증을 남겼다. |
| 18 | all-targets Clippy의 기존 실패가 숨겨진다 | 변경하지 않은 `generic_direct_text_image/fixture.rs:311`의 `clippy::map_identity`와 library-only 통과를 분리했다. |
| 19 | focused tests나 formatter가 실패로 바뀌거나 근거 없이 완료된다 | runtime/renderer focused tests, FFI route test, `cargo fmt`와 library Clippy를 evidence README의 결과와 대조했다. |
| 20 | GitHub PR check가 비어 있는데 CI 통과라고 쓴다 | 계획·상태·roadmap에 check rollup이 비었다고 쓰고 CI success를 주장하지 않는다. |
| 21 | release archive 경계가 Android APK/iOS IPA 전체 제외를 증명한다고 읽힌다 | artifact boundary와 status는 macOS ARM64 static FFI archive의 범위와 APK/IPA 제외 한계를 함께 둔다. |
| 22 | 제품 runtime archive와 opt-in positive control 결과가 뒤바뀐다 | artifact-boundary의 기본/fixture archive 해시와 symbol/payload 결과를 각기 링크했다. |
| 23 | fixture-only 변경이 public API나 versioned interface 추가로 기록된다 | PR #738 body와 집중 plan은 API/ABI 변경 없음, fixture opt-in이라고 설명한다. |
| 24 | matrix verification JSON 경로가 잘못되거나 이전 hidden matrix를 가리킨다 | public roadmap의 visibility verification JSON 경로가 현재 repo 파일에 존재하며 14-scene manifest를 확인한다. |
| 25 | Android screenshot 링크가 다른 run 또는 축으로 간다 | AVD latest run `055342204Z-d0ead38b`의 `android-row-visibility-hidden.png`를 확인했다. |
| 26 | iOS screenshot 링크가 다른 run 또는 축으로 간다 | iOS latest run `054646781Z-7759349f`의 `ios-row-visibility-hidden.png`를 확인했다. |
| 27 | PR 화면의 원래 branch SHA 링크가 rebase merge 뒤 불안정하다 | PR #738 body의 README·review·PNG URL을 실제 merge SHA `bf4ab21e...`로 변경하고 Agent Browser에서 이미지 로드/크기를 확인했다. |
| 28 | 공개 roadmap 내부 링크가 존재하지 않는 Rspress route다 | plan, status, evidence README/JSON/review, artifact boundary, 양 screenshot 링크가 각 원본 파일에 대응한다. |
| 29 | 정적 roadmap 편집이 다른 제품/roadmap section이나 count 로직까지 바꾼다 | diff는 C11.4 PR #738 card 추가만 포함하며 상태 집계 script와 다른 page source는 수정하지 않았다. |
| 30 | 문서 source merge와 Tailnet serving을 같은 작업으로 오인한다 | 이 문서 PR은 source만 갱신한다. Tailnet route의 hash/화면은 main merge 후 별도 route-only sync와 Agent Browser 확인이 필요하다. |

**판정:** 문서 diff에서 C11.4 공식 완료 승격, count 변경 또는 검증 범위 과장은 찾지 못했다. PR check rollup은 비어 있었고 all-targets Clippy baseline은 정확히 제한했다. Tailnet copy·HTTP hash·live page 확인은 아직 하지 않았으며 별도 post-merge 절차로 남긴다.
