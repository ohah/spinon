# C11.4 anonymous Grid text computed `font-size` 계획 적대 검토 20개

계획 PR diff와 구현을 분리하기 위해, 계획을 확정하기 전에 서로 다른 실패 경로 20개로 oracle·범위·검증 기준을 확인했다. 이 기록은 계획 자체의 검토이며 runtime 통과를 뜻하지 않는다.

| # | 실패 관점 | 계획의 방어 또는 처리 | 판정 |
| ---: | --- | --- | --- |
| 1 | opt-in fixture 결과를 일반 CSS `font-size` 지원으로 확대 | 기존 `RuntimeBlockFlexGridTrackSizingV1`의 고정 anonymous direct-text route만 다룸 | 통과 |
| 2 | scene ID의 font-size scalar가 실제 computed style 전달을 대체 | `ComputedElementStyle`의 동일 root/style revision 값과 typed `font_size_css_px`를 비교하고 paragraph에 전달 | 통과 |
| 3 | computed property 문자열과 typed float가 서로 다른 입력에서 합성 | allowlist px serialization과 typed CSS px가 모두 case와 같아야 함 | 통과 |
| 4 | 임의 CSS token parser가 `em`, `%`, `calc()`를 잘못 환산 | author CSS parsing을 추가하지 않고 다섯 개 exact computed `px` serializer만 허용 | 통과 |
| 5 | decimal 7.5px/12.5px가 정수로 반올림 | exact f32/style 및 fractional track 기대값으로 고정하고 정수 비교를 금지 | 통과 |
| 6 | Ahem을 시스템 fallback으로 대체해 font width가 환경별 변화 | 저장소 Ahem 파일을 고정 hash와 `document.fonts.check`로 확인 | 통과 |
| 7 | `1em` 폭 가정이 근거 없는 browser 관찰 | `unitsPerEm`, `X` advance와 space advance를 pinned TTF `hmtx`에서 별도로 확인 | 통과 |
| 8 | min-content 식이 space를 단어에 포함하거나 두 word를 합침 | normal whitespace collapse/wrap 아래 `XX XX`의 단어 최소값을 2em으로 산출 | 통과 |
| 9 | max-content 식에서 collapsed space 폭을 빼먹음 | 두 X, 한 space, 두 X의 총 advance를 5em으로 산출 | 통과 |
| 10 | line-height 변화가 font-size 효과로 섞임 | 10개 case 모두 absolute `line-height:10px`로 고정 | 통과 |
| 11 | explicit row track이 text block contribution을 고정 | fixture와 Chrome oracle 모두 `grid-template-rows:auto` | 통과 |
| 12 | root width 20px를 track oracle로 오인 | root border-box와 `gridTemplateColumns`를 별도 관찰·비교 | 통과 |
| 13 | min-content column의 wrapping이 빠져 inline 값만 검사 | auto row가 2×10px가 되는 두-line control을 각 font-size에 포함 | 통과 |
| 14 | max-content column이 분리되어 2줄로 계산 | max-content width가 5em이라 one-line row 10px인지 별도 확인 | 통과 |
| 15 | 20px font overflow/glyph 높이를 track 오차로 오인 | max-content overflow와 font ink/pixel을 범위 밖으로 두고 track/frame만 판단 | 통과 |
| 16 | zero font-size 거부가 CSS 전역 규칙으로 서술 | 현재 text model 제한으로만 거부하며 일반 CSS 유효성 판단이 아님을 명시 | 통과 |
| 17 | negative/non-finite input 뒤에 partial frame이 남음 | pre-publication gate와 실패 시 frame·trace 부재 negative controls를 요구 | 통과 |
| 18 | anonymous source owner 또는 style revision이 끊김 | exact root, direct text topology, source·order·style revision provenance를 검증 | 통과 |
| 19 | 새로운 fixture ABI가 default/product archive로 유출 | default artifact negative와 fixture-test opt-in positive를 각각 확인 | 통과 |
| 20 | Chrome precomparison을 Spinon/platform/WPT 완료로 보고 | Chrome 관찰, AVD/iOS Simulator 구현 검증과 제품/C11 완료 집계를 분리 | 통과 |

검토에서는 fractional font-size와 fixed 10px line-height 조합이 pixel parity를 보장하는 것처럼 보이지 않도록 glyph ink와 compositor 결과를 명시적으로 제외했다. C11.4 체크와 공식 `72/213`은 그대로 둔다.
