# PR #827 diff 적대 검토 20개

**대상:** PR #827의 source diff, 계획·공식 상태·계약 변경, simulator/PR 이미지 근거와 merge readiness. 구현/runtime 검토와 별도의 변경 패키지 검토다.

| # | PR 변경을 잘못 읽거나 병합을 위험하게 할 수 있는 경우 | 확인 근거와 판정 |
|---:|---|---|
| 1 | PR base/head가 예상과 달라 main 외의 선행 Grid 작업을 다시 포함한다. | PR #827 base는 `main`, head는 `feat/c11-4-grid-text-first-baseline`이다. diff는 현재 feature commit과 이 slice의 후속 문서 갱신만 포함한다. **PASS** |
| 2 | unrelated workspace edits·실패 capture가 PR에 섞여 review 범위를 바꾼다. | 초기 commit은 정확히 67개 C11.4 baseline source·docs·oracle·final capture 파일이다. 후속 commit은 C11.4 plan/status/contract/roadmap 및 Tailnet sync evidence로 제한한다. 기존의 unrelated untracked evidence와 다른 작업 경로는 stage하지 않는다. **PASS** |
| 3 | 새로운 GitHub label을 만들거나 필수 유형/영역 label이 빠진다. | 기존 `enhancement`, `area: layout`, `area: android`, `area: ios`, `area: testing`, `area: docs`를 지정했고 저장소 label rule의 유형·영역을 충족한다. **PASS** |
| 4 | PR title/body가 저장소 언어 및 양식을 따르지 않는다. | title 접두는 영어 `feat(c11.4):`이며 설명은 한글이다. 변경 상태 ID, 명세, 성공·오류·플랫폼 한계가 template 항목으로 기록됐다. **PASS** |
| 5 | STATUS progress record가 C11 구현 대장에서 빠지거나 별도 checkbox로 오인된다. | candidate record는 기존 C11.4 progress-record와 같은 목록 계층에 두고 별도 checkbox를 만들지 않았다. Agent Browser STATUS에서 `C11.4 intrinsic track sizing` checkbox는 unchecked다. **PASS** |
| 6 | 제한 fixture 실행이 C11.4 또는 C11 완료로 집계된다. | PR body, STATUS, internal contract, public roadmap 모두 candidate/미완료라고 썼다. Parent checkbox는 미체크다. **PASS** |
| 7 | slice가 전체 공식 완료 수를 임의 승격한다. | 현재 status·public roadmap·Tailnet 화면은 모두 공식 `72/213`을 표시한다. **PASS** |
| 8 | internal API/계약 version이 feature candidate 때문에 불필요하게 바뀐다. | 계약 0059의 숫자 버전 `0.1.0`을 유지하고 변경 내용을 제한 opt-in clause로 추가했다. **PASS** |
| 9 | Chrome precomparison을 Spinon/WPT pass로 표현한다. | Chrome 156은 구현 전 oracle/observation로 별도 분리했다. PR body와 STATUS는 Spinon WPT 적합성을 주장하지 않는다. **PASS** |
| 10 | PR 이미지 URL이 branch/ref 파싱이나 잘못된 path로 깨진다. | 두 PR 이미지 URL은 immutable commit `7e2b339080198245bbc08812fac3d7345f2343f5`의 raw GitHub path다. Agent Browser에서 두 `<img>` 모두 `complete=true`, natural size가 각각 1080×2400 및 1206×2622였다. **PASS** |
| 11 | PR screenshot이 시뮬레이터가 아닌 Chrome comparison image를 가리킨다. | 본문 Android·iOS 그림은 각 최종 simulator capture 디렉터리에서 오고, Chrome PNG는 precomparison evidence에만 있다. **PASS** |
| 12 | Android evidence가 실기기/물리 GPU 실행으로 잘못 귀속된다. | final `simulator.json`은 `emulator-5554`, API 37, ARM64, `qemu=1` 및 installed/local APK hash 일치를 기록한다. 실행 대상은 명시적 AVD serial이다. **PASS** |
| 13 | iOS screenshot이 compositor 후 화면 readback으로 과장된다. | final metadata는 iPhone 17 Pro/iOS 26.2 Simulator executable hash 일치를 기록하고 README/PR body가 Metal 표본을 compositor 이전 drawable이라고 부른다. **PASS** |
| 14 | 표면 sample exact가 glyph painting까지 검증했다고 오해된다. | PR body, plan, 계약, runtime README와 public roadmap은 표본을 root/item fill·gap으로 제한하고 WGPU glyph paint를 미검증으로 표시한다. **PASS** |
| 15 | trace baseline 값과 frame 값이 서로 다른 입력/scene인데 한 결과로 합쳐진다. | capture script는 scene ID, node ID, line baseline, frame, column/row track을 함께 검사하고 source-order 역전도 별도 기대값을 쓴다. raw runtime chunks가 final result에 보존된다. **PASS** |
| 16 | Simulator app process가 검증 뒤 남아 다음 작업을 오염시킨다. | runner는 Android `force-stop`, iOS `terminate`를 `finally`에서 수행하며 final evidence는 네 scene 전체다. **PASS** |
| 17 | static archive result를 APK/IPA 전체 artifact에서 fixture 제외됐다는 증거로 확대한다. | archive README는 named symbol/scene ID probe와 positive control을 적고 debug static archive 한계 및 APK/IPA 비검증을 명시한다. **PASS** |
| 18 | Tailnet update가 삭제·backup 없는 덮어쓰기·server restart를 수행한다. | 최종 pass02 preflight/copy/verify는 61 route, 교체 전 backup 12/12, no deletes다. 기존 preview listener는 재시작하거나 종료하지 않았다. **PASS** |
| 19 | Tailnet text/hash 결과와 실제 browser-rendered page/PR images가 다르다. | pass02에서 local 61/61, backup 12/12, remote HTTP 200·SHA-256 61/61을 확인했다. Agent Browser DOM에서 roadmap candidate와 PR diff 링크, STATUS의 unchecked C11.4 및 `72/213`을 확인했다. PR 본문의 Android/iOS image element도 각각 1080×2400/1206×2622로 로드됐다. 최종 screenshot과 DOM, close evidence를 Tailnet sync README에 연결했고 종료 뒤 agent-browser process는 없었다. **PASS** |
| 20 | PR check 부재를 CI success로 취급하거나 merge strategy가 보호 설정과 다르다. | PR #827에 check run이 보고되지 않고 main required status checks가 설정되지 않았다. PR merge state는 `CLEAN`, branch protection은 linear history만 요구한다. CI 통과로 말하지 않고 저장소 지정 `Rebase and merge`만 merge 방식으로 사용한다. **PASS** |

**결과:** 20개 변경 패키지 관점 모두 PASS. Follow-up source diff에는 공개 상태/roadmap의 PR review 링크와 pass02 Tailnet 검증 증거가 포함된다. STATUS candidate의 들여쓰기와 screenshots의 glyph-paint 설명은 바로잡았으며 `72/213` 및 C11.4/C11 미완료 상태를 유지한다. PR check run은 보고되지 않았고 CI 통과로 간주하지 않는다.
