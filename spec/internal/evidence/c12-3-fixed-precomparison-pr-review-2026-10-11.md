# C12.3 Chrome 사전 비교 변경 검토

변경 파일이 계획·공식 상태·사용자 roadmap과 맞는지 각 경계를 따로 대조했다. 이 변경은 기준 fixture를 추가하며 C12.3 런타임 완료를 주장하지 않는다.

| # | 공격 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 변경이 고정 배치 런타임 코드를 몰래 포함하는가? | `crates/` 변경은 없다. CSS fixture·reference·capture·문서만 추가한다. |
| 2 | package script가 명령을 다른 fixture에 연결하는가? | `css:reference:c12-3-fixed-positioning`은 전용 capture 파일을 실행한다. |
| 3 | `test:css-reference` 자동 탐색에서 전용 테스트가 빠지는가? | 기존 wildcard `tools/css-reference/*.test.mjs`가 새 테스트를 포함한다. |
| 4 | inventory와 실제 Chromium DOM이 서로 다른 node 목록을 갖는가? | capture가 inventory node를 직접 생성하고 parent/case/ID/position/owner를 각 관찰에서 검사한다. |
| 5 | 같은 owner 기준을 reference test가 재검증하지 않는가? | 5개 test가 schema/hash, node 관계, DPR/resize, geometry 대표값, negative effect 경계를 확인한다. |
| 6 | Chrome 기준이 사용자 전역 browser profile을 열어 오염시키는가? | helper는 임시 profile·임시 DevTools를 사용하며 프로세스 그룹 소유를 확인한 뒤 종료한다. |
| 7 | cleanup이 소유를 확인하지 않고 다른 Chrome 프로세스를 종료하는가? | helper의 process-group ownership 검사에 실패하면 자동 종료·profile 삭제를 중단한다. 이번 캡처 중 한 번의 거부는 정확한 임시 profile/PGID를 읽기 전용으로 확인한 뒤 해당 그룹만 정리했다. |
| 8 | reference 재생성이 일반 실행으로 승인 없이 덮어쓰는가? | 존재하는 reference를 기본 capture가 거부하며 명시 플래그만 교체를 허용한다. |
| 9 | hashes가 변경 중인 다른 capture 도구를 가리키는가? | reference와 사전 비교 문서의 inventory/HTML/capture/helper digest가 현재 파일과 같다. |
| 10 | 시각·환경 정보가 누락돼 결과를 재현할 수 없는가? | Chrome, revision, binary hash, host, flags, viewport, DPR, locale/timezone/media를 JSON에 기록한다. |
| 11 | resize 결과가 첫 관찰과 혼합되거나 no-op을 잘못 비교하는가? | 네 단계 resize 배열을 분리하고 같은 크기의 연속 관찰과 시작/복귀를 별도 비교한다. |
| 12 | spec 상태표가 사전 비교 후에도 잘못 “다음에 비교”라고 표시하는가? | 상태표는 기준을 완료로 설명하고 기능 구현은 미완료로 유지한다. |
| 13 | C12.3 계획이 아직 완료되지 않은 기준 작업을 요구하는가? | 다음 구현 관문을 viewport owner·revision·ancestor 거부 runtime으로 분리한다. |
| 14 | internal README에 새 근거 링크가 누락되는가? | precomparison 근거 링크를 추가했다. |
| 15 | public roadmap이 이전 단계 문구를 계속 보여주는가? | roadmap을 Chrome 기준 완료·runtime 구현 다음 단계로 동기화했다. |
| 16 | 새 route를 RSPress가 생성하지 못하거나 잘못된 route로 가는가? | RSPress 2.0.22 빌드에서 계획·사전 비교 route를 생성했고, agent-browser로 Tailnet roadmap·계획·사전 비교 화면을 열어 제목·표·새 링크를 확인했다. 사전 비교 표의 Chromium revision/hash도 컨테이너 밖으로 가로 넘침이 없는지 viewport 폭을 확인했다. |
| 17 | 변경이 금지된 GitHub Pages workflow를 실행하는가? | GitHub Pages 배포 workflow는 실행하지 않는다. |
| 18 | PR 설명에 비공개 Tailnet 주소를 노출하는가? | PR 본문에는 Tailnet 주소를 넣지 않는다. 변경된 문서의 임시 Tailnet 미리보기는 경로별 HTTP 200과 실제 브라우저 렌더링으로 확인했다. |
| 19 | 기존 사용자 변경이나 미추적 파일이 PR에 섞이는가? | C07.2 실기기 미추적 증거와 두 Stylo spike 디렉터리는 stage 대상에서 제외한다. |
| 20 | WPT·Android/iOS·WGPU 실행을 완료로 잘못 기술하는가? | WPT는 미실행이고 mobile runtime 미측정이라고 reference, evidence, 상태표에 유지한다. |

남은 조건은 C12.3 변경 파일만 선택해 커밋하고, PR checks·mergeability를 확인하는 것이다. C07.2 미추적 실기기 증거와 두 Stylo spike 디렉터리는 이 변경에 포함하지 않는다.
