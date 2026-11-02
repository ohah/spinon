# Root HTML column-flow implicit columns PR diff 실패 관점 30개

**대상:** 구현 후보 전체 diff, 내부 계약, 실행 근거 및 제출 예정 PR 내용을 검토했다. 구현/runtime 실패 관점 20개와는 다른 통합·문서·산출물·병합 관점이다. 계획 PR의 30개 검토는 재사용하지 않았다. 현재 source fingerprint는 [source-revision.json](source-revision.json)에 있다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | C11 상위 또는 C11.1을 좁은 candidate로 완료 표시했는가 | 상태 대장 변경 없이 C11과 C11.1 미완료를 유지한다. |
| 2 | 전체 roadmap 완료 수를 candidate merge 전 올렸는가 | 공식 `72/213`을 그대로 둔다. |
| 3 | internal contract 숫자 버전을 기능 추가만으로 올렸는가 | 내부 계약 `0.1.0` 고정 문구를 유지한다. |
| 4 | public API/일반 CSS Grid 지원으로 과장했는가 | 기본 Root HTML 제한 경로와 공개 CSS 미지원 경계를 문서에 명시한다. |
| 5 | 이전 PR #1056의 explicit-cell contract와 모순되는가 | 앞선 계약을 좁게 확장하고 이전 입력/거부 조건을 유지한다고 기록한다. |
| 6 | row flow나 다른 profile이 우연히 신규 auto-column list를 받는가 | Root HTML column-flow 분기와 placement mode 경계를 문서 및 코드가 일치하게 제한한다. |
| 7 | `%`/`fr`/intrinsic/calc 지원 범위가 계획·계약·코드에서 다르게 서술되는가 | computed finite px scalar/list만 지원하고 percentage-dependent 계산은 거부한다고 일치시켰다. |
| 8 | `grid-auto-columns:auto`의 명시 용량 내/초과 동작이 모호한가 | explicit cell 내 `auto` 유지, implicit column 필요 시 거부로 계약에 구분했다. |
| 9 | track cap이 리스트와 총 column 수 중 한 곳에서 빠지는가 | 두 입력 모두 10,000 제한을 적고 projection/layout 테스트가 경계를 나눠 확인한다. |
| 10 | 에러/부분 publication 설명이 구현과 다른가 | property 문맥 오류·partial scene 미발행을 계약과 runtime 근거에 기록했다. |
| 11 | 독립 expected JSON이 browser 관측값을 복사해 기대값 순환 논증이 되는가 | fixture 산술로 작성한 기대값과 Chrome observation을 별도 artifact로 유지한다. |
| 12 | Chrome browser version/viewport/screenshot hash가 누락되는가 | 사전 비교 README에 Chrome 156, DPR 1, viewport 및 screenshot/fixture hash가 기록되어 있다. |
| 13 | precomparison fixture hash가 앱 번들과 다른가 | 설치된 Android/iOS의 `index.html`이 fixture와 같은 SHA-256임을 검증했다. |
| 14 | marker script가 HTML/DOM 경로 밖에서 단순 고정 성공을 내는가 | same-document root/body/main 관계, direct `<div>` 12개와 marker 왕복을 확인한다. |
| 15 | 임시 bootstrap 수정이 checkout에 남는가 | 종료 trap으로 복구했고 현재 두 파일 hash가 보관본과 각각 일치한다. |
| 16 | Android build가 opt-in fixture ABI를 제품 archive에 포함하는가 | build log에서 `c04_runtime_gpu_fixture_test=0`; smoke는 asset 입력만 교체한 기본 앱 경로다. |
| 17 | iOS bundle 자산이 Android와 다르거나 이전 빌드 산출물인가 | 설치 bundle의 HTML/JS/executable hash를 별도 `installed-artifacts.json`에 기록했다. |
| 18 | Android install log만 있고 실제 설치 artifact 확인이 없는가 | 설치 APK를 기기에서 읽어 빌드 APK 전체 hash와 비교했다. |
| 19 | 테스트 기기 선택이 실기기 또는 baseline Simulator 상태를 왜곡하는가 | Android는 API 37 AVD, iOS는 별도 iPhone 17 Pro UDID를 썼고 기존 C11 baseline Simulator는 유지했다. |
| 20 | simulator `layout=ready`를 item geometry와 혼동하는가 | app log는 marker·box 수·draw/present만 주장하고 정확한 item geometry는 Rust runtime test와 Chrome 비교에 귀속했다. |
| 21 | screenshot을 GPU pixel readback으로 과장하는가 | PixelCopy, Metal drawable readback, compositor 이후 equality가 아님을 README에 적었다. |
| 22 | platform build 경고를 숨기거나 build success로 둔갑시키는가 | 원본 build logs를 보존했고 Android SDK compatibility warning과 성공 종료를 분리해 읽을 수 있다. |
| 23 | 전체 package test 실패를 감추고 모든 검사 green이라고 하는가 | 전체 style-to-layout 308/309와 C07 serialization mismatch를 README에 명시했다. |
| 24 | focused Rust 검사 결과를 raw logs 없이 요약만 기록하는가 | fmt, layout, cascade, projection, runtime, Clippy 각각의 raw log를 링크했다. |
| 25 | Clippy 범위에 영향 crate나 test target이 빠지는가 | 네 영향 crate의 `--all-targets -D warnings` 로그를 연결했다. |
| 26 | 계획·구현·PR 검토가 하나의 표를 재사용하는가 | 계획 20/계획 PR diff 30과 새 구현 20/구현 PR diff 30 자료를 분리한다. |
| 27 | screenshot evidence 파일이나 prior failed pass를 덮어썼는가 | 새 simulator 캡처는 전용 runtime 디렉터리에 저장했고 사전 비교 pass01~03을 보존한다. |
| 28 | unrelated untracked roadmap/evidence 파일이 candidate에 섞이는가 | PR stage 단계에서 이 작업 경로만 선택하며 기존 사용자 산출물을 삭제하지 않는다. |
| 29 | CI check rollup이 비어 있는데 통과했다고 PR 본문에 적는가 | PR #1059의 `statusCheckRollup`은 비어 있고 `gh pr checks`는 “no checks reported”를 반환했다. 본문은 CI 통과를 주장하지 않는다. |
| 30 | PR 이미지가 reviewer가 접근하지 못하는 local path인가 | PR #1059 본문의 두 raw GitHub screenshot을 Agent Browser로 확인했다. Android 1080×2400과 iOS 1206×2622 이미지가 모두 `naturalWidth>0`으로 로드됐다. 검증 뒤 Agent Browser 세션을 닫았고 해당 프로세스도 종료됐다. |

이 검토에서 blocking diff 결함은 찾지 못했다. 라벨, 실제 PR 번호·본문 렌더링, GitHub check 상태는 PR 생성 후 한 번 더 확인한다.
