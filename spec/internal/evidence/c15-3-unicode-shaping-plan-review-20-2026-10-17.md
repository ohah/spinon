# C15.3 계획 amendment 적대 검토 20개

**검토 범위:** C15.3 수용 기준 표, 고정 font/oracle provenance, Chrome observation, WPT 연결, STATUS/0001 문서 변경. 20개는 서로 다른 실패 경로를 각각 확인했다. 이 검토는 계획·oracle 품질 검토이며 Spinon 구현 통과 판정이 아니다.

| # | 독립 실패 관점 | 공격 입력·실패 조건 | 확인 결과 |
| --- | --- | --- | --- |
| 1 | 계획 수치가 구현 완료로 오인됨 | Chrome geometry만으로 C15 완료 체크 또는 지원 표기가 올라가는지 확인 | evidence, plan, STATUS와 0001에 Chrome DOM 사전 비교만 추가했고 C15/C16/C19 checkbox/status는 미완료로 유지한다. |
| 2 | system font가 oracle을 바꿈 | Noto asset 미로드 시 OS 한글 글꼴이 대신 측정되는 상황 | explicit @font-face URL, 각 font check, pinned file hash가 있고 runtime gate는 system fallback에 의존하지 않는다. |
| 3 | font file 자체가 바뀜 | 같은 이름의 다른 Noto 버전 또는 재생성 subset | upstream commit·version·SHA-256을 font README와 evidence에 둔다. |
| 4 | variable font axis가 달라짐 | 브라우저가 400 외 weight/width를 선택해 advance가 변함 | 각 fixture에서 400을 지정하고 Arabic width 축은 기본 100을 쓴다. 이후 runtime input도 font revision/axis를 묶어야 한다. |
| 5 | Hangul codepoint fallback | 한글 테스트 중 일부 음절만 asset에서 빠져 혼합 글꼴이 됨 | 지정 sample codepoint coverage와 browser font check를 기록한다. 전체 Korean coverage를 주장하지 않는다. |
| 6 | CJK 줄 시작 금지 위반 | 좁은 40px에서 U+3001가 새 줄 맨 앞에 배치됨 | 관찰 grouping이 甲 / 乙、 / 丙丁이며 U+3001의 위치를 별도 acceptance 조건으로 적었다. |
| 7 | 줄바꿈 input과 WPT가 혼동됨 | custom Chrome sample이 upstream WPT 원문과 같다고 오인 | WPT는 고정 revision의 관련 candidate paths로만 연결했고 실행/동일 입력/pass 주장은 명시적으로 부정한다. |
| 8 | combining mark가 글자 두 개로 계산됨 | x와 U+0323의 UTF-16/codepoint 수를 glyph count로 사용 | UTF-16 [0,2] grapheme range와 DOM advance를 고정한다. shaped cluster mapping은 runtime gate로 분리한다. |
| 9 | ligature Canvas 측정 false negative | Canvas가 CSS font-variant-ligatures:none을 따르지 않음 | enabled/disabled DOM widths를 oracle로 쓰고 Canvas가 둘을 구별하지 않는다는 caveat를 기록했다. |
| 10 | ligature 존재와 glyph ID 혼동 | DOM range 폭만으로 ffi glyph ID를 단정 | 계획은 DOM geometry를 reference로 두고 shaped glyph/run evidence를 별도 요구한다. |
| 11 | Arabic joining을 개별 문자 advance로 재현 | marks와 contextual joining이 분리되거나 line boundary에서 풀림 | Noto Sans Arabic의 단어와 marks를 고정하고 WPT shaping candidate를 연결했다. |
| 12 | Arabic/Latin Bidi ordering 회귀 | RTL 문단 안에서 123이 역순이 되거나 Arabic run이 LTR화 | container direction과 digit 순서를 별도 조건으로 기록했다. |
| 13 | Arabic font size와 line-height 혼동 | 28px line-height를 28px em으로 잘못 구현 | fixture를 재대조해 입력을 20px font / 28px line-height로 명시했다. |
| 14 | Emoji ZWJ fallback·다색 환경 오염 | OS color emoji가 local fixed font 대신 선택됨 | Noto Emoji 단색 outline, explicit font check, GSUB mapping을 고정했다. browser glyph ID는 검증되지 않았다고 한정한다. |
| 15 | Emoji grapheme와 glyph 수 혼동 | UTF-16 5-unit sequence를 5 glyph로 취급하거나 DOM range 하나를 glyph proof로 취급 | DOM은 한 grapheme range만 입증한다. actual shaped cluster/glyph identity/ink는 runtime 증거가 필요하다고 적었다. |
| 16 | intrinsic width가 container별로 달라짐 | 같은 text shaping 결과가 Block/Flex/Grid에서 다른 max-content width를 냄 | 세 container의 56.8125×24 관찰을 독립적으로 기록하고 같은 revision-bound shaping consumer를 요구한다. |
| 17 | visible overflow가 parent에 잘려 보임 | 15px parent의 20px text가 frame만 확장하고 ink/hit은 숨김 | visible DOM hit point를 기록했고 실제 ink/clip/hit 결과는 Simulator runtime gate에 남겼다. |
| 18 | hidden clip과 hit-test 혼동 | clipped text가 외부 point에서 계속 hit 되거나 visible overflow hit 결과를 hidden에 재사용 | hidden 외부 point null 조건과 visible 외부 hit을 반대 control로 따로 둔다. |
| 19 | scroll container를 hidden/forced scrollbar와 혼동 | fixture가 overflow:scroll이라고 문서화됐지만 실제 CSS는 auto | fixture CSS와 oracle 모두 overflow:auto로 바로잡았고 clientWidth/scrollWidth를 고정한다. |
| 20 | paint order와 antialias 비교의 경계 | text antialias 차이가 overlay z-order 실패를 가림 | overlay rect 중첩과 위쪽 paint 순서를 geometry로 적고 후속 pixel gate는 flat-color 내부 표본을 비교한다. |

## 수정 및 잔여 gate

검토 중 계획 입력의 Arabic/emoji font-size 표기와 overflow:auto 대 overflow:scroll 불일치를 발견해 문서 설명을 실제 fixture와 맞췄다. 고정 oracle은 Chromium DOM geometry와 화면만 제공한다. 구현 후 실제 WGPU glyph paint, Android API 37 ARM64 AVD, iPhone 17 Pro/iOS 26.2 Simulator, parent-border 바깥 ink·clip·hit-test 및 opt-in/default artifact boundary를 별도로 검증해야 한다. 이 검토는 해당 작업의 implementation-review-20 또는 PR-delta-review-30을 대신하지 않는다.
