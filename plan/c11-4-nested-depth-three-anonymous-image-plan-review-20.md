# C11.4 depth-three nested anonymous text + image 계획 실패 관점 20개

이 검토는 구현 전 계획만 대상으로 한다. Chrome observation과 별도로 CSS Grid 2025 CRD와 고정 자원에서 기대값을 유도했고, 구현 검토나 PR diff 검토 횟수에 포함하지 않는다.

| # | 공격 관점 | 계획의 방어·판정 |
|---:|---|---|
| 1 | Chrome 값이 기대값 계산에 몰래 들어가면 독립 oracle이 아니다. | 기대값은 Ahem advance, PNG natural size, CSS Grid item/track 절차로 JSON에 먼저 적고 Chrome 관측은 별도 파일에 둔다. |
| 2 | 공백이 min-content를 70px이 아닌 단어 전체 폭으로 만들 수 있다. | Ahem `XX XXXXXXX`에서 가장 긴 non-breakable 조각 `XXXXXXX`를 70px로 명시하고 두 줄 fragment를 검사한다. |
| 3 | max-content가 문장 전체 advance와 다르면 100px 기대가 틀린다. | Ahem의 10px glyph advance로 2 + space + 7 glyph를 합산해 100px로 고정한다. |
| 4 | PNG가 예상한 24×16이 아니거나 decode 전에 크기를 읽을 수 있다. | 저장소 고정 PNG를 사용하고 Chrome에서는 decode와 natural size를 확인하며 runtime에서는 decoded snapshot을 검사한다. |
| 5 | 이미지 CSS width/height가 자연 크기를 전이하면 비교가 다른 기능을 섞는다. | `width:auto;height:auto`를 유지하고 authored dimension/ratio-transfer는 범위에서 제외한다. |
| 6 | anonymous text가 직접 item으로 잘못 분할되면 두 column 기댓값이 무효다. | inner Grid에는 한 개의 정확한 direct text sequence만 두고 child topology와 text-source grouping을 확인한다. |
| 7 | text와 img 사이 공백이 세 번째 anonymous item을 만들 수 있다. | inner HTML을 공백 없는 source order로 고정하고 runtime 생성 코드도 direct child 개수 2를 확인한다. |
| 8 | img가 inline이라 Grid item이 되지 않는다고 오인할 수 있다. | CRD §6.1의 grid-level/blockification을 근거로 삼고 실제 element identity/frame 및 auto-placement를 확인한다. |
| 9 | source order 반전이 overall size에 영향을 주지 않을 수 있으므로 중복 scene일 수 있다. | 기대값에서 outer size 불변을 명시하되 inner track order, item source order와 x 위치는 반전되어야 한다. |
| 10 | auto-placement가 row보다 column-first로 달라질 수 있다. | inner Grid는 명시적 두 column/한 row에 item 두 개를 고정하고 §8.5 기준으로 `0,0`과 `1,0` placement를 관찰한다. |
| 11 | track count가 두 column 대신 implicit column을 추가할 수 있다. | scene CSS와 판정 JSON이 두 intrinsic column만 선언하며 computed track count를 exact 검사한다. |
| 12 | row height가 text만의 20/10px로 정해져 natural image 16px를 놓칠 수 있다. | min-content row는 `max(20,16)=20`, max-content row는 `max(10,16)=16`을 각각 유도해 image가 최대인 두 번째 case를 잡는다. |
| 13 | outer Grid 세 경계 중 하나가 contribution을 전달하지 않을 수 있다. | root/middle/branch/inner 네 Grid 각각의 track, dimensions, parent relation과 frame을 기록한다. |
| 14 | nesting depth 명명에 off-by-one이 있을 수 있다. | chain을 네 Grid와 세 child-to-parent 경계로 정의하고 모든 node identity와 depth-four 거부를 포함한다. |
| 15 | root-relative 좌표에 page/section offset이 섞이면 비교가 viewport 배치에 민감하다. | outer/root 좌표를 원점으로 상대화하고 페이지 장식 위치는 판정하지 않는다. |
| 16 | margin, border, padding, gap이 더해져 94/124px 합이 틀릴 수 있다. | 모든 box edge/gap을 0px로 고정하고 computed track 총합과 frame을 함께 검사한다. |
| 17 | fallback system font나 Ahem 로딩 지연이 oracle을 바꿀 수 있다. | Chrome의 `document.fonts.check`, 런타임 font registry revision, resource/source hash를 완료 관문에 포함한다. |
| 18 | image와 text input이 다른 document/style revision에서 합쳐져 stale contribution이 될 수 있다. | 같은 revision stamp 및 contribution source trace를 assert하고 mismatch negative control을 요구한다. |
| 19 | 광범위한 nested/anonymous 지원으로 해석될 수 있다. | 네 scene id와 정확한 topology만 opt-in 허용하며 임의 입력, 일반 admission, WPT는 명시적으로 제외한다. |
| 20 | simulator screenshot을 glyph/image paint 적합성 증거로 과장할 수 있다. | screenshot은 보조 자료로만 쓰고 frame/track/trace/draw-present gate와 paint pixel 미검증 경계를 분리한다. |

판정: 계획에서 발견된 oracle·fixture·scope 위험은 구현 전 고정할 판정 기준과 거부 조건으로 반영했다. 미해결 상태로 남겨 둔 일반 CSS/Grid 적합성은 이 slice의 완료 조건으로 주장하지 않는다.
