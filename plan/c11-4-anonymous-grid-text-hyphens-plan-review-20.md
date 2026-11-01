# C11.4 익명 Grid 텍스트 `hyphens` 계획 실패 관점 20개

| # | 실패 관점 | 검토 결과 |
| --- | --- | --- |
| 1 | CSS Text 3의 초기값·상속 여부를 잘못 모델링하는가 | `manual` 초기값과 inherited 동작을 계획에 명시했다. **통과** |
| 2 | U+00AD가 `manual`에서 min-content 기회가 아닌 일반 숨김 문자로만 처리되는가 | min-content를 30px로 고정해 Chrome과 독립 계산 양쪽에서 확인한다. **통과** |
| 3 | `none`이 U+00AD를 막지 못해 min-content가 30px으로 줄어드는가 | `none` 기대값 40px과 fixed 30px 한 줄 overflow를 둔다. **통과** |
| 4 | `manual`이 max-content 폭에 가짜 하이픈을 상시 더하는가 | max-content 양쪽 모두 40px이며 원문 U+00AD를 유지한다. **통과** |
| 5 | discretionary hyphen glyph advance가 min-content에 빠지는가 | U+2010 측정용 복사본과 Ahem 1000/1000 em을 명시했다. Parley metric test가 구현 관문이다. **통과** |
| 6 | `hyphens:none`이 U+002D 일반 break를 금지하는가 | visible-hyphen control은 none/manual 모두 30px min-content와 두 줄이다. **통과** |
| 7 | soft hyphen 없는 단어에서 기본 segmentation이 바뀌는가 | `ABCD` 40px control을 둔다. **통과** |
| 8 | 여러 U+00AD 중 첫 위치만 적용되는가 | 두 기회 scene을 추가하고 30px/3 lines와 60px/1 line을 분리한다. **통과** |
| 9 | 고정 30px에서 line wrap과 intrinsic width가 서로 다른 mode를 쓰는가 | `none` overflow와 `manual`의 30/20 advances를 함께 검사한다. **통과** |
| 10 | fixture label이나 부모 Grid의 stretch가 row 값을 오염시키는가 | label을 block 흐름으로 바꿔 Grid content와 분리했다. 잘못된 16px 초기 관찰은 제외했다. **통과** |
| 11 | 조상에 선언된 inherited property가 direct-text owner에서 사라지는가 | 두 inherited case를 Chrome fixture와 runtime matrix에 포함한다. **통과** |
| 12 | Stylo 0.22 unknown-property 진단을 무시하고 기본값인 것처럼 성공 처리하는가 | unknown-property 기존 negative를 대체하고 computed-style 전달 여부를 별도 검증한다. **통과** |
| 13 | 예약 custom property alias가 selector/source order/`!important` cascade를 잃는가 | 실제 Stylo custom-property computed value를 읽고 priority matrix를 통과해야 한다. **통과** |
| 14 | CSS-wide `initial`이 부모의 `none`을 잘못 상속하는가 | `initial`을 inherited property의 initial `manual`로 매핑하는 control을 둔다. **통과** |
| 15 | `unset`, `inherit`, `revert`, `revert-layer`를 raw keyword 문자열 비교로 처리하는가 | CSS-wide semantics를 custom property cascade에서 유지하는지 style test로 판정한다. **통과** |
| 16 | 원래 CSS custom property alias 선언 또는 `@property`가 값·상속을 탈취하는가 | 원본 author source 충돌과 registration을 미지원으로 거부한다. **통과** |
| 17 | `all` shorthand가 hyphens를 재설정하지만 alias는 유지하는가 | `all` 사용을 fixture profile에서 fail closed하도록 계획했다. **통과** |
| 18 | CSS `auto`를 지원하는 척하거나 잘못된 keyword가 파서에서 사라져 initial `manual`처럼 처리되는가 | `auto`는 dictionary가 없으면 거부하고, authored invalid keyword는 Stylo parsing 전에 관찰해 미선언 상태와 구분되지 않으면 그 scene을 계산 전에 거부한다. **구현 관문** |
| 19 | U+00AD→U+2010/U+2060 변환이 source byte/UTF-16 mapping을 밀어버리는가 | HostDocument 원문을 유지하고 transformed byte range 재계산과 UTF-16 source map 검사를 요구한다. **구현 관문** |
| 20 | Parley 0.12가 U+00AD break를 Chrome처럼 선택하거나 line metrics를 보존하지 않는가 | 현재 source는 soft hyphen glyph가 default-ignorable임만 보인다. 실제 line-break control은 아직 없어 구현 전 Rust probe/test에서 확인해야 한다. **열린 구현 위험** |

이 검토는 계획 문서 검토다. 실제 Parley line breaking, computed custom-property adapter, runtime scenes 또는 simulator 결과를 통과했다고 주장하지 않는다.
