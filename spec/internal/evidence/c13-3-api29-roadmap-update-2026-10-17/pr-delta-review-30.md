# C13.3 API 29 로드맵 변경 PR 실패 관점 검토 30개

검토 대상은 `spec/public/roadmap.html`의 API 29 요약 문단과 이 문서다. 아래 검토에서 발견한 과장·경로 결함은 없다.

| # | 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | API 29를 API 28 이하까지 지원하는 결과로 읽을 수 있는가 | 최소 API선만 표시했다. 통과. |
| 2 | Android 10과 API 29 대응이 잘못 연결됐는가 | source evidence의 AVD 정보를 대조했다. 통과. |
| 3 | ARM64 AVD를 실기기로 잘못 표현했는가 | ARM64 AVD라고 한정했다. 통과. |
| 4 | test-only uinput 입력을 일반 사용자 입력으로 오인하는가 | test-only virtual mouse로 표시했다. 통과. |
| 5 | uinput이 실제 Android callback 경로를 우회했는가 | 근거 README가 `ACTION_SCROLL`/`SOURCE_MOUSE` callback과 FFI 직접 enqueue 미사용을 기록한다. 통과. |
| 6 | 실행 결과가 빌드 성공만으로 추론됐는가 | 카드가 고정된 실행 결과를 요약하고 상세 실행 자료에 연결한다. 통과. |
| 7 | 8/8이 축 부호나 대각을 누락했는가 | ±H/V와 네 대각을 뜻하며 matrix 원본이 링크 대상에 있다. 통과. |
| 8 | Control-wheel 거부 수가 과장됐는가 | 좌우 Control × H/V의 네 경우다. 통과. |
| 9 | Control-wheel 입력이 잘못 분류됐는가 | 원본에서 각 callback `modifiers=2`와 queue 전 거부를 확인했다. 통과. |
| 10 | 화면 픽셀 결과가 별도 설정과 섞였는가 | 대각 positive-positive 사례 하나로 정확히 한정했다. 통과. |
| 11 | 661,500 pixel 수가 crop 면적과 맞지 않는가 | run JSON의 aligned SurfaceView 비교 면적과 대조했다. 통과. |
| 12 | 바깥 변경 0개를 모든 시나리오로 확대했는가 | 문장이 해당 positive-positive 사례에만 귀속한다. 통과. |
| 13 | 내부 changed pixel도 0개라고 오인하게 하는가 | 문장은 scrollport 바깥만 말한다. 통과. |
| 14 | software renderer 특성이 감춰졌는가 | software renderer AVD 제한을 명시했다. 통과. |
| 15 | SwiftShader 결과가 hardware GPU 성능·정확성 증거로 읽히는가 | hardware GPU 주장은 없고 renderer 제한을 명시했다. 통과. |
| 16 | emulator 근거가 실기기 보증으로 확대되는가 | 실기기 검증으로 확대하지 않는다고 명시했다. 통과. |
| 17 | virtual mouse가 물리 trackpad 증거로 읽히는가 | 물리 trackpad 검증으로 확대하지 않는다고 명시했다. 통과. |
| 18 | Android touchpad 분류가 확인된 것으로 보이는가 | 이 후속 요약은 mouse 입력만 다룬다. 통과. |
| 19 | iOS native callback이 통과한 것처럼 보이는가 | callback/sign 미관찰을 명시했다. 통과. |
| 20 | C13.3 전체 완료로 체크됐는가 | 완료 문구 없이 명시적으로 미완료라고 적었다. 통과. |
| 21 | STATUS의 공식 완료 수가 증가하는가 | 상태 대장을 수정하지 않고 roadmap 설명만 추가했다. 통과. |
| 22 | 계획 문서 링크가 존재하지 않는 경로를 가리키는가 | `plan/c13-3-native-wheel-input.md` 경로가 main에 있다. 통과. |
| 23 | API 29 근거 링크가 Markdown 렌더 경로와 맞는가 | `README.md`는 `/README.html` route로 연결된다. 통과. |
| 24 | 링크가 Tailnet base path 밖으로 나가는가 | 두 링크는 기존 `/spinon/` 경로 규칙을 따른다. 통과. |
| 25 | 긴 요약이 이전 C13.3 사실을 지우는가 | 기존 synthetic-wheel 문단을 보존하고 별도 follow-up 문단을 추가했다. 통과. |
| 26 | 2026-10-17 날짜가 run 시각과 맞지 않는가 | run timestamp는 `2026-10-17T03:44:58.250Z`다. 통과. |
| 27 | 양의 대각 시각 자료와 설명이 어긋나는가 | 상세 README 및 before/after PNG가 같은 case를 가리킨다. 통과. |
| 28 | simulator 화면을 compositor 이후 프레임 증거로 주장하는가 | roadmap은 표면 pixel 비교만 요약하며 displayed-frame 주장이 없다. 통과. |
| 29 | 문단이 C13.3의 공개 CSS 지원을 약속하는가 | public DOM event나 CSS 완료를 주장하지 않는다. 통과. |
| 30 | PR 문구만으로 reproducibility를 주장하는가 | 세부 수치가 기기·실행 자료를 가진 API 29 evidence README에 연결된다. 통과. |

## 결론

수정 필요 결함 없음. 이 문구는 제한된 Android API 29 AVD 결과를 기존 미완료 경계 안에 둔다.
