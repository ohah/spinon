# Two-track span overflow unit-test 구현 · 실패 관점 20개

검토 대상은 `two_track_span_max_content.rs`와 모듈 등록부, 사전 고정 expected JSON, five-case Chrome 156 observation이다. Runtime/app boundary를 따로 확인했고 product behavior는 추가하지 않았다.

| # | 독립 실패 관점 | 구현 확인 | 결과 |
| ---: | --- | --- | --- |
| 1 | 실제 columns track 수가 두 개보다 많거나 적음 | `GridContainerInput.columns`에는 두 track만 선언하고 trace에서 두 target tracks를 요구한다. | 통과 |
| 2 | grid item placement가 single-track span으로 축소됨 | Explicit column line 1→3을 사용하고 trace의 multi-track flag를 검사한다. | 통과 |
| 3 | implicit placement가 원하는 columns를 바꿈 | Explicit row/column lines로 placement를 고정한다. | 통과 |
| 4 | root width가 indefinite라 maximize가 적용되지 않음 | Root width/height 및 viewport를 각각 definite 80/20px로 지정한다. | 통과 |
| 5 | tracks 사이 gutter가 available width를 줄임 | Container 기본 gap은 0이며 두 target bases를 독립적으로 검사한다. | 통과 |
| 6 | fixed row sizing이 text wrapping을 바꿈 | One fixed 20px row와 20px item height를 확인한다. | 통과 |
| 7 | 시스템 font가 Ahem advance를 대체 | `TextLayoutEngine::new_without_system_fonts()`를 쓰고 Ahem bytes를 명시 등록한다. | 통과 |
| 8 | whitespace/wrapping으로 min-content 폭이 달라짐 | 고정 Ahem 10px `XXXXXXXXXX`와 `WhiteSpaceMode::Normal` input을 사용한다. | 통과 |
| 9 | Y overflow가 target X automatic minimum을 가림 | Item Y overflow는 `Visible`로 고정하고 test case는 X만 바꾼다. | 통과 |
| 10 | `visible` 또는 `clip`이 0 automatic minimum으로 분류 | 각 입력에 100px `AutomaticMinimum` source/value를 요구한다. | 통과 |
| 11 | `hidden`, `auto`, `scroll` 중 한 값이 빠짐 | 세 값을 독립 `Case`로 실행하며 각각 0px `OverflowAutomaticMinimum`을 검사한다. | 통과 |
| 12 | span item이 flexible track 예외에 걸림 | 두 tracks 모두 `max-content` maximum이고 trace flag는 flexible maximum 없음이어야 한다. | 통과 |
| 13 | span content-based flag가 해제됨 | `content_based=true`와 `spans_auto_min_track=true`를 정확히 매치한다. | 통과 |
| 14 | 같은 aggregate width지만 track별 분배가 잘못됨 | `[50,50]`/`[40,40]` 기대값과 각 final track base를 쌍별 비교한다. | 통과 |
| 15 | root frame이 item frame과 혼합됨 | Root `80×20px`과 case별 item width를 따로 검사한다. | 통과 |
| 16 | item frame의 origin이 root origin과 다름 | X/Y origin이 root와 같은지 확인하고 root origin은 0/0px로 고정한다. | 통과 |
| 17 | Minimum contribution 누락 또는 중복 | 동일 item의 Minimum trace가 정확히 하나여야 한다. | 통과 |
| 18 | MaxContent trace 누락 또는 중복 | 동일 item의 MaxContent trace가 정확히 하나이고 value는 100px이어야 한다. | 통과 |
| 19 | 기존 single-track regressions와 입력 충돌 | 기존 modules/expectations는 수정하지 않고 새 submodule만 추가한다. | 통과 |
| 20 | `1/64px` 근사 helper가 계획의 zero-tolerance 기준을 느슨하게 만듦 | 새 module은 local `exact` helper의 `assert_eq!`로 track/frame/contribution을 비교한다. | 통과 |

최종 focused test 1/1, `spinon-layout` 298/298, rustfmt와 crate all-target Clippy가 통과했다. 미결 구현 오류는 없다.
