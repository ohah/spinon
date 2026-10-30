# C11.4 anonymous Grid text pre-line 계획 PR diff 적대 검토 20개

계획 PR의 실제 staged diff를 대상으로 서로 다른 문서·근거 실패 경로를 확인했다. 구현 결과 검토와 PR merge/CI 검증은 포함하지 않는다.

| # | 검토 관점 | 확인한 실패 가능성 | 결과/처리 |
| ---: | --- | --- | --- |
| 1 | 계획 범위 | fixture 개선이 일반 CSS support로 읽힐 수 있음 | opt-in `RuntimeBlockFlexGridTrackSizingV1` 경계와 비포함 범위를 반복해 명시, 통과 |
| 2 | 현재 동작 설명 | 이미 지원되는 engine 작업을 새 구현으로 설명할 수 있음 | 기존 `PreLine` intrinsic engine과 root HTML adapter 코드를 근거로 적음, 통과 |
| 3 | 허용값 | `pre-line` 추가가 다른 white-space keyword도 포함하는지 불명확 | `PreLine` 하나만 추가하고 `pre`, `pre-wrap`, `break-spaces`를 범위 밖으로 씀, 통과 |
| 4 | CSS shorthand와 longhand | `pre-line` computed style을 잘못 투영할 수 있음 | `preserve-breaks/wrap`을 runtime validator 기대값으로 명시, 통과 |
| 5 | 동일 입력 통제 | normal/pre-line을 다른 text와 비교할 위험 | 양 장면에 정확히 같은 `XX\nXX`를 고정, 통과 |
| 6 | max-content 계산 | nowrap 또는 min-content 값을 기대값으로 혼동할 수 있음 | `max-content` track과 50/20px 값을 별도 표에 기록, 통과 |
| 7 | segment-break 계산 | normal에서 LF가 무조건 사라진다고 오인할 수 있음 | collapsible space로 변환됨을 명시하고 `20+10+20` 계산을 표시, 통과 |
| 8 | forced break 계산 | pre-line에서 wrapping 자체를 막는다고 오해할 수 있음 | `preserve-breaks/wrap`과 강제 줄바꿈 보존을 분리, 통과 |
| 9 | font metric | system font fallback으로 픽셀 계산이 흔들릴 수 있음 | Ahem asset과 SHA-256, 폰트 준비 상태, 10px/10px 고정, 통과 |
| 10 | root와 track | root box 폭과 column 폭을 동일시할 수 있음 | 20px root와 50px normal track 및 30px overflow를 별도 기재, 통과 |
| 11 | Range 한계 | Range rect를 item frame이나 glyph pixels로 과대 해석할 수 있음 | README·plan 모두 fragment observation의 경계를 표시, 통과 |
| 12 | LF zero-width rect | newline rect를 실제 글자 또는 두 번째 줄로 셀 수 있음 | raw rect를 남기되 glyph line count와 item size에서 제외, 통과 |
| 13 | browser 위치 | section의 page y 좌표를 acceptance에 고정할 수 있음 | local size와 fragments만 acceptance로 삼음, 통과 |
| 14 | Chrome와 명세 | browser oracle을 규범으로 역수입할 수 있음 | 별도 CSS Text, Grid, Box Sizing 계산과 Chrome 관측 열을 분리, 통과 |
| 15 | runtime scene 입력 | arbitrary HostDocument가 허용될 수 있음 | exact scene ID, direct text, style profile 검증을 계획, 통과 |
| 16 | negative boundary | nonzero `order`나 비opt-in route가 열릴 수 있음 | `order:0`, opt-in off, 일반 admission negative control을 계획, 통과 |
| 17 | artifact 경계 | fixture ID/FFI payload가 product artifact로 유출될 수 있음 | 기본 artifact와 fixture-test artifact를 따로 검사하도록 계획, 통과 |
| 18 | 플랫폼 증거 | frame/draw-presentation을 glyph pixels나 실제 기기 증거로 오인할 수 있음 | AVD/Simulator 범위와 캡처 한계를 구체적으로 적음, 통과 |
| 19 | 공식 진행도 | 계획을 구현 완료나 C11.4 완료로 표시할 수 있음 | 문서에 구현 전, C11.4/C11 미완료, `72/213` 불변을 표시, 통과 |
| 20 | 문서·evidence 무결성 | 상대 파일 링크, 이미지, 입력/font hash가 어긋날 수 있음 | evidence paths와 SHA-256 대조; JSON parse 및 staged file set 확인, 통과 |

결과: 현재 계획 PR diff에서 수정이 필요한 문제가 남지 않았다. PR에는 계획·검토 기록·Chrome 사전 관찰 근거와 상태표의 다음 계획 링크만 포함되며 제품 코드나 공식 완료 수를 바꾸지 않는다.
