# C13.1 전체 물리 픽셀 PR delta 실패 관점 검토 30개 · 2026-10-18

대상 PR [#281](https://github.com/ohah/spinon/pull/281), head `b868026e819c351fe5df040fd9b39535b5381a52`. 계획 단계 20개 검토와 구현 단계 20개 검토를 복사하지 않고, 최종 코드·근거·PR 본문 간 결합을 다시 확인했다.

| # | 독립 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 세 플랫폼 비교가 서로 다른 Chrome oracle 입력을 사용 | Android/iOS full-surface manifest와 README가 Chrome 155 DPR 3 PNG 및 observation SHA를 같은 값으로 고정한다. 통과. |
| 2 | CSS pixel 크기의 oracle를 물리 surface로 오인 | oracle 300×100 CSS, DPR 3, PNG/readback 900×300 관계를 manifest와 comparator가 검증한다. 통과. |
| 3 | 일부 crop이나 가장자리 제외로 mismatch 숨김 | 비교 입력은 전체 900×300이며 270,000 pixel 전부가 대상이다. 통과. |
| 4 | RGB만 비교하고 alpha 차이를 무시 | expected byte cardinality는 1,080,000 RGBA byte이며 alpha를 포함해 exact 비교한다. 통과. |
| 5 | tolerance가 잘못된 픽셀을 성공으로 처리 | 전체 comparator는 threshold/crop을 적용하지 않고 mismatch와 max error를 보고한다. 세 run씩 mismatch 0, error 0이다. 통과. |
| 6 | DPR calibration 진단을 최종 성공 집계에 섞음 | DPR 2.625/2.626 시도와 mismatch 수를 README에 분리하고 최종 성공에서 제외했다. 두 번째 JSON 부재도 공개했다. 통과. |
| 7 | Chrome 이미지를 fixture source나 앱 surface와 혼동 | PR 본문과 근거 문서는 Chrome을 oracle, PixelCopy/Metal artifact를 앱 결과로 별도 지정한다. 통과. |
| 8 | Android status/navigation 영역이 SurfaceView 비교에 유입 | comparator 입력은 별도 PixelCopy PNG이며 기기 전체 screenshot은 PR 화면 자료로만 첨부한다. 통과. |
| 9 | iOS simulator 전체 화면 screenshot을 drawable 비교로 제출 | PR 본문과 README는 drawable `.rgba` artifact를 비교하고 full-device screenshot은 시각 자료로 분리한다. 통과. |
| 10 | Metal readback row padding이 RGBA 데이터로 계산됨 | 256-byte padded row에서 유효 texel만 packed 저장하고 comparator가 packed byte length·hash를 검증한다. 통과. |
| 11 | Metal format에 따른 byte 순서 또는 색 변환이 숨겨짐 | manifest는 `Bgra8Unorm`과 packed RGBA 순서를 선언하고 comparator가 지원 format, dimensions, format identity를 확인한다. color correction을 적용하지 않았다. 통과. |
| 12 | iOS 여러 batch가 surface generation을 바꿔 서로 다른 frame이 합쳐짐 | generation, format, dimensions를 batch report마다 고정·대조한다. 최종 세 run은 generation 1이다. 통과. |
| 13 | Android PixelCopy 결과가 다른 process/surface에서 생성 | begin/end log에 PID·generation·dimensions가 연결되고 세 run은 PID가 다르며 generation 2다. 통과. |
| 14 | 중단되거나 부분적인 Metal artifact를 성공으로 판정 | comparator는 manifest `complete`, byte length, cardinality와 SHA-256을 요구한다. incomplete case는 tests에서 거부한다. 통과. |
| 15 | 같은 texel을 중복 제출하거나 일부를 빠뜨림 | 전체 coordinate/cardinality와 artifact hash 확인, 누락·추가·중복·재배열 mutation 거부 테스트가 있다. 통과. |
| 16 | surface 좌표 밖 border 출력이 fixture에서 누락 | border 밖 `(270,72)`와 physical center `(811,217)`은 900×300 전체 readback에 포함된다. 통과. |
| 17 | batch 요청 수·마지막 부분 batch를 잘못 계산 | iOS row-major 4,219 batch가 기록되고 최종 수·byte length를 확인한다. 세 run 전부 270,000 texel이다. 통과. |
| 18 | 잘못된 route가 fixture host 초기화 후에야 실패 | iOS 단독 잘못된 route와 다중 route negative control이 host 생성 전에 거부된 로그를 남긴다. 통과. |
| 19 | AppDelegate 경유 시 새 route가 실행 dispatch에서 누락 | route allowlist와 dispatch를 함께 갱신했고 최종-source iOS 3회 캡처가 통과했다. 통과. |
| 20 | 테스트 뒤 iOS app process가 남아 재실행 결과를 오염 | runner가 각 캡처와 negative control 뒤 terminate를 수행하며 최종 cleanup log를 보존한다. 통과. |
| 21 | Android 실행이 예상치 않은 emulator나 연결 device를 수정 | runner는 지정 serial과 API37 ARM64 AVD를 검사하고 한 AVD만 호출한다. `localhost:5555`는 유지됐다. 통과. |
| 22 | density 수정이 logical/physical display size까지 바꿈 | density는 420→480 override로 scale 3을 만들고 `wm size` 전·중·후 1080×2400 동일성을 검사한다. 통과. |
| 23 | 실패 경로에서 Android density가 복구되지 않음 | cleanup 경로가 이전 override/reset을 복원하고 결과를 assert한다. 세 성공 run은 420 복구를 기록한다. 통과. |
| 24 | Android app이 실행 후 남아 환경 상태에 영향을 줌 | runner가 force-stop 후 PID 부재를 확인한다. 세 최종 PID 모두 종료 상태다. 통과. |
| 25 | PixelCopy 전후 frame/surface 신원이 불일치 | surface generation, PID, scale, width·height, frame/query/hit logs를 상호 대조한다. 통과. |
| 26 | 전체 surface 추가가 기존 center/sample/hit 회귀를 가림 | Android와 iOS 최종 run에서 30,000 center/hit identities, 52 fixed samples, 41 frames도 별도로 통과했다. 통과. |
| 27 | 기본 빌드에 테스트 fixture FFI가 새어 들어감 | host debug archive default/opt-in 비교와 symbol/string positive control을 근거 파일에 남겼다. 이는 host debug archive에 한정함을 명시했다. 통과. |
| 28 | 문서가 제한 fixture를 제품 전체 완료로 과장 | STATUS의 C13/C13.1은 미완료이고 공식 완료 수는 61/197로 유지한다. roadmap과 내부 계약도 실기기·WPT·일반 CSS 한계를 밝힌다. 통과. |
| 29 | PR screenshots가 잘못된 raw URL·깨진 링크로 표시 | Agent Browser에서 PR 페이지의 Android/iOS `<img>`가 `complete=true`, natural size 1080×2400/1206×2622임을 확인했다. 통과. |
| 30 | 요약·증거·PR 커밋이 다른 source revision을 가리킴 | 최종 실행은 구현 revision `85f2899…`에 연결되고 PR head에는 그 구현 및 근거 문서 두 commit이 이어진다. 1,047 workspace tests, clippy, fmt와 7 comparator tests의 기록이 연결됐다. 통과. |

## 병합 직전 경계

PR에는 GitHub status check가 게시되지 않았고 상태는 `CLEAN`이다. 여기의 simulator/AVD 근거는 13-case fixture 및 도구 경로를 한정 검증한다. Android hardware renderer·실기기·표시 패널, iOS compositor 이후, 일반 HTML/CSS, DOM event dispatch, WPT 적합성, C13.1/C13 제품 완료를 증명하지 않는다.
