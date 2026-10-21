# C11.4 nested Grid PR diff 적대 검토 30개

검토 대상은 PR [#442](https://github.com/ohah/spinon/pull/442)의 layout test, opt-in runtime/FFI fixture, Android·iOS verifier, simulator capture parser, evidence와 규범 문서 변경이다. 기존 계획 검토와 구현 검토 표를 재사용하지 않고, 파일 경계를 넘는 별도의 입력·trace·화면·산출물 실패 경로를 확인했다.

| # | 실패 경로 / 불변 조건 | 확인 결과 |
| ---: | --- | --- |
| 1 | fixture 추가가 production Grid sizing algorithm을 조용히 바꾸는가 | layout 변경은 `#[cfg(test)]` direct-layout test 등록뿐이다. runtime 계산 알고리즘 변경은 없다. |
| 2 | 앱 검증 fixture가 기본 FFI artifact에 포함되는가 | `c04-runtime-gpu-fixture-test`에서만 runtime fixture feature가 연결된다. 기본 archive의 fixture symbol/payload 부재와 opt-in positive control 포함을 확인했다. |
| 3 | FFI가 세 장면 이외의 중첩 입력을 받아들여 arbitrary CSS 지원처럼 보이는가 | 세 고정 scene만 source를 제공하고 미등록 ID는 거부한다. |
| 4 | 다른 fixture ID가 nested Grid 경로에 잘못 매칭되는가 | JS scene registry는 C11.4 총 176개와 ID 유일성을 검사하고, nested scene 세 개를 각각 고정 입력에 연결한다. |
| 5 | DOM root가 foreign namespace 또는 다른 tag로 바뀌는가 | runtime adapter가 root의 HTML namespace, `div` tag, scene별 ID를 확인한다. |
| 6 | root에 sibling이나 wrapper가 추가되어 tree 모양이 달라지는가 | root의 direct child가 nested Grid 하나인지 검사하며 추가 자식·wrapper는 거부한다. |
| 7 | nested Grid가 다른 parent에서 재사용되거나 ID만 일치하는가 | nested node의 tag·ID와 정확한 `HostParent::Node(root)`를 함께 검사한다. |
| 8 | nested Grid 아래 text item이 추가·삭제되는가 | direct child 개수가 정확히 둘이어야 한다. |
| 9 | item 순서·ID·owner가 바뀌어 서로 다른 source order가 섞이는가 | 두 item을 고정 순서의 ID와 parent로 검사한다. |
| 10 | text leaf가 간접 descendant, element child 또는 다른 문자열이 되는가 | 각 item은 direct text node 하나와 고정 text payload만 허용한다. |
| 11 | font, wrapping, direction 외 letter/word spacing·indent 변경이 계산 기대값을 무효화하는가 | 누락된 typography profile을 발견했다. `line-break`, `text-align`, `overflow-wrap`, `text-transform`, `writing-mode`, `letter-spacing`, `word-spacing`, `text-indent`까지 검증하고 변형 거부 test를 추가했다. |
| 12 | item margin·padding·border가 intrinsic contribution과 frame을 바꾸는가 | 기존 validator의 box-edge 공백을 발견했다. text item의 margin/padding zero와 기본 border, static position을 추가 확인한다. |
| 13 | 바뀐 outer width·height·min/max·flex basis가 scene 기대값에 섞이는가 | 장면별 typed dimensions와 min/max/flex-basis를 검사한다. |
| 14 | outer Grid columns, rows, auto-repeat 또는 implicit defaults가 달라지는가 | 예상 track size와 row, repeat·template areas·auto-flow·auto-track defaults·line names를 대조한다. |
| 15 | nested Grid의 auto width·inner tracks·gap·box edges가 outer 결과로 가려지는가 | nested style의 dimensions, `min-content max-content`, 4px/0px gap, zero edge 및 typed Grid defaults를 따로 확인한다. |
| 16 | span-two와 one-track placement가 서로 바뀌는가 | `Span(2)`와 `Line(2)`를 scene별 기대 placement와 대조한다. row placement도 auto인지 검사한다. |
| 17 | Ahem test가 호스트 시스템 폰트에 의존하는가 | direct layout은 system fonts 없이 고정 Ahem을 등록하고 runtime은 같은 fixture font resource를 등록한다. |
| 18 | nested Grid 자체가 내부 text leaf intrinsic input으로 중복 기록되는가 | input 목록은 nested Grid owner 아래 text leaf 둘만 포함하고 nested Grid node 자체는 leaf로 기록하지 않는다고 검사한다. |
| 19 | non-leaf에 measure callback이 없는데 빈 `all(...)`이 성공을 가장하는가 | 빈 callback 검사를 제거했다. leaf 입력의 개수·source와 parent contribution trace를 각각 직접 검사한다. |
| 20 | inner text의 min/max intrinsic source 또는 source order가 틀리는가 | 두 text input의 owner grid, node, source order, `Text` kind와 70/100·60/60 크기를 검증한다. |
| 21 | contribution이 nested item 아닌 다른 node·kind·span·값으로 기록되는가 | 두 intrinsic scene에서 outer item ID, `MinContent`, span 2/1, 134px를 trace에서 요구한다. |
| 22 | flex control에서 contribution 자체가 없어도 상한 검사만 통과하는가 | vacuous 검사를 발견해 `Minimum=0/span 2` 존재 요구를 추가했다. |
| 23 | flex control에 NaN, 음수 또는 outer width 초과 contribution이 남는가 | nested contribution 모두 finite·non-negative·120px 이하인지 확인한다. |
| 24 | nested item의 Grid span이 trace stage에서 빠져도 기하만으로 통과하는가 | direct-layout test가 outer `IntrinsicItemBatch` stage의 기대 span을 요구한다. |
| 25 | trace 순서가 뒤집혀 inner trace를 outer로 읽는가 | Rust와 JS verifier는 trace 배열 위치가 아니라 outer/nested Grid node ID로 각각 선택한다. |
| 26 | gutter/collapsed track 처리나 track 수 오류가 고정 크기 비교를 우회하는가 | 최종 Columns stage에서 gutter·collapsed track를 분리한 뒤 기대 non-gutter track 개수와 각 base size를 대조한다. |
| 27 | root frame만 맞고 nested/item frame 또는 내부 gap offset이 틀리는가 | 세 장면 모두 root·nested·두 item의 frame과 두 번째 item의 74px offset을 검사한다. |
| 28 | native app allowlist·box count·expected frame이 JS scene과 불일치하는가 | Android Java와 iOS Swift에 세 ID·box count·frame 목록을 같은 값으로 등록했고 두 simulator에서 전체 frame trace가 통과했다. |
| 29 | flex overflow의 edge 표본이 좌표·target·색 오류를 놓치는가 | internal gap, x=119/120, x=133, x=134를 target node와 RGBA로 확인했다. 수정 후 AVD PixelCopy와 iOS Metal drawable 각 5/5 exact다. PR 본문 두 화면은 Agent Browser에서 각각 1080×2400·1206×2622로 로드됨을 확인했다. |
| 30 | 제한 fixture가 C11.4 완료나 전체 CSS 지원으로 문서화되거나 내부 버전이 올라가는가 | STATUS는 72/213·미완료, 내부 계약은 0.1.0을 유지하고 계획·명세·Simulator evidence·PR을 서로 연결했다. |

## 조치와 최종 확인

검토에서 발견한 누락은 runtime text-style validator의 typography 항목 및 item box edge 검사였다. 수정 후 profile mutation test, nested Grid runtime test, fixture-enabled runtime Clippy와 Android/iOS 앱을 다시 빌드했다. 최종 Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 세 장면 3/3, flex 경계 표본 각각 5/5 exact다. 전체 trace, app hash, 화면과 build log는 [최종 확인 evidence](../c11-4-nested-grid-intrinsic-final-confirmation-2026-10-21/README.md)에 있다.

검토 범위는 고정된 한 단계 nested Grid와 Ahem text 입력이다. 일반 CSS admission, arbitrary nesting, subgrid, WPT, Android 실기기·hardware GPU 및 iOS compositor 이후 표면은 확인하지 않았다.
