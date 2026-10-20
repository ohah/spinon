# C11.4 direct-leaf text multi-span 계획 PR diff 적대 검토 30개

검토 범위는 집중 계획, 상위 C11.4 plan, `spec/STATUS.md`, 내부 evidence 색인, Chrome precomparison fixture/JSON/screenshot, 계획 실패 관점 20개다. 기능 구현은 포함하지 않는다.

| # | PR diff 실패 경로 | 검토 결과 |
| ---: | --- | --- |
| 1 | 계획 PR에 runtime 구현이 섞임 | Rust runtime/layout/FFI 소스는 변경하지 않았다. 통과. |
| 2 | 구현 시작 전에 계획 검토가 빠짐 | 계획 실패 관점 20개를 별도 문서로 추가했다. 통과. |
| 3 | 계획 검토와 PR diff 검토를 합산 | 20 plan review와 30 PR diff review를 별도 파일로 유지했다. 통과. |
| 4 | C11.4를 완료로 표시 | STATUS checkbox와 완료 집계는 그대로며 후보 구현은 시작 전이다. 통과. |
| 5 | Chrome 관찰을 규범 기대값으로 표시 | ideal values와 Chrome used values를 표에서 분리했다. 통과. |
| 6 | CSS 규범 링크가 필요한 절을 가리키지 않음 | 공식 CSS Grid Level 1 §11.5.1 anchor를 사용한다. 통과. |
| 7 | Chrome 버전·OS·DPR 누락 | Chrome 155.0.8059.85, macOS 26.5.1 ARM64, DPR 1과 Agent Browser 0.31.1을 기록했다. 통과. |
| 8 | screenshot이 agent browser가 아닌 terminal-browser에서 생성 | Agent Browser로 Chrome fixture를 열고 캡처했다. 통과. |
| 9 | Agent Browser가 끝난 뒤 계속 실행됨 | 세션을 닫고 실행 프로세스가 사라진 것을 확인했다. 통과. |
| 10 | screenshot이 네 scene 중 일부를 누락 | full-page screenshot과 four scene DOM geometry를 대조했다. 통과. |
| 11 | Ahem 경로가 fixture 디렉터리에서 잘못 해석됨 | 기존 fixture와 같은 상대 resource 경로를 사용하고 font-ready 상태를 확인했다. 통과. |
| 12 | hidden probe가 grid layout을 변경 | intrinsic probe는 fixed-position이고 측정 직후 제거되어 Grid track sizing tree에 참여하지 않는다. 통과. |
| 13 | `devicePixelRatio`를 빠뜨림 | observation JSON과 plan에 DPR 1을 기록했다. 통과. |
| 14 | intrinsic width 값이 browser output과 불일치 | JSON은 min/max-content 50/80, 70/120, 180/180px을 보존한다. 통과. |
| 15 | 같은 span pair에서 text가 다르게 바뀜 | forward/reversed scenes의 content, placement, track list가 동일하다. 통과. |
| 16 | fourth track을 빼고 비교해 out-of-span 변경을 못 잡음 | 네 번째 0px track을 모든 case에 명시한다. 통과. |
| 17 | 0px gap이 used track으로 계산됨 | fixture가 gap 0을 명시하고 CSSOM track list만 비교한다. 통과. |
| 18 | flex sizing이 intrinsic track result에 섞임 | 네 column은 `min-content`이며 flexible sizing function을 쓰지 않는다. 통과. |
| 19 | grid root dimension이 scene마다 달라짐 | 모든 scene은 220×60px root다. 통과. |
| 20 | row auto sizing이 column 기여 비교에 섞임 | 3개 20px row를 고정한다. 통과. |
| 21 | Chrome frame이 scene container offset을 포함 | 계획은 root-relative item frame을 비교하도록 한다. 통과. |
| 22 | fractional thirds가 양자화 차이로 오판정 | Chrome 1/64 CSS px serialization과 runtime numeric tolerance를 적는다. 통과. |
| 23 | fourth track nonzero가 숨겨짐 | raw JSON에 네 used track 전부 저장됐다. 통과. |
| 24 | scene ID가 중복되거나 네 개가 아니게 됨 | 네 고정 ID를 계획·fixture·review에서 대조했다. 통과. |
| 25 | fixture가 nested/anonymous items 지원을 암시 | non-goals와 exact topology guard에서 계속 범위 밖이다. 통과. |
| 26 | 내부 계약 숫자 버전이 불필요하게 변함 | 공개/내부 계약 버전 변경을 포함하지 않는다. 통과. |
| 27 | WPT를 실행한 것으로 잘못 보고 | WPT는 미실행 범위로 분명하게 적었다. 통과. |
| 28 | Android 실기기 검증을 Android AVD와 혼동 | 계획은 physical, API 37 AVD, iOS Simulator를 별도 결과로 요구한다. 통과. |
| 29 | screenshot 링크의 경로 오타 | screenshot 경로는 evidence 디렉터리 내 실제 파일과 일치하며 PR 본문에는 업로드 뒤 commit-pinned URL을 사용한다. 통과. |
| 30 | 새 후보 때문에 official completion count가 바뀜 | STATUS와 internal index가 후보 계획이며 체크·집계 미변경이라고 명시한다. 통과. |

최종 plan diff에는 blocker가 없고, plan PR 병합 후 구현을 시작할 수 있다.
