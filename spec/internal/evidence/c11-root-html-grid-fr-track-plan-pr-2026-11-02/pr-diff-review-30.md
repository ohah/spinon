# C11.1 Root HTML Grid `fr` 계획 PR 변경분 적대적 검토 30회

범위는 이 계획 PR의 staged diff 전체다. 20개 계획 실패 관점 검토와 별도로 문서·fixture·근거의 일치, 오해 가능한 완료 주장, 변경 경계를 확인했다.

| 회차 | 변경분 실패 관점 | 결과 |
|---:|---|---|
| 1 | C11 parent plan이 계획 검토 전에 구현 중인 기능처럼 보이는가 | 통과. 구현 전 후보이며 계획 PR 뒤 구현한다는 상태를 적었다. |
| 2 | 기존 C11.1/C11.4/`72/213` 병합 사실을 수정하거나 지우는가 | 통과. 기존 문단의 실질 내용은 그대로고 Root HTML 섹션 앞에 링크만 추가했다. |
| 3 | C11 상위나 하위 체크를 완료로 표시하는가 | 통과. 이 PR은 체크박스·`spec/STATUS.md`를 수정하지 않는다. |
| 4 | 내부 계약 0059 숫자 버전을 올리는가 | 통과. 계획은 `0.1.0` 고정을 명시하고 계약 파일을 변경하지 않는다. |
| 5 | 다른 runtime profile에서 fraction을 지원한다고 읽히는가 | 통과. parsed Root HTML `RuntimeRootHtmlTextPaintV1` 경로로 제한했다. |
| 6 | 기본 런타임 대신 fixture constructor로 목표를 달성하는가 | 통과. 기본 Root HTML 진입점과 실제 selector/computed-style 경로를 목표로 삼았다. |
| 7 | fixed px track 회귀 범위가 누락되는가 | 통과. 고정 track과 기존 Root HTML 회귀를 유지한다. |
| 8 | named-area fixed profile이 fraction 허용으로 넓어지는가 | 통과. area validator는 현재 px-only 조건을 그대로 둔다. |
| 9 | `minmax()`·intrinsic·auto-repeat이 우연히 수용되는가 | 통과. direct `LengthPx`/`Fraction`만 허용하는 조건과 negative matrix를 적었다. |
| 10 | negative/NaN/infinite factor가 통과하는가 | 통과. finite non-negative 조건과 거부 관점을 명시했다. |
| 11 | `0fr` item이 겹치므로 결과가 잘못됐다고 숨기는가 | 통과. zero-width frame과 overlap을 expected에 그대로 남겼다. |
| 12 | factor 합이 1보다 작을 때 잔여 공간을 전부 채운다고 잘못 쓰는가 | 통과. `.25fr + .25fr`에서 50px trailing free space를 기록했다. |
| 13 | mixed fixed/fr track에서 gap을 중복 차감하는가 | 통과. 식에 두 gap을 한 번씩 포함하고 item 시작점을 별도 기대한다. |
| 14 | 행 축과 열 축을 같은 dimension으로 검증하는가 | 통과. row/column positive scene과 축별 definite-size gate가 분리됐다. |
| 15 | min/max 또는 aspect ratio가 definite basis를 바꾸는가 | 통과. 제한 조건과 negative coverage를 계획에 남겼다. |
| 16 | DOM ID/class 순서를 layout provenance로 사용한다고 오해되는가 | 통과. rename/reverse control과 source-order 검증을 요구한다. |
| 17 | implicit placement·extra item이 새 implicit track을 만드는가 | 통과. capacity 초과 reject와 partial output 부재를 요구한다. |
| 18 | CSS-invalid 입력을 유효한 negative runtime control로 취급하는가 | 통과. 세 runtime negative control은 CSS-valid임을 고정 기록한다. |
| 19 | 큰 factor 회귀 입력이 실제 f32 overflow를 일으키는가 | 통과. 잘못된 두-factor 전제를 제거했고 3-factor finite control/4-factor overflow를 나눴다. |
| 20 | 표준 계산과 Chrome 156 오류 관측을 같은 기대값으로 합치는가 | 통과. W3C `[30,30,30,30]`과 Chrome `[0,0,0,0]`을 서로 다른 필드에 저장한다. |
| 21 | Chromium current source를 Chrome 156 바이너리와 동일한 revision이라고 주장하는가 | 통과. source 인용은 조사 단서이고 정확한 build revision 증거가 아니라고 적었다. |
| 22 | Agent Browser를 terminal-browser로 잘못 보고하는가 | 통과. 기록과 screenshot은 Agent Browser HeadlessChrome 156 관측이다. |
| 23 | observation이 실제 fixture와 다른 입력에서 만들어졌는가 | 통과. URL, scene 목록과 SHA-256 verifier를 연결했다. |
| 24 | observation의 track count/frame 또는 negative controls가 검증되지 않는가 | 통과. verifier가 8 positive, 1 divergence, 3 negative 전체의 tracks/frames를 확인한다. |
| 25 | verifier가 browser divergence를 조용히 pass 처리해 expected spec 값으로 둔갑시키는가 | 통과. Chrome 값을 원본과 대조하고 f32 overflow 및 독립 f64 비율 계산도 assert한다. |
| 26 | screenshot이 root/item pixel equality나 앱 결과를 증명한다고 오해되는가 | 통과. README가 browser geometry만 증명하고 app/pixel/WPT 범위를 제외한다. |
| 27 | screenshot hash와 README의 값이 어긋나는가 | 통과. 최종 PNG SHA-256을 기록했고 화면을 직접 열어 네-factor 빈 track 장면도 보임을 확인했다. |
| 28 | simulator 결과가 계획 PR에서 이미 실행된 것처럼 적히는가 | 통과. Android/iOS는 구현 후 수행할 gate로만 기술했다. |
| 29 | CI/check rollup이 보고되지 않았는데 green이라고 쓰는가 | 통과. 계획 PR은 CI 통과를 주장하지 않고 실제 check는 PR 생성 뒤 확인한다. |
| 30 | unrelated WIP, 다른 evidence 또는 cache가 함께 staged 되었는가 | 통과. staged path는 계획 문서 3개, 이 PR diff 리뷰, 해당 fixture 및 전용 precomparison evidence 5개뿐이다. |

## 결과

30개 모두 변경분 범위에서 수정 또는 계획 gate로 처리했다. 계획 실패 관점 검토 20개와 분리된 PR diff 검토다. CI, Rust implementation test, Android/iOS simulator 결과를 나타내지 않는다.
