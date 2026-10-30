# 구현/runtime 적대 검토 20개

대상은 anonymous Grid text collection·validation, Ahem intrinsic/layout tests, 새 runtime/FFI scene, Android/iOS scene matcher 및 simulator trace다. 구현 전 계획 검토와 분리해 실제 변경과 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 결과를 대조했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | `white-space:normal`의 기존 익명 텍스트 경로가 nowrap 허용으로 바뀌며 줄바꿈이 달라진다 | **통과.** normal `min-content` control은 intrinsic width 20px, height 20px, 두 줄과 20px track을 고정한다. 두 simulator에서도 root/item 20×20px다. |
| 2 | nowrap의 min-content가 공백에서 잘못 분할되어 20px가 된다 | **통과.** Rust·두 runtime trace에서 min/max width 모두 50px, 한 줄을 확인한다. |
| 3 | nowrap 결과가 max-content 값만 우연히 읽어 min-content 회귀를 숨긴다 | **통과.** layout assertion과 `MinContent` track contribution을 별도로 50px로 검사한다. |
| 4 | min-content track은 50px여도 root auto height가 두 줄 높이로 남는다 | **통과.** nowrap root/row는 20×10px이고 text item은 50×10px다. |
| 5 | 부모 20px보다 넓은 anonymous item overflow가 잘리거나 20px로 축소된다 | **통과.** Rust 및 양 플랫폼 trace에서 item frame 50×10px, root 20×10px를 분리해서 확인한다. |
| 6 | `max-content` 또는 normal block의 기존 측정값이 달라진다 | **통과.** normal control은 min/max 20/50px, height 20/10px를 유지하고 기존 anonymous text tests도 함께 통과한다. |
| 7 | NoWrap이 높이 축 intrinsic 계산을 오염시킨다 | **통과.** normal 20/10px와 nowrap 10/10px min/max heights를 각각 검사한다. |
| 8 | `ParagraphStyle`과 computed CSS `text-wrap-mode`가 서로 다르다 | **통과.** scene ID별로 wrap/nowrap을 정하고 runtime validator가 동일한 기대값을 요구한다. |
| 9 | 새 scene에서 root width 또는 authored height가 달라 기대값이 우연히 맞는다 | **통과.** validator가 width 20px와 height `auto`를 비교하고 FFI source test가 두 style 문자열을 고정한다. |
| 10 | `pre-line` 같은 미지원 whitespace가 신규 allowlist로 흘러든다 | **통과.** layout negative control은 `UnsupportedTextParagraph`를 확인하며 runtime scene은 normal/nowrap 외 입력을 받지 않는다. |
| 11 | `order != 0` 익명 item source order가 잘못 지원된다 | **통과.** order 1 negative control은 partial layout input 없이 명시 오류를 낸다. |
| 12 | leading collapsible whitespace나 source provenance가 anchor를 잃는다 | **통과.** 기존 provenance·anchor 테스트를 보존했고 focused layout suite에서 통과했다. |
| 13 | comment나 `display:none` 자식이 direct text run을 끊는다 | **통과.** 기존 continuation cases를 보존하고 7-scene matrix에서도 기존 scene ID를 유지한다. |
| 14 | block child separator 또는 source order가 바뀐다 | **통과.** 기존 runs-around-block 입력, block separator 오류 검사 및 source-order assertions를 변경하지 않았다. |
| 15 | runtime validator가 nowrap scene의 다른 Grid profile을 허용한다 | **통과.** 단일 `min-content` column과 auto row를 정확히 요구하고 mutation rejection tests를 유지한다. |
| 16 | Android catalog·expected root/node frame·box count 중 하나가 새 scene을 빠뜨린다 | **통과.** 두 ID가 allowlist/classifier에 있으며 AVD trace는 각 장면 1 box, 예상 root frame과 track을 기록한다. |
| 17 | iOS matcher가 Android와 다른 geometry를 기대한다 | **통과.** 두 scene의 iOS expected frame/count/classifier가 추가됐고 두 simulator 결과가 Rust oracle과 일치한다. |
| 18 | 새 scene payload 또는 host FFI export가 기본 제품 archive로 들어간다 | **범위 내 통과.** `c04-runtime-gpu` static archive에서 새 ID 두 개와 C11.4 host exports 두 개는 없다. 기존 RuntimeSession track-sizing constructor symbol은 남아 있어 전체 내부 fixture 격리는 주장하지 않는다. |
| 19 | 시뮬레이터 로그만으로 draw/present 또는 glyph pixel을 과장한다 | **통과.** JSON에 draw/present 성공이 있고 screenshots는 surface 실행 증거로만 쓴다. 줄 수와 item frame 판정은 trace로 한정하며 pixel exactness를 주장하지 않는다. |
| 20 | 부분 slice를 일반 CSS support·C11.4/C11 완료로 승격한다 | **통과.** plan·status는 범위를 opt-in fixture로 한정하고 C11.4/C11 및 `72/213`을 미완료로 유지한다. |

**판정:** 20개 실패 관점을 확인했다. layout 10/10, runtime 3/3, FFI matrix 1/1, fixture module boundary 23/23, rustfmt, workspace Clippy 및 두 simulator 장면을 검증했다. WPT, 실제 기기, Android hardware GPU, iOS compositor 뒤 pixels와 전체 CSS admission은 범위 밖이다.
