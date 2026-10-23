# C11.4 양 축 auto-repeat + template areas 구현 PR 변경사항 검토 30개

구현 PR의 staged diff와 연결된 Chrome/AVD/iOS evidence를 대상으로 확인했다. 각 항목은 별도 변경 영향, 실패 경로 또는 계약 경계다. 검토에서 새 코드 결함은 찾지 못했다. 아래 27번의 산출물 경계와 문서에 명시한 다른 미검증 범위는 통과 증거가 아니라 남은 제한이다.

| # | 검토 관점 | 판정 |
| --- | --- | --- |
| 1 | 공식 C11.4 전체 완료로 잘못 표시하는가 | `spec/STATUS.md` checkbox와 `77/213`은 유지한다. runtime slice만 기록한다. |
| 2 | 내부 계약 수치 버전을 기능 추가만으로 올리는가 | `0059`의 `0.1.0`을 유지한다. 새 공개 API를 추가하지 않는다. |
| 3 | cascade가 잘못된 runtime profile에 양축 repeat+areas를 허용하는가 | track-sizing profile만 허용하며 TemplateV1 rejection 회귀를 유지한다. |
| 4 | style projection이 profile gate와 area payload를 다르게 처리하는가 | target profile은 양축 metadata와 area bounds를 투영하고 TemplateV1은 거부한다. |
| 5 | layout validator의 기존 one-axis 조건이 양축 입력을 거부하는가 | XOR를 “at least one axis”로 바꿨고 양축 acceptance 및 full layout frame 회귀가 있다. |
| 6 | 기존 single-axis auto-repeat+areas 행렬이 회귀하는가 | 13개 single-axis scene을 dual-axis 12개 및 신규 9개와 함께 재실행했다. |
| 7 | areas와 repeat가 모두 없는 grid에서 metadata가 잘못 허용되는가 | validator는 no-repeat area 입력을 거부한다. |
| 8 | repeat 없는 쪽의 track list와 area dimensions가 어긋나는가 | authored track 개수 일치 조건을 보존하고 mismatch 거부 회귀가 있다. |
| 9 | area dimension이 0 또는 grid track 상한을 넘는가 | zero와 최대 초과 입력을 거부하며 10,000 track 경계를 문서화했다. |
| 10 | area 이름이 빈 문자열, NUL 또는 중복으로 alias를 충돌시키는가 | 세 경우 모두 layout validation에서 거부한다. |
| 11 | 영역 bounds가 빈 범위거나 template 외부로 나가는가 | row와 column의 start/end가 half-open 범위 안인지 검사한다. |
| 12 | metadata count/byte 산술이 overflow하거나 예약 실패 뒤 계속되는가 | checked arithmetic, 1 MiB 제한, fallible reserve 오류 경로를 유지한다. |
| 13 | columns와 rows auto-repeat metadata가 서로 뒤바뀌거나 교차 검증되는가 | 두 축의 repeat range 및 line metadata를 별도 validator로 검사한다. |
| 14 | area 확장 후 named area placement line에 off-by-one이 생기는가 | 3×3 확장 Rust 회귀와 runtime `area-added-both`의 `(44,40,8,8)`을 대조했다. |
| 15 | 추가된 explicit track이 예상 auto-size 대신 기본값을 쓰는가 | 고정 fixture에서 16px columns 및 12px rows를 사용하고 AVD/iOS track 결과와 일치한다. |
| 16 | area가 repeat 수보다 작아도 authored repeat tracks가 사라지는가 | `area-short`는 4 columns와 3 rows를 유지한다. |
| 17 | columns만 확장할 때 row track, item frame 또는 scroll width가 바뀌는가 | `area-added-columns`: columns `[20,16,16]`, rows `[20,20,20]`, item x=44, scroll width=52. |
| 18 | rows만 확장할 때 column track, item frame 또는 scroll height가 바뀌는가 | `area-added-rows`: columns `[20,20,20]`, rows `[20,12,12]`, item y=40, scroll height=48. |
| 19 | 양축 확장 item이 root 밖에서 clip되거나 overflow extent가 잘리는가 | 40×40 root, `(44,40)` item과 52×48 scroll extent를 두 simulator에서 확인했다. |
| 20 | auto-fill 결과가 area dimension과 혼합되어 collapse되는가 | single-axis controls와 dual-axis `area-short`의 non-collapsed repeat tracks를 비교했다. |
| 21 | fill/fit mode가 한 축의 설정을 다른 축에 전파하는가 | `fill-fit`과 `fit-fill`에서 axis별 track/collapse 배열과 item frame이 각각 다르게 기대값과 일치한다. |
| 22 | item이 없는 area가 auto-fit 공간을 남기거나 빈 item serialization을 깨뜨리는가 | `fit-fit-empty`는 0 item, 양축 0 used size 및 100×76 scroll을 보이고 root-only frame 문자열을 파싱한다. |
| 23 | spanning item이 실제 차지하는 auto-fit repeat tracks까지 collapse되는가 | `fit-fit-span`은 44×44 item과 occupied 2×2 tracks를 보존하고 unused tracks만 collapse한다. |
| 24 | grid-area/column/row longhand 생성이 `grid-grid-*` 오류나 camelCase 누락을 만드는가 | generator를 분기했고 Rust source 회귀와 실제 `area-longhand` scene을 실행했다. |
| 25 | 신규 scene ID·허용 목록이 Rust, Node, Android, iOS 사이에 어긋나는가 | 22 template-area fixture scenes와 최종 matrix의 34 ID set을 비교하고 scene boundary 검사 7/7을 통과했다. |
| 26 | scene fixture 코드나 payload가 기본 제품 산출물에 연결되는가 | module은 `c04-runtime-gpu-fixture-test` gate 아래 있다. 다만 current-source default/product archive와 최종 APK/IPA 제외는 확인하지 못해 이 위험은 미해결 검증 한계로 남겼다. |
| 27 | 화면 sample이 기대 색상/좌표와 맞지 않거나 화면 proof를 과장하는가 | Android PixelCopy 및 iOS 제출 전 Metal drawable이 4/4 exact다. border stroke, Android hardware GPU와 iOS compositor 이후 화면은 주장하지 않는다. |
| 28 | 캡처 harness가 node frame·track trace·draw/present 실패를 누락하는가 | Android와 iOS 최종 result JSON은 각각 34 scene, 34 `drawPresented=true`이며 scene별 frame·tracks·trace chunk를 보존한다. |
| 29 | PR screenshots의 이미지 경로가 branch와 증거 파일 위치를 가리키는가 | Agent Browser로 PR #545를 열어 두 이미지가 모두 `complete=true`로 렌더링되고 각각 1080×2400, 1206×2622 natural pixels를 반환함을 확인했다. 지정 session을 닫고 fresh session/process scan에서 실행 항목이 없는 것도 확인했다. [검증 JSON](agent-browser-pr-545-images.json). |
| 30 | plan, conformance, contract, STATUS가 서로 다른 완료 범위를 선언하는가 | 네 문서와 runtime README가 동일한 제한 slice, `0.1.0`, unchecked C11.4 및 `77/213`을 기록한다. |

## 판정

구현과 simulator runtime 검토 및 GitHub 이미지 렌더링 확인은 통과했다. 26번의 current-source product archive negative control과 최종 APK/IPA exclusion은 여전히 증명이 필요하다. WPT, 실기기·hardware GPU, iOS compositor 이후 pixels, border stroke와 전체 C11.4 지원은 PR 범위 밖으로 남는다. 이 파일은 독립 reviewer 수를 뜻하지 않으며, 30개의 개별 관점과 그 판정 근거를 기록한다.
