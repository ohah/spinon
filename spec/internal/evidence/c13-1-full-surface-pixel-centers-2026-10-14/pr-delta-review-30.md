# C13.1 전체 surface pixel-center PR 변경 관점 검토 · 2026-10-14

병합 전 변경 delta를 서로 다른 30개 실패 경로와 계약 경계로 검토했다. 각 항목은 계획 문서의 사전 검토나 구현 실패 검토를 반복하지 않고, PR에 포함할 코드·증거·상태·플랫폼 설명이 잘못 전달될 위험을 확인한다.

| # | PR delta 실패 관점 | 변경에서 확인한 근거와 판정 |
|---:|---|---|
| 1 | C13.1 상태가 전체 완료로 체크된다 | `spec/STATUS.md`의 C13.1 미완료 항목과 남은 WPT/scroll/event 범위를 유지했다. **통과.** |
| 2 | 내부 계약 버전이 구현 증분만으로 올라간다 | `0060`의 `0.1.0` 버전을 유지하고 관찰 범위만 보충했다. **통과.** |
| 3 | 계획이 관찰 전 기대 결과를 사후에 바꾼다 | 사전 oracle·860 mismatch·pass 기준이 있는 계획을 별도 보존하고 결과를 추가했다. **통과.** |
| 4 | pre-fix evidence가 최종 결과와 덮어 쓰인다 | 수정 전 JSON/PixelCopy와 수정 후 JSON/PixelCopy를 별도 파일명으로 보존했다. **통과.** |
| 5 | renderer 변경이 C13.1 fixture-only 상태를 없애 일반 지원으로 표현된다 | 변경 범위는 paint-order 수정이며 status와 contract는 대상 profile/한계를 명시한다. **통과.** |
| 6 | stacking-order 변화와 hit-order 변화가 하나의 순서로 합쳐진다 | renderer 회귀가 두 order 값을 각각 확인하며 README도 독립 계약으로 설명한다. **통과.** |
| 7 | clip ancestry가 paint 재배치 결과에서 다시 추론된다 | clip test 및 evidence가 clip 계보를 별도 값으로 유지한다. **통과.** |
| 8 | comparator가 한 축에 다른 scale을 무시한다 | x=`788/300`, y=`263/100`을 독립 계산하고 comparator unit test가 non-square scale을 사용한다. **통과.** |
| 9 | 중심점 변환에서 픽셀 경계 반올림 규칙이 다르다 | 구현은 `floor((css+0.5)*scale)`이며 JSON에 scale과 결과 좌표를 보존한다. **통과.** |
| 10 | 각 CSS center가 같은 physical pixel로 중복 매핑돼 30,000점을 과장한다 | comparator가 mapped coordinate set으로 중복을 거부한다. scale≥1 guard와 undersized width/height tests가 있다. **통과.** |
| 11 | screenshot 자체가 Chrome 기준 viewport와 크기가 다르다 | pinned observation을 다시 검사하고 reference PNG 크기를 viewport와 비교한다. **통과.** |
| 12 | 실제 PixelCopy PNG가 작거나 crop돼 빈 표본을 읽는다 | 양의 실제 dimensions와 최소 scale을 확인하며 out-of-range sample은 assertion 실패다. **통과.** |
| 13 | RGBA의 alpha 차이를 RGB 일치로 통과시킨다 | 네 channel을 모두 비교하며 R/G/B/A 각각 1-byte mutation을 거부한다. **통과.** |
| 14 | 1-byte mismatch가 허용 오차로 숨겨진다 | pass 조건은 exact equality이며 허용치가 없다. 단일 alpha mismatch test의 결과도 좌표·최대 오차 1로 기록한다. **통과.** |
| 15 | decoder 또는 oracle fixture가 바뀌어 결과가 신뢰할 수 없다 | 기존 PNG decoder와 Chrome observation validator를 재사용하고 Chrome version/revision·PNG SHA·viewport를 JSON에 기록한다. **통과.** |
| 16 | 출력 JSON은 success만 남겨 mismatch 원인을 잃는다 | 전체 mismatch별 CSS/PixelCopy 좌표와 expected/actual RGBA, 범위, scale, hashes를 저장한다. **통과.** |
| 17 | 기존 고정 52점 비교를 전체 surface 30,000점으로 오인한다 | README, plan, STATUS가 52점과 CSS center grid의 범위를 따로 기록한다. **통과.** |
| 18 | before/after 이미지가 같은 파일 또는 잘못된 캡처다 | 수정 전·후 각각 PixelCopy와 화면 PNG의 SHA 및 실제 dimensions를 manifest에 고정했다. **통과.** |
| 19 | PR 화면 이미지 링크가 비공개/만료 경로다 | PR 본문은 공개 PR commit SHA 기반 raw GitHub 경로를 사용하고 agent-browser에서 실제 로딩을 확인한다. **게시 후 확인 예정.** |
| 20 | 첨부 화면이 실행 앱 대신 crop된 표면만 보여준다 | Android 전체 앱 화면과 원본 PixelCopy surface를 별도 이미지로 제공하고 iOS Simulator 화면도 별도 표기한다. **통과.** |
| 21 | AVD 캡처가 실기기 GPU 증거로 표현된다 | API 37 ARM64 AVD, `ro.kernel.qemu=1`, ANGLE/Vulkan/SwiftShader와 `PixelCopy` 범위를 명시한다. **통과.** |
| 22 | APK 실행이 빌드된 앱과 연결되지 않는다 | 설치 APK와 산출물 SHA-256 일치값 `1b5212…3406a` 및 runtime run ID/PID를 로그에 보존했다. **통과.** |
| 23 | fixture route가 부분 frame 뒤 capture되어 최종 결과를 왜곡한다 | 최종 Android log는 route generation 2, 41/41 frame과 52/52 query/sample을 기록한다. **통과.** |
| 24 | iOS Simulator 화면을 Metal surface byte readback으로 확장 해석한다 | iOS 결과는 41/41 frame·52/52 hit와 1206×2622 화면만이며 Metal drawable readback 미실행이라고 명시한다. **통과.** |
| 25 | 실기기 연결 요구를 충족했다고 오인한다 | Android만 AVD를 사용했고 연결 장치가 `emulator-5554`였으며 실제 기기 결과를 주장하지 않는다. **통과.** |
| 26 | WPT/scroll/event/CSSOM 결과가 존재하는 것처럼 상태를 쓴다 | WPT 미실행과 scroll/CSSOM 및 event dispatch 미검증을 README·STATUS의 남은 범위로 유지했다. **통과.** |
| 27 | 빌드 로그에 개발자 로컬 경로 또는 사용자 정보가 유출된다 | 제출 대상 `*.log`에서 작업자 home, worktree 절대경로, username 패턴이 없는지 재검색했다. **통과.** |
| 28 | 증거 PNG/비교 JSON이 PR 중간에 교체돼 문서의 수치와 달라진다 | SHA-256 manifest와 비교 JSON이 각 artifact bytes를 연결한다. **통과.** |
| 29 | PR 설명이 필수 template의 status/spec/evidence/limits를 빠뜨린다 | `.github/pull_request_template.md`의 각 항목을 PR 본문에서 채우고 type/area label을 확인한다. **게시 전 확인 대상.** |
| 30 | CI/mergeability가 바뀐 뒤 이전 상태만 보고 병합한다 | push 후 PR checks, mergeability와 protection을 다시 조회하고 전부 만족할 때만 Rebase and merge한다. **병합 직전 확인 대상.** |

게시 후 확인 예정 항목은 PR URL과 실제 GitHub 화면에서 결과를 갱신한다. branch SHA와 required checks가 확정되기 전에는 PR 게시물의 이미지/병합 조건을 통과로 세지 않는다.
