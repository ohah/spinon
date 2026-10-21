# C11.4 depth-three nested Grid PR diff 실패 관점 검토 30개

이 검토는 이 변경의 구현 코드, Android/iOS fixture validator, 캡처 도구, 계획·상태·runtime 근거를 각각 다른 경계에서 확인한다. 구현/runtime 실패 관점 20개와 계획 PR 검토는 별도 문서다. C11.4는 고정 fixture slice이며 제품 지원 완료로 승격하지 않는다.

| # | 독립 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | 새 ID allowlist가 임의 depth의 입구가 되는가? | Runtime와 FFI `SCENES`는 기존 8개 control과 신규 depth-three 2개, 총 10개만 고정한다. 다른 ID는 source adapter에서 `None`이다. 통과. |
| 2 | depth 값이 이름 역할과 어긋나는가? | FFI generator와 Runtime adapter 모두 depth 1/2/3을 각기 명시한 role sequence로 변환한다. 미등록 depth는 받아들이지 않는다. 통과. |
| 3 | depth-four 이름이 root ID만 바꿔 들어오는가? | FFI unknown ID negative check와 Runtime unlisted depth-four mutation이 거부를 확인한다. 통과. |
| 4 | Grid가 문서 root 직속이 아니어도 같은 frame으로 오인되는가? | root의 HostRoot parent, `div` identity, `data-node` 및 child chain을 Runtime 준비 단계에서 확인한다. 통과. |
| 5 | 생성된 `nodeIds` 순서와 DOM tree 순서가 다를 수 있는가? | FFI는 root→중간 Grid→inner→text 순으로 배열을 만들고, 각 다음 Grid를 바로 전 Grid의 child로 연결한다. 통과. |
| 6 | depth-three `branch`가 `middle` 또는 `inner`로 잘못 인식되는가? | role mapping은 depth 3의 두 번째 중간 노드를 `branch`로 제한한다. wrong-branch mutation이 거부된다. 통과. |
| 7 | fixture 시각 표식이 기존 계층 색을 바꾸거나 제품 palette로 오인되는가? | 배경색은 opt-in diagnostic source 안에서만 root/middle/branch/inner 식별자로 생성된다. 제품 style admission을 변경하지 않는다. 통과. |
| 8 | 각 Grid의 width/height가 track keyword와 다르게 입력되는가? | 제한 adapter가 해당 scene의 모든 Grid에서 block size와 두 단일 track 축의 min/max keyword를 검사한다. 통과. |
| 9 | 기존 depth-one·depth-two 한 축 scene의 auto sizing이 바뀌는가? | scene별 `two_axis` 설정을 유지해 기존 8개 control은 기존 height/row/column 조건을 그대로 사용한다. 양 플랫폼에서 함께 10/10 통과했다. 통과. |
| 10 | 두 축 scene에서 Rows track이 Columns 값으로 검증되는가? | 기대값은 별도 axis map으로 Columns `[70]`/`[100]`, Rows `[20]`/`[10]`을 기록하고 Simulator verifier가 각 축을 따로 비교한다. 통과. |
| 11 | track 외 margin·padding·border·gap이 intrinsic contribution에 섞이는가? | fixture는 해당 edges와 gaps 0, align-start를 고정하며 Runtime adapter는 그 외 값을 거부한다. 통과. |
| 12 | 중간 Grid가 추가 child나 reordered child를 받아 부분 layout을 내는가? | 각 단계의 직계 자식 ID·역할·개수를 제한하고, root의 추가 직계 자식 mutation은 layout 오류로 끝난다. 통과. |
| 13 | leaf가 inner Grid의 직접 text item이 아닌데 같은 intrinsic 값을 쓰는가? | Runtime adapter가 inner Grid의 유일한 child가 정확한 HTML `div`·ID인 것을 확인하고 text item 소유 Grid와 node ID를 검사한다. 통과. |
| 14 | 직접 text 값 또는 줄바꿈 입력이 달라져도 기대값이 유지되는가? | source generator와 재현 fixture는 `XX XXXXXXX`, collapse wrapping, normal word-break 및 10px line-height를 고정한다. 통과. |
| 15 | Ahem fallback/font registry revision이 플랫폼마다 달라질 수 있는가? | simulator validator는 intrinsic input의 Ahem source와 font revision 1을 검사하고, artifact README가 기기·빌드 조건을 구분한다. 통과. |
| 16 | min-content contribution이 세 중첩 경계 중 하나에서 끊기는가? | Rust layout trace와 양 플랫폼 결과가 네 Grid 및 leaf frame `70×20px`, Columns `[70]`, Rows `[20]`을 확인한다. 통과. |
| 17 | max-content의 공백 포함 폭이 상위에 전파되지 않는가? | Rust trace와 양 플랫폼 결과가 네 Grid 및 leaf frame `100×10px`, Columns `[100]`, Rows `[10]`을 확인한다. 통과. |
| 18 | trace 한 개 누락 또는 중복 pass가 최종 frame만으로 통과하는가? | Runtime assertions와 capture verifier는 네 Grid의 각 track-sizing trace, source order, span 및 contribution kind/value를 확인한다. 통과. |
| 19 | frame 검증이 root만 보거나 depth-three node 5를 빼먹는가? | Java·Swift validators는 box count 5와 node 1–5의 frame 문자열을 각각 고정 비교한다. 통과. |
| 20 | 화면 present gate를 계산 성공으로 잘못 대체하는가? | Android AVD와 iOS Simulator의 10 scene 각각에서 `drawPresented=true`를 기록한다. 실제 GPU pixel equality는 범위 밖이라고 문서에 썼다. 통과. |
| 21 | 잘못된 intrinsic keyword 한 단계가 정상 scene으로 통과하는가? | 세 번째 Grid의 keyword를 반대로 바꾼 fixture mutation이 fail-closed 검증을 통과했다. 통과. |
| 22 | child identity만 바꾼 malformed tree가 canonical role로 정규화되는가? | branch ID mismatch mutation이 예상 branch identity와 다르면 오류를 반환한다. 통과. |
| 23 | capture scene 목록과 app/runtime allowlist 중 한쪽만 갱신되는가? | CLI 목록은 총 194개이며 신규 두 ID를 노출한다. AVD·iOS 캡처와 양 앱 allowlist는 모두 두 ID를 포함한다. 통과. |
| 24 | 캡처 validator가 새 scene trace 대신 첫 trace를 잘못 사용해도 통과하는가? | nested depth map은 root Grid ID, 전체 Grid node list, leaf node, axis tracks, expected frames와 intrinsic input을 지정한다. 트레이스 부재·개수 불일치를 거부한다. 통과. |
| 25 | 기존 depth-two verifier 이름 변경 중 회귀가 남는가? | capture script에서 `nestedGridDepthTwo` 참조를 모두 일반 `nestedGridDepth`로 바꾸고 stale identifier 검색 결과가 없다. fixture-module boundary 6/6도 통과했다. 통과. |
| 26 | source 변경과 결과 artifact가 서로 다른 revision인데 같은 실행으로 보이는가? | Android/iOS manifest가 commit, base tree, tracked diff SHA-256과 변경 path를 기록한다. README는 실행 APK/app SHA-256을 별도로 연결한다. 통과. |
| 27 | source revision 추가가 capture manifest 파서를 깨뜨리거나 모든 run에 신규 scene을 넣는가? | revision은 JSON의 추가 field이고 scene selection은 요청 ID allowlist와 별개다. `--list-scenes` 및 선택 run 결과를 확인했다. 통과. |
| 28 | fixture FFI가 기본 제품 archive에 실수로 포함되는가? | 기본 archive에 신규 ID가 없고 opt-in fixture-test archive에는 포함되는 positive control을 확인했다. 통과. |
| 29 | 로컬 APK만 새 scene을 담고 설치 APK는 이전 artifact인가? | Android API 37 AVD에서 local/installed `base.apk` SHA-256이 모두 `49b4455c…379e7ba`다. 이전 8-scene APK 시도는 성공 증거에서 제외하고 재빌드 실행만 인정했다. 통과. |
| 30 | 문서가 한 slice의 결과를 전체 CSS Grid 또는 완료 수로 확대하는가? | plan·STATUS·runtime README는 C11.4 unchecked와 `72/213`을 유지하고 0059 `0.1.0` 및 공개 API를 바꾸지 않는다. Android 실기기, WPT, glyph paint, iOS compositor 이후 pixel과 전체 Grid는 미검증으로 남긴다. 통과. |

## 판정

추적 source 및 문서 diff에서 별도의 merge 차단 결함을 찾지 못했다. 집중 Rust/FFI tests, workspace 및 fixture Clippy, formatter, JavaScript boundary test, Android/iOS fixture build와 10-scene Simulator 실행은 [runtime 근거](./README.md)에 기록했다. PR 본문 화면 링크는 병합 전 Agent Browser에서 실제 표시를 확인한다.
