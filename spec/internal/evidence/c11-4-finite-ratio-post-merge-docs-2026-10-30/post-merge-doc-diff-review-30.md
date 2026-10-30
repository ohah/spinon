# C11.4 PR #842 병합 후 상태·로드맵 문서 diff 검토 — 30 checks

**검토 대상:** PR #842 병합 정보가 계획·공식 상태·내부 계약·evidence README·공개 roadmap에 일치하게 반영된 문서 변경. 원 구현 PR은 `78fd0b84e4132e1f91bd293735e97b33cbf0d9a0`에 반영됐다. 이번 문서 변경만으로 추가 feature completion을 선언하지 않는다.

| # | 실패 관점 | 근거와 판정 |
| --- | --- | --- |
| 1 | PR이 실제 merge되기 전에 문서가 merged라고 기록하는가 | GitHub PR #842의 `state=MERGED`, `mergedAt=2026-10-30T05:04:03Z`를 확인했다. **PASS** |
| 2 | 문서 merge SHA가 GitHub의 merge commit과 다르게 기록되는가 | 계획, 상태 대장, 내부 계약, evidence README와 roadmap이 `78fd0b84e4132e1f91bd293735e97b33cbf0d9a0`을 사용한다. **PASS** |
| 3 | branch head SHA와 merge commit SHA를 혼동하는가 | PR head `1e3b3fc89f37a063039a74c9606de92c221ec1d6`와 merge SHA를 각각 구별하고 문서에는 merge SHA를 기록했다. **PASS** |
| 4 | PR merge 시간대를 잘못 표기하거나 로컬 timezone과 섞는가 | GitHub의 UTC `2026-10-30T05:04:03Z`를 모든 상태 문서에서 동일하게 사용한다. **PASS** |
| 5 | PR의 빈 status rollup을 CI 성공으로 서술하는가 | 상태 대장, 계획, evidence 및 roadmap은 check rollup이 비어 있음을 밝히고 CI 성공으로 세지 않는다. **PASS** |
| 6 | `main` protection 응답에 없는 required-check 설정을 있다고 기록하는가 | main branch protection에는 required status checks 항목이 없고 PR은 `CLEAN`이었다. 문서는 검사가 설정되지 않았다고 적는다. **PASS** |
| 7 | GitHub의 required approving reviews가 없는데 승인 완료를 암시하는가 | branch protection의 승인 수 0을 확인했으며 문서는 리뷰 승인 상태를 완료 근거로 쓰지 않는다. **PASS** |
| 8 | PR의 actual `baseRefOid`와 merge base가 다른 상태를 같은 기록으로 섞는가 | PR #842는 `e56dd00264e9ed2ad05539a70f9e143e6d9ac1bf`를 base로 하며 merge 후 `origin/main`은 GitHub merge SHA로 이동했다. **PASS** |
| 9 | 공식 C11 parent checkbox가 실수로 체크되는가 | `spec/STATUS.md`의 C11 checkbox는 그대로 미완료다. **PASS** |
| 10 | 공식 C11.4 checkbox가 bounded PR merge로 완료 처리되는가 | C11.4 intrinsic track sizing checkbox가 unchecked로 유지되고 candidate 범위는 하위 note에 적는다. **PASS** |
| 11 | 공식 전체 완료 수가 `72/213`에서 바뀌는가 | status 및 public roadmap의 공식 완료 수가 계속 `72/213`이다. **PASS** |
| 12 | 계획 문서 첫 줄이 PR merge 전 상태로 남는가 | finite-positive plan header가 PR #842 merge와 SHA를 명시한다. **PASS** |
| 13 | parent C11.4 track-sizing plan의 최신 slice가 미병합 후보로 표시되는가 | parent plan 제목과 paragraph가 PR #842 merge 시각/SHA 및 checks 부재를 기록한다. **PASS** |
| 14 | 내부 contract version이 implementation merge 때문에 불필요하게 증가하는가 | 내부 계약 `0059`의 숫자 버전 `0.1.0`을 유지한다. **PASS** |
| 15 | 내부 계약 문구가 일반 author CSS 지원으로 넓어지는가 | 섹션은 제한 opt-in fixture candidate로 남기고 일반 CSS admission, WPT, hardware GPU를 보장하지 않는다고 명시한다. **PASS** |
| 16 | Taffy behavior를 standalone crate test 통과로 과장하는가 | Evidence는 workspace에서 `cargo test -p taffy`를 실행할 수 없는 경계를 기록하고 library Clippy/downstream runtime 검사를 구별한다. **PASS** |
| 17 | Chrome used geometry와 독립 산술 기대값을 exact equality처럼 설명하는가 | 계획과 계약은 1/64 CSS px quantization/tolerance를 구별하며 `3/2`의 관측값을 별도로 쓴다. **PASS** |
| 18 | `column-1-2` item frame과 root/track geometry가 하나로 뭉개지는가 | 문서는 40px track, 40×80px item, 100×60px root, 20px overflow를 각각 구분한다. **PASS** |
| 19 | platform sample total이 전체 16 scene에 대해 계산된 값과 다른가 | Android/iOS result JSON은 각각 16 scene과 49 sample을 보고하며 문서도 49/49 per platform으로 쓴다. **PASS** |
| 20 | Android AVD를 real device/hardware GPU로 오인하게 기록하는가 | AVD를 API 37 ARM64 QEMU/software renderer로 명시한다. **PASS** |
| 21 | iOS Metal drawable을 compositor-after screenshot이라고 확대 해석하는가 | Evidence/plan/contract는 drawable을 compositor 제출 전 readback으로 구별한다. **PASS** |
| 22 | simulator image path가 이전 2/1 screenshot을 가리키는가 | PR #842의 branch screenshots를 Agent Browser로 열어 Android 1080×2400, iOS 1206×2622 natural size와 complete load를 확인했고 roadmap은 동일 final run prefix를 쓴다. **PASS** |
| 23 | 현재 screenshot이 40×80/overflow 장면이 아니거나 label과 다른가 | PNG와 run ID를 확인했다. Android/iOS 둘 다 red fill이 root 60px boundary 아래로 이어지는 `column-1-2` capture다. **PASS** |
| 24 | 과거 PR #834 baseline 기록이 extension으로 덮이거나 오래된 자료가 삭제되는가 | 기존 four-scene/#834 README/evidence history를 유지하고 finite-positive 결과를 후속 section/card에 추가한다. **PASS** |
| 25 | 이전 계획 note가 아직 구현되지 않았다고 최신 상태를 반박하는가 | Roadmap의 historical plan card는 계획 당시 상태로 설명하고 PR #842 후속 merge를 연결한다. **PASS** |
| 26 | status/plan/evidence 링크가 public roadmap에서 서로 다른 ID를 가리키는가 | 새 roadmap card는 finite plan, range precomparison, runtime evidence, final Android/iOS PNG와 STATUS를 link한다. **PASS** |
| 27 | implementation PR의 actual GitHub URL이 잘못된 번호로 적히는가 | 모든 새 merge link가 `https://github.com/ohah/spinon/pull/842`를 가리킨다. **PASS** |
| 28 | screenshot 링크 확인을 hosted Tailnet delivery로 과장하는가 | 이번 diff는 PR body 이미지 로드를 언급할 뿐이며 Tailnet sync 완료를 주장하지 않는다. **PASS** |
| 29 | implementation artifact 결과와 docs-only follow-up이 feature behavior를 바꾸는가 | 이번 branch는 문서 상태/link만 갱신하고 source, API 또는 official checkbox를 변경하지 않는다. **PASS** |
| 30 | 독립 구현/runtime 및 initial PR diff 검토가 병합 기록에서 사라지는가 | 20관점 구현/runtime 및 30관점 implementation diff review 링크를 유지한다. **PASS** |

**결과:** 병합 증명, 공식 상태, simulator scope, 기존 역사 기록과 문서 링크를 서로 다른 30개 실패 관점에서 대조했다. 이 검토는 Tailnet publication 완료를 주장하지 않으며 Tailnet browser 확인은 동기화 뒤 별도로 기록한다.
