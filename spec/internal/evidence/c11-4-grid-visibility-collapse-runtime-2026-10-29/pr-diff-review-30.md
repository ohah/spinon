# C11.4 Grid `visibility:collapse` 구현 PR diff 검토 30개

검토 대상은 PR [#804](https://github.com/ohah/spinon/pull/804)의 구현 diff와 본문, evidence path다. 구현/runtime의 동작 실패 검토는 [별도 20개](./implementation-runtime-review-20.md)에 있다. 이 검토는 계획 단계의 [계획 PR diff 20개](../../../../plan/c11-4-grid-visibility-collapse-pr-diff-review-20.md)를 재사용하지 않는다.

| # | PR diff 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 작업 브랜치가 현재 main보다 오래되거나 별도 worktree에서 나옴 | PR base와 fetch한 `origin/main`은 `6be4b0c67`이고 같은 checkout에서 순차 작업했다. 새 worktree를 만들지 않았다. 통과. |
| 2 | 다른 진행 중인 C11 작업이나 오래된 snapshot이 PR에 섞임 | 명시한 109개 path만 커밋했다. workspace의 다른 미추적 evidence와 이전 재시도 폴더는 diff에 없다. 통과. |
| 3 | 증거 링크가 가장 최신 실행 대신 이전 성공/실패 실행을 가리킴 | README와 review는 final run `mvtfme5e` Android 및 `mvtfn06e` iOS만 링크한다. 두 run의 각 manifest는 10 scene·14 표면 표본과 앱 binary hash를 담는다. 통과. |
| 4 | `sourceDirty` manifest에 무관한 작업 경로·파일 목록이 노출됨 | Capture runner는 이번 runtime source와 runner 경로만 상태로 기록한다. final result JSON은 각각 약 3.1 KB이며 다른 미추적 evidence 이름을 포함하지 않는다. 통과. |
| 5 | 계획 문서가 사전 비교 단계로 남아 실제 구현 결과와 충돌 | 계획 상태를 제한 fixture 구현·Simulator 검증 완료로 갱신하고 전체 C11.4 게이트는 열어 두었다. 통과. |
| 6 | 상태 대장이 전체 Grid 기능 지원처럼 기록 | C11.4 항목 아래 narrow runtime evidence만 덧붙였고 부모 C11/C11.4 checkbox를 바꾸지 않았다. 통과. |
| 7 | 전체 공식 완료 수가 하위 slice 때문에 잘못 증가 | diff는 `72/213`을 유지하며 C11/C11.4 체크 상태를 변경하지 않는다. 통과. |
| 8 | 내부 계약 버전이 구현 추가만으로 올라감 | 내부 계약 0059는 `0.1.0`을 유지한다. 통과. |
| 9 | Grid hidden-equivalence inference가 규격의 직접 문장처럼 서술 | CSS Display fallback과 Grid item sizing 규칙을 결합한 inference라고 명세·상태·PR 본문에서 구분한다. 통과. |
| 10 | 제한 fixture slice가 일반 CSS author support로 읽힘 | 계약, 계획, status, PR body가 일반 author CSS admission을 미지원으로 명시한다. 통과. |
| 11 | 다른 Grid profile 또는 Flexbox collapse 경로가 의도치 않게 허용 | 새 cascade gate는 fixture-test route 전용이며 Flex/Table collapse를 제외한다. 기존 generic route의 거부 회귀를 유지한다. 통과. |
| 12 | default/product static archive에 새 route marker가 들어감 | 기본 `c04-runtime-gpu` archive는 marker 0개, fixture-test positive control은 10개 matching strings다. 근거는 inspected macOS static archive로 범위를 제한했다. 통과. |
| 13 | archive 결과를 APK/IPA 전체 fixture exclusion으로 확대 | README와 계약이 static archive 한정이며 Android APK/iOS app executable 제외를 주장하지 않는다. 통과. |
| 14 | 고정 route 외 scene ID가 임의 JS source를 생성 | FFI catalog는 exact 10 IDs만 라우팅하고 빈 값·suffix 변형 negative cases를 거부한다. 통과. |
| 15 | Android/iOS scene catalog가 Rust fixture ID와 다름 | 두 native stage expected-value set을 FFI 10-scene catalog와 대조했고 각 플랫폼 capture runner가 전 장면을 통과했다. 통과. |
| 16 | 잘못된 node kind/namespace/parent 또는 extra child가 기대값을 통과 | runtime preparation은 HTML element identity·부모·direct child 수·element 수를 검사하고 불일치는 typed failure로 반환한다. 통과. |
| 17 | 다른 document/style/layout revision trace가 연결됨 | snapshot profile, generation, document revision, render-tree revision 검사와 root-scoped Grid trace 조회가 있다. 통과. |
| 18 | cascade가 `collapse`를 `hidden`으로 바꾸어 computed 결과를 잃음 | focused cascade test와 scene runtime summary는 computed `collapse`를 그대로 요구한다. 통과. |
| 19 | collapse와 `display:none`의 sizing 차이가 가려짐 | 양축 collapse/hidden은 50px 두 track을, none은 0px 두 track과 0×0 item을 검사한다. Rust trace와 Simulator logs가 구분한다. 통과. |
| 20 | item frame 보존과 item paint suppression이 혼동 | layout validator는 item frame을 유지하고 render snapshot은 invisible item box를 제외한다. Rust render test와 앱 box counts에서 각각 검사한다. 통과. |
| 21 | collapsed parent의 visible descendant가 사라짐 | computed `visible`, 8×8 frame, blue box, child hit target 및 surface sample을 확인한다. Android/iOS 양축 모두 통과. |
| 22 | hidden overflow가 root 밖에서 계속 paint/hit 됨 | root-local 1px 바깥에서 collapse/hidden 표본은 흰색이고 hit target은 없어야 한다. Android PixelCopy/iOS Metal 표본이 통과. |
| 23 | root 경계 positive control 부재로 overflow sample이 무의미 | visible-column/row는 같은 경계점에서 red paint와 content hit을 요구한다. 양 플랫폼 통과. |
| 24 | `display:none` scene의 pixel 표본 생략을 paint 성공으로 오해 | 결과는 `pixelSamples=0`으로 분리하고 computed display, frames, zero tracks와 draw boxes를 확인한다. 통과. |
| 25 | fixture white clear state가 후속 Grid scene에 남음 | PR 구현은 매 C11.4 eval 앞에서 flag를 초기화하고 성공한 visibility scene에서만 다시 켠다. final app matrices를 그 뒤 재실행했다. 통과. |
| 26 | FFI에 invalid scene, 포인터 또는 sample count가 전달돼 crash | unknown scene catalog test와 기존/new FFI null, UTF-8, length/count guard를 확인했다. public input contract를 추가하지 않는다. 통과. |
| 27 | simulator가 build 산출물과 다른 설치본을 실행 | Android installed APK SHA-256과 iOS installed executable SHA-256이 build 산출물과 각각 일치한다. 통과. |
| 28 | 표면 좌표·채널·scene summary를 느슨하게 비교 | 고정 root-local sample ID/target/좌표/RGBA, trace chunk 완결성, expected frames와 draw/present box count를 엄격 비교한다. 통과. |
| 29 | simulator 결과를 실기기, hardware GPU 또는 compositor 이후 pixel로 과장 | Android는 API 37 ARM64 software AVD PixelCopy, iOS는 compositor 제출 전 Metal drawable로 기록하고 나머지는 미검증으로 적는다. 통과. |
| 30 | PR 본문의 동작 화면 URL이 잘못되어 이미지가 보이지 않음 | Agent Browser에서 PR #804 화면을 열어 commit-pinned raw URL 두 개를 확인했다. iOS `naturalWidth=1206`, Android `naturalWidth=1080`, 둘 다 `complete=true`다. 캡처 [screenshot](./pr-body-agent-browser.png)와 [이미지 확인 JSON](./pr-image-load-agent-browser.json)을 남겼으며 Browser session은 `active=false`, `pid=null`로 닫았다. 통과. |

**검토 결과:** 위 30개의 서로 다른 PR diff 경계에서 추가 수정 blocker를 찾지 못했다. GitHub에서 PR의 `mergeStateStatus`는 `CLEAN`이다. main branch protection에는 required status check가 설정되지 않았고 required approval 수는 0이며 linear history가 필수다. PR check query는 비어 있어 CI 성공으로 기록하지 않는다.
