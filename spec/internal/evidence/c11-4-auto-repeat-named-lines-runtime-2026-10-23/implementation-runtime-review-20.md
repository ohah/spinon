# 구현/runtime 독립 실패 관점 검토 20개

대상은 auto-repeat named line bridge, 제한 profile guard, simulator fixture adapter와 최종 capture다. 각 관점은 코드 경로와 고정 scene/test를 별도로 대조했다. 발견 사항은 수정 후 표의 회귀 검사와 두 플랫폼 final capture에서 다시 확인했다.

| # | 실패 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | 확장 후 positive occurrence가 authored pattern count에서 풀려 index가 밀림 | `positive` scene은 `(50,0) 40×40`, columns `[40,40,40]`; 두 simulator frame과 track trace가 일치한다. |
| 2 | bare named placement의 생략 occurrence가 0 또는 첫 authored line이 아닌 곳을 선택 | `bare` scene은 `(0,0) 40×40`; 구현은 symbolic placement를 Taffy에 전달한다. |
| 3 | negative occurrence가 repeat 이전의 authored line까지만 역산 | `negative` scene은 `(100,0) 40×40`; 세 expanded repeat에서 끝 기준 위치를 양쪽 runtime이 맞췄다. |
| 4 | named span이 다음 반복 fragment를 건너지 못하거나 한 track 덜 확장 | `span` scene은 `[40,40,40]` tracks와 `90×40` item을 기록한다. |
| 5 | `auto-fit` collapse 이후 이름 있는 collapsed line을 삭제 | `fit`은 `[0*,40,0*]`, item `(0,0) 40×40`; collapse flags와 표면 표본이 일치한다. |
| 6 | fixed prefix/suffix가 repeat placement line ordinal을 바꿈 | `neighbors`는 `[20,40,30,40,30,10]`, item x=120; prefix/suffix와 two-track pattern을 함께 검사한다. |
| 7 | 반복 끝과 다음 fragment 시작이 같은 물리 line인데 이름을 두 occurrence로 중복 계수 | `merge`의 `[10,20,20,20,10]` 및 item x=20을 확인했고 기존 named-line projection tests도 경계 중복을 다룬다. |
| 8 | 반복 경계의 서로 다른 이름이 merge 과정에서 유실 | cascade와 projection은 outer와 repeated line groups를 분리 보존하고 Taffy track repetition에 전달한다. `merge` scene이 배치 결과를 확인한다. |
| 9 | row auto-repeat만 column helper를 사용하거나 y 좌표를 x로 적용 | `row` scene은 rows `[40,40,40]`, item y=50, 크기 `100×40`을 두 runtime에서 확인한다. |
| 10 | `auto-fill`을 `auto-fit`으로 잘못 정규화하거나 mode별 track trace가 합쳐짐 | scene JSON에서 mode를 고정하고 `fit`의 collapse trace와 나머지 fill scene tracks를 별도 비교한다. |
| 11 | outer/inner line-group 개수가 tracks+1 규칙에서 어긋남 | cascade는 list와 repeat `N+1` shape를 검사하고 fixed cascade tests가 외부/내부 이름을 각각 비교한다. |
| 12 | area 또는 지원하지 않는 repeat shape가 일부 metadata와 함께 통과 | negative cascade cases는 areas, 복수/혼합 repeat, 비-`px` neighbor 등 미지원 shape를 거부한다. |
| 13 | 없는 이름이 조용히 implicit line을 생성하거나 숫자 line으로 변환 | 제한 profile은 authored metadata에 없는 이름을 placement validation에서 거부한다. implicit fallback은 범위 밖으로 남았다. |
| 14 | named placement 지원이 repeat 없는 축의 기존 numeric placement를 바꿈 | 일반 축은 기존 `resolve_axis` 경로를 사용하고 layout/style-to-layout 회귀 suite가 통과했다. |
| 15 | `String` clone/그룹 vector 예약 실패가 panic 또는 부분 snapshot으로 이어짐 | tracks, line groups, names, name text 각각 failpoint에서 `GridAllocationFailure`의 node/property를 확인하고 후속 호출 복구를 검사한다. |
| 16 | auto-repeat track allocation이 rows 경로에서 먼저 실패해 columns context를 잃음 | 테스트에서 failpoint 소비 순서가 이를 드러냈다. auto-repeat reserve에 `Tracks` failpoint을 연결했고 네 지점 모두 columns property로 닫힌다. |
| 17 | size arithmetic overflow 또는 과도한 이름 metadata가 비bounded allocation으로 넘어감 | checked count/metadata-byte budget 및 최대 track 수 guard를 확인했다. 기존 budget/track limit tests가 계속 통과한다. |
| 18 | normalized placement를 Taffy용 owned value로 복사하는 중 이름 identity/order가 변함 | named `GridItemInput`의 four placement fields를 property별 fallible clone하고, positive/negative/span scenes와 named-line tests가 결과를 검증한다. |
| 19 | app scene matcher가 새 scene을 잘못된 fixture로 보내거나 pixel sample id를 누락 | Android allowlist와 expected RGB mapping을 여덟 ID로 확장했다. 최종 AVD run은 scene 8/8, PixelCopy 29/29 exact다. |
| 20 | 화면 결과가 layout log와 다르거나 iOS 표본 기능이 꺼진 artifact를 통과로 오인 | final run source diff hash를 양 platform `simulator.json`에서 같게 확인했다. iOS Metal 표본 29/29 exact 및 `drawPresented=true` 8/8; 최초 feature-off run은 통과 근거에서 제외했다. |

## 수정한 검토 발견

- Android expected RGB map과 scene allowlist가 새 sample ID를 인식하지 못했다. 두 지점을 보완하고 최신 APK로 8 scene을 다시 캡처했다.
- iOS Metal 표본은 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1` 없이 비활성화됐다. 빌드 feature를 켜고 최신 앱으로 8 scene을 다시 캡처했다.
- Workspace Clippy가 `dense_column.rs`의 두 축 `Option::expect()`와 Taffy 0.15 iterator callback을 지적했다. 축 Option은 pattern match로 바꾸고 callback의 Taffy 시그니처 제약은 국소 allow와 설명으로 남겼다. workspace `-D warnings`가 통과했다.
- 새 auto-repeat allocation test에서 `Tracks` failpoint이 auto-repeat reserve에 연결되어 있지 않은 사실이 드러났다. failpoint을 추가하고 네 예약 지점 및 recoverability를 다시 통과시켰다.

## 실행 경계

이 검토는 구현 failure paths와 두 simulator fixture만 다룬다. WPT, 실기기, Android hardware GPU, iOS compositor 이후 display pixels, 일반 HTML/CSS admission 또는 전체 C11.4를 증명하지 않는다. 상세 runtime 로그·hash·screenshots는 [실행 근거](./README.md)에 둔다.
