# C11.4 repeated intrinsic text content · PR diff review 30

**검토 대상:** 제한 ASCII content grammar, Root HTML validator admission, Blink 1/64 CSS px row remainder distribution, RuntimeSession 810-scene matrix, plan/status/roadmap 및 Android/iOS smoke evidence.

| # | 독립 실패 관점 | 변경 코드·근거 확인 | 결과 |
|---:|---|---|---|
| 1 | 32바이트 판정을 Unicode 문자 수로 처리 | Layout과 Style projection 모두 UTF-8 `as_bytes().len()`을 사용하고 32보다 큰 입력을 거부한다. | 통과 |
| 2 | 빈 단어 또는 0자 token 허용 | 첫/마지막 token과 공백 앞 token의 길이를 검사한다. 빈 text/공백 mutation을 RuntimeSession에서 거부한다. | 통과 |
| 3 | 단어별 6/7자 경계 off-by-one | 1–6자 단어와 7자 거부 단위 검사가 있고 boundary corpus도 실행했다. | 통과 |
| 4 | 전체 32/33 byte 경계 off-by-one | 32-byte accepted case와 33-byte rejection을 각각 Layout/Style/RuntimeSession에서 확인했다. | 통과 |
| 5 | 선행·후행·연속 space collapse 우회 | grammar는 단일 ASCII space 사이의 비어 있지 않은 대문자 token만 허용한다. 세 spacing mutation이 실패한다. | 통과 |
| 6 | tab, line feed, 다른 공백의 정규화 우회 | ASCII space 외의 byte는 모두 거부하고 tab/newline rejection을 실행했다. | 통과 |
| 7 | 소문자·숫자·문장부호·Unicode 유입 | `A..Z`와 ASCII space만 허용한다. 각 문자군과 한글을 음성 입력으로 확인했다. | 통과 |
| 8 | 여러 text source를 하나로 합쳐 profile 통과 | Layout은 paragraph source 1개, Style projection은 payload의 직접 text child 1개만 받는다. 추가 text source mutation은 거부한다. | 통과 |
| 9 | nested inline이 glyph 기여에 영향을 주는데 gate 누락 | `inline_ancestors`, inline style/run style, payload direct child 구조를 유지하고 nested inline mutation을 거부한다. | 통과 |
| 10 | text-transform·white-space가 source 문자열과 glyph를 다르게 만듦 | projection의 computed properties와 Layout text style 양쪽에서 transform/wrapping 조건을 제한한다. nowrap mutation도 거부한다. | 통과 |
| 11 | font family·font size·weight·style 차이가 row 크기를 바꿈 | monospace 10px, weight 400, normal font style 및 computed font size 조건을 유지한다. 각 변형 음성 검사가 있다. | 통과 |
| 12 | line-height 또는 letter/word spacing이 text contribution을 바꿈 | line-height 10px와 zero spacing/indent gate를 확인했고 line-height mutation이 실패한다. | 통과 |
| 13 | locale, direction, vertical writing 또는 ligature mode가 바뀜 | LTR/horizontal/mixed/normal text style allowlist를 Layout와 projection에서 대조했다. ASCII grammar 밖 writing-system claim은 하지 않는다. | 통과 |
| 14 | six-member track list만으로 다른 profile에 보정 적용 | admission과 normalization 모두 `RootHtmlFitContentAutoRowsV1`, 지원 track list, 9 visible item 및 단일 text payload 구조를 확인한다. | 통과 |
| 15 | 첫 explicit item이나 뒤 8개 item의 cardinality/자식 형태 변형 | 기존 contract의 explicit-row item, 9개 visible children, 각 text item's 단일 payload 조건이 유지되고 topology mutation은 실패한다. | 통과 |
| 16 | 숨김 subtree가 placement 순서에 섞이거나 dense flow가 row mapping을 바꿈 | layout normalizer는 `hidden_subtrees`로 visible children을 모으며 root contract는 row auto-flow와 허용 geometry를 검사한다. matrix가 30 tail 순서를 순회한다. | 통과 |
| 17 | track trace를 normalization 전에 읽어 rounding 정보 누락 | `compute`에서 intrinsic inputs 및 track trace를 수집한 뒤 `normalize_repeated_text_layout_frames`를 호출한다. | 통과 |
| 18 | stale/wrong axis 또는 이전 track pass를 사용 | 현재 grid trace에서 Rows axis의 마지막 sequence pass를 고르고 `IntrinsicSizingComplete` stage를 요구한다. | 통과 |
| 19 | gutter, 숨은 track, 누락 track trace가 row 인덱스를 어긋나게 함 | gutter 제외 후 intrinsic/final track 수와 row count가 일치하지 않으면 trace 보정을 적용하지 않는다. 지원 CSS route에서는 좁은 track list에 trace를 요청한다. | 통과 |
| 20 | 마지막 stage base가 실제 row geometry와 다름 | 각 final `base_size`가 raw row size에서 0.01px 넘게 다르면 보정 대신 fallback한다. 810 scene 모두 trace 경로에서 oracle 허용치를 만족했다. | 통과 |
| 21 | percentage fit-content cap을 viewport나 width 기준으로 해석 | bounded fixture의 definite Grid block size인 `grid_frame.height`를 사용하며 px/direct percentage 외 `calc`는 fail-closed fallback한다. 0%, 120%, px 및 overflow 조합이 matrix에 있다. | 통과 |
| 22 | f32 noise가 정확한 1/64 unit을 fractional로 오인 | floor/nearest 판정은 크기에 비례한 f32 error bound를 사용한다. 정수 unit boundary를 포함한 810 scene geometry가 통과했다. | 통과 |
| 23 | 음수·NaN·무한 row size가 정수 변환으로 넘어감 | raw row가 finite이고 0 이상인지 먼저 확인한다. 조건 위반은 trace 기반 보정을 거부한다. | 통과 |
| 24 | residual 총량 반올림이 음수 또는 fractional row 개수 초과 | floor total과 rounded raw total 차이를 검사하고 허용 residual 범위를 벗어나면 fallback한다. | 통과 |
| 25 | residual unit을 track 성장 가능성과 무관하게 분배 | intrinsic growth limit와 fit-content cap으로 growth potential을 계산해 큰 potential 순으로 최대 1/64 unit을 나눈다. 810 geometry는 Chrome 157 기대값과 일치한다. | 통과; Chromium 소스와 관측값에 근거한 추론이며 Chrome binary 내부 trace는 아님 |
| 26 | repeated auto-row track list의 explicit/implicit 반복 위치 혼동 | explicit row 이후에는 auto-row track-list 길이로 modulo 위치를 계산한다. 30개 distinct tail order와 네 cap × 두 alignment를 포함한 810 scene이 통과했다. | 통과 |
| 27 | absolute boundary floor와 row-size 분배가 누적 오차를 재도입 | row size units를 정수로 누적해 다음 item top을 정하고 4개 frame component를 ±0.01px로 비교한다. 초기 absolute-floor 후보의 1,176 failure artifact도 보존했다. | 통과 |
| 28 | normalizer가 render frame만 움직여 flow frame과 child 위치가 달라짐 | row subtree의 `frames`와 `flow_frames`를 함께 이동하고 item height만 normalized row height로 갱신한다. RuntimeSession의 Grid/item/payload frame 비교가 통과했다. | 통과 |
| 29 | 넘친 text를 row 크기만 검사해 Grid 경계 밖 영역을 놓침 | 각 corpus마다 tail별 100px Grid 장면을 두고 payload bottom이 Grid bottom보다 0.01px 넘는지 assert한다. Android/iOS 앱 screenshot은 별도 visual smoke로 기록했다. | 통과; Simulator screenshot은 pixel readback이 아님 |
| 30 | 실패가 partial output을 publish하거나 실패 후 session을 복구하지 못함; unrelated full-suite failure를 숨김 | 음성 mutation의 `completed=None`, 동일 RuntimeSession dark/light rejection-recovery를 확인했다. 전체 Style suite C07.3 aspect-ratio 문자열 mismatch는 별도로 재현됐지만 baseline attribution은 미확정이며 전체 suite는 green이 아니다. | 범위와 제한을 문서화 |

## 전체 판정

새 결함은 발견하지 못했다. Focused RuntimeSession 3/3, Layout track sizing 127/127, Layout 및 Style grammar 2/2씩, 변경 3 crate Clippy `-D warnings`, rustfmt와 diff whitespace 검사가 통과했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 세 원본 fixture hash를 양쪽 동일하게 실행했다. 화면·데이터 범위는 [runtime README](README.md)를 참고한다. C11.4/C11 완료 체크는 이 검토로 바뀌지 않는다.
