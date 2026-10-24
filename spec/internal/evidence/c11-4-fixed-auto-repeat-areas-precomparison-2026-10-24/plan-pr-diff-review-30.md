# 계획 PR 변경분 실패 관점 검토 30개

초기 검토 대상은 계획 PR의 staged diff 11개 파일이었다. 계획 자체 실패 관점 20개와 별도로, 실제 변경 파일·oracle artifact·상태 기록이 배포나 구현 경계에서 잘못 작동하는지 확인했다. 이 검토 기록을 추가한 최종 diff는 12개 파일이다. 12번째 파일은 검토 결과만 기록하고 제품 코드·fixture·oracle 값을 추가하지 않는다. 최종 stage에서 이 파일의 링크·수치·체크 결과 연결도 다시 확인했다.

| # | 변경분 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 다른 작업 PR 또는 원격 `main` 이동이 빠진 상태로 계획을 올림 | 계획 전 `git fetch origin` 뒤 `HEAD`와 `origin/main`은 `a82dbc59`였고 열린 PR·이슈는 없었다. |
| 2 | 기존 928개 untracked evidence가 계획 PR에 섞임 | 지정한 11개 파일만 stage했다. `git add -A`를 하지 않았고 나머지 증거 파일은 그대로 untracked다. |
| 3 | 계획-only 로그가 코드 구현/완료로 보임 | `spec/STATUS.md`는 계획-only와 구현/runtime 미시작을 명시하고 C11.4 unchecked 및 `72/213`을 유지한다. |
| 4 | 9개 fixture case가 expected JSON과 다른 inventory를 가짐 | Validator는 observation ID set과 expected scene key set의 완전 일치를 요구한다. |
| 5 | CSS input이 browser observation 이후 바뀌어도 evidence가 그 사실을 숨김 | Environment JSON에 canonical fixture SHA가 있고 SHA256SUMS가 fixture와 관찰 파일을 검증한다. |
| 6 | observation이 Agent Browser 외 다른 browser 결과처럼 표기됨 | Environment에 Agent Browser 0.31.1, Chrome 156.0.8078.48 Headless, macOS ARM64를 기록했다. |
| 7 | viewport/DPR/direction/writing-mode가 재현되지 않음 | `1280×633`, DPR 1, LTR, horizontal-tb를 Browser API에서 읽어 보존했다. |
| 8 | 화면 캡처에 9개 case 또는 root overflow가 빠짐 | full-page `#scenes` screenshot에는 columns/rows collision, empty/occupied fit, area larger/smaller case가 모두 보인다. |
| 9 | pre-comparison screenshot을 Spinon app 실행 결과로 혼동함 | README와 screenshot 설명이 Chrome oracle임을 명시하고 simulator/app/WPT 및 pixel proof로 확장하지 않는다. |
| 10 | CSSOM used tracks를 독립 expected list와 비교하지 않음 | Validator는 각 scene의 serialized px values를 expected vector와 정확히 대조한다. |
| 11 | track-list comparison이 fractional rounding 차이를 조용히 허용함 | Used track size list는 정확히 대조하고 frame 좌표만 `1/64 CSS px` 이내를 허용한다. |
| 12 | relative geometry가 페이지의 absolute card offset에 결박됨 | Validator는 root origin을 빼고 각 child의 root-relative x/y와 width/height를 비교한다. |
| 13 | fixture width/height가 expected definite-size와 다름 | 각 root의 해당 축 CSS size를 expected 148/168/188px과 직접 비교한다. |
| 14 | auto-repeat count formula가 구현 observation과 같은 값에서 역산됨 | Formula는 fixed prefix/suffix track lengths와 gap으로 독립 계산하고 3/4/4를 확인한다. 초기 고정 상수 방식은 검토 중 발견해 수정했다. |
| 15 | CSS Grid's named line `hero` aliases and area rectangles collide but expected order is guessed | Expected order is based on CRD §7.3.2/§8.3 and pinned Taffy test; Chrome is separate observation. |
| 16 | explicit and implicit alias on same physical line increments occurrence twice | Validator parses serialized track line groups and asserts distinct line indices; area edge positions are derived separately from the matrix. |
| 17 | positive named occurrence starts at wrong line | `hero-start 3 / hero-end 3` expected box begins at relative 48px on columns or rows. |
| 18 | negative named occurrence counts from the wrong side | `-1/-1` probe expected box begins at relative 120px, the fourth/final repeated track. |
| 19 | area matrix dimension parser only sees named rectangles, not unnamed cells | Validator reads full serialized `grid-template-areas` matrix dimensions including all `.`/named columns and rows. |
| 20 | area-larger template incorrectly rejects added explicit track | Expected used list has ten tracks versus nine expanded-template tracks and includes the `grid-auto-*` 20px addition. |
| 21 | overflow amount is claimed without geometry support | Root-relative item frame ends at 168px for 148px root in both column and row scenes: 20px overflow. README does not claim CSS border pixels. |
| 22 | area-shorter case shrinks the explicit template | `columns-area-shorter` asserts ten used tracks despite an eight-cell area and checks the negative numeric tail placement. |
| 23 | auto-fit area definition alone prevents collapse | Empty-area cases expect four 0px tracks; area declarations alone do not add an occupied in-flow item. |
| 24 | auto-fit collapses occupied repeated tracks | Occupied area cases expect the four repeated tracks at 20px and the area frame at 92px. |
| 25 | row/column adapter axes differ without an oracle comparison | Collision, fit empty/occupied and area-larger test both axes with axis-specific coordinate expectations. |
| 26 | valid plan references resolve to missing paths | Checked the plan, review, fixture, evidence README, contract, source, and vendored Taffy link targets in the checkout. |
| 27 | evidence checksum omits a source needed to reproduce the comparison | SHA256SUMS covers fixture, README, environment, CSSOM observation, expected values, validator and screenshot; every entry passes `shasum -c`. |
| 28 | validator silently accepts stale or missing files/keys | Node asserts nine scene IDs, child inventory, tracks, matrix dimensions, alias indexes, formula, and frames; it prints 9/9 and 30 boxes. |
| 29 | plan adds new runtime dependency, product API, or Taffy patch prematurely | The PR adds a CSS reference fixture and planning evidence only; Rust/vendor source and internal contract are untouched. |
| 30 | browser or temporary server remains running after oracle capture, or Tailnet preview is stopped | Agent Browser close completed; its process and local port 8891 are absent. Existing Tailnet listeners on 8768 and 4174 remain active. |

## 결과

Formula verifier의 고정 상수는 발견 즉시 fixture의 fixed breadth와 gutter에서 유도하도록 수정했고, 재실행 결과는 `PASS: 9 scenes, 30 child boxes; tolerance 0.015625 CSS px`다. `git diff --cached --check`, SHA-256 대조도 통과했다. 최종 12-file diff도 staged 이후 다시 확인한다. 남은 범위는 계획 PR을 병합한 뒤 별도 implementation/runtime PR에서 검증한다.
