# 구현 PR diff 적대 검토 30개

대상 diff: 계획 PR #919 병합 직후의 `34d233e22f4795b125b3b6de0015fab4e542a706` 대비 현재 staged C11.4 computed `letter-spacing` 후보. `git diff --cached`의 구현·테스트·플랫폼 validator·상태 문서·실행 근거를 서로 대조했다. 아래는 구현/runtime 20개 검토와 분리한 PR 변경 검토다.

| # | 적대 검토 관점 | 판정 |
|---:|---|---|
| 1 | 제한 fixture 구현을 일반 CSS 지원처럼 포장하는가 | **통과.** 계획, 내부 계약, STATUS 모두 고정 anonymous Grid profile이며 public CSS admission이 아님을 밝힌다. |
| 2 | C11.4/C11 완료나 공식 수를 앞당기는가 | **통과.** 상위 checkbox는 미완료이고 `72/213`을 유지한다. |
| 3 | 승인된 계획 PR과 구현 후보의 관계가 분명한가 | **통과.** 계획 PR #919 병합 SHA에서 시작하는 구현 slice이고 계획 문서에 PR이 아직 남았다고 기록했다. |
| 4 | 입력·글꼴·writing mode·whitespace 범위가 문서와 code fixture에서 다른가 | **통과.** Ahem 10px/400, horizontal-tb/LTR, normal whitespace/word-break, ASCII `X`와 collapsed space를 양쪽에 고정한다. |
| 5 | Chrome 156 관측값을 규범 oracle로 혼동하는가 | **통과.** 독립 CSS Text/Sizing 표를 사용하고 관찰 차이를 precomparison과 실행 문서에 남긴다. |
| 6 | Stylo의 authored `0px` serialization을 일반 CSSOM 사실로 확장하는가 | **통과.** `0px → normal`을 이 snapshot 경로로 한정한다. |
| 7 | 기대값 JSON에서 누락/중복 scene이 simulator 성공률을 부풀리는가 | **통과.** FFI와 runtime test는 16개 고유 ID를 요구하며 Android/iOS allowlist 정적 검사도 추가됐다. |
| 8 | 생성된 source가 scene text 또는 ID를 JavaScript 문자열에 직접 삽입해 escape하지 않는가 | **통과.** Rust debug string formatting으로 ID/text literals를 인코딩하고 source generator test가 생성 문자열을 검사한다. |
| 9 | scene root에 anonymous direct text 대신 element child를 넣는가 | **통과.** 생성 source는 root Grid 아래 text node 하나를 append하고 node ID 목록은 root만 포함한다. |
| 10 | 새 `ParagraphStyle` 필드가 기존 paragraph의 default rendering을 변경하는가 | **통과.** 새 필드와 기존 fixture literal이 0으로 초기화된다. 해당 text/runtime regression gates가 통과했다. |
| 11 | Rust paragraph 입력에는 값을 기록하지만 Parley가 값을 받지 않는가 | **통과.** builder에 `StyleProperty::LetterSpacing`을 넣고 trace의 paragraph style에 실값을 직렬화한다. |
| 12 | line-edge correction이 hanging advance나 empty layout 처리를 덮어쓰는가 | **통과.** hanging advance를 먼저 빼고 non-empty visible line만 spacing을 한 번 보정한다. empty paragraph는 기존 zero-size 경로를 유지한다. |
| 13 | min-content를 두 단어가 붙은 한 줄 폭으로 계산하는가 | **통과.** break-all-lines 결과의 각 line 폭을 보정해 최대값을 선택한다. 두 단어 scene은 2-line intrinsic block과 단어 폭을 확인한다. |
| 14 | max-content가 공백 포함 한 줄 폭을 놓치거나 줄별 max 대신 합을 내는가 | **통과.** no-wrap layout의 line advance 최댓값을 사용하고 16-case oracle이 `XX XX` 폭을 비교한다. |
| 15 | 계산된 paragraph style이 다른 text profile로 새는가 | **통과.** 값은 고정 scene의 root computed style에서만 검증되며 기존 profile initializer는 zero spacing이다. |
| 16 | 범위를 벗어난 `-2px` parser 값이 scene allowlist 없이 실행되는가 | **통과.** 내부 parser 경계와 runtime admission을 분리해 문서화하고, 없는 `-2px` scene을 negative test로 거부한다. |
| 17 | 각 simulator adapter가 Rust 기대표와 다른 min/max 산식을 쓴다 | **통과.** Android와 iOS는 동일한 `20 + spacing` min-content 및 `50 + 4 × spacing` two-word max-content 값을 검사한다. |
| 18 | Android에서 fixture flag가 꺼져도 새 scene이 열리는가 | **통과.** Android scene lookup은 `SPINON_C04_RUNTIME_GPU_FIXTURE_TEST`가 켜져 있을 때만 결과를 반환한다. |
| 19 | iOS에서 fixture compile flag가 꺼져도 새 scene이 열린가 | **통과.** scene lookup은 `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST_1` 조건부 컴파일 안에 있다. |
| 20 | 두 platform validator가 computed value·paragraph style·frame을 빠뜨리는가 | **통과.** 두 validator가 exact `letterSpacing`, line count/advance, root/item geometry 및 intrinsic source를 대조한다. |
| 21 | `rootWidth=120` 추가가 기존 anonymous text scenes의 기대 frame을 깨뜨리는가 | **통과.** 기존 constructor default는 100px를 유지하고 공통 frame builder가 해당 값을 사용한다. 이번 simulator capture는 신규 16 scene 한정이며 focused Rust regression도 통과했다. |
| 22 | `letterSpacing` trace key가 다른 platform의 JSON key와 달라 값 검사가 무의미한가 | **통과.** FFI trace의 `letterSpacing` key를 Android/iOS validators가 같은 이름으로 읽는다. |
| 23 | 오래된 HostDocument/style snapshot 결과가 성공으로 처리되는가 | **통과.** runtime test는 computed style document revision과 request snapshot을 비교하고 두 platform report는 HostDocument input revision을 요구한다. |
| 24 | runtime mismatch가 error를 반환한 뒤 partial layout을 발행하는가 | **통과.** negative integration test는 mismatch 시 layout publication이 없고 expected error category가 남는지 확인한다. |
| 25 | default product archive에 fixture adapter/validator symbol 또는 scene text가 들어가는가 | **통과.** 기본 archive scan은 target symbols 0개, scene IDs 0/16이다. opt-in positive control은 두 symbols와 16/16 IDs를 포함한다. |
| 26 | Xcode LLVM 도구가 Rust object를 읽지 못한 결과를 성공으로 오인하는가 | **통과.** symbol scan에는 archive producer와 같은 Rust LLVM 22.1.2 `llvm-nm`을 사용했다. |
| 27 | static archive scan 결과를 APK/IPA 전체 release exclusion으로 확대하는가 | **통과.** 근거와 STATUS는 static `spinon-ffi` archive로 한정하고 mobile run은 fixture flag build로 분리한다. |
| 28 | Android/iOS 화면 캡처를 glyph paint 또는 compositor pixel 증거로 주장하는가 | **통과.** 캡처가 route/status만 보이고 renderer 표면은 검다는 제한을 명시한다. |
| 29 | README·계약·roadmap 링크와 현재 실행 evidence가 서로 어긋나는가 | **통과.** C11 Grid index, 내부 계약, plan, STATUS가 같은 runtime 및 archive evidence를 링크한다. |
| 30 | 테스트/정적 검사 결과와 남은 lint·CI 상태를 숨기는가 | **통과.** focused test/build/matrix와 fmt 결과를 분리 기록한다. strict Clippy에서 변경 밖의 기존 formatting lint가 나온 사실과 단일 lint만 허용한 대상 Clippy 명령을 공개한다. PR check rollup은 아직 생성 전이다. |

**판정:** 30개 별도 diff 관점을 검토했다. 이 검토는 현재 staged tree에 대한 것이며, PR 생성 후 head SHA가 달라지면 최종 PR diff와 비교해 같은지 다시 확인한다. simulator matrix는 각 16/16, 기본/fixture archive 경계는 0/16 대 16/16, targeted Clippy와 rustfmt는 통과했다. 공식 C11 완료 집계는 바꾸지 않았다.
