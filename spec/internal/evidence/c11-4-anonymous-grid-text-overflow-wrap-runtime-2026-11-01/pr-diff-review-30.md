# PR #948 diff 적대적 검토 30개

2026-11-01, PR #948의 실제 GitHub diff 135 files를 검토했다. GitHub head `70b1713b51b7c74d9b34b1430b34fa822e5b6161`과 local `origin/main...HEAD`의 파일 목록은 모두 같았다(135/135, 차이 0). 계획 PR의 30관점과 implementation/runtime 20관점을 재사용하지 않고 API/소유권, track semantics, 앱 adapter, fixture 경계, evidence/publication 실패 경로를 각각 점검했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 새 struct field 때문에 기존 anonymous-text/Grid fixture 기본값이 달라지는가 | `OverflowWrapMode::Normal`이 `ParagraphStyle::default()`이며 기존 명시적 struct initializer도 모두 `Default::default()`를 둔다. text-layout 83, layout 326 및 runtime suite 통과. |
| 2 | 내부 계약이 임의 overflow-wrap 지원으로 넓어졌다고 오해하게 만드는가 | enum 문서, plan, 0059/0070은 C11.4 고정 Ahem direct-text fixture만 허용한다. 숫자 버전과 일반 C15 admission은 유지했다. |
| 3 | mode가 snapshot 생성 뒤 바뀌거나 동일성 비교에서 누락되는가 | `TextLayoutSnapshot`의 definite-width 생성과 `PartialEq` 모두 mode를 저장·비교한다. 세 mode test 통과. |
| 4 | empty paragraph에서 computed style identity가 사라지는가 | empty layout fast path도 `input.style.overflow_wrap`을 snapshot에 넣고 별도 test가 BreakWord 보존을 확인한다. |
| 5 | `anywhere` 또는 `break-word`가 Parley enum으로 잘못 mapping되는가 | 세 `OverflowWrapMode`가 Parley `Normal|Anywhere|BreakWord`로 각각 mapping된다. Ahem fixed-width lines와 Android/iOS trace에서 확인. |
| 6 | BreakWord min-content 예외를 일반 definite layout에도 적용하는가 | Normal override는 intrinsic pass에만 주며 `build_parley_layout()`과 snapshot은 computed BreakWord를 유지한다. `XXXX`는 min-content 40px, fixed 10px은 4줄. |
| 7 | min-content 재생성에서 text/source mapping이 달라져도 조용히 통과하는가 | 재생성된 normalized text가 첫 shaping 결과와 다르면 `InvalidGlyphLayout`을 반환한다. 허용 profile에는 한 direct text source만 있다. |
| 8 | max-content 결과가 min-content 전용 override에 오염되는가 | computed mode layout을 무제한 폭으로 먼저 측정하고, BreakWord Normal override는 min-content 전용이다. 세 mode의 max-content 40px/10px 일치. |
| 9 | `white-space:pre`/`break-spaces`의 기존 min-content 경로가 새 변수 분리 때문에 바뀌는가 | `min_layout`에서 기존 `PreWithTabSize`와 `BreakSpaces` 분기를 유지한다. runtime/text-layout 전체 regression 통과. |
| 10 | 고정 Ahem ASCII oracle와 다른 font/script를 지원한다고 주장하는가 | FFI generator와 runtime validator는 정확한 `XXXX`, Ahem 10px, 3 keyword, 3 track 조합만 만든다. plan과 STATUS도 일반 조합/WPT를 제외한다. |
| 11 | 미등록 scene이 기본 `normal` 값으로 떨어져 실행되는가 | FFI `runtime_source()`는 9개 ID에서만 `Some`을 반환한다. runtime scene allowlist도 같은 9개며 미등록 ID test 통과. |
| 12 | valid keyword지만 scene expected value와 다른 값을 받아들이는가 | computed property와 typed enum 양쪽을 scene expectation에 대조한다. `normal` scene에 `anywhere`를 넣은 negative test는 layout publication을 거부한다. |
| 13 | property 누락 또는 허용 밖 keyword가 `normal` fallback이 되는가 | missing/unsupported 값은 style failure다. runtime validator negative tests 및 runtime fixture suite 통과. |
| 14 | computed style과 paragraph가 다른 document revision/owner에서 섞이는가 | computed root style, anonymous child text, typed `ParagraphStyle`, trace/layout revision을 한 calculation으로 확인한다. Android/iOS 9 scene logs 모두 검사한다. |
| 15 | 예상하지 않은 child/text source가 이 allowlist에서 anonymous direct text로 처리되는가 | generator가 child text 하나만 생성하고 runtime direct-child validator가 정확한 topology를 확인한다. source generator test는 text/element 개수도 검사한다. |
| 16 | root CSS property만 확인하고 text paragraph에 mode가 전달되지 않는가 | runtime은 computed property에서 enum을 만들고 `paragraph_style.overflow_wrap`에 지정한다. runtime integration test가 input style과 output snapshot 둘 다 비교한다. |
| 17 | scene 중복/누락으로 matrix coverage가 사실상 9 미만인가 | FFI, runtime catalog, runtime integration test 및 JS boundary test가 각각 9개 ID 고유성과 전체 조합을 검사한다. |
| 18 | min-content track의 row height와 block contribution이 inline 폭과 따로 틀리는가 | normal/anywhere/break-word 예상 `(inline, block)` 및 final auto row를 Rust와 플랫폼 adapter가 검사한다. 두 platform matrix 9/9. |
| 19 | fixed 10px column이 computed track 폭 또는 line wrapping을 잘못 섞는가 | final column은 10px다. normal은 line advance 40px로 한 줄 overflow; anywhere/break-word는 네 줄·40px row. trace와 frame이 일치한다. |
| 20 | fixed column인데 Grid가 nonexistent intrinsic column measurement를 기록하는가 | Android/iOS 기대값은 fixed scene에서 Columns intrinsic contribution이 비어 있음을 확인하고 final fixed track을 따로 검사한다. 이전 잘못된 measure 기대값은 실패 로그 보존 후 고쳤다. |
| 21 | fixed-track early return 때문에 iOS가 실제 auto Rows 검증을 건너뛰는가 | iOS Columns는 `FixedTrackEarlyReturn`, Rows는 auto track path로 각각 나눠 검사한다. fixed scenes 3개 모두 실행 통과. |
| 22 | Android가 normal overflow의 available width 10과 laid-out text width 40을 같은 값으로 처리하는가 | Android expected item width는 `max(trackWidth, lineAdvances)`이며 available width는 고정 track 10px다. screenshot trace의 line advance 40px. |
| 23 | Android style trace에서 computed mode 또는 locale absence가 잘못 승인되는가 | `overflowWrap`을 정확한 keyword로 비교하고 `lang` 없는 scene은 locale null로 검사한다. 이 mismatch의 이전 실패 log를 남기고 수정 뒤 9/9 통과. |
| 24 | iOS adapter가 Android와 다른 nullable locale 또는 mode 값을 기대하는가 | iOS expected snapshot은 overflow-wrap scenes의 locale을 nil로, mode를 Normal/Anywhere/BreakWord로 계산해 style trace에 대조한다. 9/9 통과. |
| 25 | trace JSON에 mode가 빠져 app이 computed property만 보고 성공 처리하는가 | trace report에 `overflowWrap`을 직렬화하고 두 app이 해당 필드, line metrics, min/max, revision, frame을 비교한다. raw logs에 기록됨. |
| 26 | Android/iOS capture script가 scene 실패 뒤 앱 프로세스를 남기는가 | script의 cleanup/finally 경로와 simulator logs를 확인했다. 최종 captures는 각 9/9이며 앱 종료 뒤 raw trace와 screenshot을 저장한다. |
| 27 | fixture host ABI와 신규 scene code가 기본/product archive에 새어 나가는가 | 기본 archive hash `d8d75e…55f5`에는 9 marker/host export 0개, fixture positive control `8ff98d…545b`에는 9 marker/2 exports. archive 범위로만 주장한다. |
| 28 | source boundary test가 새 submodule의 경로나 앱 adapter 연결을 놓치는가 | Node boundary test는 route/source modules, 9 scene IDs, platform validators, capture script, installed hashes를 확인한다. 28/28 통과. |
| 29 | evidence가 캡처 source와 다른 변경본을 가리키거나 실패 실행을 숨기는가 | `source-revision.json`은 두 앱 hash를 implementation commit `5d129758…`에 연결한다. 실패한 locale/width/stage 시도도 개별 log와 attempts 표에 보존한다. |
| 30 | PR 이미지가 끊기거나 화면 캡처를 glyph proof로 과장하는가 | Agent Browser에서 PR #948을 열고 raw image 4개 모두 `complete=true`, Android 1080×2400 및 iOS 1206×2622로 로드됨을 확인했다. 본문과 evidence는 이 캡처가 frame만 보이고 glyph pixels는 증명하지 않는다고 명시한다. 브라우저를 닫았고 `agent-browser` 프로세스 0개를 확인했다. |

## 결론과 미완료 범위

이 30관점에서는 새 수정 사항을 찾지 못했다. 리뷰 당시 PR check rollup은 비어 있었고 `gh pr checks --required`는 branch에 보고된 check이 없다고 반환했다. CI 성공이라고 표시하지 않는다. C11.4/C11 및 공식 `72/213`은 미완료다. Simulator geometry/trace, static archive boundary, WPT, 일반 CSS admission, glyph paint, 실기기/hardware GPU 및 iOS compositor 이후 pixel은 서로 다른 evidence 경계로 유지한다.
