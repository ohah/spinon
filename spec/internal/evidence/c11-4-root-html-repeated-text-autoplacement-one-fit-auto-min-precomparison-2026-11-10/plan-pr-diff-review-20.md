# C11.4 one-fit auto-placement 자동 최소 크기 계획 PR diff · 적대 검토 20개

검토 범위는 이 PR의 제한 계획, 실패 관점 목록, Chrome 사전 비교 산출물, `spec/STATUS.md`의 계획 기록과 공개 roadmap의 다음 작업 링크다. 각 행은 서로 다른 diff 회귀 경로를 검사했다.

| # | Diff 공격 관점 | 깨질 수 있는 결과 | 확인·판정 |
|---:|---|---|---|
| 1 | 계획 PR에 구현 결과가 섞임 | Chrome 계획 근거가 RuntimeSession 완료처럼 읽힘 | 계획 문서와 STATUS가 제품 구현 전이라고 명시 · 통과 |
| 2 | 직전 구현 경계 오연결 | one-fit 계획이 merged two-fit auto-placement 코드를 자기 범위로 주장 | 계획은 PR #1318을 직전 경계로, one-fit을 새 대상이라 분리 · 통과 |
| 3 | 기존 one-fit 기준 링크 단절 | `min-height:0` corpus와 scene 비교가 재현되지 않음 | 기준 README/관측 경로가 실제 repository 내 존재 · 통과 |
| 4 | 계획·증거 상대 경로 오타 | GitHub 문서에서 계획·README 이동이 404 | 새 계획, 리뷰, 사전 비교 상대 경로 확인 · 통과 |
| 5 | roadmap 카드 누락 | repo 내 공개 요약이 다음 작업 선택 전으로 남음 | summary에 one-fit auto-min plan 링크와 구현 전 표시 · 통과 |
| 6 | STATUS/C11.4 상태 승격 | bounded plan만으로 C11.4나 공식 완료 수가 변함 | 체크 상태와 `78/214`, internal `0.1.0`을 그대로 둠 · 통과 |
| 7 | matrix 분모 축소 | 일부 position/cap/alignment만으로 1,440 완료를 주장 | 6×30×4×2 산식과 1,440행/720 list 검증 · 통과 |
| 8 | baseline key drift | candidate와 기준 scene의 input ordering 차이를 동작으로 오인 | 동일 `(position,tail,cap,align)` key 집합 1,440개 비교 · 통과 |
| 9 | track multiplicity 오기 | one-fit profile이 아니라 다른 track family를 검증 | 정확히 fit 1, min-content 2, max-content 2, auto 1 · 통과 |
| 10 | tail 중복 순서 손실 | 30개 순서가 아닌 dedup/표본 order만 통과 | 독립 permutation set 30개와 computed order 검사 · 통과 |
| 11 | 계획 검토의 cap 오기 | 사전 비교에 없는 `0px`와 `0%`를 review가 짝으로 주장 | 발견한 문구를 `20px`와 `20%` 비교로 고쳤고 실제 cap 집합 `20px`, `20%`, `0%`, `120%`로 맞춤 · 수정 후 통과 |
| 12 | control item auto minimum 변조 | explicit control의 automatic minimum이 target에 포함 | computed `0px`와 `[0,0,40,10]` frame를 검사 · 통과 |
| 13 | floor를 computed style만으로 판정 | `min-height:auto` 표기만 있고 used track은 cap 아래 | 모든 fit cycle row와 payload floor를 비교 · 통과 |
| 14 | duplicate alignment 계산 | stretch 결과를 start와 같다고 가정하거나 자유 공간 누락 | list마다 40px 분배와 auto-only row delta를 독립 판정 · 통과 |
| 15 | 결과 범위 과장 | 이 corpus에서 보이지 않은 overflow를 지원이라 보고 | overflow 0/1,440을 기록하고 overflow 지원 주장은 배제 · 통과 |
| 16 | 브라우저 artifact 재생성 단절 | screenshot/scene JSON과 fixture·collector가 다른 입력 사용 | fixture 및 collector SHA-256을 verifier가 metadata와 비교 · 통과 |
| 17 | 독립 검증기 자기참조 | 검증 summary만 신뢰하고 raw scene 비교를 생략 | verifier가 NDJSON scene, baseline, CSS Grid 관계를 직접 읽음 · 통과 |
| 18 | Agent Browser 세션 잔존 | Chrome이 열린 상태로 남거나 Terminal Browser로 대체 | lifecycle은 정상 close, terminal false; 프로세스 inventory에서 전용 session 없음 · 통과 |
| 19 | 이미지 링크로 oracle 대체 | 화면만 보고 geometry를 판정 | screenshot을 visual reference로 한정하고 verifier/NDJSON을 수치 근거로 지정 · 통과 |
| 20 | 공유 simulator data 삭제 | 다른 프로젝트나 다음 slice 데이터가 제거됨 | 이 사전 비교는 simulator를 시작하지 않았고 계획은 task-owned 임시 resource만 제거 · 통과 |

**결과:** PR diff의 서로 다른 적대 관점 20개를 확인했다. 사전 비교는 Chrome 관측에 한정되며 Spinon 구현 증거는 없다.
