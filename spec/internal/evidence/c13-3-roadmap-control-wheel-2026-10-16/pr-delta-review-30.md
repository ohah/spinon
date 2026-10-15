# C13.3 Control-wheel 로드맵 PR 변경 검토 · 30개

검토 대상은 `spec/public/roadmap.html`의 C13.3 Control-wheel 문장과 실행 근거 링크, 이 검토 기록 및 Agent Browser 캡처다. PR #199에서 병합한 [실행 근거](../c13-3-android-control-wheel-runtime-2026-10-16/README.md), [공식 상태 대장](../../../STATUS.md), [입력 계획](../../../../plan/c13-3-native-wheel-input.md)과 대조했다. 남은 결함은 없다.

| # | 적대적 확인 | 결과 |
|---:|---|---|
| 1 | 부분 검증을 완료 항목 수 증가로 계산하는가? | 아니다. counter는 61/197 그대로다. |
| 2 | C13.3 전체가 완료된 것으로 읽힐 수 있는가? | 아니다. C13.3 미완료와 잔여 범위가 그대로 남는다. |
| 3 | 기존 Control-wheel 미검증 문구가 다른 위치에 남는가? | 대상 문구 하나를 실행 근거로 대체했다. |
| 4 | AVD API 버전이 실행 기록과 일치하는가? | Android API 37로 일치한다. |
| 5 | CPU ABI를 실제 장치 범위로 과장하는가? | ARM64 AVD라고 명시한다. |
| 6 | 가상 입력 장치가 실제 주변 장치처럼 표현되는가? | 가상 uinput 장치라고 명시한다. |
| 7 | left Control 결과가 범위에 들어 있는가? | 좌 Ctrl을 포함한 4/4 행렬이다. |
| 8 | right Control 결과가 범위에 들어 있는가? | 우 Ctrl을 포함한 4/4 행렬이다. |
| 9 | horizontal wheel 결과가 명시되는가? | 수평 축을 포함한다. |
| 10 | vertical wheel 결과가 명시되는가? | 수직 축을 포함한다. |
| 11 | 네 조합의 수가 검증 README와 같은가? | 좌우 Ctrl×수평/수직 4/4로 일치한다. |
| 12 | 거부 시점이 queue 뒤로 잘못 기술되는가? | queue 전에 거부라고 명시한다. |
| 13 | enqueue/process 결과를 성공 처리처럼 표현하는가? | 각 case 0건이라고 명시한다. |
| 14 | 거부 후 draw revision이 바뀌었다고 읽힐 수 있는가? | draw revision 1을 명시한다. |
| 15 | 시작 scroll offset 보존을 누락하는가? | 초기 offsets 보존을 명시한다. |
| 16 | pixel sample 범위가 SurfaceView 전체와 혼동되는가? | SurfaceView crop 661,500 pixel이라고 적는다. |
| 17 | 시각 비교 결과를 흐리는가? | changed pixel 0개라고 명시한다. |
| 18 | 입력 종료 뒤 가상 장치 정리를 누락하는가? | 입력 장치 제거를 적는다. |
| 19 | modifier 상태 누수를 놓치는가? | 제거 뒤 meta state 0을 적는다. |
| 20 | negative path만 있고 양성 대조가 없는가? | 일반 wheel 양성 대조와 revision 2를 적는다. |
| 21 | 물리 trackpad 검증으로 오해될 수 있는가? | 실기기·외장 장치 입력이 아님을 적는다. |
| 22 | iOS 검증을 암시하는가? | iOS native callback 미관찰 상태가 유지된다. |
| 23 | public WheelEvent/DOM scroll 지원을 암시하는가? | public event 연결이 잔여 범위에 남는다. |
| 24 | compositor 이후 표시를 입증한다고 오해되는가? | 해당 경계가 미검증으로 남는다. |
| 25 | `STATUS.md`와 계획의 결과가 충돌하는가? | 4/4 세부와 61/197 상태가 일치한다. |
| 26 | 실행 근거 링크가 Tailnet base와 맞는가? | `/spinon/spec/internal/evidence/.../README.html` 경로다. |
| 27 | 링크가 존재하지 않는 evidence 문서를 가리키는가? | PR #199에 병합된 README 경로다. |
| 28 | 실행 페이지의 화면 증거가 깨질 수 있는가? | Agent Browser에서 두 이미지 모두 1080×2400으로 로드됐다. |
| 29 | HTML anchor 문법이나 텍스트가 깨졌는가? | Agent Browser DOM에서 문구와 목적 URL을 확인했다. |
| 30 | 좁은 화면에서 문서 너비가 viewport를 넘는가? | 390×844에서 document/body 폭 390, 가로 overflow 없음. |

## 검토 결론

수정은 오래된 한 문장과 C13.3 실행 근거 링크에 한정된다. 제품 코드, 공개 API, 공식 완료 체크는 변경하지 않았다.
