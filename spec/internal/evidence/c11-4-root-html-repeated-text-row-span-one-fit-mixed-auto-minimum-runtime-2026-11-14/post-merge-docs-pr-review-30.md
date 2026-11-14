# C11.4 PR #1381 병합 후 상태·로드맵 동기화 · diff 실패 관점 30개

- 검토 대상: PR #1381 병합 후 상태 대장, 전용 계획, 내부 계약 0059, C11.4 plan index, evidence index/merge 기록, 공개 roadmap 변경
- 비교 기준: `origin/main` / base `416bd54519269851f07192bfc15a8c539154d866` → 문서 후보 `c60ddb876e211173d7450ad8b445811c5bec9c8c` (이 보고서는 다음 commit에서 추가)
- 문서 후보 diff SHA-256: `7e062227c48a911145e4699b54af2de8ddd6a8eebf8ac2934236afc8daebbc58`
- 검토 목적: bounded slice의 원격 merge 사실, 검증 분모·한계 및 공식 미완료 상태가 각 문서에 일치하는지 확인한다. 구현 코드는 이번 문서 동기화 diff에서 변경하지 않았다.

| # | 실패 관점 | 대조·판정 |
| ---: | --- | --- |
| 1 | PR source head를 rebase 후 main SHA로 오인 | GitHub의 `headRefOid`와 merge result를 따로 기록했다: `886e1b5…`와 `416bd54…`. |
| 2 | merge timestamp를 로컬 시간대로 바꿔 잘못 기록 | PR의 `mergedAt` UTC 값 `2026-11-14T00:42:53Z`를 그대로 썼다. |
| 3 | PR base가 달라 diff 범위가 잘못됨 | `baseRefOid`와 local base `929c447…`가 동일하다. |
| 4 | source diff hash가 다른 범위를 가리킴 | `929c447…`부터 PR source head `886e1b5…`까지 source diff를 다시 계산했다. SHA-256은 `2d6e4d1d…a4672f20`이다. |
| 5 | 저장소가 요구하는 merge method와 기록이 다름 | PR result는 Rebase and merge이며 main history에 merge commit이 생기지 않았다. |
| 6 | PR이 open/draft인데 문서가 merged로 표시 | `state=MERGED`, `mergedAt` 및 main ref `416bd545…`를 모두 확인했다. |
| 7 | GitHub checks 0개를 CI green으로 기록 | commit status와 workflow run이 각각 0이라 CI green 문구를 넣지 않았다. |
| 8 | required checks 부재를 checks green으로 오해 | `main` 보호 설정에 required status checks가 없음을 별도 확인해 문서에 분리했다. |
| 9 | bounded slice 병합을 공식 기능 완료로 승격 | C11.4/C11 unchecked와 공식 `78/214`를 유지했다. |
| 10 | 내부 계약 numeric version을 올리거나 낮춤 | 내부 계약 `0.1.0`을 그대로 기록했다. |
| 11 | 전용 계획이 PR review 중이라고 계속 표시 | 계획 상단과 실행 heading을 #1381 merge state로 갱신했다. |
| 12 | C11.4 plan index가 #1381을 계속 후보/미구현으로 표시 | one-fit slice를 최신 병합 항목으로 바꾸고 실제 결과 분모를 연결했다. |
| 13 | 이전 PR #1378의 기록을 덮어써 역사 손실 | #1378 기록은 보존하고 이전 병합 항목으로만 재분류했다. |
| 14 | STATUS의 최신 C11.4 문구는 #1378에 머물거나 오래된 후보를 가리킴 | #1381 기록을 추가하고 #1378 뒤의 “제품 구현 미시작” 문장을 제거했다. |
| 15 | checklist나 집계가 `78/214`에서 바뀜 | status checkbox/count 변경은 staged diff에 없다. |
| 16 | 공식 계약 0059가 one-fit slice를 draft로 표시 | heading과 본문을 merged metadata 및 검증 한계로 갱신했다. |
| 17 | evidence directory에 README가 없어 roadmap/status 경로가 404 | 디렉터리에 README가 없음을 확인해 evidence index를 추가했다. |
| 18 | 새 README의 simulator 그림 링크가 잘못됨 | 네 PNG의 tracked 경로와 각 링크를 대조했다. |
| 19 | Chrome oracle과 RuntimeSession의 scene 수가 섞임 | RuntimeSession `120,960/120,960` scene과 `900/900` track-list batch를 구분했다. |
| 20 | eligible item 분모가 scene 수로 오해됨 | `169,344`를 automatic-minimum item occurrence로 표시했다. |
| 21 | 기존 회귀 분모가 endpoint끼리 혼합 | one-fit `2,880`/`4,320` pairs, two-fit row-span `3,840`, auto-placement `1,440`, repeated-text `810`을 따로 기록했다. |
| 22 | runtime review와 PR diff review를 같은 검토로 합산 | implementation/runtime 20과 실제 implementation PR diff 30을 별도 링크했다. |
| 23 | 추가한 Markdown 상대 링크가 branch 안에서 깨짐 | plan, STATUS, contract, README, merge record의 새 local link target을 모두 확인했다. |
| 24 | public roadmap evidence route가 merge 기록 대신 raw log로 연결 | runtime matrix, merge record, plan, contract와 공식 STATUS를 구분해 연결했다. |
| 25 | roadmap 정적 문구가 #1381을 review/check pending이라 표시 | section summary와 hero progress를 merge SHA/status로 교체했다. |
| 26 | roadmap dynamic board가 완료된 PR을 active review로 표시 | checks pending 문구를 제거하고 남은 C11.4 work와 미선정 후보를 구분했다. |
| 27 | 다음 bounded 후보가 이미 결정된 것처럼 임의 게시 | 후속 계획이 없으므로 다음 후보는 미선정으로 적었다. |
| 28 | simulator smoke를 mixed-mask app matrix 또는 pixel parity로 과장 | visual smoke 범위, Android llvmpipe, surface readback/parity 한계를 명시했다. |
| 29 | iOS light PR 이미지가 계속 깨지거나 asset 손상으로 오진 | PR body `<img src>`의 SHA 8자 누락을 고쳤다. Agent Browser에서 네 이미지가 로드되고 Android `1080×2400`, iOS `1206×2622`임을 확인했다. |
| 30 | 정리 문서가 reusable resource나 selected preview를 제거한 것으로 오해 | task AVD/Simulator와 Agent Browser session 종료, 무접속 8769/8770 stale preview 종료를 기록했다. 선택된 Tailscale PID 90711과 reusable cache/profile은 보존했다. |

**판정:** merge metadata와 공식 미완료 수치가 일치한다. 이전 evidence README 경로는 index를 추가해 해결했다. 다음 bounded C11.4 후보는 아직 계획되지 않았으므로 임의로 지정하지 않았다.
