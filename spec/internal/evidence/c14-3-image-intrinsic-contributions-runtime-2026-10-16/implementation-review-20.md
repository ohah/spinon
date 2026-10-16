# C14.3 구현 실패 관점 20개 — 2026-10-16

검토 대상은 변경된 layout solver, Stylo projection, CSS fixture 테스트, Android/iOS opt-in adapter와 빌드 경계다. 각 항목은 별도 실패 원인과 근거를 확인했다. 발견한 문제는 수정하고 영향 경로를 다시 검사했다.

| # | 실패 관점 | 확인 결과와 조치 |
|---:|---|---|
| 1 | Chrome fixture의 실제 브라우저·버전·viewport가 다른 값으로 오염됨 | Chrome 155 `155.0.8059.85`, DPR 1, viewport `1280×633`을 observation과 reference JSON에 묶었다. |
| 2 | 이미지 파일이 바뀌어 intrinsic 크기 oracle이 다른 바이트를 측정함 | PNG SHA-256을 reference와 app adapter에서 검사한다. 값은 `5d8c9509…398bce4a2`다. |
| 3 | SVG `image` 또는 임의 `img` 이름이 HTML replaced element로 처리됨 | HTML namespace와 local name `img`를 모두 확인한다. 다른 노드의 intrinsic keyword는 명시 오류다. |
| 4 | detached node나 다른 문서 generation의 intrinsic metadata가 layout에 유입됨 | root snapshot 순회와 `LayoutSourceRevision` 비교로 연결·generation/document/render revision을 고정한다. |
| 5 | 자식이 있는 host `img`를 leaf처럼 sizing해 descendants를 누락함 | visible intrinsic-size `img`의 자식을 검사하고 `UnsupportedIntrinsicSizing`으로 거부한다. non-leaf negative test가 추가됐다. |
| 6 | required image metadata가 빠졌는데 0·auto·기본 object size로 조용히 대체됨 | visible `auto`/intrinsic keyword image에 snapshot이 없으면 `MissingIntrinsicSize`를 반환한다. |
| 7 | 입력 snapshot은 현재 문서지만 revision만 오래된 경우를 허용함 | source와 intrinsic revision을 별도로 비교하고 stale/source mismatch를 보존한다. 기존 C14.1 regression도 workspace에서 통과한다. |
| 8 | border-box intrinsic contribution에서 padding/border를 natural content에서 빼 버림 | Chrome 결과는 `34×26`; 첫 구현의 edge subtraction 결함을 수정해 natural content `24×16`에 edge를 한 번 더한다. oracle·AVD·Simulator에서 재확인했다. |
| 9 | explicit border-box width에 padding/border edge를 두 번 적용함 | explicit width/height는 `dimension_to_content` 경로에서만 edge를 뺀다. intrinsic keyword contribution은 이 조정을 건너뛴다. |
| 10 | authored `aspect-ratio`와 natural ratio가 서로 덮어쓰거나 축 변환이 중복됨 | authored ratio 우선, 그 외 natural ratio 순서를 유지한다. width/height 각 축과 flex/grid cases를 비교했다. |
| 11 | percentage width와 height가 같은 containing-block 축을 사용함 | width는 content width, height는 content height를 기준으로 한다. block·flex·fixed-grid에서 분리된 oracle case를 비교했다. |
| 12 | indefinite percentage가 0/natural fallback으로 잘못 해석됨 | 제한 cyclic min-content 외에 basis가 없는 percentage는 식별 가능한 unsupported error로 거부한다. |
| 13 | 모든 `min-content` parent percentage를 0으로 바꿔 다른 layout context까지 지원한다고 오인함 | parent와 전체 조상이 block이어야 하고 min/max constraints, aspect ratio, padding, border가 없어야 한다. flex ancestor와 `min-width` negative control은 거부한다. |
| 14 | cyclic percentage가 natural `24px` 또는 최종 definite basis를 min-content pass에 누출함 | 고정 Chrome cyclic control은 parent와 image 모두 `0×0`; solver와 simulator report에서 같은 결과를 검사한다. |
| 15 | Flex의 기본 stretch가 image의 명시 intrinsic size를 다시 늘림 | adapter가 replaced item을 start 정렬로 고정한다. `align-items:start` 파싱은 Template profile에 한정했다. |
| 16 | Grid의 intrinsic track sizing, spanning contribution 또는 implicit track까지 허용됨 | C14.3의 Grid route는 definite fixed track만 받고 나머지 track profile은 열지 않는다. |
| 17 | `fit-content()` 함수 또는 일반 block의 `min-content`가 `auto`/0으로 침묵 변환됨 | `fit-content(<length-percentage>)`와 허가되지 않은 element keyword는 node별 `UnsupportedIntrinsicSizing`으로 닫는다. |
| 18 | `min/max` clamp가 intrinsic contribution 뒤 빠지거나 box-sizing edge를 중복함 | `max-width:12px` control은 `12×8`; existing replaced ratio constraint tests와 신규 oracle cases를 통과한다. |
| 19 | fixture C ABI·문자열이 제품 기본 artifact에 포함되거나 Android/iOS flag가 어긋남 | Cargo·C header·JNI/Objective-C++·Java/Swift gates를 확인했다. 두 기본 artifact에서 symbol/payload 부재, opt-in positive control에서 존재를 검사했다. |
| 20 | app case count, report chunking 또는 화면이 14-case 성공을 일부만 표시함 | Java/Swift 14-case gate와 로그 chunk 수를 맞췄다. 초기 앱 gate의 13-case 회귀를 고쳤고 두 Simulator report는 14/14다. 화면은 frame geometry만 말하며 image pixel paint/clip은 주장하지 않는다. |

## 검토 중 발견해 수정한 문제

- image 자신의 `border-box min-content`가 Chrome의 `34×26` 대신 edge를 빼는 문제를 oracle 대조로 찾고 수정했다.
- oracle을 14개로 늘린 뒤 Android/iOS 앱 gate가 13개만 기대하던 문제를 수정했다.
- `align-items:start`를 모든 Grid runtime profile에 허용해 C11.2.1 경계를 깬 문제를 workspace regression으로 찾았다. Template profile 한정으로 scope를 좁히고 fallback parser의 무조건 `start` 허용도 제거했다.
- cyclic percentage guard가 Flex 조상·min-width 변형을 구분하지 않던 점과 비-leaf image 입력을 수용하던 점을 추가 negative test로 고쳤다.
- Clippy의 함수 인자 과다와 Option early-return 경고를 입력 구조체/`?`로 정리했다.

최종 코드로 focused style-to-layout 187 tests, 전체 workspace test, Clippy, 두 Simulator opt-in/default route 및 fixture-module boundary 검사를 다시 실행했다. 최종 상태는 인접 [실행 근거](README.md)의 로그를 따른다.
