# C11.4 depth-three 병합 기록 문서 PR 실패 관점 검토 30개

이 문서 PR은 구현 PR #458의 병합 사실을 계획과 공식 상태 대장에 연결한다. 기능 범위를 다시 검증하거나 C11.4를 완료로 승격하지 않는다.

| # | 독립 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | 구현 PR 번호가 계획 PR #457과 뒤바뀌는가? | 계획 PR #457과 구현 PR #458 링크를 별도로 표기한다. 통과. |
| 2 | 구현 PR의 병합일시가 로컬 시간과 UTC로 혼동되는가? | GitHub `mergedAt` `2026-10-21T16:44:41Z`를 UTC로 기록했다. 통과. |
| 3 | merge SHA가 head commit SHA로 잘못 기록되는가? | PR metadata의 mergeCommit `4df8d09cb2512d5198733e2eda578ba9c8936230`을 사용하고 PR head `b95e5440…`와 구분했다. 통과. |
| 4 | 병합 방식을 잘못 주장하는가? | GitHub PR을 `Rebase and merge`로 병합한 기록과 일치한다. 통과. |
| 5 | merge state `UNKNOWN`을 병합 실패로 기록하는가? | 병합 여부는 state `MERGED`, mergedAt 및 mergeCommit으로 확인했다. 통과. |
| 6 | check rollup이 빈 것을 green CI로 표현하는가? | check rollup이 비었다고 쓰고 checks 통과라고 표현하지 않는다. 통과. |
| 7 | main required checks가 없다는 문장이 확인 자료와 일치하는가? | GitHub branch-protection API가 `Required status checks not enabled` 404를 반환했다. 통과. |
| 8 | 상태 대장 문구가 새로운 기능 PR을 pending으로 남기는가? | #458 merge 정보를 추가해 실제 상태와 일치시켰다. 통과. |
| 9 | 문서 정리 때문에 C11.4 상위 checkbox가 완료되는가? | 상위 checkbox는 그대로 `[ ]`이고 추가로 체크 표시하지 않았다. 통과. |
| 10 | 공식 완료 수가 구현 slice 때문에 오르는가? | `72/213` 수치를 유지하고 문서에도 반복 명시한다. 통과. |
| 11 | Grid 고정 fixture가 일반 제품 지원으로 오인되는가? | depth-three fixed fixture-test 범위로 표기하고 공개 API/profile 확장을 부인한다. 통과. |
| 12 | 전체 CSS Grid 동작을 확인한 것으로 과장하는가? | 전체 C11.4, WPT와 arbitrary nesting이 미완료라고 유지한다. 통과. |
| 13 | 내부 계약 숫자 버전이 무심코 바뀌는가? | 0059 `0.1.0` 고정을 그대로 기록했다. 통과. |
| 14 | plan의 구현 완료 문구가 PR 병합 사실을 누락하는가? | 구현 PR 번호, UTC 병합 시각과 merge SHA를 함께 기록한다. 통과. |
| 15 | min-content 치수가 spec 예상값과 다르게 바뀌는가? | plan·runtime 근거의 `70×20px`, Columns `[70]`, Rows `[20]`을 일치 확인했다. 통과. |
| 16 | max-content 치수가 spec 예상값과 다르게 바뀌는가? | plan·runtime 근거의 `100×10px`, Columns `[100]`, Rows `[10]`을 일치 확인했다. 통과. |
| 17 | Grid nesting 역할을 잘못 나열하는가? | root → middle → branch → inner와 direct text leaf 범위를 일치시켰다. 통과. |
| 18 | Android 장치를 AVD가 아니라 실기기로 표기하는가? | Android API 37 ARM64 AVD라고 표기하며 실기기는 미검증으로 남긴다. 통과. |
| 19 | iOS 결과를 실기기/컴포지터 결과로 확대하는가? | iPhone 17 Pro Simulator / iOS 26.2로 한정하고 compositor 이후 pixel은 미검증이라고 쓴다. 통과. |
| 20 | controls 포함 scene 수가 두 새 scene 수와 섞이는가? | 플랫폼당 10/10은 기존 8 controls와 신규 2 scene 전체이며 신규는 각각 2개라고 구분한다. 통과. |
| 21 | `draw/present` 성공을 GPU pixel parity로 읽게 하는가? | draw/present gate만 확인했고 glyph paint와 표면 pixel은 미검증으로 명시한다. 통과. |
| 22 | 초기 stale APK 실패가 최종 통과 수에 섞이는가? | 재빌드 전 실패를 분리하고 최신 fixture APK 재실행만 10/10 근거로 인정한다. 통과. |
| 23 | source revision manifest를 제품 release provenance로 부풀리는가? | manifest는 fixture 실행의 commit/tree/diff hash 근거로만 설명한다. 통과. |
| 24 | 상대 링크가 저장소/preview route에서 깨지는가? | plan·STATUS 링크 대상이 현재 main의 tracked 파일로 존재하는지 확인했다. 통과. |
| 25 | docs route가 depth-two plan으로 잘못 연결되는가? | 새 depth-three plan을 가리키고 기존 depth-two 문서 링크는 해당 역사 설명에만 남긴다. 통과. |
| 26 | parent roadmap과 집중 plan이 서로 다른 simulator 결과를 주장하는가? | 두 문서는 Android/iOS 각각 10/10과 동일 기대 track/frame을 사용한다. 통과. |
| 27 | 미검증 한계가 docs PR에서 사라지는가? | Android hardware GPU/실기기, WPT, arbitrary nesting, glyph 및 iOS compositor pixel 제한이 유지된다. 통과. |
| 28 | Tailnet/Pages가 이미 배포됐다고 과장하는가? | 이 PR 문서 변경은 preview 동기화를 주장하지 않는다. Tailnet 동기화는 main 병합 후 별도 수행한다. 통과. |
| 29 | 단순 문서 반영을 새 API/계약 변경으로 분류하는가? | 변경은 merge metadata와 roadmap link뿐이며 interface code/version은 변경하지 않는다. 통과. |
| 30 | 문서 근거를 실제 runtime 구현 근거와 분리하지 않는가? | merge record는 기존 implementation 20·PR diff 30·Simulator runtime evidence로 연결한다. 통과. |

## 판정

문서 diff에서 상태·수치·병합 metadata 간 불일치를 찾지 못했다. 링크 대상과 C11.4 checkbox/count를 다시 확인했다. Tailnet 미리보기는 이 문서 PR 병합 후 main 기준으로 별도 동기화한다.
