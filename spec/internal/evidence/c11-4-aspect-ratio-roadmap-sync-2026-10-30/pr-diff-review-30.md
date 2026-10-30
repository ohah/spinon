# C11.4 aspect-ratio 병합 상태·로드맵 동기화 PR diff 검토 30개

검토 대상은 `spec/STATUS.md`, C11.4 집중·상위 계획, 공개 `spec/public/roadmap.html`, 이 동기화의 빌드·Tailnet 증거와 helper다. 제품 runtime 코드는 변경하지 않았다.

| # | 별도 실패 관점 | 결과 |
|---:|---|---|
| 1 | PR #834 번호가 다른 작업을 가리킨다. | 구현 PR URL과 live PR metadata를 대조했다. #834다. 통과. |
| 2 | merge timestamp가 로컬 시간으로 잘못 기록된다. | `2026-10-30T02:49:15Z` UTC를 GitHub `mergedAt`과 대조했다. 통과. |
| 3 | merge SHA가 원 head SHA로 적힌다. | main의 `31361a9b0b01532fb0baa019f8c30d8b75caeb8a`와 PR mergeCommit을 대조했다. 통과. |
| 4 | PR check가 비었는데 CI 성공을 주장한다. | rollup이 비고 `gh pr checks --required`가 no checks, branch endpoint가 required checks 미설정 404임을 기록했다. 성공 주장은 없다. 통과. |
| 5 | 병합 기록을 근거로 C11 전체를 완료 처리한다. | C11 parent checkbox를 unchecked로 보존했다. 통과. |
| 6 | 최근 slice 때문에 C11.3을 완료 처리한다. | C11.3 checkbox를 unchecked로 보존했다. 통과. |
| 7 | bounded profile을 C11.4 전체 완료로 승격한다. | C11.4 checkbox를 unchecked로 보존했다. 통과. |
| 8 | 전체 공식 카운트를 PR 하나만큼 임의 증가시킨다. | `72/213`을 상태·계획·로드맵 전반에서 유지했다. 통과. |
| 9 | fixture-only route를 일반 CSS admission으로 설명한다. | opt-in fixture-test profile이라는 제한을 명시했다. 통과. |
| 10 | 4개 고정 scene을 전체 CSS Grid coverage로 오해시킨다. | 네 장면 고정 범위와 전체 C11.4 미완료를 썼다. 통과. |
| 11 | Chrome 사전 관측을 Spinon WPT 통과로 승격한다. | Chrome 비교와 전체 WPT 비포함을 구분했다. 통과. |
| 12 | Android AVD를 실기기 결과로 표현한다. | API 37 ARM64 QEMU AVD 표기를 유지했다. 통과. |
| 13 | iOS Simulator를 iPhone 실기기 결과로 표현한다. | iPhone 17 Pro/iOS 26.2 Simulator라고 썼다. 통과. |
| 14 | 12/12 표본을 장면 수나 glyph 수로 오인한다. | 4/4 scene과 12/12 표면 표본을 별도 수치로 표기했다. 통과. |
| 15 | Metal 표본이 compositor 이후임을 암시한다. | 로드맵은 관측된 표면 표본만 언급하고 README는 기존 runtime 한계를 유지한다. 통과. |
| 16 | 기본 archive의 marker 0/4를 전체 앱 bundle 배제 증명으로 쓴다. | 상태 문구는 archive marker 검사로 한정하고 제품 bundle 증명으로 확대하지 않는다. 통과. |
| 17 | 설치 산출물 hash 일치가 화면 동등성 증명으로 확대된다. | hash는 설치·로컬 산출물 연결로만 기술했다. 통과. |
| 18 | 상태 대장의 기존 checked C11.2 profile 상태가 변한다. | 동기화 전후 C11.2.2·C11.2.3 checked 상태를 유지했다. 통과. |
| 19 | PR #833 계획 PR 병합 기록이 새 구현 PR 기록으로 대체된다. | 계획 PR #833과 구현 PR #834 역할을 집중 계획에서 분리했다. 통과. |
| 20 | 계획 입력의 100px root 보정 이력이 빠진다. | 집중 계획 상태에 Chrome 156 재비교 전 root 폭 보정 사유를 유지했다. 통과. |
| 21 | runtime 또는 fixture 파일이 이 문서 PR에 섞인다. | tracked diff는 상태·계획·로드맵·sync 증거/helper로 제한했다. 통과. |
| 22 | roadmap의 새 항목이 C11.3의 이전 기록 뒤에 묻힌다. | 최신 병합 slice를 C11 progress 목록 최상단에 넣었다. 통과. |
| 23 | roadmap screenshot 링크 경로가 실제 artifact와 다르다. | 선택 output의 Android/iOS PNG 필수 경로 확인 및 Tailnet 252-route hash 검증을 했다. 통과. |
| 24 | RSPress route alias 또는 Markdown 제목 줄바꿈이 잘못 렌더링된다. | route manifest를 검사했고 pass01에서 발견한 상위 계획 제목 결합을 고친 뒤 pass02 Agent Browser에서 두 heading을 별도로 확인했다. 통과. |
| 25 | 전체 문서 build의 기존 dead-link 실패를 C11.4 변경 성공으로 감춘다. | 전체 build 실패와 selected build 성공을 README에서 별도 기록했다. 통과. |
| 26 | selected build 성공을 전체 문서 링크 검증으로 과장한다. | README가 선택 build의 범위를 명시적으로 제한한다. 통과. |
| 27 | preview replacement 전에 이전 파일을 덮어쓴다. | preflight에서 교체 6개를 분류하고 사전 백업 6/6 hash를 확인했다. 통과. |
| 28 | preview sync가 무관한 route를 삭제한다. | helper는 삭제하지 않고 `no_deletes`를 전 단계에서 기록한다. 0개 삭제다. 통과. |
| 29 | 중간 실패로 preview 파일이 부분 기록된다. | 파일별 temporary path와 rename을 사용하고 source/target SHA-256을 확인한다. 통과. |
| 30 | Agent Browser 세션이나 process를 남긴다. | 최종 pass02에서 `close --all`, active session 0, process scan 0을 기록했다. 통과. |
