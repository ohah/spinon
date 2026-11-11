# C11.4 mixed automatic minimum 계획 PR diff · 적대 검토 30개

**범위:** 계획 PR의 `spec/STATUS.md`, C11.4 상위 계획, focused plan/review, Chrome fixture·capture·raw observation·verifier·screenshot 및 이 검토 기록. 제품 구현 코드는 포함하지 않는다.

| # | 별도의 실패 경로·불변 조건 | 확인 결과 |
|---:|---|---|
| 1 | STATUS가 후보 선택을 완료 처리나 78/214 증가로 오표기 | `C11.4/C11` 체크와 `78/214`가 유지되고 제품 코드는 미시작으로 적혀 있다. 통과. |
| 2 | C11.4 상위 계획이 이전 후보 “미선정” 문구를 더 새 선택보다 우선하도록 둠 | 최신 mixed-mask 후보 섹션을 문서 상단에 두고 #1365 이후의 선택 근거를 적었다. 통과. |
| 3 | Plan이 이전 #1365의 uniform 제한과 새 후보를 혼동 | #1365는 all-zero/all-auto control이며 이번 항목은 item별 혼합 상태라고 구분한다. 통과. |
| 4 | §6.6 auto minimum 조건을 §11.5 span contribution과 혼동 | 규범 기준·검증 관문에서 eligibility와 spanning contribution을 분리한다. 통과. |
| 5 | scene count에서 topology factor를 누락하거나 두 번 곱함 | `720 × (32+16) × 2 alignment × 2 DOM = 138,240`로 fixture와 일치한다. 통과. |
| 6 | topology별 mask bit 순서와 DOM reverse가 엇갈림 | bit는 semantic A–E/A–D 순서이고 reverse는 placement 배열만 바꾼다. verifier가 매 scene 배치를 검사한다. 통과. |
| 7 | A topology의 E 또는 B topology의 가짜 E가 mask 계산에 들어감 | fixture metadata와 verifier에 각각 32/16개 mask와 exact child set이 있다. 통과. |
| 8 | mask `0` 또는 full-auto control 빠짐 | 11,520 uniform scenes를 기존 독립 capture와 대조한다. 통과. |
| 9 | cap order 중복·누락 또는 computed order 정렬 | 720 unique permutation set, CSSOM order와 typed cap map을 확인한다. 통과. |
| 10 | `20%` cap basis가 viewport로 변할 수 있음 | fixed definite 220px content basis 및 44px resolution을 명시한다. 통과. |
| 11 | six implicit rows 중 마지막 span이 일곱 번째 implicit row를 만들 수 있음 | 두 topology placement를 exact 검사하고 used row 수를 explicit 1 + implicit 6으로 고정한다. 통과. |
| 12 | `min-height:auto`의 payload floor와 `0px` visible overflow 기대를 한 assertion으로 합침 | auto item floor violation과 zero item floor shortfall을 별도 계산한다. 통과. |
| 13 | mixed auto item이 Grid-end 밖으로 나가도 표의 overflow 0 주장이 검출되지 않음 | verifier가 모든 mask 및 auto/zero item의 Grid-end overflow를 assertion으로 막는다. 통과. |
| 14 | explicit row control에 자동 최소값이 적용됨 | 각 observation에서 control computed minimum과 40×10 frame을 검증한다. 통과. |
| 15 | row/frame test가 CSSOM expected values와 동일 입력만 재사용함 | row 합과 item span frame은 별도 산술로 계산한다. 통과. |
| 16 | source-order/alignment 쌍이 하나 빠짐 | mask마다 start/stretch × forward/reverse 네 조합의 순서를 검증한다. 통과. |
| 17 | line-box/ payload observation이 누락되어 text floor 판단이 무력화 | 각 text item의 payload frame과 비어 있지 않은 line rect를 강제한다. 통과. |
| 18 | screenshot이 observation matrix 이후 다른 fixture/state를 보여 줌 | sample index는 관측된 A/C auto mixed scene을 찾아 같은 browser page에서 표시한다. 통과. |
| 19 | browser screenshot 크기 확대가 수치 capture에 적용됨 | CSS transform은 전체 matrix 기록이 끝난 뒤 sample screenshot에만 적용한다. 통과. |
| 20 | capture script가 138k scene 전부를 무제한 메모리에 보관함 | 각 batch를 gzip stream에 바로 쓰고 count·permutation을 순차 확인한다. 통과. |
| 21 | capture 실패 뒤 session이나 partial output이 방치됨 | `finally`에서 전용 Agent Browser session을 닫고 실패한 task-owned gzip을 제거한다. 통과. |
| 22 | metadata fixture hash가 실제 oracle 입력과 다름 | capture가 fixture bytes를 hash하고 verifier가 현재 fixture와 비교한다. 통과. |
| 23 | 이전 uniform control corpus의 파일 교체를 조용히 수용 | 이전 capture scene 수와 control key 11,520개를 각각 검증한다. 통과. |
| 24 | verifier에서 forward/reverse placement를 이름만 비교하고 payload ID를 틀리게 연결 | `PLACEMENTS`와 scene order를 매 record 비교하고 item별 geometry를 ID로 정렬한다. 통과. |
| 25 | verifier의 line rect count/value가 source order와 alignment 쌍에서 변함 | geometry error는 line rect 개수와 각 좌표를 허용 오차 안에서 비교한다. 통과. |
| 26 | 계획 PR 화면 링크가 GitHub에서 깨지거나 인증이 필요한 경로를 사용 | PR #1368 본문의 commit-pinned raw PNG를 Agent Browser로 열었다. 이미지 `complete=true`, `naturalWidth=1280`, `naturalHeight=900`이고 GitHub 대화 화면에 표시됨을 확인했다. 통과. |
| 27 | 계획 review 20개와 구현 검토 또는 PR diff review를 같은 검토로 중복 계산 | 별도 plan review 20개, plan PR diff review 30개, 후속 runtime review 20개를 각각 독립 문서로 둔다. 통과. |
| 28 | 범위가 wider six-fit, 다른 row-span, FFI 또는 public CSS까지 확장 | plan은 existing exact profile, item minimum mask, 기본 Root HTML route만 제한한다. 통과. |
| 29 | public roadmap이 상태 대장의 후보 선택과 다르거나 공식 완료 수를 바꿈 | C11.4 row를 mixed per-item 후보와 계획·Chrome 비교 링크로 갱신했다. 78/214와 C11.4/C11 미완료는 유지한다. 통과. |
| 30 | 배포·tailnet을 plan-only PR에서 무단 변경 | 공개 site/preview 파일이나 배포 산출물은 변경하지 않는다. 통과. |

**결론:** diff의 30개 실패 경로를 확인했다. PR #1368 본문 screenshot은 Agent Browser에서 정상 표시됐다. 현재 PR은 open/mergeable이고 status check은 0개이며, 병합 여부는 별도 최종 gate로 확인한다.
