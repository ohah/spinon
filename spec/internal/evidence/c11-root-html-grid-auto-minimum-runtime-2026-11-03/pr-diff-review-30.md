# C11.1 Root HTML Grid automatic minimum PR diff 검토 30개

검토 대상은 GitHub PR [#1109](https://github.com/ohah/spinon/pull/1109)의 `main` base `a7838b7de949f79ee7e39030845fb83ee3fafeb7` 대비 head `1ca9ad1bb29d554524112b263951228a72141440` diff다. 비교 patch는 94 files, 3,980 insertions, 53 deletions이며 SHA-256은 `ca755f370110b01493af0b7b4043725d6fff1f7e179e45dcfb67f1e11d629333`이다. 아래 관점은 앞선 구현/runtime 검토 20개와 다른 PR 통합·플랫폼 경계·문서 일관성을 대상으로 한다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | base 이후 unrelated commits가 함께 PR에 들어와 C11.1 변경 경계를 흐린다 | **통과** — base는 계획 merge tip이고 branch commits는 이번 구현·status note·iOS 기본 argument 경계 보정이다. |
| 2 | 매우 지저분한 checkout의 다른 WIP가 실수로 PR에 포함된다 | **통과** — PR diff의 94개 경로를 확인했다. 격리된 worktree를 만들지 않았고 unrelated untracked files를 stage하지 않았다. |
| 3 | feature가 public API/ABI 변경처럼 보이거나 새 FFI route를 요구한다 | **통과** — 공개 API signature와 C11.1 FFI entry point는 추가하지 않았다. Android는 기존 PixelCopy API, iOS는 opt-in surface reader를 재사용한다. |
| 4 | 내부 계약을 수정하며 pre-release 숫자 버전 정책을 깨뜨린다 | **통과** — internal contract 0059의 `0.1.0`을 유지하고 bounded input/error/simulator 한계만 추가했다. |
| 5 | 제한 slice를 C11 전체 완료로 세거나 공식 집계를 올린다 | **통과** — `spec/STATUS.md`의 C11 상위 checkbox는 unchecked이고 공식 `72/213`도 유지했다. PR note는 현재 PR 상태를 하위 기록으로만 추가한다. |
| 6 | 테스트 파일이 저장소의 test module 구조에서 빠지거나 중복 선언된다 | **통과** — 새 runtime/style-to-layout test modules가 각각 기존 test root에서 한 번씩 연결된다. |
| 7 | fixture source가 앱 payload와 독립적으로 재현되지 않거나 다른 파일을 가리킨다 | **통과** — CSS fixture는 `tests/fixtures/css/c11/`에 있고 runtime source index와 별도로 hash를 기록한다. |
| 8 | simulator capture 시 원본 bootstrap 파일을 남겨 다음 실행에 오염을 전파한다 | **통과** — `examples/bootstrap/index.html` 및 `root-app.js` hash가 evidence backup과 byte-for-byte 일치한다. |
| 9 | normative expected values와 Chrome 관찰이 뒤섞여 browser output이 oracle로 격상된다 | **통과** — spec-derived `expected.json`, Chrome observation JSON, screenshot을 분리했고 manifest가 모두 확인됐다. |
| 10 | 구현 중 발견한 2px root border gate 변경이 plan review 기록과 다르다 | **통과** — 계획과 plan review 20은 finite fr-maximum `minmax()` Grid root로 border 허용을 제한하고 item/다른 root gates를 유지한다고 맞춰 기록한다. |
| 11 | existing typed cascade 대신 CSS 문자열 재파싱 또는 parallel parser를 넣는다 | **통과** — projection은 existing Stylo `ComputedCssGridTrackSize::MinMax`를 받고 문자열 parser를 추가하지 않는다. |
| 12 | 기존 bounded `minmax()` support를 덮어써 다른 breadth를 넓히거나 좁힌다 | **통과** — 새 `Auto + finite non-negative Fraction` allow는 기존 typed track conversion 및 fail-closed validation 흐름에 연결된다. |
| 13 | default Root HTML path와 C11.4 fixture-only route가 혼합되어 지원 범위를 과장한다 | **통과** — plan·contract·runtime evidence는 `RuntimeRootHtmlTextPaintV1` 경로와 별도 fixture profile을 구분한다. |
| 14 | layout preflight 변경이 Taffy의 auto sizing contribution을 소비하거나 새 값을 주입한다 | **통과** — `root_html_gap` preflight는 auto-min track의 fixed extent만 0으로 계산하고 typed `Auto`를 Taffy에 남긴다. |
| 15 | `fr`-maximum border helper가 computed style projection 밖의 전역 border admission을 연다 | **통과** — helper는 Root HTML Grid computed style 분기에서만 root border gate와 결합하며 item border 경로를 바꾸지 않는다. |
| 16 | 변경 규모 대비 새 Rust 파일이 책임을 흐리거나 새 dependency를 요구한다 | **통과** — test 파일은 353·331·123 lines이고 기존 test tree 아래 분리되어 있으며 dependency manifest 변경은 없다. |
| 17 | Android intent extra가 default root HTML boot를 다른 fixture route로 전환한다 | **통과** — capture는 기존 default Root HTML route 조건과 build flag를 확인하고 별도 JNI/FFI host를 만들지 않는다. |
| 18 | Android PixelCopy 구현이 새 native readback ABI 또는 platform renderer를 추가한다 | **통과** — system `PixelCopy`와 현재 `SurfaceView`를 사용하며 native ABI나 renderer 변경은 없다. |
| 19 | auto-minimum과 fit-content percentage 동시 인수에서 Android가 하나를 조용히 선택한다 | **통과** — 충돌 AVD log는 argument error를 기록하고 capture dispatch를 하지 않는다. |
| 20 | Android asynchronous result가 이전 renderer/surface generation을 새 run에 섞는다 | **통과** — 요청·callback은 renderer handle, generation, dimensions, surface availability와 closing state를 확인한다. |
| 21 | Android PixelCopy와 전체 simulator screenshot이 같은 pixel 증거로 서술된다 | **통과** — `PixelCopy PNG`와 full simulator screenshot을 별도 파일·SHA·설명으로 둔다. Android renderer 한계는 ANGLE/SwiftShader로 명시한다. |
| 22 | Android default artifact가 capture를 켠 제품 build인 것처럼 검증된다 | **통과** — default BuildConfig 값은 false이고 native artifact에 fixture symbols가 없다. debug APK에 capture harness labels가 남는 점과 payload 부재를 별도로 기록했다. |
| 23 | iOS auto-minimum argument가 fixture flag 없이 default app에 남는다 | **수정 후 통과** — 검토에서 `#else`의 argument recognition 두 곳을 발견해 제거했다. default iOS Simulator build 성공, 8개 bundle files scan은 새 argument 문자열 0건이다. |
| 24 | iOS root HTML parser가 여러 capture mode나 unrelated Spinon flags를 함께 허용한다 | **통과** — opt-in parser는 root HTML run 하나만 요구하고 다른 `--spinon-` 인수가 있으면 fail closed한다. |
| 25 | iOS capture run ID가 길이·문자 제한 없이 output identity/path로 들어간다 | **통과** — 8–80 ASCII alphanumeric/`-`/`_` 형식 검증을 유지한다. |
| 26 | iOS CSS sample coordinates가 3× drawable에서 잘못된 physical pixel을 읽는다 | **통과** — runtime evidence는 CSS viewport, scale 3, 1206×2334 drawable과 four expected RGBA samples를 각각 기록한다. |
| 27 | iOS Metal readback을 UIKit compositor 이후 화면 결과로 보고한다 | **통과** — contract·PR body·evidence는 compositor 제출 전 drawable 한계를 명시한다. |
| 28 | default와 opt-in FFI archive 결과를 혼동하거나 APK 전체를 native scan으로 오해한다 | **통과** — default staticlib, opt-in positive control, Android native library, debug DEX labels를 각각 분리해 기록한다. |
| 29 | 실패한 run/기존 좌표·중복 run ID가 유효 cohort에 섞이거나 hash가 깨진다 | **통과** — 잘못된 좌표·duplicate PNG 거부는 성공 수에서 제외했고, 원본 로그/화면은 보존했다. Evidence SHA manifest는 통과한다. Full diff whitespace warning은 원본 Android logcat trailing spaces, ADB CRLF, Xcode build log의 padded export/blank lines에 한정되며 source·문서 diff check는 통과한다. |
| 30 | PR 설명·라벨·화면이 누락되거나 check가 없는 것을 CI 성공으로 주장한다 | **통과** — PR에는 `enhancement`, `area: layout`, `area: android`, `area: ios`가 있다. Agent Browser에서 Chrome/Android/iOS 3개 image가 모두 load됨을 확인했고 종료 후 browser process가 없었다. GitHub PR checks는 비어 있고 main required checks endpoint는 비활성이라 CI pass로 세지 않는다. |

## 판정

PR review 중 발견한 iOS default argument exposure는 수정하고 default simulator artifact로 재검증했다. Android debug harness diagnostic strings는 DEX에 존재하지만 request BuildConfig가 false이고 HTML boot payload 및 native fixture symbols는 없다. 이 차이를 plan, contract, evidence에 구분해 적었다. 다른 수정 사항은 찾지 못했다. 공식 C11 완료 판정은 이 PR의 범위가 아니다.
