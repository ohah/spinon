# C11.3.3 area-dimension implicit tracks 변경 diff 실패 관점 20개

이 검토는 구현·런타임 검토와 분리해 현재 PR 후보 diff 및 문서·fixture 연결만 본다. PR 전 review 20개다. 프로젝트 규칙의 병합 전 PR 검토 gate는 별도로 완료해야 한다.

| # | diff 실패 관점 | 확인과 근거 | 판정 |
|---:|---|---|---|
| 1 | 현재 작업 branch가 다른 작업 branch 또는 main을 수정 | `git status`의 branch는 `feat/c11-3-3-area-auto-tracks`, 기준 HEAD는 `ca854215020c331c08defe72bf06a126b57e7b65`다. 변경 파일은 C11.3.3 구현·fixture·근거 범위다. | PASS |
| 2 | 공식 완료 수나 C11 parent checkbox를 앞당김 | `spec/STATUS.md`, 계획 및 conformance entry는 공식 `72/213`과 C11.3.3 candidate/미완료 문구를 유지한다. | PASS |
| 3 | internal interface numeric version을 구현만으로 올림 | 내부 계약 0059의 고정 `0.1.0`은 바뀌지 않았다. 새 단락은 candidate 증거만 기록한다. | PASS |
| 4 | 공개 지원/전체 CSS Grid 또는 WPT 적합성으로 과장 | `spec/0001-conformance.md`, plan, 0059 모두 opt-in profile·simulator-only·실기기/hardware/WPT 경계를 명시한다. | PASS |
| 5 | 전용 계획의 상위 선행 조건과 PR 순서를 지움 | plan은 계획 PR 병합 뒤 별도 구현 PR이라는 순서를 유지하고, 이 diff는 구현 PR 생성 전 candidate로 쓴다. | PASS |
| 6 | 기존 26-scene Chrome 154 reference를 덮어쓰거나 재생성 | 원본 HTML/inventory/reference 경로는 변경하지 않았다. generated matrix는 pinned 26 scene을 유지하고 Chrome 156 관측 5개만 supplemental으로 결합한다. | PASS |
| 7 | Chrome 보충 관측 파일이 다른 fixture 입력에 연결됨 | `chrome-observed.json`의 fixture path와 SHA-256을 builder가 supplemental HTML bytes와 비교한다. README에도 같은 SHA를 기록한다. | PASS |
| 8 | Agent Browser 관측이 screenshot/CSSOM/frame과 분리됨 | evidence 폴더는 관측 JSON, 원시 eval envelope, PNG screenshot과 fixture SHA를 함께 보존한다. README는 관측 경계를 CSSOM/geometry로 제한한다. | PASS |
| 9 | generated runtime matrix가 수동 edit 뒤 원본 oracle과 불일치 | `build-c11-3-3-grid-template-areas-runtime-matrix.mjs --check`가 current bytes를 재생성 결과와 비교하고 31/90 summary를 확인한다. | PASS |
| 10 | 새 scene ID·node ID 순서가 fixture와 FFI/app route에서 다름 | builder는 supplemental scene 순서와 node preorder를 oracle에 대조하며 FFI/Android/iOS allowlist에 동일한 5 ID가 있다. | PASS |
| 11 | 기존 rejection이 사라지거나 새 rejection code가 잘못 연결 | `area-track-count-mismatch`는 Chrome 154 geometry success 대조군으로 남고, 기대 rejection은 unknown implicit sizing 한 건만 exact `unsupported_grid_auto_rows`다. | PASS |
| 12 | Android capture가 기존 26 scene만 실행하고 새 oracle은 무시 | Android capture는 `scene.oracle` supplemental override를 사용하고 기준 Chrome 154 observation이 정확히 26개인지 계속 확인한다. | PASS |
| 13 | iOS capture가 새 matrix 요약은 받지만 supplemental oracle geometry는 미사용 | iOS input validator와 geometry capture가 `scene.oracle`을 우선 사용하며 기존 Chrome 154 reference 길이 26을 따로 고정한다. | PASS |
| 14 | route mutation이나 잘못된 sample 좌표가 runtime draw/readback까지 도달 | 최신 Android/iOS result에 unknown scene rejected-before-runtime-draw, 좌표 변조 rejected-before-readback negative control이 기록됐다. | PASS |
| 15 | 최종 Android evidence README가 `result.json`보다 과거 26-scene 실행을 보고 | 이를 diff review 중 발견해 31 scene·90 node, 25/5/1로 수정했다. README 수치는 성공 `result.json`의 matrix 집계와 일치한다. | PASS |
| 16 | iOS/Android 결과를 같은 신뢰 경계로 합쳐 실제 OS compositor proof라 주장 | iOS 문서는 Metal drawable이 compositor 제출 전이라고 명시하고 Android는 API 37 AVD/SwiftShader로 한정한다. | PASS |
| 17 | local tests·CI·PR merge 상태를 한 문장으로 합침 | plan은 local focused checks 통과, implementation PR/CI/merge 미실행을 분리해 기록한다. 이 review는 CI 성공을 주장하지 않는다. | PASS |
| 18 | Tailnet preview/Pages를 사용자 요청 없이 배포 | 이 diff에는 Pages workflow 실행 또는 Tailnet preview 복사 변경이 없다. 공식 페이지 배포를 완료로 보고하지 않는다. | PASS |
| 19 | unrelated untracked evidence, vendor cache 또는 다른 roadmap 파일을 PR에 섞음 | 현재 tracked diff만 C11.3.3 대상이며, evidence/fixture는 경로를 골라 추가할 예정이다. untracked C04/C11.4·`vendor/taffy/target`은 작업 트리에서 보존하고 stage 대상에서 제외한다. | PASS |
| 20 | PR 본문의 화면 증거가 로컬 경로이거나 깨진 링크 | 구현 PR 전용 PR 본문은 아직 만들지 않았다. 생성 후 iOS Simulator 화면을 GitHub에서 렌더링되는 image URL로 올리고 Agent Browser를 닫은 뒤 공개 화면에서 검증한다. | PENDING |

## 판정

19개 관점은 현재 diff와 evidence에서 PASS다. 20번은 PR이 없어 아직 수행할 수 없으며, PR 생성 후 image URL의 표시 여부를 확인해 PASS 또는 결함으로 기록한다. 이 문서는 병합 승인이나 required checks 완료를 뜻하지 않는다.
