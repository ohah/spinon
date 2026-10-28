# C11.4 hidden item 구현 PR diff 적대 검토 30개

계획 검토·구현 runtime 20관점과 별도로, 이 PR 변경 연결성, 플랫폼 parity, 증거 lineage 및 문서 범위를 확인했다. 각 행은 다른 file boundary 또는 제출 회귀를 대상으로 한다.

| # | 별도 실패 가설 | 확인과 결과 |
| ---: | --- | --- |
| 1 | FFI의 새 하위 모듈이 임의 경로에 놓이거나 fixture 경계를 벗어난다 | canonical `runtime_gpu/fixtures/c11_4/generic_intrinsic_flex_fixed/hidden.rs`를 부모 fixture 모듈에서만 연다. 기본 artifact 검사에 hidden payload가 없다. |
| 2 | FFI 등록이 기존 8개 scene 매칭을 바꾼다 | 기존 ID 검색을 우선 유지하고 그 밖에만 hidden matcher로 위임한다. 기존 8개가 두 simulator에서 모두 유지됐다. |
| 3 | FFI에서 이름이 비슷한 unsupported scene이 hidden source를 받는다 | hidden helper는 두 문자열만 매칭한다. FFI unit test는 unknown/visible-control 변형 세 개를 거부한다. |
| 4 | runtime adapter가 root ID가 아닌 전체 subtree를 재귀적으로 지원한다 | root ID에서 axis를 확정하고 direct child 4개의 고정 schema만 처리한다. unsupported subtree를 확장하는 fallback은 없다. |
| 5 | child tag·namespace·순서·ID·parent의 일부 검사가 빠진다 | 각 element에 `validate_element`를 호출하고 exact children count와 hidden/A/image/C 순서를 확인한다. |
| 6 | hidden text 길이 또는 text node 수가 달라져도 같은 결과로 처리한다 | direct text equality를 15개의 `X`로 고정한다. nested node와 복수 children은 helper의 direct-text contract가 거부한다. |
| 7 | CSS computed `display`와 `visibility` 입력이 style projection에 연결되지 않는다 | C11.4 fixture projection에 `visibility`와 4 Grid line longhand를 등록하고 hidden matcher의 exact computed 값으로 검증한다. |
| 8 | `visibility:hidden`을 box-generation 차이로 오해한다 | hidden style은 `display:none` + `visibility:visible`만 허용하며 변형 negative test가 있다. |
| 9 | hidden item의 orthogonal writing mode 검사가 전역 완화로 바뀐다 | projection은 `display:none` node/ancestor만 computed vertical set에서 제외한다. root와 visible paragraph의 expected/computed 대조는 그대로 둔다. |
| 10 | projection 변경이 host snapshot node를 안전하지 않게 dereference한다 | 각 handle은 `snapshot.node`를 통해 확인하고 불일치는 `SnapshotMismatch`로 반환한다. hidden ancestry 탐색은 snapshot child iterator만 사용한다. |
| 11 | hidden paragraph가 adapter의 measurement map에 남는다 | hidden prepare는 A/C 두 visible paragraph만 생성한다. test는 hidden key의 부재와 A/C key의 존재를 확인한다. |
| 12 | PNG image 준비가 hidden path에서 우회된다 | 기존 URL, image request/decode, snapshot 생성 및 24×16 natural-size 확인을 그대로 재사용한다. 관련 오류 tests가 유지됐다. |
| 13 | scene 축·definite 크기·gap·track style의 변경을 받아들인다 | 공통 root/track validator와 hidden expected data가 Columns/Rows 각 치수·gap·track vector를 서로 독립적으로 고정한다. |
| 14 | mixed fixture trace에 hidden contribution이 들어가도 capture 검증을 통과한다 | JS와 Python 검증기는 두 axis pass 모두에서 hidden node ID contribution을 거부한다. negative control로 mutation을 실제 거부했다. |
| 15 | hidden node를 없앤 뒤 source order를 0,1,2로 재번호 매긴다 | runtime test와 양 플랫폼 matcher는 A/image/C 원래 순서 1,2,3을 확인한다. 현재 20 scene manifest가 일치한다. |
| 16 | NodeId가 hidden node 삭제 때문에 visible sibling에서 바뀌어 matcher가 혼동한다 | Android/iOS frame ID map과 intrinsic ID map은 `1,2,4,6,7` 및 `4,6,7`을 각각 고정한다. capture frame logs와 대조했다. |
| 17 | hidden 0×0 bookkeeping frame을 visible frame 또는 draw box로 센다 | 캡처 data model은 `boxCount=4`, `frameCount=5`, `drawBoxCount=4`를 명시하고, native matcher의 expected frame은 hidden zero rectangle을 포함한다. |
| 18 | Java와 Swift가 track base나 frame geometry에서 서로 다른 기대값을 사용한다 | hidden pair는 두 언어에서 동일한 `[100,40,140]` 및 root-relative frame arrays다. 각 플랫폼 10/10 capture가 비교를 통과했다. |
| 19 | Swift multiline `case` labels가 matcher table parser에서 누락된다 | verifier가 Swift multiline case labels를 모아 비교한다. self-test의 플랫폼 matcher table comparison이 통과했다. |
| 20 | C0410 app route 중 한 플랫폼에만 scene selector가 추가된다 | Android Java와 iOS Swift scene ID sets 양쪽에 두 hidden IDs가 추가됐고 manifest target은 각각 Android와 iOS로 판정됐다. |
| 21 | invalid scene/profile 요청이 앱의 기존 다른 fixture로 fallback한다 | unknown scene, visible control, conflicting C11.2.1 selector 세 요청은 양쪽 manifest에서 reject됐다. |
| 22 | hidden expected geometry가 capture script 내 독립 상수로 중복돼 oracle 변경과 어긋난다 | hidden scene geometry는 precomparison `expected.json`에서 불러온다. 독립 산술·Chrome observation은 별도 자료에 유지된다. |
| 23 | 새 source file이 manifest input hash에 빠져 source provenance가 불완전하다 | FFI hidden module, runtime hidden adapter, runtime style projection과 expected JSON을 source input path set에 넣었다. current run manifests의 dirty path list와 hash를 확인했다. |
| 24 | frame record와 visible box count가 같은 수라고 verifier가 가정한다 | JS와 Python 모두 hidden scene의 frame count 5와 visible boxes 4를 별도 보고한다. 실제 Android/iOS run JSON이 각각 같은 결과를 낸다. |
| 25 | image/layout trace chunk가 잘려도 parser가 성공으로 처리한다 | verifier는 run ID·scene ID별 chunk reassembly와 exact stage/track data를 요구한다. self-test에서 누락 stage와 비유한 값 변형을 거부한다. |
| 26 | capture가 drawn/presented 증거 대신 layout-ready만 성공 처리한다 | capture 중단 조건 및 verifier는 expected draw count/presentation marker를 요구한다. 두 플랫폼 screenshot hash와 presentation marker가 통과했다. |
| 27 | screenshot file이 누락·변조·다른 크기여도 matrix 통과한다 | verifier는 manifest의 screenshot path, dimensions, SHA-256을 검사한다. 현재 run은 Android 1080×2400, iOS 1206×2622다. |
| 28 | 예상하지 않은 runtime record가 frame list 끝에 붙어도 승인된다 | ordered expected frames와 exact count가 비교된다. hidden scene은 정확히 5 records와 4 visible boxes로 통과했다. |
| 29 | dev build에서만 fixture exclusion/inclusion을 주장한다 | release static archive를 두 feature 구성으로 빌드했다. 기본에는 C11.4 new/eval symbol과 marker가 없고 positive control에는 symbol과 marker 10개가 있다. |
| 30 | 제한 fixture 실행을 전체 C11.4 공식 완료나 full visual parity로 과장한다 | README와 artifact 문서는 simulator-only, static archive-only와 미검증 경계를 명시한다. `spec/STATUS.md`, 상위 checkbox와 `72/213`은 이 PR에서 바꾸지 않는다. |

**판정:** 현재 diff에서 구현을 막는 미해결 결함은 발견하지 못했다. 발견된 feature Clippy 경고는 수정 후 기본·fixture Clippy에서 재검사했다. 실제 두-platform run, static archive positive/negative boundary와 미검증 경계는 [근거 README](README.md), [구현·런타임 20관점 검토](hidden-item-implementation-runtime-review-20.md), [artifact boundary](artifact-boundary.md)에 연결했다.
