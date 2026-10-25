# C11.4 다음 독립 slice 계획 PR diff 검토 · 2026-10-25

**검토 범위:** 계획 후보 두 개, Chrome 사전 비교 evidence, parent plan, `spec/STATUS.md`, 공개 roadmap HTML의 staged diff. 이는 계획 문서 PR 검토이며 Spinon runtime이나 Simulator 구현을 승인·검증하지 않는다.

## 독립 실패 관점 20개

| # | 판정 | 실패 관점과 확인 결과 |
|---:|:---:|---|
| 1 | PASS | **공식 완료 수를 잘못 올리는가?** 새 기록은 모두 계획 후보로 부르고 `72/213`을 유지한다. `spec/STATUS.md`의 C11.4 체크박스도 미완료다. (`spec/STATUS.md`, 두 plan) |
| 2 | PASS | **계획을 구현 상태로 오인할 수 있는가?** Status와 public roadmap은 두 후보의 runtime/Simulator 결과가 없다고 명시한다. 계획 문서도 Chrome 관측과 제품 구현을 분리한다. |
| 3 | PASS | **상위 계획이 이미 완료된 범위를 다시 계획하는가?** Parent note는 #611/#620의 direct text/image 열 및 수평 행 결과를 요약하고, #620의 20px/20px가 min/max algorithm을 분리하지 못한다는 구체적 간극을 제시한다. (`plan/c11-4-grid-track-sizing.md`) |
| 4 | PASS | **Cyclic 계산이 규범 절차와 맞는가?** 계획은 `max-content` intrinsic pass를 indefinite로 보고 한 번 repeat한 뒤, 24px definite 최종 pass에서 두 번 repeat하는 단계를 CSS Grid 2025 CRD 절에 연결한다. |
| 5 | PASS | **24px intrinsic 폭이 브라우저 값에서 역산됐는가?** 독립 산식은 고정 24px item, cyclic 50% maximum의 intrinsic `auto` 처리, intrinsic growth limit과 max-content maximization을 설명한다. Chrome 최종 CSSOM은 별도 관측으로 표시된다. |
| 6 | PASS | **Chrome final CSSOM으로 첫 pass count를 과장하는가?** Evidence는 `[12,12]` 최종 CSSOM이 intrinsic 1회 repeat를 증명하지 않는다고 명시하고, 이 차이를 runtime trace gate에 남긴다. |
| 7 | PASS | **최종 percentage basis를 잘못 잡는가?** 계획은 0 padding/border와 24px content box를 명시하고 `[12,12]` 기대를 이 basis로 계산한다. Definite 40px control은 `[20,20]`으로 따로 고정한다. |
| 8 | PASS | **Browser 전용 marker가 runtime child로 들어가는가?** Cyclic README와 plan은 절대 배치 `.boundary`가 screenshot용이고 in-flow sizing item이나 runtime node가 아님을 밝힌다. Browser/runtime ID mapping도 기록한다. |
| 9 | PASS | **Cyclic trace가 없는 상태를 성공으로 받아들이는가?** 계획은 semantic phase, repeat count, percentage basis가 기존 trace에 없음을 명시한다. Test-only callsite 계측으로도 이를 실제 사용값에서 보존할 수 없으면 첫 feasibility gate에서 중단한다. |
| 10 | PASS | **오류가 잘못된 계층에서 처리되는가?** Cyclic evidence는 scene selector, runtime input validator, FFI report, test-only phase gate의 8개 negative control과 실패 시 layout/report/presentation 경계를 owner별로 구분한다. |
| 11 | PASS | **Orthogonal row의 논리축을 물리축과 혼동하는가?** 계획과 README는 horizontal-tb 부모 block axis와 vertical-rl 자식 inline axis가 물리 세로축으로 대응한다고 CSS Writing Modes 3에 연결한다. |
| 12 | PASS | **Orthogonal 기대값이 text/image 기여를 더하거나 뒤집는가?** Ahem 입력의 min/max text contribution `50/80px`, natural image block contribution `16px`, span-1 최댓값 계산이 각각 적혀 있고 row `[50]`/`[80]`, root 74×50/80px를 도출한다. |
| 13 | PASS | **Browser geometry의 computed-style 조건이 계획과 다른가?** JSON 네 scene에 root/text/image writing-mode와 direction이 있으며, text typography·whitespace·wrap 속성, alignment, columns, rows, frames가 README·계획 입력과 대응한다. |
| 14 | PASS | **계획 자체의 선행 검토가 빠졌거나 숫자를 재사용했는가?** 두 precomparison 디렉터리에 각기 별도 `plan-review-20.md`가 있고 최초 20개 관점과 수정 후 후속 검토를 구분한다. 구현/runtime 검토는 아직 수행하지 않았다고 명시한다. |
| 15 | PASS | **계획·evidence 링크가 올바른 대상을 가리키는가?** 로컬 markdown 파일 간 참조는 존재한다. Status와 plan review의 새 HTML 링크는 line anchor를 포함한 절대 `/spinon/...html#L...` route로 정리했고, 규범 출처는 W3C primary source URL을 직접 연결한다. |
| 16 | PARTIAL | **공개 roadmap의 새 URL이 실제 문서 내용으로 열리는가?** 이전 Agent Browser 확인은 HTTP 200만 보았지만 이는 local Rspress app shell 응답이었다. 엄격한 navigation에서 새 orthogonal plan route는 실제 404 page로 확인됐다. 새 plan/evidence route가 Tailnet에 배포된 뒤 Agent Browser에서 문서 본문과 여섯 링크를 다시 확인해야 닫힌다. |
| 17 | PASS | **사전 비교 파일이 변조되거나 JSON이 깨졌는가?** Orthogonal README에 적힌 HTML/geometry JSON/PNG SHA-256을 다시 계산해 일치했다. Cyclic README의 다섯 해시도 각 파일과 일치하며 두 디렉터리의 JSON 파일은 모두 parse된다. |
| 18 | PASS | **Agent Browser 프로세스 정리가 증거 없이 선언됐는가?** 각 evidence 디렉터리에 별도 lifecycle JSON이 있고 close/list/PID/profile 정리 결과를 보존한다. 현재 이 검토는 브라우저를 실행하지 않았다. |
| 19 | PASS | **시뮬레이터 또는 실기기 검증을 한 것처럼 보이는가?** 두 계획 모두 이후 Android API 37 ARM64 AVD와 iOS 26.2 Simulator를 acceptance로 요구하지만, 현재는 실행되지 않았다고 분리한다. 실기기·hardware GPU·전체 pixel equality를 성공으로 확대하지 않는다. |
| 20 | PASS | **병렬 작업이 공용 파일을 동시에 편집하게 되어 있는가?** 두 계획은 feature-specific module을 나누고, cyclic이 shared FFI dispatcher/trace report/Android·iOS adapter를 먼저 통합한 뒤 orthogonal이 그 base에 rebase하는 순서를 동일하게 적었다. 기존 #611/#620 회귀도 통합 gate로 남긴다. |

## 판정

**19 PASS / 1 PARTIAL / 0 FAIL.** 문서상 scope·산식·공식 상태·병렬 소유 순서는 서로 맞고, markdown 및 절대 HTML anchor/source 링크도 수정됐다. Public roadmap의 새 route는 Tailnet 배포와 Agent Browser 본문 검증이 남아 있다. 이 PR은 계획 승인 범위에 한정하며, runtime feasibility와 Android/iOS Simulator 검증을 대신하지 않는다.
