# C13.1 Android 앱 hit identity grid 계획 PR 변경 관점 검토 · 2026-10-14

계획 PR에 실제로 포함될 문서·precomparison artifacts를 서로 다른 30개 failure surface로 검토했다. 구현 코드는 아직 포함하지 않으며 별도의 다음 implementation PR에서 추가 검토한다.

| # | PR delta 위험 | 계획/증거에서 확인한 결과 |
|---:|---|---|
| 1 | 계획 PR이 C13.2 scroll 구현을 앞당겨 소유 범위를 섞는다 | task는 C13.1 Android hit identity fixture만 다룬다. **통과.** |
| 2 | `STATUS.md`가 pixel-center surface matrix를 여전히 미완료로 중복 기록한다 | PR #162의 30,000-point paint grid는 완료, Android app hit grid는 별도 남은 상태로 구분했다. **통과.** |
| 3 | paint RGBA 검증과 hit NodeId 검증을 같은 결과로 합친다 | 계획은 30,000 pixel-center surface scan과 integer hit grid를 각각 분리한다. **통과.** |
| 4 | fixed 52 hit sample을 full grid coverage로 승격한다 | 기존 52 query는 subset control이고 30,000 app query가 별도 acceptance다. **통과.** |
| 5 | host full viewport 1.4M oracle를 app runtime 성능/연동 증거로 표현한다 | 문서는 host Rust scan과 Android app route를 따로 부른다. **통과.** |
| 6 | Chrome compact HTML source가 바뀐 뒤 옛 grid를 재사용한다 | HTML SHA-256을 validator에서 고정하고 test로 바뀐 source를 거부한다. **통과.** |
| 7 | compact fixture JavaScript node order가 바뀌어 index digest가 달라진다 | JS SHA와 exact 41-node order가 pinned다. **통과.** |
| 8 | Chrome reference observation JSON이 다른 source/browser를 가리킨다 | observation SHA, screenshot-linked record, version/revision/UA를 검사한다. **통과.** |
| 9 | browser revision은 고정이지만 실제 UA가 다른 executable에서 나온다 | capture UA와 fixed observation UA를 byte-for-byte 대조한다. **통과.** |
| 10 | viewport나 DPR이 바뀌고 query count만 같아진다 | width/height/DPR/scroll/root exact match를 요구한다. **통과.** |
| 11 | test coordinate convention이 정수에서 `+0.5`로 조용히 바뀐다 | convention 문자열을 고정하고 center-probe를 별도 failure artifact로 보존했다. **통과.** |
| 12 | CSSOM View fractional-edge 차이를 Spinon fail로 판정한다 | browser observation과 normative page가 다른 지점을 기록하고 current plan에서 exclusion한다. **통과.** |
| 13 | RLE run count 870만 믿고 RLE 전체 coverage를 검증하지 않는다 | 각 row의 adjacency와 300-wide terminal boundary를 실제 전개 검사한다. **통과.** |
| 14 | 41 IDs가 유일해도 index 순서가 잘못된다 | expected ID vector를 fixture source case order에서 다시 만든다. **통과.** |
| 15 | histogram 합이 30,000이어도 위치가 틀린다 | 100x300 expanded digest와 기존 52 coordinate subset을 같이 검사한다. **통과.** |
| 16 | digest만 고정해 oracle corruption 위치를 놓친다 | rows, count, node order, histogram, source pins를 독립적으로 검증한다. **통과.** |
| 17 | validator의 mutation tests는 한 failure class만 반복한다 | viewport/convention, row gap/overlap, ID, digest, source/provenance, fixed sample 변형을 별도로 거부한다. **통과.** |
| 18 | initial center capture mismatch를 삭제하고 evidence를 정제한다 | 오른쪽 마지막 열 100점과 아래 마지막 행 300점의 합에서 우하단 1점을 뺀 399 null 관찰을 invalid initial probe로 보존하며 pass oracle와 분리했다. **통과.** |
| 19 | 계획 PR의 browser screenshot이 Spinon Android app 화면처럼 보인다 | 계획 PR evidence는 Chrome oracle screenshot이라고 분명히 표시한다. 실제 Android screen은 implementation PR에서 첨부한다. **통과.** |
| 20 | 새 route가 기존 compact fixture와 다른 host를 만든다 | 동일 handle/snapshot/generation을 사용한다는 plan invariant를 둔다. **통과 기준 포함.** |
| 21 | 30k expected/actual JSON이 JNI output buffer를 넘는다 | full RLE는 Rust 내부 comparison으로 쓰고 bounded summary만 FFI로 반환한다. **통과 기준 포함.** |
| 22 | mismatch가 16개를 넘으면 누락돼 total이 틀려 보인다 | total count와 preview count를 구분한다. **통과 기준 포함.** |
| 23 | high-frequency query가 per-point Logcat 비용을 만든다 | per-point logging을 금지하고 one summary plus bounded mismatch preview를 정한다. **통과 기준 포함.** |
| 24 | CSS coordinates를 Android physical pixel scale로 변환한다 | app hit API는 CSS integer coords를 직접 받으며 PixelCopy density mapping과 분리한다. **통과.** |
| 25 | route collision/중복 activity가 test host 재사용을 가린다 | existing single route만 호출하고 route collision negative control을 유지한다. **통과 기준 포함.** |
| 26 | fixture FFI symbol이 default artifact에 유출된다 | 기존 opt-in fixture-test feature에서만 symbol/data를 허용하고 positive/negative artifact proof를 계획했다. **통과 기준 포함.** |
| 27 | iOS screen/build가 없는 Android-specific PR을 iOS app integration으로 과장한다 | iOS는 shared Rust FFI build boundary만 확인하고 runtime grid는 제외한다. **통과.** |
| 28 | app evidence가 route/PID/APK/generation와 연결되지 않는다 | API 37 ARM64 AVD, route, run ID, PID, APK hash, generation, frame/query/sample counts를 기록한다. **통과 기준 포함.** |
| 29 | PR body template/실행 화면 요구가 implementation PR에서 빠진다 | 이 plan PR은 Chrome screenshot만 oracle로 표기하고, implementation PR은 Android screen inline + agent-browser image load 확인을 요구한다. **통과.** |
| 30 | 계획 PR과 implementation PR을 한 번에 합쳐 사전검토 경계가 흐려진다 | 계획 PR rebase merge 뒤 oracle 재검사 후 구현 PR을 시작하도록 순서를 고정했다. **통과.** |

이 검토는 계획 PR delta만 대상으로 한다. 구현 뒤에는 새로운 implementation review와 해당 implementation PR의 독립 delta 검토가 필요하다.
