# C11.4 max-content span-group PR diff 적대 검토 30개

**검토 대상:** runtime/FFI fixture, 고정 oracle, Android·iOS 검증기, capture runner, artifact 경계와 STATUS·명세·계획·근거 문서. 별도 [구현·런타임 failure review 20개](./implementation-runtime-review-20.md)와 겹치지 않도록 파일 간 계약, 배포 범위, 회귀 방지 중심으로 살폈다.

| # | 실패 경로·불변식 | diff와 독립 근거 | 판정 |
| --- | --- | --- | --- |
| 1 | runtime과 FFI의 scene ID 목록이 서로 달라진다. | 양쪽 allowlist에 같은 네 ID를 추가했고 각 모듈의 개수·중복 검사가 8개를 확인한다. | 통과 |
| 2 | 역순 scene의 fixture source가 DOM key 순서를 바꿔 버린다. | FFI scene은 A/B/C 및 C/B/A 고정 순서를 생성하고 양방향 source 비교 테스트가 item key를 보존한다. | 통과 |
| 3 | runtime scene parser가 `max-content`를 min-content로 projection한다. | `Width::MaxContent`와 `ComputedCssDimension::MaxContent`를 직접 연결하며 고정 사례가 사용 폭 110px을 확인한다. | 통과 |
| 4 | definite control의 CSS width와 expected geometry가 어긋난다. | FFI source의 `80px`, oracle case root/content 크기, 플랫폼 expected frame이 80/84px을 일관되게 사용한다. | 통과 |
| 5 | 새 rational 기대값이 반올림 상수로 바뀐다. | case 표는 thirds를 rational expression으로 표현하고 플랫폼 frame 비교는 동일 분수 및 1/64px 허용치를 사용한다. | 통과 |
| 6 | `minimumRows` 기본값 변경으로 기존 263개 scene이 바뀐다. | 신규 cyclic 장면에서만 override를 넘기고 나머지는 기존 `intrinsic ? 20 : 0` 기본값을 유지한다. | 통과 |
| 7 | 선택적 `pixelSamples`가 기존 definite 장면의 기본 표본을 제거한다. | 전달된 경우만 override하고 없는 definite 장면은 기존 overflow sample 경로를 유지한다. | 통과 |
| 8 | 새 sample 좌표가 root border가 아니라 item border로 해석된다. | 네 고정 장면은 root ID를 대상으로 113/83 CSS px 경계와 116/86 바깥 점을 기록한다. 플랫폼 검증기도 같은 좌표를 요구한다. | 통과 |
| 9 | Android color expectation이 신규 sample을 sentinel 값으로 처리한다. | 최초 run에서 해당 결함이 드러나 `c114GridExpectedRgb`에 exact ochre border 및 black clear 매핑을 추가했다. 재실행 PixelCopy 8/8 exact다. | 수정 후 통과 |
| 10 | Android sample count가 두 표본 외 입력을 허용한다. | 새 장면 검증기는 정확히 두 sample을 요구하고 둘 중 다른 ID가 오면 실패한다. | 통과 |
| 11 | Android sample은 맞지만 root/node 귀속 검사가 생략된다. | root node ID, sample별 좌표, direction prefix를 묶어 확인하며 알 수 없는 ID는 거부한다. | 통과 |
| 12 | Android expected root size가 scene prefix만으로 충돌한다. | max-content scene 분기를 기존 mixed-span geometry보다 먼저 두고 intrinsic/definite 크기를 별도로 반환한다. | 통과 |
| 13 | iOS scene allowlist에는 있지만 pixel switch에는 없다. | Swift ID set과 Metal pixel scene switch 모두 네 새 ID를 명시하고 scene별 sample 이름을 만든다. | 통과 |
| 14 | iOS Metal pixel compile flag가 빠져 비활성 readback이 성공으로 오인된다. | 첫 실행의 `status=-90`을 기록했고 빌드 시 전용 feature를 켠 재실행에서 모든 표본을 읽었다. | 수정 후 통과 |
| 15 | iOS pixel 결과가 compositor 제출 뒤의 화면 픽셀이라고 과장된다. | 근거와 계획 문서는 표본 경계를 compositor 제출 전 Metal drawable로 한정한다. | 통과 |
| 16 | reverse 플랫폼 geometry 배열이 DOM 순서 기반이 되어 item별 위치가 바뀐다. | Android와 Swift 검증기는 reverse branch에서 기대 node ID와 item별 frame을 짝지으며 실제 결과 JSON도 동일 keyed geometry다. | 통과 |
| 17 | capture runner의 새 ID가 cyclic percentage parser에서 빠진다. | 네 scene ID를 `cyclicPercentSceneIDs`에 넣었고 각 결과 trace가 simulator JSON으로 파싱됐다. | 통과 |
| 18 | global scene count가 네 개 증가한 실제 fixture set과 불일치한다. | capture script count를 263에서 267로 갱신하고 고유 ID 검사도 유지한다. | 통과 |
| 19 | 기존 scene ID가 변경 또는 중복돼 인접 회귀 fixture가 사라진다. | diff에는 네 ID 추가만 있고 기존 값은 유지된다. 최종 capture는 지정한 네 ID를 exact 매치한다. | 통과 |
| 20 | 텍스트 최종 line advance 검사를 Simulator glyph raster 검사로 오인한다. | README는 line advance가 Rust oracle이며 Simulator 검증은 frame/trace/pixel이라고 구분하고 glyph paint 증거가 아니라고 명시한다. | 통과 |
| 21 | Android capture 명령이 새 앱을 빌드한다고 착각해 stale binary를 검증한다. | 캡처 스크립트는 사전 빌드된 앱을 설치한다. 재현 문서가 명시 build를 앞에 두고 installed APK SHA 일치를 기록한다. | 수정 후 통과 |
| 22 | Android evidence가 API/ABI 또는 renderer 특성을 생략한다. | metadata에 API 37, ARM64, AVD 및 Emulator OpenGL ES Translator를 기록하고 실기기/hardware GPU 증거가 아님을 밝힌다. | 통과 |
| 23 | iOS capture가 기종·OS·UDID를 잘못 묶는다. | simulator metadata는 iPhone 17 Pro/iOS 26.2를 식별하고 result 경로와 executable hash를 고정한다. | 통과 |
| 24 | 성공 run과 실패 재시도가 섞여 PR 결과가 모호해진다. | README는 성공 run directory만 연결하고 원인·수정·재실행을 분리한다. 실패 실행 폴더는 PR stage 대상에서 제외한다. | 통과 |
| 25 | screenshot 링크가 로컬 임시 경로나 잘못된 image name을 가리킨다. | 근거 README는 성공 evidence 하위의 실제 PNG를 상대 링크로 연결하며 PR에는 branch raw URL로 옮길 수 있다. | 통과 |
| 26 | screenshot을 glyph paint 또는 실기기 렌더링 증거로 쓴다. | app 화면의 용도는 box surface와 상태 확인으로 제한하고 README가 glyph paint 및 물리 장치 검증을 부인한다. | 통과 |
| 27 | fixture-test 코드가 기본 제품 정적 library로 포함된다. | default `c04-runtime-gpu`와 fixture positive archive의 SHA, 크기, strings, symbol 결과가 반대로 확인됐다. 결론도 static archive 한정이다. | 통과 |
| 28 | archive 결과를 APK/IPA 전체 또는 배포 산출물 검사로 확대한다. | artifact JSON과 README는 archive만 비교했으며 APK/IPA/final bundle exclusion은 증명하지 않는다고 기록한다. | 통과 |
| 29 | 내부 계약 변경 없이 새로운 일반 CSS 기능이 승인된 것처럼 STATUS가 바뀐다. | 내부 contract version `0.1.0` 유지, C11.4 checkbox 미완료 및 공식 count `77/213` 보존을 diff에서 확인했다. | 통과 |
| 30 | plan·spec·evidence가 다른 결과나 완료 범위를 주장한다. | 계획과 `spec/STATUS.md`, 내부 0059, runtime README를 대조했다. 모두 제한 fixture·Simulator 결과만 보고하고 WPT/실기기/전체 C11.4 완료를 주장하지 않는다. | 통과 |

## 결론

최종 diff에서 미해결 blocker를 찾지 못했다. 실제 발견한 Android APK 선택, Android RGB mapping, iOS Metal pixel flag 문제는 수정 후 해당 경로를 다시 검증했다. 병합 여부와 GitHub CI 상태는 PR 생성 후 새로 조회해야 한다.
