# 구현/runtime 실패 관점 검토 20개

**검토 시점:** 2026-10-28

**검토 대상:** `5df6da5665e7df2cb4c3e6ec0d7278ce8b50ad17`의 13-case Rust 회귀 검사와 Android/iOS runner·pass03 evidence
**범위:** 제품 layout 코드는 바꾸지 않았다. 고정 `TEXT_CASES` legacy route의 test coverage와 기존 opt-in app route만 검사했다.

## 검토 결과

| # | 실패 관점 | 결과 | 확인 |
| ---: | --- | --- | --- |
| 1 | case가 빠지거나 중복돼 13개 matrix가 조용히 축소되는가 | PASS | `TEXT_CASES` 길이 및 별도 유일성 검사, Android/iOS matrix 각각 13행을 확인했다. |
| 2 | 새 scene이 추가돼도 intrinsic 기대값을 갱신하지 않는가 | PASS | Rust intrinsic oracle은 알 수 없는 scene ID에서 실패한다. |
| 3 | fixture root가 실제 case의 root ID·HTML `div`와 다른가 | PASS | `TEXT_CASES.root_id`로 HTML `div` root를 만들고 전체 dispatcher가 root identity를 확인한다. |
| 4 | item이 case ID·직접 자식·단일 Text node topology와 다른가 | PASS | 각 HostDocument에 case item ID를 가진 직접 자식 `div`와 단일 Text node만 만든다. |
| 5 | source text가 case와 달라도 다른 intrinsic 결과가 우연히 통과하는가 | PASS | HostDocument Text와 `ParagraphInput`이 case content를 사용하고 app matcher도 raw/normalized source를 확인한다. |
| 6 | 이전 고정 adapter가 generic collector 전에 case를 가로채는가 | PASS | dispatcher와 같은 순서로 14개 specialized adapter를 호출해 모든 case에서 `None`인지 검사한다. |
| 7 | generic collector가 one-item input을 잘못 수용하는가 | PASS | 각 case의 collector 결과가 `Ok(None)`이어야 한다. |
| 8 | Rust CSS fixture가 source의 track/dimension과 다른가 | PARTIAL | stylesheet는 `TextCase`에서 만들어 독립 CSS 원본은 아니다. FFI source와 두 플랫폼 matcher의 13개 ID를 대조하고 app matrix를 실행했다. |
| 9 | percentage `fit-content()`에 필요한 `align-items` 조건이 틀리는가 | PASS | percentage track scene은 `normal`, 나머지는 `start`로 cascade하고 legacy validator가 이를 수용한다. |
| 10 | calc/length/auto/min/max track serialization이 달라지는가 | PASS | `TextTrack` variant를 CSS로 직렬화하고 case별 dispatcher 및 typed app trace를 확인한다. |
| 11 | content-box edge·min/max width·min-height item style이 빠지는가 | PASS | case의 box sizing·padding·border·min/max 값을 stylesheet에 포함하고 `validate_text_style`을 통과한다. |
| 12 | HostDocument와 computed-style revision이 달라지는가 | PASS | 한 fixture snapshot에서 cascade, adapters, full dispatcher를 실행한다. |
| 13 | specialized adapter error를 `None`으로 숨겨 fallback pass를 만드는가 | PASS | helper가 오류를 adapter 이름 및 scene ID와 함께 panic 처리한다. |
| 14 | full dispatcher가 fallback 전 성공하거나 invalid variant를 잘못 허용하는가 | PASS | 13개 positive에서 선행 adapter가 모두 `None`이고 full `prepare`가 성공한다. 별도 negative control 다섯 개가 unknown root/item, changed text, extra direct item, wrong track을 거부한다. |
| 15 | legacy dispatcher가 item paragraph style을 반환하지 않거나 두 축 nested-grid mode를 켜는가 | PASS | 정확히 한 item의 Ahem paragraph style을 확인하고 `two_axis_nested_grid == false`를 검사한다. |
| 16 | Ahem font가 등록되지 않아 intrinsic 기대값이 다른 환경에서 성립하는가 | PASS | dispatcher가 반환한 engine으로 case paragraph intrinsic을 재측정한다. 등록 누락·family 오류는 test 실패다. |
| 17 | normal, break-all, nowrap, lowercase cyclic 입력의 min/max-content 폭이 틀리는가 | PASS | 13 scene별 고정 기대값과 C11.4 text focused tests 21/21 결과를 확인한다. |
| 18 | Android/iOS FFI 또는 matcher에서 scene ID가 빠져 다른 fixture를 실행하는가 | PASS | FFI source, Android matcher, iOS matcher의 ID가 각각 13/13이고 pass03 matrix도 양 플랫폼 13/13이다. |
| 19 | 이전 iOS trace/draw가 현재 scene 결과로 오인되는가 | PARTIAL | pass03은 현재 run-ID의 summary·trace 뒤 draw/present를 확인한다. 다만 draw line 자체에는 run ID가 없고 summary와의 순서 및 scene별 terminate/launch 경계로 연관 짓는다. |
| 20 | box screenshot을 glyph paint 또는 최종 화면 pixel로 과장하는가 | PARTIAL | `runtime_text_count=0`, `glyph_batch_draw_count=0`이다. 화면은 layout/surface 확인용이며 glyph paint·iOS compositor 이후 pixel은 범위 밖이다. |

## 결론

차단 결함은 없다. 13개 one-item scene 모두 specialized adapter와 generic collector를 거쳐 기존 `TEXT_CASES` full-dispatch 경로로 준비된다. 다섯 fail-closed negative control, Ahem intrinsic input 및 Android/iOS app matcher 결과가 pass03 evidence에 남았다. 첫 runner 시도는 generic 3-box 기대값을 재사용해 2-box legacy scene을 거부했다. 제품 matcher는 `status=0`이었고, runner를 2-box 및 현재 run summary 뒤 draw/present 조건으로 고친 다음 pass03에서 다시 실행했다. 세 PARTIAL은 CSS test input의 독립성, draw event의 run-ID 부재, glyph paint 제외를 나타내며 계획 범위에 기록했다.
