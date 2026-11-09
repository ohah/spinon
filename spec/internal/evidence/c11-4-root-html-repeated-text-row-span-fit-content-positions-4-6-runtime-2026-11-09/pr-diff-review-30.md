# PR diff 적대적 검토 30개

검토 대상은 현재 C11.4 bounded slice의 코드, fixture 검사, 계획 문서와 런타임 근거 전체 diff다. 앞서 작성한 구현/runtime 20관점 표를 재사용하지 않고, 변경 파일 사이의 계약과 PR에 실릴 주장까지 별도로 확인했다.

## Root HTML admission과 topology

1. **일반 Root HTML admission 확장 여부** — 새 조건은 기존 제한 fit-content route 안의 반복 텍스트 row-span 후보에만 연결된다. 일반 HTML/CSS admission 또는 feature 전체 지원을 주장하는 경로는 추가하지 않았다. [style admission](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows.rs).
2. **auto-placement profile 간섭** — 기존 9개 item 반복 텍스트 auto-placement은 별도 profile branch에 남아 있으며, 새 row-span recognizer가 그 topology를 대신 허용하지 않는다. [profile split](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows.rs) · [기존 회귀](./checks/matrix-auto-placement-fit-content.log).
3. **track-list 멤버 순서 보존** — fit-content 위치를 허용하도록 순회 제한은 풀었지만, 여섯 멤버와 min-content 2개·max-content 2개·auto 1개·fit-content 1개 multiplicity는 계속 확인한다. [computed track validator](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs) · [4320 scene assertions](../../../../crates/spinon-runtime/src/session/tests/root_html_grid_fit_content_auto_rows/later_fit_content_row_span/positions_4_6/matrix.rs).
4. **fit-content index 기준** — authored 목록의 1-based 위치는 computed vector의 index에 1을 더해 얻는다. 고정 corpus는 위치 4·5·6 각각 1,440개이며 4,320 key가 중복 없이 실행된다. [profile gate](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows/validation_helpers.rs) · [matrix log](./checks/matrix-positions-4-6.log).
5. **이전 3 implicit-row 계약 보존** — 위치 1–3은 기존 3-row topology에서 허용되고, 6-row topology와 결합하면 거부된다. 이전 1–3 matrix와 새 atomic rejection이 이를 따로 확인한다. [position gate](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/profiles.rs) · [1–3 matrix](./checks/matrix-positions-1-3.log) · [rejection cases](../../../../crates/spinon-runtime/src/session/tests/root_html_grid_fit_content_auto_rows/later_fit_content_row_span/positions_4_6/rejections.rs).
6. **6-row에 위치 4–6만 허용** — 새 branch는 positive implicit row가 정확히 여섯일 때 authored 위치 4–6을 통과시키고 1–3은 fail-closed한다. 임의 행 수는 지원하지 않는다. [layout gate](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/profiles.rs).
7. **span2-six topology 과소·과대 생성** — row 2–6에 span 2를 각각 한 번 둔 다섯 placement만 첫 six-row family를 구성한다. 중간 line 누락이나 span 교체는 정확한 multiset 검사에서 거부된다. [topology recognizer](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs).
8. **span2-span3 topology 경계** — row 2/span 2, row 3/span 2, row 4/span 3, row 5/span 3만 두 번째 family다. start 3/span 3 또는 row 6/span 3은 family로 추정하지 않는다. [layout recognizer](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs) · [style recognizer](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows/validation_helpers.rs).
9. **중복·추가 item** — placement multiset 길이와 각 expected placement의 출현 수를 함께 검사한다. 하나를 중복하고 다른 것을 빼거나 유효 placement를 더 붙인 topology는 허용되지 않는다. [topology helpers](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows/validation_helpers.rs) · [rejection suite](../../../../crates/spinon-runtime/src/session/tests/root_html_grid_fit_content_auto_rows/later_fit_content_row_span/positions_4_6/rejections.rs).
10. **DOM source order 의존성** — 두 topology를 source order 정방향·역방향으로 비교하고 semantic placement, frame 및 text 결과를 검사한다. 정렬 때문에 배치 의미가 바뀌는 오류는 corpus에서 잡힌다. [matrix](../../../../crates/spinon-runtime/src/session/tests/root_html_grid_fit_content_auto_rows/later_fit_content_row_span/positions_4_6/matrix.rs) · [4320 log](./checks/matrix-positions-4-6.log).
11. **explicit 첫 행 item** — 첫 item이 비어 있고 row/column 1–2에 고정된 입력만 받는다. 직접 text나 자식 payload가 섞이면 layout topology와 style 검증 중 적어도 하나에서 실패한다. [layout recognizer](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs) · [style validator](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows/validation_helpers.rs).
12. **column placement 누출** — 첫 item과 text item 모두 column line 1–2만 허용한다. 새 row topology를 이용해 다른 열의 track을 row normalization에 섞을 수 없다. [layout recognizer](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs) · [style item kind](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows/validation_helpers.rs).
13. **writing context 상속** — Grid와 각 span item은 LTR·horizontal-tb를 요구하고, 각 payload는 기존 text profile에서 direction과 writing-mode를 확인한다. Grid/item 변형 거부와 payload style 검사 경로가 구분돼 있다. [writing context helper](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows/validation_helpers.rs) · [payload checker](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows/repeated_text.rs) · [post-format test](./checks/row-span-writing-context-rejections-pass02.log).
14. **누락된 style·node·placement** — style vector 수집은 `Option`을 유지하고, layout recognizer는 node/placement 조회 실패에서 `None`을 반환한다. 누락 입력을 빈 기본값으로 대체하지 않는다. [style projection](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows.rs) · [layout recognizer](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs).
15. **text payload 경계** — 각 span item은 직접 payload div 한 개, 제한 ASCII direct-text grammar, 고정 font/line-height·spacing profile을 요구한다. arbitrary inline tree나 다른 shaping 입력을 row-span 지원으로 넓히지 않는다. [payload validator](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows/repeated_text.rs) · [text runtime matrix](./checks/matrix-repeated-text-810.log).
16. **바깥 Grid geometry** — 고정 definite size, track list, gap·edge·alignment·overflow 경계를 기존 validator가 계속 제한한다. row-span 추가로 임의 border/padding 또는 scroll container를 받아들이지 않는다. [style contract](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows.rs) · [4320 matrix](./checks/matrix-positions-4-6.log).
17. **DOM owner·item 집합** — direct visible HTML div 목록과 Grid item owner count가 같아야 한다. hidden 또는 다른 owner의 자식이 기대 item 수를 채우지 않는다. [owner/count gate](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows.rs).
18. **`order` 등 미지원 item 값** — row-start/end gate만으로 style을 승인하지 않고 기존 fixed-item/style-to-layout 경계가 지원되지 않는 order·dimensions를 차단한다. 기존 profile regressions와 함께 확인했다. [item projection](../../../../crates/spinon-style-to-layout/src/projection/style_values/root_html_fit_content_auto_rows.rs) · [existing row-span matrix](./checks/matrix-row-span-960.log).

## Track trace와 frame normalization

19. **trace owner 중복·누락** — 대상 Grid trace는 정확히 하나여야 한다. trace가 없거나 같은 owner의 trace가 중복되면 row 크기를 추측하지 않고 실패한다. [trace selector](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs) · 관련 unit tests.
20. **row axis와 최신 pass 선택** — row axis pass 중 sequence가 가장 큰 pass의 마지막 stage를 읽는다. column pass 또는 중간 pass를 final used row로 오인하지 않는다. [trace stage selector](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs).
21. **explicit/implicit track 개수** — trace의 explicit 1개와 topology의 positive implicit 3개 또는 6개가 맞아야 하며, 반환 track vector도 각각 4개/7개여야 한다. [trace count guard](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs) · [trace count tests](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs).
22. **negative·collapsed·gutter track** — negative implicit row와 collapsed index는 거부한다. gutter는 used row 목록에서 제외한 뒤 내부 index가 1,3,5,… 순서인지 검사한다. [trace topology guard](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs).
23. **NaN·무한·음수 track 크기** — 각 base size가 finite하고 0 이상이어야 frame 산술에 들어간다. 변조된 trace를 결과 frame으로 전파하지 않는다. [track state guard](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs).
24. **item raw frame 대조** — 정규화 전 각 item top과 height를 trace row prefix sum 및 실제 span 합과 비교한다. row 수만 맞고 item layout이 다른 trace는 통과하지 않는다. [raw-frame guard](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs).
25. **line index·overflow 안전성** — CSS line을 zero-based row offset으로 변환한 뒤 checked conversion/addition을 하고, start/end가 track vector 범위 안인지 검사한다. 경계 밖 line 또는 span은 거부된다. [placement normalization](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs).
26. **Blink layout-unit 분배** — raw float row를 그대로 반올림하지 않고 Grid frame과 row list를 `blink_layout_unit_distribute_rows`에 전달한다. returned unit vector length가 달라져도 normalization을 중단한다. [rounding path](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs).
27. **descendant frame 일관성** — item의 delta를 subtree stack으로 전달해 `frames`와 `flow_frames`의 descendant y 위치를 함께 갱신하며, item height만 span 합으로 교체한다. [subtree update](../../../../crates/spinon-layout/src/grid/root_html_fit_content_auto_rows/repeated_text/span.rs) · [frame matrix](../../../../crates/spinon-runtime/src/session/tests/root_html_grid_fit_content_auto_rows/later_fit_content_row_span/positions_4_6/matrix.rs).
28. **부분 mutation의 외부 노출** — normalization 도중 후속 item이 실패하면 내부 output 일부가 수정됐을 수 있지만, session 결과는 failed revision으로 취급되고 `completed` snapshot을 게시하지 않는다. atomic reject와 같은-session recovery를 runtime에서 확인한다. [rejection/recovery](../../../../crates/spinon-runtime/src/session/tests/root_html_grid_fit_content_auto_rows/later_fit_content_row_span/positions_4_6/rejections.rs).

## Vendored sizing, matrix와 문서 주장

29. **span-group zero-space commit 및 cap 분배** — Taffy의 growth-limit 분배는 affected track이 없는 경우에만 건너뛰고, 0 증가도 group 끝에서 flush한다. fit-content cap 및 다음 span group 영향은 고정 4,320 Chrome 비교와 127 track-sizing tests로 확인했다. [Taffy diff](../../../../vendor/taffy/src/compute/grid/track_sizing.rs) · [matrix](./checks/matrix-positions-4-6.log) · [track tests](./checks/grid-track-sizing-127.log).
30. **플랫폼·완료 주장 경계** — README와 계약 0059는 iOS 6 scene 및 Android API 37.2 6 scene을 visual smoke로 한정하고, 다른 API 37 AVD의 QEMU SIGSEGV와 broad layout의 C15 실패를 구분한다. 같은 C15 assertion을 base `abe50125`에서 재현했으며, 계획은 구현 후보로 공식 상태 78/214와 C11.4/C11 미완료를 유지한다. [runtime limits](./README.md) · [internal contract](../../0059-c11-grid-layout.md) · [baseline reproduction](./checks/c15-host-document-baseline-origin-main.log) · [plan](../../../../plan/c11-4-root-html-repeated-text-row-span-fit-content-positions-4-6.md).

## 검토 결과

이 PR diff 검토에서 별도 제품 결함은 추가로 찾지 못했다. 구현/runtime 20관점 검토에서 이미 발견한 writing-context admission 누락과 Taffy 설명 주석은 수정됐으며, 영향받는 거부 검사는 포맷 후 재실행해 통과했다. Android API 37.2 시각 smoke는 통과했다. broad `spinon-layout --lib`의 C15 assertion은 base `abe50125`의 동일 filtered test에서도 재현되며 이 PR이 도입한 실패가 아님을 분리했다. 전체 base suite의 통과 수는 추정하지 않는다.

## 플랫폼 smoke runner 후속 변경 검토

후속 수정은 GPU mode 선택, Android process-start 진단, 테스트 패키지 정리, lifecycle 기록 보존만 다룬다. 제품 layout/runtime 코드는 바뀌지 않았다.

1. **GPU 입력 경계** — 허용된 emulator GPU mode만 통과하며 잘못된 값은 에뮬레이터 실행 전에 종료한다.
2. **프로세스 시작 관찰** — `pidof`의 정상적인 미발견 반환값이 `set -e`를 발동하지 않게 하고, 제한 시간 후에도 프로세스가 없으면 logcat·activity·package 상태를 저장한다.
3. **테스트 앱 소유 경계** — 이 실행에서 설치한 앱만 종료 때 제거한다. 설치 전 실패는 기존 패키지 상태를 건드리지 않는다.
4. **플랫폼 정리 순서** — uninstall은 에뮬레이터·시뮬레이터 종료 전에 수행하고 결과를 개별 로그로 보존한다. uninstall 실패는 smoke 결과를 실패로 만든다.
5. **교차 실행 lifecycle** — Android-only 실행은 기존 iOS 종료 결과를 보존하며, 반대도 같다. 다른 lifecycle 항목은 키 단위 수정에서 유지된다.
6. **증거 경계** — 앱 실행·layout-ready·draw/present·screenshot은 기록하지만 이 검증을 geometry parity나 pixel oracle로 승격하지 않는다.

이 항목들은 단일 smoke runner diff에 대한 보완 검토이며, 기존 PR의 서로 다른 30관점 검토를 대체하지 않는다. 후속 Android 및 iOS 실 실행에서 package uninstall, simulator shutdown, 반대 플랫폼 lifecycle 행 보존을 확인했다. 최종 raw output은 `checks/platform-logs-final-smoke-2026-11-09.tar.gz`에 남겼다.

## 내부 계약 0059 candidate 추가 검토

1. **candidate 상태** — 계약 절은 PR #1306 미병합 후보로 표시하고 merge SHA나 공식 완료를 미리 주장하지 않는다.
2. **profile 분리** — 변경 범위를 row-span authored positions 4–6으로 한정하고 auto-placement positions 1–6 및 기존 row-span positions 1–3을 다시 정의하지 않는다.
3. **track multiplicity** — `fit-content ×1`, `min-content ×2`, `max-content ×2`, `auto ×1`의 여섯 member를 보존한다.
4. **position·tail corpus** — authored position당 30개 tail 순서로 제한하고 모든 여섯-member permutation 지원을 암시하지 않는다.
5. **cap corpus** — 실행한 6개 px/% cap만 적고 definite 220px basis를 적었다.
6. **placement topology** — 두 six-implicit-row topology를 따로 명시하고 다른 row span이나 implicit columns를 허용하지 않는다.
7. **alignment·writing context** — `start|stretch`, LTR/horizontal 조건을 유지한다.
8. **장면 수 검산** — 3×30×6×2×4가 4,320이며 Chrome 157 비교 corpus와 일치한다.
9. **platform·baseline 한계** — iOS/Android는 visual smoke로 한정하고 C15 baseline filtered test가 전체 baseline suite를 증명하지 않는다고 적었다.
10. **상태 대장 경계** — candidate entry가 `78/214`, C11.4/C11 checkbox와 contract `0.1.0`을 바꾸지 않는다.

이 추가 검토는 계약 문구의 새 candidate 절을 실제 matrix·runner 근거와 대조한 기록이다. PR의 30개 구현 diff 관점과 별도이며 이를 대체하지 않는다.
