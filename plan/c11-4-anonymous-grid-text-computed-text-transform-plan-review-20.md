# C11.4 computed `text-transform` 계획 적대 검토 20개

| # | 실패 관점 | 계획의 방어 또는 남은 판정 |
|---:|---|---|
| 1 | scene ID만 보고 computed 값이 맞다고 가정한다 | exact root style에서 computed serialization을 읽고 typed value와 검증한다. |
| 2 | 부모나 `<html>`의 `lang`을 암묵적으로 상속해 언어 casing을 추정한다 | scene root에 직접 `lang="en"`을 요구하고 누락/변형을 거부한다. |
| 3 | Turkish `I`/`i` 규칙을 English ASCII case mapping으로 처리한다 | `lang=en`만 허용하고 다른 locale은 fixture 입력 거부다. |
| 4 | Unicode uppercase가 여러 code point로 확장되어 source offsets를 어긋나게 한다 | 변환 모드에서는 source 전체 ASCII만 허용하고 one-to-one mapping만 쓴다. |
| 5 | lowercase/uppercase transform이 source DOM string을 덮어쓴다 | HostDocument 원문과 text node는 불변이며 normalized layout buffer만 변환한다. |
| 6 | collapsed spaces가 case conversion 중 사라지거나 늘어난다 | whitespace normalization과 source mapping regression을 함께 둔다. ASCII space는 변환 대상이 아니다. |
| 7 | `capitalize`, `full-width`, `full-size-kana`가 우연히 처리된다 | exact scene allowlist에서 세 값은 명시 거부한다. |
| 8 | CSS-wide keyword 또는 invalid computed serialization이 기본값으로 조용히 대체된다 | computed string/typed enum 불일치를 publication 전에 거부한다. |
| 9 | Noto Sans KR fallback/system font가 oracle과 달라진다 | fixture engine은 system fonts 없이 고정 font bytes를 등록하고 hash를 검증한다. |
| 10 | font face load 누락인데 fallback 관찰을 Chrome oracle로 고정한다 | Agent Browser가 `document.fonts.check`와 `fontStatus=loaded`를 기록한다. |
| 11 | weight variation·language가 glyph advance를 바꾼다 | font family, 400 weight, direct `lang=en`을 고정하고 independent shape advances를 남긴다. |
| 12 | CSSOM track은 예상과 달라도 line fragment만 보고 통과시킨다 | 주 assertion은 computed `gridTemplateColumns`/`gridTemplateRows`다. Range는 보조 관찰이다. |
| 13 | min-content word break가 변환된 longest token과 일치하지 않는다 | 동일 source의 `IIII ii`와 세 transform을 min/max로 구분해 row/track을 고정한다. |
| 14 | CSS font metric quantization 차이로 지나치게 빡빡한 비교가 된다 | CSSOM oracle과 independent advance 합산을 분리하고 0.03125px 이내 tolerance를 사용한다. |
| 15 | test scene만 늘리고 root/item revision을 검증하지 않는다 | exact owner, computed-style revision, HostDocument revision을 같은 scene trace에서 확인한다. |
| 16 | invalid transform이 partial intrinsic contribution을 publish한다 | 미등록 값/scene/source를 fail-closed로 거부하고 publication absence를 검사한다. |
| 17 | 10MB Noto Sans KR bytes가 기본 artifact에 포함된다 | default/product와 opt-in fixture positive-control artifact를 분리 검사한다. |
| 18 | screenshot을 glyph paint pixel 증거로 과장한다 | renderer text area의 실제 제한을 캡처 설명과 PR에 기록한다. |
| 19 | AVD/iOS simulator 성공을 hardware/physical result로 표기한다 | 두 Simulator의 runtime/hash만 보고하고 hardware device는 claim하지 않는다. |
| 20 | fixed ASCII fixture가 일반 CSS/Text/WPT 지원으로 격상된다 | STATUS, contract, plan, PR, roadmap에서 fixed six-scene opt-in 범위와 `72/213` 유지 문구를 반복 명시한다. |
