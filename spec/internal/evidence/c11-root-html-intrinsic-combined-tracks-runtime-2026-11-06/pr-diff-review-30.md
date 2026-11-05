# PR #1195 실제 변경분 적대 검토 — 30개 관점

검토 대상은 PR #1195의 실제 `639a821f490a4bccb0b1a82a395a36072f9ac3c..b7af2b16c5056f976ed106bd56c43f21f9946d57` 변경분이다. 계획 검토와 구현/runtime 검토는 재사용하지 않았다.

| # | 실패 관점 | 결과 및 확인 근거 |
| ---: | --- | --- |
| 1 | 신규 조합이 일반 public CSS 지원으로 잘못 설명됨 | 통과: PR 제목·본문·fixture는 parsed Root HTML bounded route로 제한하고 C11.4/C11 및 `78/214`를 유지한다. |
| 2 | 다른 runtime profile에도 intrinsic implicit rows가 켜짐 | 통과: projection flag는 기존 `RuntimeRootHtmlTextPaintV1`에만 적용되고 profile 전환/FFI는 없다. |
| 3 | intrinsic column grammar가 새로 확장됨 | 통과: projection은 기존 intrinsic explicit-track validator를 계속 통과해야 한다. |
| 4 | `grid-auto-rows`가 단일 min-content 외 값을 포함 | 통과: 정확히 `[MinContent]`만 조합 gate에 들어가며 max-content·복수 값 negative가 유지된다. |
| 5 | 명시 row가 두 개 이상이거나 0/음수 길이로 변형 | 통과: 하나의 finite positive `LengthPx` explicit row를 요구하고 다중 row runtime negative가 원자 거부를 확인한다. |
| 6 | `minmax()`의 max-content 최대값이 projection/layout에서 불일치 | 통과: 검토 중 발견한 layout predicate 누락을 고쳤고 `minmax(10px,max-content)` combined matrix가 통과한다. |
| 7 | 기존 `minmax(auto,1fr)` sizing mode가 새 intrinsic helper로 오분류 | 통과: 기존 auto-minimum fraction 예외가 두 predicate에서 유지된다. |
| 8 | percentage-minimum/fraction-maximum의 기존 intrinsic 경로가 바뀜 | 통과: 양쪽 helper의 예외 분류는 그대로이며 focused layout/projection tests가 통과한다. |
| 9 | explicit row 자체의 intrinsic track가 implicit text row 조건을 만족 | 통과: combined route의 row list는 단일 positive fixed-px로 제한한다. |
| 10 | `grid-auto-columns`를 설정해 implicit column을 만들 수 있음 | 통과: computed validator, capacity gate와 layout preflight가 모두 `auto`를 요구하고 runtime negative가 검증한다. |
| 11 | dense auto-placement가 sparse row algorithm을 통과 | 통과: row flow만 허용하고 `row dense` 입력은 property-context 오류로 원자 거부된다. |
| 12 | column-flow placement가 intrinsic cross-axis sizing으로 섞임 | 통과: flow gate와 기존 `fixed_px_placement_flow` intrinsic rejection이 유지된다. |
| 13 | named area가 implicit row mode에 영향을 줌 | 통과: area composition은 거부되고 named-area negative의 진단은 `grid-template-areas`다. |
| 14 | named line이 expanded track index를 바꿔도 지원으로 오인 | 통과: 기존 explicit intrinsic validator와 layout preflight가 line-name groups를 거부한다. |
| 15 | `auto-fit`/`auto-fill`이 intrinsic columns와 결합 | 통과: auto-repeat은 이 조합의 공통 gate에서 제외된다. |
| 16 | `fit-content()`가 intrinsic list의 이웃 track으로 진입 | 통과: column/row fit-content는 조합과 capacity predicate에서 제외되고 default-session negative가 통과한다. |
| 17 | CSS invalid input을 runtime unsupported와 혼동 | 통과: 신규 negative는 cascade가 Ready인 CSS-valid source 뒤에 layout rejection이 오는지 검사한다. |
| 18 | fixed-only implicit-row 동작이 새 percentage gap 허용을 상속 | 통과: fixed-only legacy branch는 single fixed tracks와 zero gaps를 유지하고 combined percentage gap은 기존 intrinsic allowlist를 거친다. |
| 19 | percentage row gap이 height 또는 indefinite basis를 잘못 사용 | 통과: definite 40px basis에서 5%→2px, item y offset `10/12/74px`를 style/layout tests로 확인한다. |
| 20 | `calc()` gap, 음수/비유한 gap 또는 aggregate overflow가 허용 | 통과: 기존 typed gap allowlist와 finite/aggregate 검사가 변하지 않았고 intrinsic gap negative tests가 통과한다. |
| 21 | 새 capacity 경로가 `item_count` 비교를 생략하거나 overflow | 통과: checked multiplication, explicit fallback과 기존 10,000-track limit을 유지한다. |
| 22 | capacity 확장이 generic fixed implicit-row 또는 dense route에 적용 | 통과: 세 경로 predicate가 분리되어 있고 새 intrinsic capacity는 해당 route만 만족한다. |
| 23 | intrinsic column의 used inline size가 text measurement 이전에 고정 폭으로 바뀜 | 통과: Ahem-backed style test와 default RuntimeSession text test가 직접 text measurement, line count, frame을 확인한다. |
| 24 | OS monospace metric 차이를 Chrome 측정값으로 덮어씀 | 통과: Chrome oracle와 runtime font width가 분리되고 runtime min-content width는 `6..=7px` 허용 범위로 검사한다. |
| 25 | text line 수가 같아도 implicit row trace/placement가 틀림 | 통과: row trace, text frame, source-order item y positions와 visible overflow를 함께 assert한다. |
| 26 | layout reject 후 일부 frame/paint가 공개됨 | 통과: fit-content·named-area·dense·다중-row negative 및 기존 implicit-column negative가 `completed.is_none()`을 확인한다. |
| 27 | successful scene의 CSS/revision identity가 달라짐 | 통과: default runtime test는 cascade/layout key와 text generation/document/render-tree/style revisions를 비교한다. |
| 28 | 시뮬레이터가 fixture 아닌 원본 asset을 실행하거나 플랫폼별 source가 달라짐 | 통과: APK와 iOS app asset, Android/iOS runtime log가 모두 fixture SHA `fc9ff608…3380e`를 기록한다. |
| 29 | PR 본문 화면 링크가 깨지거나 다른 simulator image를 가리킴 | 통과: Agent Browser에서 두 PR `img` 모두 `complete=true`; Android는 1080×2400, iOS는 1206×2622를 로드했다. 브라우저는 확인 직후 닫았다. |
| 30 | 빌드/화면 근거 또는 merge status를 더 강한 증거로 과장 | 통과: build logs, runtime logs, screenshots 및 한계가 분리되어 있다. GitHub PR은 `CLEAN`, status-check rollup 0건이며 이 상태를 CI 통과로 표현하지 않는다. |

PR 본문 이미지의 실제 `currentSrc`, `naturalWidth`/`naturalHeight`를 Agent Browser로 확인했다. GitHub에서 보고된 status check는 0건이다. 로컬 broad crate runs는 layout 453/1, style-to-layout 413/10, runtime 283 passed/8 failed/3 ignored로 전부 green이 아니며 이 PR diff만으로 baseline 실패라고 주장하지 않는다. 새로운 fixed/profile/route bypass는 검토에서 발견되지 않았다.
