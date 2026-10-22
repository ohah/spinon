# C11.4 교차 축 span groups 구현 PR diff 실패 관점 검토 30개

**대상:** runtime implementation PR에 포함할 최종 tracked source, plan/status/conformance 문서 및 simulator evidence. 계획 검토와 구현/runtime 검토에서 사용한 질문을 반복하지 않고 diff 경계·통합·릴리스·문서 오표기를 따로 확인했다.

| # | 변경 실패 관점 | diff 판정 |
|---:|---|---|
| 1 | 새 FFI fixture가 기존 C11.4 registry 외부나 public module에 등록되는가? | 기존 `c11_4.rs` fixture 경로에서만 등록했다. |
| 2 | runtime Ahem adapter가 새 node type 또는 public CSS 경로를 넓히는가? | 기존 opt-in text fixture registry에만 새 adapter를 연결했다. |
| 3 | Rust source 파일 경로가 fixture/module boundary 규칙과 맞는가? | FFI, runtime, style-to-layout 각각 지정된 `fixtures`/fixture-test 디렉터리 안에 있다. boundary 6/6 통과. |
| 4 | default product FFI가 fixture payload와 진입 ABI를 끌어오는가? | 새 default release archive에는 네 scene ID와 entry marker가 없고 opt-in archive에는 모두 있다. |
| 5 | FFI allowlist가 scene prefix만 보고 미등록 suffix를 통과시키는가? | 네 ID exact lookup을 사용하며 세 unlisted control은 `None`이다. |
| 6 | Java/Swift host가 scene id를 인정하지 않아 Rust path 전에 종료되는가? | 두 native C0410 scene allowlist에 네 ID를 등록했다. 최종 앱 source hash가 시뮬레이터 manifest와 일치한다. |
| 7 | root frame만 비교하고 두 item frames는 건너뛰는가? | item별 frame 문자열의 native formatting 문제를 숫자 parser로 처리하고 `1/64px` 이하만 허용한다. |
| 8 | reverse DOM 순서에서 node id와 frame association이 바뀌는가? | 실제 child order에 따른 Node ID frame을 forward/reverse 모두 검사하고 item identity별 기대 위치를 유지한다. |
| 9 | native report가 direct text를 anonymous text source로 잘못 판정하는가? | `source.kind=text`, font revision과 intrinsic values를 확인하며 `textSources=[]` 계약을 검사한다. |
| 10 | item intrinsic metric의 순서가 reverse scene에서 고정 잘못되었는가? | source order별 두 item expectation을 scene suffix에 따라 뒤집는다. |
| 11 | track trace가 마지막 axis pass 대신 이전 pass를 읽는가? | focused Rust test는 axis의 마지막 pass와 final stage에서 track 값을 비교한다. |
| 12 | contribution verifier가 axis/item/kind/span을 부분 비교하는가? | JS runtime verifier는 두 axis, 두 node, Minimum/MinContent/MaxContent, span=2를 개별 확인한다. |
| 13 | same-track contribution 증가량을 더해 wrong result를 승인하는가? | CSS 독립 값과 trace를 `[7,17,17]`로 검사하며 Rust focused test도 같은 값을 확인한다. |
| 14 | cyclic calc min term이 absolute length를 떨어뜨리는 원래 bug가 남는가? | `track_sizing.rs` fallback이 zero percentage basis에서 `2px`를 보존한다. 초기 trace `[2,2,2]`를 검사한다. |
| 15 | percentage rerun이 final definite container width를 받지 못하는가? | resolved container content box를 definite available grid space로 전달하며 final track expectations를 확인한다. |
| 16 | percentage rerun이 기존 intrinsic bases/contributions를 다시 지우는가? | `base_size` callback과 non-reset path를 유지하고 `cyclic_` regression 16개가 통과했다. |
| 17 | unrelated fixed/non-percentage tracks에도 new cyclic fallback이 실행되는가? | `GridTrackKind::Track`, indefinite axis size, `uses_percentage()` gate를 모두 요구한다. |
| 18 | width와 height가 cross-axis wrapping 사례를 실제로 다르게 만들지 않는가? | intrinsic A line 2/block 20px, definite A line 3/block 30px를 trace와 root rows로 검사한다. |
| 19 | min-content constraint size와 final used size를 같은 값으로 문서화하는가? | plan/evidence에서 bases `[9,19,19]`와 final columns `[15.671875,15.671875,15.65625]`를 별도 표기한다. |
| 20 | root width에 border와 gutter 포함 방식이 틀리는가? | CSS content width 51/40px와 root border boxes 55/44px를 각각 기록했다. |
| 21 | definite item B가 root border 너머로 가지 않아 overflow 표본이 무효가 되는가? | expected geometry에서 root 44px, item B `x=16,width=40`; root-relative x=50은 item 안이며 root 밖이다. |
| 22 | Android pixel oracle가 새 sample ID에 색을 매핑하지 않는가? | 두 새 sample ID를 red/black expected RGB table에 등록하고 AVD PixelCopy 4/4를 기록했다. |
| 23 | Android sample 좌표 scale/density에서 잘못된 physical pixel로 옮겨지는가? | viewport 320×240, scale 2.625와 forward physical `(132,11)`/`(158,11)` 로그를 남겼다. |
| 24 | iOS sample이 root-relative CSS coordinate나 3× scale에서 어긋나는가? | Metal drawable 960×720의 `(151,13)`/`(181,13)` 표본이 각 scene 기대색과 exact다. |
| 25 | iOS readback을 compositor-final display proof로 과장하는가? | README와 conformance가 drawable 제출 전 검사로 한정한다. |
| 26 | screenshot이 geometry만 아니라 glyph equality·border stroke까지 입증한다고 설명하는가? | 화면은 simulator render 참고이고 exact 판정은 surface sample로 구분한다. glyph paint/border stroke는 미검증이라고 썼다. |
| 27 | prebuilt app 불일치나 잘못된 simulator run이 final success에 섞이는가? | 최종 run manifest의 app/APK SHA와 installed Android APK SHA가 기록돼 있고 scene capture는 각 4/4다. |
| 28 | README의 build/test 결과가 실행 로그와 다르거나 검증 수를 합산하는가? | Cargo logs, fixture boundary 6/6, Clippy, formatter 및 syntax log를 각각 연결했다. 두 플랫폼 4/4 scene과 pixel 4/4를 분리해 썼다. |
| 29 | C11.4 parent checkbox나 공식 완료 집계가 제한 fixture 때문에 올라가는가? | `STATUS.md`, plan, conformance 모두 C11.4 미완료 및 `77/213`을 유지한다. |
| 30 | 남은 platform/WPT 범위가 사라지거나 Tailnet page가 source docs를 앞서는가? | WPT, Android physical/hardware GPU, iOS compositor-after 미검증을 공개 문서에 적었다. Tailnet pass47은 merge 후 최신 main을 기준으로 별도 해시 확인한다. |

**결론:** 최종 변경은 기존 opt-in fixture 계약을 따르고 새 scene이 default release archive로 새지 않는다. 시뮬레이터·문서·artifact 근거의 판정 경계는 기록했으며, 일반 CSS Grid 완료를 주장하지 않는다.
