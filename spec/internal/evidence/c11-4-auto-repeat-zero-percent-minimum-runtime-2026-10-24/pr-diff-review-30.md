# C11.4 0% auto-repeat minimum PR diff 적대 검토 30개

검토 대상은 현재 C11.4 zero-percent 구현, fixture adapter, capture 변경과 이에 대응하는 계획·명세·상태 대장·공개 roadmap diff다. 각 항목은 구현/runtime 20개 검토와 겹치지 않도록 변경 경계와 문서 주장을 다시 대조했다.

| # | 별도 실패 경로 | diff 판정과 확인 근거 |
| ---: | --- | --- |
| 1 | cascade가 authored percentage 전부를 허용 | finite nonnegative percentage minimum만 기존 supported-maximum gate와 함께 허용한다. 직접 percentage track과 unsupported maximum은 계속 거부된다. |
| 2 | NaN·양/음의 무한대가 스타일 snapshot에 진입 | finite check가 먼저 실행되며 invalid values는 typed cascade error 경로에 남는다. |
| 3 | 음수 percentage가 0% 구현에 섞임 | 비교는 0 이상 조건이며 negative percentage는 허용하지 않는다. |
| 4 | signed zero가 잘못 거부됨 | computed negative zero도 수학적 0이므로 같은 0% typed path에 들어간다. |
| 5 | calc evaluator가 의도치 않게 확대됨 | 이 변경은 Stylo가 calc(0px + 0%)를 Percentage(0)으로 정규화한 경우만 허용하며 새 evaluator나 임의 calc grammar를 추가하지 않는다. |
| 6 | minmax가 아닌 authored form이 auto-repeat gate를 우회 | 기존 MinMax structural match가 유지되고 control test에서 out-of-scope percentage form을 거부한다. |
| 7 | maximum sizing gate가 우회 | cascade와 layout 모두 기존 supported maximum 판정을 계속 적용한다. |
| 8 | min-size / max-size 조건이 사라짐 | layout preflight는 기존 zero min-size와 auto maximum 조건을 유지한다. |
| 9 | indefinite 또는 cyclic axis가 definite로 오인 | explicit positive content-axis 요구가 유지되고 indefinite/zero basis controls는 typed rejection을 기대한다. |
| 10 | percentage basis가 border-box 외곽 크기를 사용 | content-axis resolution과 border-box scene의 116px content basis가 명세·fixture 기대값에 고정됐다. |
| 11 | basis 곱셈 overflow가 finite로 보임 | resolved percentage value는 finite 검사 뒤 Taffy 입력으로 전달된다. |
| 12 | UA floor가 실제 minimum sizing 값에 저장 | floor는 반복 횟수 계산의 지역 값에만 적용되고 initialize / used sizing에는 원래 sizing function이 남는다. |
| 13 | 반복 그룹을 한 번만 floor | 반복 definition의 각 track contribution을 따로 floor한다. 두-track group control은 60회 반복을 기대한다. |
| 14 | subpixel repeated track이 floor를 빠져나감 | 0.5px explicit repeated track도 1px count contribution으로 계산되며 0.5% control의 actual minimum은 별도 값으로 보존된다. |
| 15 | definite maximum 대신 minimum을 floor | 기존 max-if-definite / otherwise-minimum 선택을 보존한 뒤 선택된 count size에만 floor한다. fixed 12px maximum control이 별도 검증된다. |
| 16 | flex maximum의 zero minimum이 실제 1px minimum이 됨 | minmax(0%, 1fr)의 zero minimum을 count에만 floor하고 flex distribution 결과는 fixture에서 독립 확인한다. |
| 17 | gap이 track floor에 중복 포함 | gap은 기존 별도 count 항으로 유지되고 gap 4px의 24회 count control이 있다. |
| 18 | explicit prefix/suffix가 floor 영향 받음 | count 계산의 non-repeating tracks는 원래 크기를 사용한다. 0.5px 양쪽 이웃 control은 23 repeats를 기대한다. |
| 19 | first repetition gap 수가 하나 어긋남 | 첫 반복과 주변 non-repeating tracks의 gutter 수, 이후 반복별 gutter 수를 따로 계산하며 gap 0/4 controls가 구분돼 있다. |
| 20 | 음수 generic repeated size가 1px floor에 숨겨짐 | negative contribution을 invalid sentinel로 전환해 floor 적용 전에 실패시킨다. 단독 및 음수+양수 혼합 regression case가 있다. |
| 21 | NaN·Infinity가 나눗셈 또는 float-to-int cast에 도달 | intermediate finite gate와 invalid count fallback이 있고 NaN·양/음의 무한대 입력의 count 결과를 확인한다. |
| 22 | 0 또는 음수 denominator로 과대 반복 | positive container, nonnegative gap, valid first-space, positive finite per-repeat contribution을 나누기 전에 확인한다. zero container control은 한 번 반복 fallback을 기대한다. |
| 23 | 큰 반복 수가 정수 변환에서 wrap | 기존 saturating conversion 및 MAX_GRID_TRACKS clamp를 유지하며 invalid arithmetic은 cast 전에 차단한다. |
| 24 | 13개 scene이 fixture catalog에 빠지거나 중복 | scene JSON의 수와 고유 ID를 capture catalog 및 focused selection 결과에서 대조한다. capture 전체 count는 신규 13개를 포함해 405다. |
| 25 | Android/iOS adapter가 scene을 기본 frame으로 검증 | 양 플랫폼 allowlist와 scene별 root/item expectation에 13개 ID를 연결했고 실제 최신 재빌드에서 각각 13/13 결과가 있다. |
| 26 | item 없는 auto-fit scene에서 box count가 2로 판정 | empty item frame과 native box count가 별도로 처리돼 empty/occupied/span-two 장면의 기대값을 구분한다. |
| 27 | AVD 로그 burst가 trace 중간 chunk를 누락해도 pass | opt-in logger의 pacing과 capture의 complete part-set 대기를 함께 적용한다. missing/불일치 part는 scene 검증 실패다. |
| 28 | trace 조각 순서나 payload 손상에도 결과가 유효 | capture는 run ID·전체 part index·총수 일관성을 확인하고 index 순서로 조립한 JSON을 parse한다. |
| 29 | fixture ABI가 일반 product archive에 노출 | default/product archive scan은 scene 및 host symbol marker 부재를, fixture-test positive control은 두 marker 존재를 기록한다. 이는 static archive 범위로만 설명한다. |
| 30 | 제한 slice가 상위 기능 완료로 집계 | 내부 계약·공식 STATUS·공개 roadmap은 모두 13 scene slice만 서술하고 C11.4 미완료 및 72/213을 유지한다. 일반 CSS admission, WPT, 실기기, hardware GPU와 compositor 이후 pixels를 완료 근거로 확대하지 않는다. |

**결과:** diff에서 미해결 결함은 찾지 못했다. 구현/runtime 검토의 negative generic repeated-size와 trace-chunk 누락 발견은 최종 코드에서 각각 invalid sentinel, Android log pacing 및 complete-set capture gate로 닫혀 있으며 최종 simulator 결과는 두 플랫폼 모두 13/13이다. 근거: [구현/runtime 검토](implementation-runtime-review-20.md) · [runtime 결과와 제한](README.md) · [Android 결과](android-avd-c114-auto-repeat-zero-percent-minimum-android-avd-20261023T170458589Z-c0537217/result.json) · [iOS 결과](ios-simulator-c114-auto-repeat-zero-percent-minimum-ios-simulator-20261023T170458589Z-b80ae17b/result.json).
