# C11.4 anonymous Grid text + image 구현 PR diff 실패 관점 검토 30개

대상은 PR [#425](https://github.com/ohah/spinon/pull/425)의 병합 전 diff와 저장된 Android API 37 AVD/iOS 26.2 Simulator 결과다. [구현/runtime 20개 검토](./implementation-review-20.md)와 관점을 분리했다. 아래 30개는 최종 diff를 서로 다른 경계에서 대조했고 차단 이슈는 남지 않았다.

| # | 독립 실패 관점 | diff·근거 대조 및 판정 |
| --- | --- | --- |
| 1 | 계획된 작업 외 파일이나 사용자 데이터가 PR에 섞임 | PR에는 C11.4 결합 fixture 코드·runner·계획과 지정된 실행 근거만 들어 있다. 다른 미커밋 evidence는 stage하지 않았다. |
| 2 | 내부 adapter가 새 공개 API로 오인될 수 있음 | 추가 projection 함수와 export는 `c11-4-grid-track-sizing-fixture-test` opt-in 안에 있고 공개 JS/C API나 일반 CSS support flag는 추가하지 않았다. |
| 3 | 기본/제품 artifact가 fixture ABI나 payload를 포함 | 기본/opt-in archive의 symbol·scene 문자열 대조에서 기본은 0, opt-in은 신규 ABI 2개와 scene ID 5개다. [측정 자료](./ffi-artifact-boundary.md). |
| 4 | 내부 계약 버전 또는 공개 support 상태가 무단 변경 | 내부 계약 0059와 0.1.0을 그대로 두고 `spec/STATUS.md` 체크/집계도 변경하지 않았다. |
| 5 | 새 helper가 맞지 않는 profile/viewport에 실행 | helper에서 `RuntimeBlockFlexGridTrackSizingV1`과 viewport를 확인하고 mismatch는 기존 `StyleLayoutError`로 반환한다. |
| 6 | combined scene이 기존 mixed fixture 경로에서 먼저 거부 | runtime dispatch는 결합 입력 준비를 먼저 수행하고 인식하면 기존 mixed/text/image input 준비를 생략한다. 다른 scene의 기존 우선순위는 보존한다. |
| 7 | 장면 ID typo가 일반 layout 성공으로 숨겨짐 | 고정 prefix의 익명 text/image scene ID는 알 수 없는 경우 unsupported 오류를 낸다. 다섯 알려진 ID 외에는 source generator도 `None`을 반환한다. |
| 8 | root tag, namespace, parent 또는 scene child 순서가 달라도 계산 | HTML root `div`, document-root 소유권, 직접 자식 수와 순서, parent 및 text/image identity를 사전 확인한다. |
| 9 | text node가 익명 item 수나 stable anchor와 혼동 | 입력 diagnostics는 anonymous item anchor와 `textSources` node 집합을 별도로 기록한다. Rust/native 검증이 anchor, 순서와 개별 source를 대조한다. |
| 10 | Chrome literal comment가 host comment node 지원처럼 표시 | Chrome 입력은 실제 comment를 쓴다. 앱 host fixture는 인접 Text node로만 동작을 모사하고 README/plan에 차이를 적었다. |
| 11 | whitespace-only text가 배치 슬롯을 차지 | 앞쪽 whitespace-only sequence의 item 부재와 뒤 text item의 source/frame을 다섯 관찰 자료에서 확인했다. |
| 12 | 숨김 subtree text가 anonymous run을 분리하거나 item이 됨 | `display:none` span 및 자식 text를 fixture에 두고 visible item만 유지하는 track/source/frame을 확인했다. |
| 13 | `<img>`의 기본 inline display가 collector flush 경계를 만들지 못함 | 결합 fixture projection만 HTML image Grid item의 computed inline을 `LayoutDisplay::Block`으로 변환한다. 다섯 장면에서 image가 독립 input이 된다. |
| 14 | blockification이 non-image 또는 모든 profile로 새어 나감 | flag는 새 gated helper만 켜고, image Grid node set predicate와 HTML namespace/tag 검사로 scope를 제한한다. 나머지 projection options는 false다. |
| 15 | image style이 자동화된 기대값과 달라져도 수용 | 원본 runtime source에는 `display:block`이 없다. adapter는 span 2, auto width, height 48px, zero edges와 default ratio condition을 검증한다. |
| 16 | PNG 요청이 네트워크 응답 또는 오래된 cache에 좌우 | 고정 document URL에서 `ImageRequest`를 만들고 고정 PNG byte를 decode한다. natural size 24×16을 확인하고 network fetch를 추가하지 않았다. |
| 17 | text/font input과 image intrinsic input의 revision 불일치 | 하나의 intrinsic snapshot revision을 결합 layout 계산에 넘기고 result revision에 bind한다. Rust 회귀와 native validator가 revision 1을 확인한다. |
| 18 | host system font나 폰트 로딩 순서가 text metrics 변경 | fixture는 system fonts 없는 engine에 고정 Ahem bytes를 등록하고 10px metrics와 font registry revision을 검증한다. |
| 19 | root/Grid CSS 차이가 우연히 같은 frame을 만들어 통과 | adapter는 root dimensions, tracks/rows, gap, line names, flow, alignment 및 상속 text style을 고정 scene 값과 비교한다. |
| 20 | 실제 item 순서/frame이 틀려도 track 합만 맞아 통과 | Rust 회귀는 각 scene의 node IDs/sourceOrder, textSources, natural size, image frame, root frame, track trace를 확인한다. |
| 21 | FFI source route가 기존 scene을 가로챔 | 새 exact allowlist lookup 뒤 기존 `anonymous_text_items`와 나머지 generator route를 유지했다. `--list-scenes`는 165개, 고정 새 scene 5개를 보고한다. |
| 22 | source generator가 ID 충돌·scene 누락 또는 display:block을 끼워 넣음 | fixture source test는 scene count/uniqueness, 각 ID/source size, image/text node 수, `height:48px`와 inline style에서 block display가 빠진 조건을 확인한다. |
| 23 | Android scene allowlist/frame count가 layout scene과 다름 | Java adapter는 다섯 ID, root 220×60, image 좌표, 2개 presented boxes와 empty item-frame 조건을 추가했다. AVD 결과 5/5가 통과했다. |
| 24 | Android intrinsic JSON validator가 text/image 순서나 metrics를 부분 비교 | validator는 grid/node/sourceOrder/kind, text source nodes/metrics, replaced natural size와 revision을 모두 비교한다. runtime summary에 assertion error가 없다. |
| 25 | iOS validator 동작이 Android와 다르게 완화 | Swift adapter도 다섯 ID·frames 및 같은 intrinsic JSON 필드를 검사한다. 지정 iPhone 17 Pro/iOS 26.2에서 5/5 통과했다. |
| 26 | 한 trace chunk만 읽어 뒤쪽 trace mismatch를 누락 | capture runner는 표시된 전체 chunk를 합쳐 JSON을 판정하고 Columns/Rows stage, intrinsic spans와 contributions, final tracks를 검사한다. 최종 Android/iOS는 모든 chunk가 포함됐다. |
| 27 | trace assertion의 예상 규칙 자체가 잘못돼 실제 pass를 오판 | 첫 AVD 실행에서 text `Minimum` 0 항목 생략과 반복 `MinContent`를 확인해 runner를 수정했다. raw AVD log를 근거로 바꾼 검증기로 두 플랫폼을 다시 실행했다. |
| 28 | Android run ID 길이 초과 또는 이전 log 재사용 | 실행 ID를 evidence 폴더 prefix에서 분리해 80자 인수 제한에 맞추고, run별 ID/PID log를 저장한다. 첫 scene superseded retry도 별도 log로 보존했다. |
| 29 | 시뮬레이터 target이나 APK가 검증 대상과 다름 | Android는 `qemu=1`, API 37, ARM64 및 설치 APK/local APK hash 일치를 기록한다. iOS는 정확한 booted UDID, iPhone 17 Pro 이름, iOS 26.2 runtime과 app hash를 저장한다. 실기기 실행은 없다. |
| 30 | PR screenshot link 오류 또는 증거가 지원 범위를 과장 | PR 본문을 Agent Browser에서 열어 두 이미지가 완전히 로드됐고 각각 1080×2400/1206×2622임을 확인했다. README/PR은 `drawPresented`와 simulator evidence로 한정하고 texel paint·compositor·WPT·C11.4 완료를 주장하지 않는다. |

PR diff 확인 시 `git diff --check`, formatter, focused tests, fixture boundary, Clippy와 양 플랫폼 빌드 결과가 통과했다. GitHub는 필수 status check를 설정하지 않았고 PR #425는 `mergeStateStatus=CLEAN`이다. 저장소는 linear history를 요구하므로 rebase merge가 맞다. 이 검토 파일은 생성 뒤 링크·목차·표 개수와 whitespace를 별도로 다시 확인한다.
