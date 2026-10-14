# C13 계획 PR #157 · 변경 검토 30개 관점

검토 대상은 [`PR #157`](https://github.com/ohah/spinon/pull/157)의 `main` 대비 변경과 기준 커밋 `c2796e6`이다. 계획 문서의 20개 실패 관점 검토와 별개의 PR 변경 무결성 검토다. 이 기록은 기능 구현 검토를 대신하지 않는다.

| # | 독립 PR 변경 실패 관점 | 확인 결과와 조치 |
| ---: | --- | --- |
| 1 | PR이 현재 사용자가 편집 중인 큰 작업 트리의 변경을 함께 담아 의도치 않은 파일을 올린다. | 변경은 `/tmp/spinon-c13-overflow-scroll-plan` 격리 worktree에서 만들었다. PR commit의 9개 파일은 C13 계획·상태·비교 근거에 한정된다. 원본 dirty worktree에는 쓰지 않았다. |
| 2 | 새 branch가 main이 아닌 열린 feature PR 위에서 시작하거나 merge queue에 잘못된 순서를 만든다. | branch base는 당시 `origin/main` `c2796e6`, PR #157 base는 `main`이다. 열린 #155/#156 위에 쌓이지 않는다. |
| 3 | main에 직접 push하거나 계획 변경이 제품 기능으로 섞인다. | commit `4d5ee84`는 `docs/c13-overflow-scroll-clipping-plan` branch에 push했고 PR 경로로 제출했다. 코드 변경은 포함하지 않는다. |
| 4 | 파일 변경량에 우발적 삭제, 생성물 또는 사용자 산출물이 섞인다. | `git show --stat`과 변경 파일 목록은 계획 1개, 공식/내부 인덱스 2개, 증거 4개, 계획 검토 1개, 비교 fixture 1개로 한정된다. 삭제 파일은 없다. |
| 5 | C13 상태가 준비 문서만으로 완료 체크된다. | `spec/STATUS.md`의 C13은 unchecked이며 runtime/API/Simulator 미실행을 명시한다. |
| 6 | C12.5 sticky를 C13 기반 없이 이미 가능하다고 표현한다. | 상태 대장의 sticky 선행 조건과 계획 서두 모두 C13 scrollport/offset/clip 의존성을 유지한다. |
| 7 | PR 설명에서 Chrome oracle를 Spinon 동작 증거로 오인한다. | PR 본문·상태표·근거 README가 자료를 사전 비교라고 분류하며 제품 실행 근거가 없음을 밝힌다. |
| 8 | 공개 CSSOM API나 S05 event 계약이 이 계획 PR에서 구현 완료로 보인다. | PR 본문은 API 미구현을 선언한다. 계획은 public scroll API와 event 공개 동작을 C29/S05 소유로 둔다. |
| 9 | 내부 README가 C13 계획을 구현된 ABI 계약 목록에 잘못 끼워 넣는다. | 인덱스 추가 문구는 이를 구현 전 계획/oracle 기준으로 명시하고 C13 runtime·내부 계약 미구현을 함께 적는다. |
| 10 | 계획에서 요구하는 20개 검토 기록이 링크되지 않거나 PR/구현 검토에 재사용된다. | 별도 `c13-overflow-scroll-clipping-plan-review` 파일을 만들고 PR 변경 검토·향후 구현 검토와 분리했다. |
| 11 | 계획 검토 20개가 독립된 실패 경로 대신 중복 표현으로 채워진다. | 20행은 cascade, BFC, clip/hit, root owner, revisions, gesture, overscroll, scrollbar, snap, artifact 경계로 나뉜다. 표 자체 재검토도 기록했다. |
| 12 | repository의 PR 필수 변경 검토 30회를 계획 검토 20회로 대체한다. | 계획에 모든 PR의 30개 변경 검토와 각 기능 구현의 신규 20개 코드·실행 검토를 별도 관문으로 적었다. 이 문서가 PR #157의 30개 기록이다. |
| 13 | C13 단계들이 앞뒤 의존성이 없는 한 PR에 혼합되거나 순서가 역전된다. | 13.1 clip, 13.2 scroll state, 13.3 input, 13.4 scrollbar, 13.5 snap, 13.6 C17 logical axes로 분리하고 선행 C17 의존성을 고정했다. |
| 14 | 범위가 C22/C23/C14/C15/C27/C29/S05의 책임까지 조용히 흡수한다. | 계획은 radius/transform, replaced/text overflow, animation, CSSOM/events를 후속 교차 단계·owner 경계로 분리한다. |
| 15 | W3C 초안·CR의 시점을 숨겨 움직이는 규범 문서를 고정 계약처럼 인용한다. | 문서명과 날짜를 계획 표에 적고 snapshot/WD의 상호운용성 한계를 밝힌다. |
| 16 | 제안된 browser oracle이 재실행 불가능하거나 브라우저 patch build를 모른다. | baseline README에 Chrome `155.0.8059.40`, revision, UA 형태, macOS, viewport, DPR, agent-browser 버전을 기록한다. |
| 17 | 같은 이름의 다른 Chrome 바이너리 결과를 기준으로 오인한다. | Chrome 실행 파일 SHA-256을 baseline README에 기록했다. 이 누락은 최초 초안 검토에서 확인해 커밋 전에 보완했다. |
| 18 | 비교 fixture가 수정되지만 observation은 이전 source를 가리킨다. | fixture SHA-256은 README에 적혀 있고 재계산값 `d08ee25b…f147cd8`과 일치한다. |
| 19 | screenshot이 다른 viewport/DPR에서 생성됐거나 파일 손상 후 그대로 사용된다. | 캡처 조건은 `1280×633`, DPR 1로 기록됐고 screenshot 재계산 SHA-256 `25e1bc32…37019f6`이 문서값과 일치한다. |
| 20 | 숨겨진 scrollbar가 보이는 캡처로 설명되어 scrollbar 모양 증거가 부풀려진다. | README는 `--hide-scrollbars`를 명시하고 PNG를 scrollbar 시각 근거로 사용하지 않는다고 기록한다. |
| 21 | raw observation 없이 수치 결과를 서술해 요약을 독립적으로 재확인할 수 없다. | 원본 DOM/CSSOM observation JSON과 agent-browser eval output을 각각 보존하고 README에서 연결한다. |
| 22 | fixture가 지원되지 않는 효과를 관찰했는데 fixture 범위보다 넓게 설명한다. | baseline은 overflow values, 두 mixed-axis 사례, clip margin/radius, float/BFC만 담는다. gesture/snap/root owner/scrollbar 외관은 포함했다고 주장하지 않는다. |
| 23 | 사전 비교 fixture가 제품 runtime 또는 공식 WPT 적합성 근거로 보인다. | plan, STATUS, PR body, precomparison README 모두 oracle-only 경계를 유지한다. WPT 미실행도 남은 범위에 기록한다. |
| 24 | float 관찰을 box height와 연결했지만 `flow-root` 대조가 없어 BFC 원인을 구별할 수 없다. | fixture는 hidden, clip, clip+flow-root를 별도 구성해 비교하고 README에 결과 해석을 제한해 설명한다. |
| 25 | 새 문서 링크가 상대경로 오류로 evidence를 열 수 없다. | 세 신규 C13 Markdown 문서의 상대 링크를 로컬에서 확인했으며 대상이 모두 존재한다. |
| 26 | 상태 대장이나 내부 인덱스에서 계획 위치를 찾을 수 없다. | 공식 C13 행과 `spec/internal/README.md`에 계획·검토·사전 관찰 링크를 연결했다. |
| 27 | evidence를 내부 문서에 추가하면서 `/roadmap.svg` 또는 기존 근거 경로를 잘못 변환한다. | 기존 STATUS의 절대 site route와 과거 증거 링크는 변경하지 않았다. 전체 STATUS link scan에서 보인 기존 비저장소 target들은 이번 PR 범위가 아니며 새 C13 경로 검사는 별도로 통과했다. |
| 28 | fixture를 `tests/fixtures`에 놓은 것만으로 제품 artifact 격리가 증명됐다고 주장한다. | 계획은 경로가 아닌 default/product artifact와 opt-in `*-fixture-test` artifact의 부재/존재 control을 향후 구현 gate로 요구한다. 이 문서는 artifact build를 실행했다고 주장하지 않는다. |
| 29 | 시뮬레이터 계획이 실기기 연결을 요구하거나 simulator 결과를 실기기 결과로 확대한다. | Android API 37 ARM64 AVD와 iOS Simulator만 계획하며 physical hardware 동등성 주장을 금지한다. 이번 PR에서는 앱 시뮬레이터 실행도 하지 않았다. |
| 30 | 문서 전용 PR에 제품 테스트를 실행한 것처럼 보고하거나 체크가 없는데 merge-ready라고 결론낸다. | `git diff --check`, C13 신규 링크 검사, fixture/screenshot hash 대조만 실행했다. PR #157은 OPEN/CLEAN이고 `gh pr checks`는 현재 체크 없음으로 응답했으므로 병합하지 않는다. |

검토에서 발견한 단일 누락은 Chrome 실행 파일 SHA였다. 첫 문서 검토에서 발견해 초기 커밋 `4d5ee84`에 반영했다. PR #157에서 확인한 최종 diff에는 별도 차단 결함이 없다. 필요한 CI 결과가 없는 상태이므로 merge 허용 판정은 하지 않는다.
