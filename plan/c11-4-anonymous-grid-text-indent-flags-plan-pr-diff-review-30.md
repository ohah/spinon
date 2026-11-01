# C11.4 `text-indent` flags 계획 PR 변경 검토 30개

**검토 대상:** `hanging`·`each-line` 제한 계획, Chrome 사전 비교, 독립 기대값, 계획 자체 검토, C11 문서와 공개 로드맵 변경.

각 관점은 다른 실패 경계나 문서 계약을 확인한다. 이 검토 기록 자체는 검토 대상 diff에서 제외했다. 구현/runtime 검토 20개와 구현 PR diff 검토 30개는 이후 별도로 진행해야 한다.

| # | 독립 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 변경에 제품 Rust·FFI·앱 runtime 코드가 섞였는가? | 계획·fixture·oracle·문서만 포함하고 제품 코드 변경은 없다. **PASS** |
| 2 | 사전 비교 fixture를 runtime scene admission으로 오해할 수 있는가? | 파일명과 문서가 precomparison 입력이라고 밝히며 app fixture 등록을 하지 않는다. **PASS** |
| 3 | C11 parent를 완료 처리했는가? | `spec/STATUS.md`의 C11 checkbox를 바꾸지 않았다. **PASS** |
| 4 | C11.4 parent를 완료 처리했는가? | C11.4 checkbox를 그대로 미완료로 두었다. **PASS** |
| 5 | 프로젝트 완료 집계가 바뀌었는가? | 계획·oracle만 추가하고 `72/213`을 유지한다고 명시한다. **PASS** |
| 6 | 새 카드가 이미 구현되거나 simulator 검증됐다고 주장하는가? | STATUS·상위 계획·공개 roadmap 모두 구현과 Android/iOS 실행 전이라고 적는다. **PASS** |
| 7 | 전체 CSS `text-indent` 지원으로 확대 표현했는가? | 계획은 고정 anonymous direct-text opt-in route만 다룬다. **PASS** |
| 8 | Spinon WPT 결과로 browser oracle를 혼동하는가? | Chrome 관측을 사전 비교로 분리하고 Spinon WPT를 통과했다고 쓰지 않는다. **PASS** |
| 9 | 계획의 규범 출처가 추적 가능한가? | CSS Text §8.1, CSS Sizing §5.1, CSS Grid §11과 고정 버전을 기록한다. **PASS** |
| 10 | Chrome 관측을 독립 기대값의 산출 근거로 썼는가? | Ahem advance와 규범 계산을 expected JSON에 두고 Chrome observation을 교차 확인으로 분리한다. **PASS** |
| 11 | soft wrap과 forced break의 구분이 빠졌는가? | `XXX X`/normal과 `XXX\nX`/pre-line을 별도 행렬로 고정한다. **PASS** |
| 12 | `each-line`이 soft wrap 다음에도 적용된다고 오해하는가? | CSS Text 경계와 각 line origin 기대값에 forced/soft 차이를 반영한다. **PASS** |
| 13 | `hanging`이 어떤 line 집합을 뒤집는지 모호한가? | first/continuation line별 기대 origin과 flags 조합을 독립 JSON에 고정한다. **PASS** |
| 14 | 두 keyword 동시 사용 결과가 빠졌는가? | `10px hanging each-line`을 soft/forced 및 min/max-content 조합에 포함한다. **PASS** |
| 15 | min-content와 max-content 기대값을 섞었는가? | soft-wrap min advances `[30,10]`, max advance `50` 및 forced line advances를 따로 기록한다. **PASS** |
| 16 | forced break의 zero-width Range marker를 별도 text line으로 셌는가? | verifier와 expected data는 text fragment, root height, y origin을 함께 확인한다. **PASS** |
| 17 | line-height나 font 변동이 track 오차로 섞일 수 있는가? | Ahem 10px/400, line-height 10px, loaded-font check와 font hash를 고정한다. **PASS** |
| 18 | 사전 비교 입력 자체가 고정됐는가? | HTML fixture hash를 README에 기록하고 기대 JSON은 같은 fixture case ID를 사용한다. **PASS** |
| 19 | 외부 글꼴이 내려받기 실패해 대체 폰트로 측정될 수 있는가? | 저장소 Ahem font를 상대 경로로 로드하고 `document.fonts.check` 관측을 보존한다. **PASS** |
| 20 | Chrome 관측 환경을 재현할 수 있는가? | HeadlessChrome 156, macOS arm64, viewport와 DPR를 기록한다. **PASS** |
| 21 | 화면 이미지를 glyph raster equality로 과장하는가? | README가 검은 Ahem 기하 블록은 glyph paint 검증이 아니라고 제한한다. **PASS** |
| 22 | observation JSON이 예상값에 맞는지만 확인하고 실제 CSS 관측을 누락하는가? | 검증기는 computed serialization, geometry, Range line origin/advance와 기대값을 대조한다. **PASS** |
| 23 | floating-point 판정 허용오차가 숨겨졌는가? | verifier가 CSS px tolerance `1/64`를 명시하고 20개 mismatch count를 출력한다. **PASS** |
| 24 | Parley 지원 가정이 잠금 버전과 다른가? | 현재 Cargo.lock의 Parley 0.12.0 source에서 `each_line`·`hanging` 및 break semantics를 확인했다. **PASS** |
| 25 | intrinsic 계산이 indent를 두 번 더할 수 있는가? | 계획은 `LineMetrics.indent`와 content `advance`를 분리하고 중복 합산을 금지한다. **PASS** |
| 26 | 기존 signed `text-indent` slice가 regression gate에서 빠졌는가? | 구현 계획은 기존 0/5/10px 및 -5/-10px 행렬을 별도 regression으로 유지한다. **PASS** |
| 27 | 미등록 flags·stale revision에서 부분 scene을 게시할 수 있는가? | 계획은 mismatch/stale/mutated input을 layout publication 전 거부하고 no-frame 경계를 요구한다. **PASS** |
| 28 | 기본 제품 artifact에 fixture가 노출될 수 있는가? | 기본/opt-in archive 경계는 구현 gate로 분리하고 현재 변경에 FFI symbol을 추가하지 않는다. **PASS** |
| 29 | 계획과 공개 문서 사이 링크가 깨졌거나 상태 표현이 서로 다른가? | C11.4 tracker, C11 overview, STATUS, public roadmap에 같은 계획 단계와 완료 경계를 기록했다. **PASS** |
| 30 | 다른 작업의 untracked evidence가 PR에 섞였는가? | 최종 stage 목록은 명시 경로 13개(이 검토 기록 포함)로 제한했고 unrelated untracked paths는 staged diff 밖이다. **PASS** |

**결과:** 이 계획 PR diff의 서로 다른 30개 실패 관점을 확인했다. 이 결과는 계획 내용의 20개 검토를 대신하지 않으며, 실제 구현 정확성이나 Android/iOS runtime 통과를 뜻하지 않는다.
