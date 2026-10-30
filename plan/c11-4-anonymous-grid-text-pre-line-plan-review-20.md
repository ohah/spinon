# C11.4 anonymous Grid text `white-space:pre-line` 계획 적대 검토 20개

계획 PR 제출 전 서로 다른 실패 경로를 확인했다. 이 검토는 구현 결과 검토가 아니다.

| # | 실패 관점 | 계획의 방어 또는 처리 | 판정 |
| ---: | --- | --- | --- |
| 1 | 제한 fixture 한 값을 일반 CSS 지원으로 오인 | 변경 범위를 opt-in `RuntimeBlockFlexGridTrackSizingV1`으로 고정하고 일반 author admission을 제외 | 통과 |
| 2 | 기존 text engine에 기능이 없는데 새로 구현하는 범위로 확대 | 기존 `WhiteSpaceMode::PreLine` engine 지원과 root HTML profile mapping을 전제로 재사용 | 통과 |
| 3 | CSS declaration과 `ParagraphStyle`이 서로 다른 값 | runtime validator에서 `preserve-breaks/wrap`과 `PreLine` 고정 profile의 일치를 검사 | 통과 |
| 4 | normal control이 다른 source라 segment-break 차이를 고립하지 못함 | 두 장면 모두 동일한 `XX\nXX` 원본 source 사용 | 통과 |
| 5 | min-content와 max-content를 혼동 | `grid-template-columns:max-content` 고정 track을 사용하고 50px/20px 기여를 별도 계산 | 통과 |
| 6 | 20px root 경계가 없어 overflow 차이가 사라짐 | 양 장면 root width를 20px로 고정하고 normal의 30px overflow를 확인 | 통과 |
| 7 | `pre-line`을 pre/pre-wrap 전체 동작으로 잘못 확장 | 허용 목록은 `PreLine`만 추가하고 다른 WhiteSpaceMode 및 CSS profile은 범위 밖 | 통과 |
| 8 | Grid direct text가 anonymous item이 아니라 일반 child라고 가정 | CSS Grid §6 direct text sequence/anonymous block item 동작을 근거로 유지 | 통과 |
| 9 | Ahem 폰트 미로드로 폭 비교가 불안정 | 폰트 로드 true와 Ahem asset hash를 고정하고 10px advance를 계산 | 통과 |
| 10 | LF Range의 0폭 rect를 실제 글자나 item frame으로 계산 | Range 관찰과 item/track/frame 검증을 분리하고 raw zero-width rect를 기록 | 통과 |
| 11 | Auto row가 아닌 고정 height가 forced break를 숨김 | `grid-auto-rows:auto`와 `grid-template-rows:auto`로 10px/20px 결과를 검증 | 통과 |
| 12 | source node, anchor, grouped run provenance가 끊김 | 기존 provenance 및 revision assertion에 동일 text node/anchor를 포함 | 통과 |
| 13 | 다른 order가 새 allowlist 때문에 우연히 수용 | 기존 `order:0` 조건을 불변으로 두고 nonzero negative control 유지 | 통과 |
| 14 | Opt-in이 아닌 경로까지 바뀜 | allowlist 변경 위치를 기존 opt-in branch에 한정하고 disabled-route rejection 확인 | 통과 |
| 15 | 일반 CSS cascade를 fixture constant가 지원하는 것처럼 표현 | static fixture profile을 명시하고 general computed CSS auto-mapping을 주장하지 않음 | 통과 |
| 16 | unknown scene ID가 부분 실행으로 통과 | runtime fixture scene ID allowlist와 exact direct-child validation을 추가 | 통과 |
| 17 | 기본 앱 artifact에 fixture code/payload 유입 | default와 fixture-test artifact scan을 별도 gate로 유지 | 통과 |
| 18 | Chrome 수치를 규범으로만 취급 | CSS Text, Box Sizing, Grid 근거로 계산한 별도 기대값 표와 관측을 분리 | 통과 |
| 19 | 시뮬레이터 surface 캡처를 glyph/compositor proof로 확대 | frame/trace/draw-present 범위와 pixel 한계를 문서화 | 통과 |
| 20 | 두 scene 통과를 C11.4 또는 공식 완료 증가로 오인 | 공식 `72/213`, C11/C11.4 미완료 상태 유지 | 통과 |

결과: 계획의 범위·fixture oracle·실패 경계는 20개 관점에서 구현 전 검토를 통과했다. 코드 변경과 플랫폼 실행은 아직 수행하지 않았다.
