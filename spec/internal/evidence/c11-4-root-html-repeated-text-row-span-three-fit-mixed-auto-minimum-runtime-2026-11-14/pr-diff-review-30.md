# C11.4 three-fit row-span mixed minimum PR diff 검토 30개

대상 diff: 제한 Root HTML profile 구현, 내부 계약 0059 candidate 절, Chrome/RuntimeSession evidence, Android/iOS simulator smoke, 계획과 검토 기록. 계획 검토 20개와 구현/runtime 검토 20개를 대신하지 않는다.

각 항목에서 변경된 Rust 경로와 기록된 oracle·runtime 근거를 서로 다른 실패 관점으로 대조했다. 공식 `78/214`와 C11.4/C11 미완료 표시는 유지한다.

| # | 실패 관점 | diff 확인 및 판정 |
|---:|---|---|
| 1 | bounded three-fit slice를 C11.4 전체 완료로 읽는가 | plan·계약 candidate·evidence는 profile 한정이고 상태 수를 올리지 않는다. 통과. |
| 2 | 공개 CSS/API/FFI admission이 함께 넓어지는가 | 변경은 parsed Root HTML typed projection path에 한정되며 public API/FFI diff는 없다. 통과. |
| 3 | numeric internal contract version이 구현 추가만으로 상승하는가 | 0059는 `0.1.0`을 유지한다. 통과. |
| 4 | 세 fit 외 track count까지 세 fit predicate가 수용하는가 | six-member list에 세 cap과 세 singleton tail의 exact multiset을 요구한다. 통과. |
| 5 | 중복/missing cap 또는 tail이 양성 predicate를 통과하는가 | 중복 sentinel과 모든 tail/cap type을 확인하고 720 positive/negative unit cases를 기록한다. 통과. |
| 6 | 허용하지 않은 cap type/value가 projection 이후 정규화되어 통과하는가 | direct `0px`, `10px`, `20%`만 positive; `1px`, `30%`, calc와 duplicate value는 거부한다. 통과. |
| 7 | fit position별 값을 합쳐 authorship 순서를 잃는가 | 20 position set·6 cap order·6 tail order를 Chrome metadata와 unit tests에서 각각 보존한다. 통과. |
| 8 | percentage cap이 border box 기준으로 계산되는가 | `box-sizing:border-box`, `42×222` border box, `40×220` content box와 220px basis가 source·oracle·scene test에서 일치한다. 통과. |
| 9 | explicit row가 implicit row 수에 포함되거나 control row가 auto가 되는가 | 10px explicit control과 정확히 6 implicit rows를 분리하고 control `min-height:0`을 고정한다. 통과. |
| 10 | item mask bit가 semantic item 대신 DOM order를 가리키는가 | A–E/A–D mask와 forward/reverse topology pair를 ID별 geometry로 대조한다. 통과. |
| 11 | topology별 mask 일부를 빼고 전체 mask 지원처럼 보고하는가 | span2 topology 32개, mixed span2/span3 topology 16개를 sample과 metadata에서 확인한다. 통과. |
| 12 | `auto` minimum이 모든 spanning item에 무조건 content floor를 주는가 | computed overflow·covered auto-min track·flex track 조건을 기여 trace에 연결하고 eligible item만 payload floor를 확인한다. 통과. |
| 13 | control/explicit item 기여분이 spanning auto-minimum trace를 오염시키는가 | control은 mask 밖, min contribution source는 0으로 검사하며 invalid control `auto`는 거부한다. 통과. |
| 14 | unsupported span/order/extra owner가 제한 profile 안에 들어오는가 | 정확한 기존 topology·sparse row flow·owner 수·placement만 허용하고 dense 및 변형을 atomic reject한다. 통과. |
| 15 | grid box dimension, gap, overflow, direction 경계가 넓어지는가 | 고정 axis·zero edge/gap·LTR·visible overflow 조건을 검증하고 unsupported style mutation을 거부한다. 통과. |
| 16 | auto minimum 적용/미적용 endpoint가 새 code 때문에 바뀌는가 | 기존 three-fit uniform 11,520-scene RuntimeSession regression을 별도 실행했다. 통과. |
| 17 | 새 mixed sample이 전체 Chrome oracle와 같은 분모로 설명되는가 | Chrome oracle은 138,240, RuntimeSession mixed sample은 6,912라고 각각 명시한다. full runtime으로 확대하지 않는다. 통과. |
| 18 | sample에서 track-list 차원 일부가 빠지는가 | 선택한 36개 list가 fit position 20개와 cap/tail 여섯 순서를 모두 포함하고 mask/topology/alignment 조합을 검증한다. 통과. |
| 19 | overflow 관측을 항상 0으로 clamp하거나 control로 숨기는가 | Chrome matrix에서 overflow scene을 분리 집계하고 대표 D payload의 10px 외부 paint를 assertion 및 캡처로 확인한다. 통과. |
| 20 | row sizing replay가 Taffy trace와 안 맞는데 근사 결과를 게시하는가 | per-span 1/64px replay는 단계별 trace 및 final track과 대조하고 mixed three-fit 실패는 fail-closed한다. 통과. |
| 21 | invalid revision 뒤 stale completed layout이 재사용되는가 | bad cap·control·item·dense placement에서 completed output 부재를 검사한다. 통과. |
| 22 | rejection 후 RuntimeSession을 재생성해야만 회복되는가 | 동일 session의 다음 valid revision이 Ready가 되는 회복 test가 있다. 통과. |
| 23 | red border를 단순 style snapshot에서만 확인하고 render에서 빠뜨리는가 | feature-gated RuntimeRenderSnapshot 네 변 paint assertion과 Android/iOS screenshot을 함께 둔다. 통과. |
| 24 | simulator fixture gate가 production fixture payload나 일반 border support로 승격되는가 | runtime render gate는 전용 fixture-test feature 아래 exact test style에만 활성이다. 통과. |
| 25 | Android/iOS 화면을 실제 Root HTML route 밖에서 만든 이미지로 대체하는가 | 두 앱 로그가 fixture source hash, layout-ready, draw/present를 기록하고 동일 Root HTML fixture를 썼다. 통과. |
| 26 | simulator 화면 smoke를 surface pixel parity/실기기로 확대 주장하는가 | README와 PR 범위는 visual smoke라고 한정하고 PixelCopy, Metal readback, hardware/real device를 제외한다. 통과. |
| 27 | simulator 실행이 task-owned device/process를 남기는가 | 임시 iOS 기기는 삭제, read-only Android AVD는 중지, 앱/빌드 프로세스는 최종 확인 때 없었다. 통과. |
| 28 | 임시 HTML·앱 산출물 복원이 재현 가능한가 | Android/iOS asset의 동일 원본 SHA-256과 기존 APK/iOS executable 복원 기록을 보존했다. 통과. |
| 29 | 정적 검사 또는 Clippy 제한 결과를 숨기는가 | format/diff/scoped Clippy pass logs가 있고 runtime all-targets 기존 경고 6건은 별도 실패 로그와 README에 공개된다. 통과. |
| 30 | 새 evidence/계획 링크, screenshot, status가 PR diff에서 누락되거나 stale한가 | plan, 0059, screenshots, hashes, test logs, implementation review와 이 diff review의 경로를 교차 확인한다. 공식 `78/214` 유지와 C11.4 미완료를 기록한다. 통과. |

결론: 검토 범위에서 PR diff에 추가 수정이 필요한 결함은 발견하지 않았다. Six-fit RuntimeSession은 138,240 전체가 아닌 metadata/rejection 및 첫 11,520-scene chunk만 실행했다. Required repository checks와 GitHub CI 상태는 별도로 확인한다.
