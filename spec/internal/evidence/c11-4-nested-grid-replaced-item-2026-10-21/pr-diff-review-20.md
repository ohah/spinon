# Nested Grid text + replaced item PR diff 교차 파일 검토 20개

검토 대상은 PR #443의 `origin/main` 대비 구현·테스트·Android/iOS harness·capture runner·명세·계획·실행 근거 diff다. 구현/runtime 검토 20개와 겹치지 않도록 변경 간 계약, 회귀 경로, 결과 표현을 중심으로 독립된 실패 관점 20개를 확인했다. PR 이미지 링크는 Agent Browser 확인 결과와 종료된 프로세스 확인을 포함한다.

| # | PR diff 실패 관점 | 검토 결과 / 근거 |
|---:|---|---|
| 1 | 새 FFI scene이 기존 C11.4 scene ID와 충돌해 다른 source가 반환되는가 | 세 ID는 `nested-grid-text-image-*`로 한정되고 각 ID의 정확한 source와 비허용 ID 거부를 FFI unit test가 검사한다. 기존 ID는 변경되지 않았다. |
| 2 | 새 fixture 모듈이 opt-in feature 없이 product/runtime 빌드로 유입되는가 | runtime adapter/test module은 `c11-4-grid-track-sizing-fixture-test` 조건부이고 FFI는 기존 fixture feature 아래에 있다. default static archive에서 payload와 새 fixture symbol이 없다는 기록과 opt-in positive control이 있다. |
| 3 | 새 adapter 삽입 순서가 기존 C11.4 adapter를 가로채거나 혼합 입력을 중복 계산하는가 | 동일 feature/profile에서 새 고정 root ID만 준비한다. 성공한 경우 기존 anonymous/direct mixed adapter의 준비·dispatch를 건너뛰고, 그 외 root는 기존 순서대로 처리한다. |
| 4 | 새 route가 일반 계산 profile에서도 고정 fixture를 활성화하는가 | adapter 선택과 intrinsic revision 연결 모두 `BlockFlexGridTrackSizing` profile에 묶여 있다. product/general profile dispatch는 이 변경으로 추가되지 않았다. |
| 5 | root의 자식·namespace/tag 변형이 일부 결과를 성공으로 오인하게 하는가 | adapter가 document root 직계 HTML `div` 한 개와 그 아래 고정 HTML nested `div` 한 개를 요구한다. 불일치는 fail-closed error다. |
| 6 | nested item 순서나 parent가 바뀌어 image/text intrinsic source가 뒤섞이는가 | nested Grid 직계 자식 순서는 text then image로 고정하며 각 node의 owner·ID와 placement를 검사한다. runtime test 및 runner가 source order 0/1과 node identity를 확인한다. |
| 7 | 추가 descendant나 텍스트 조각이 고정 Ahem 문장으로 축약되는가 | text item은 정확히 하나의 직접 text node `XX XXXXXXX`여야 하며 image는 leaf여야 한다. 추가 노드나 문자열은 승인되지 않는다. |
| 8 | fixture style 입력 변경이 계산값만 그대로 둔 채 oracle 비교를 무효화하는가 | root/nested Grid track·gap·dimension·placement와 text/image box·typography의 computed style을 검사한다. nested alignment도 profile과 일치하는 `start`/`normal` 조합만 허용한다. |
| 9 | 새 scene의 basis width, gap 산술, track count가 Android/iOS에서 다르게 고정되는가 | FFI, runtime tests, Android/iOS expected-frame tables, simulator runner가 동일한 세 scene의 98/102/120px root와 70/24px inner tracks를 사용한다. |
| 10 | one-track control이 두 track span 결과로 잘못 분류되는가 | `nested-grid-text-image-one-track`은 outer first-track placement, contribution span 1, tracks 98/0, root width 102를 각 계층에서 따로 검사한다. |
| 11 | definite flex control이 min-content overflow를 zero minimum으로 잘못 세는가 | 이 장면은 120px definite width와 58/58 tracks를 고정하고 `Minimum=0px`, span 2를 요구한다. min-content 장면 둘과 별도 판정한다. |
| 12 | inner image/text source가 outer nested Grid contribution의 node로 잘못 기록되는가 | 테스트와 capture runner가 intrinsic inputs는 inner text/image node에 귀속되고 outer contribution item은 nested Grid node임을 분리해 검사한다. |
| 13 | text intrinsic 측정이 OS font fallback이나 설치 폰트 상태에 좌우되는가 | opt-in adapter는 system fonts 없는 `TextLayoutEngine`에 저장소 Ahem font를 등록한다. 기대 min/max-content는 runtime/direct-layout 테스트에서 70/100px로 검사한다. |
| 14 | image URL이 같아도 다른 bytes 또는 잘못된 natural size로 테스트가 통과하는가 | 고정 document request를 고정 PNG bytes로 decode해 resource snapshot을 만든다. metadata는 24×16px인지 확인하며 decode/request/snapshot 오류는 layout failure다. |
| 15 | 새 이미지 snapshot이 layout revision/cache freshness에서 누락되는가 | 새 mixed input의 intrinsic revision을 layout current stamp에 포함한다. Rust 및 simulator runner가 revision 1과 입력/output revision 일치를 확인한다. |
| 16 | adapter를 건드린 뒤에도 테스트가 이전 fixture만 확인하는가 | 새 direct layout test와 runtime test는 세 새 ID를 loop하며 frame·track·source·contribution·revision을 비교한다. FFI allowlist test와 simulator scene list도 새 세 ID를 별도 추가했다. |
| 17 | Android harness 등록만 되고 iOS harness에서 결과를 생략하거나 box 수를 틀리게 허용하는가 | 양 플랫폼 scene ID, root frame, 네 box, 모든 node frame 기대값이 추가됐다. 각 simulator 결과는 세 scene 모두 보고하고 `drawPresented`를 기록한다. |
| 18 | capture runner가 새 scene을 일반 trace 경로로 처리해 nested diagnostics 검증을 건너뛰는가 | runner는 nested intrinsic metadata 존재 시 outer trace를 지정해 선택하고 inner tracks, parent contribution, source identity, source order, intrinsic revision을 확인한다. 새 장면을 포함해 scene count 176→179로 일치시켰다. |
| 19 | PR screenshot markdown URL이 branch-only 경로거나 이미지 요소만 있고 실제 파일은 깨져 있는가 | PR 본문은 commit-pinned raw GitHub URL을 사용한다. Agent Browser 0.31.1에서 Android 1080×2400 및 iOS 1206×2622 이미지가 모두 complete·visible임을 확인했다. 창을 닫은 뒤 `browserLaunched=false`, page 0을 확인하고 해당 daemon PID를 종료했다. |
| 20 | 문서가 고정 fixture 결과를 전체 C11.4 완료나 미검증 픽셀 근거로 확대하는가 | conformance, STATUS, 계약 0059, 계획, 공개 roadmap은 C11.4와 공식 72/213을 유지하고 simulator·surface·WPT 한계를 구분한다. 전체 `docs:build`는 다른 tracked spec dead link로 실패했으며 새 evidence path 오류는 두 번째 실행에서 보고되지 않았다. |

## 판정

이 diff 검토에서 변경을 막는 새 결함은 확인하지 않았다. 위 제한은 fixture 범위와 실행 근거에 그대로 남긴다. 문서 전체 build는 기존 다른 spec dead link 때문에 green이 아니며, 이를 이번 PR의 성공 검사로 표현하지 않는다. PR required checks와 merge 상태는 이 기록 작성 뒤에도 GitHub에서 다시 확인해야 한다.
