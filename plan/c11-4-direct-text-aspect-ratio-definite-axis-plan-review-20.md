# C11.4 direct-text definite-axis ratio 계획 실패 관점 검토 20개

**대상:** [definite-axis 계획](./c11-4-direct-text-aspect-ratio-definite-axis.md)과 [Agent Browser Chrome 156 사전 비교](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-axis-transfer-precomparison-2026-10-31/README.md)

**기준:** `main` / `origin/main` `535cc87a6cd846634be99692d8493ce24b5a5184`의 변경 전 코드와 evidence.

**판정:** 계획 입력·실패 경계 20/20 PASS. 이는 계획 리뷰이며 구현, 플랫폼 runtime 또는 C11.4 완료를 입증하지 않는다.

| # | 독립 실패 관점 | 판정 |
|---:|---|---|
| 1 | definite `width`의 60px이 ratio 없이 30px row를 만드는 것으로 오인 | PASS — ratio/no-ratio width-control이 같은 definite width에서 auto row 10px과 30px을 분리한다. |
| 2 | definite `height`에서 ratio 방향을 height/width로 적용 | PASS — 계획은 width/height 2/1과 `30 × 2 = 60`을 고정한다. |
| 3 | track size와 item border box를 같은 관측값으로 취급 | PASS — columns/rows, item frame, root frame과 text Range를 별개로 검사한다. |
| 4 | Root의 100px width가 tracks를 100px로 강제한다고 가정 | PASS — 두 max-content tracks의 합과 고정 root width를 따로 기록해 overflow 관찰이 가능하다. |
| 5 | `max-content` row fixture 입력을 실제와 다르게 기술 | PASS — 현재 precomparison fixture의 `grid-template-rows:auto`와 CSSOM `30px` used track을 기록한다. |
| 6 | `width:60px` control에서 item의 computed height를 track height와 동일하게 해석 | PASS — item stretch 및 text line box를 분리하고 control item 60×10을 고정한다. |
| 7 | `height:30px` control에서 max-content column이 line-height 또는 row height가 됨 | PASS — 40px text contribution과 30px item height를 분리한다. |
| 8 | Ahem font load 실패를 ratio transfer 결과로 오인 | PASS — fixture는 `document.fonts.ready/check`를 기다리고 저장소 Ahem을 지정한다. |
| 9 | Screenshot 크기·DPR가 캡처 환경과 다름 | PASS — JSON에 viewport 1280×633, DPR 1이 있고 저장 화면은 1280×633으로 확인됐다. |
| 10 | 사전 입력 fixture 수정 후 해시가 오래된 값을 가리킴 | PASS — README와 expected-values source에는 현 fixture의 SHA-256과 3,499 bytes를 기록했다. |
| 11 | Observation screenshot/JSON이 서로 다른 실행을 가리킴 | PASS — README가 고정 JSON·PNG hash를 기록하고 각 scene 수치가 JSON과 일치한다. |
| 12 | Ratio value를 scene ID나 CSS source label에서 추정 | PASS — typed finite positive ratio와 computed serialization `2 / 1`의 동시 일치를 요구한다. |
| 13 | No-ratio `auto`를 degenerate ratio `0 / 1`과 혼동 | PASS — no-ratio는 typed `None` 및 exact computed `auto`이며 degenerate values는 거부한다. |
| 14 | Ratio를 short `A` 또는 여러 item에 잘못 귀속 | PASS — 유일한 positive ratio owner는 direct `AAAA`; wrong/multiple owners가 negative controls다. |
| 15 | Two dimensions both definite인 입력을 transfer 사례로 잘못 수락 | PASS — `(60px,30px)`와 후보에 없는 definite values를 제외하고 거부하도록 규정한다. |
| 16 | 일반 ratio fixture profile에 definite dimension 예외가 퍼짐 | PASS — 새 submodule/dispatch profile을 기존 auto-dimension ratio 경로와 분리한다. |
| 17 | Shared style-to-layout 또는 공개 CSS admission이 바뀜 | PASS — 기존 computed snapshot과 opt-in fixture route만 사용하고 public API/FFI 변경을 배제한다. |
| 18 | Stale style snapshot 또는 mismatch style이 다른 item geometry로 적용 | PASS — HostDocument revision, computed properties, typed values와 paragraph ownership을 함께 검증한다. |
| 19 | Android/iOS simulator frame 결과를 hardware/compositor 표시로 확대 | PASS — AVD PixelCopy와 iOS 제출 전 Metal sample의 관측 경계를 별도로 쓰도록 명시한다. |
| 20 | 새 slice가 공식 C11.4/C11 checkbox나 `72/213` 완료 수를 올림 | PASS — 계획·리뷰·구현 결과 모두 공식 aggregate와 전체 완료 판정을 유지하도록 제한한다. |

## 수정·후속 확인

검토 중 사전 비교 README에 남아 있던 이전 fixture hash와 `max-content` row 기술을 발견했다. 이를 현재 fixture의 hash/bytes와 `auto` row 입력으로 수정했다. 최종 재검증에서 README, JSON, fixture 입력, Agent Browser observation 및 screenshot이 같은 고정 scene을 가리키는지 확인한다. 이는 문서/evidence 수정이며 코드 구현은 시작하지 않았다.
