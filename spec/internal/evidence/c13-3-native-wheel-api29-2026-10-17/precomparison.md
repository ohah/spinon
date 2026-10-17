# C13.3 Android API 29 wheel 사전 판정

## 목적과 범위

기존 C13.3 native wheel 경로가 `minSdk=29`인 Android 최소 지원선에서도 실제 `ACTION_SCROLL` callback에서 bounded queue와 draw까지 연결되는지 추가 확인한다. 이 실행은 API 37 AVD의 입력·거부 행렬을 대체하거나 C13.3 완료를 선언하지 않는다. 물리 장치·외장 입력 장치·hardware GPU 근거도 아니다.

## 기준과 고정 입력

- 기준은 [C13.3 native wheel 계획](../../../../plan/c13-3-native-wheel-input.md)의 Android 방향 oracle, fixture, queue 계약 및 고정 Chrome 155 400×240 CSS px geometry다.
- fixture는 `auto-boundary`, viewport `400×240 CSS px`, 초기 offsets `root=(0,0), outer=(0,0), inner=(86,106)`이며 입력점은 `(132,112) CSS px`다. 각 행렬 case는 fresh app launch로 초기 상태를 복원한다.
- Android API 29 `input` shell에는 `mouse scroll` 명령이 없다. AVD 안에서 생성·제거하는 test-only Linux uinput mouse가 지정 screen point로 이동한 뒤 `REL_HWHEEL`/`REL_WHEEL`을 보낸다. 이는 AVD InputReader를 거치는 가상 입력이며 `WheelDelta` direct enqueue가 아니다.
- 수평 기대값은 `delta_x_css = HSCROLL × horizontalFactorPx / surfacePxPerCssPx`; 수직 기대값은 `delta_y_css = -(VSCROLL × verticalFactorPx) / surfacePxPerCssPx`다. SurfaceView의 실제 크기에서 scale을 구하며, 입력 point는 정수 screen pixel 반올림으로 생기는 최대 `0.5 / scale + 0.001 CSS px` 오차를 허용한다.
- 기존 API 37 계획 oracle의 각 방향 magnitude `64 CSS px`와 fixture owner 분배를 적용한다. `H+`는 outer X `64`, `H-`는 inner X `22`, `V+`는 inner Y `42`, `V-`는 outer Y `64`로 끝나야 한다. 대각 4행은 기존 계획의 축별 owner offsets와 일치해야 한다.

## 사전 통과 조건

8개 H/V 방향·대각 입력 각각에서:

1. callback이 `ACTION_SCROLL` (`action=8`), `SOURCE_MOUSE` (`0x2002`), pointer tool `mouse`, modifiers `0`, phase `None`으로 기록된다.
2. callback의 source, raw axes, factors, surface scale, CSS point·delta가 입력 manifest 및 위 변환식과 `0.01 CSS px` 이내 일치한다.
3. 한 sample만 `sequence=1`, `Queued`, `target=7`로 수락되고 worker 처리 뒤 scroll revision은 정확히 2가 된다.
4. root/outer/inner offsets와 revision 2의 presented draw가 기존 방향 oracle과 정확히 일치한다.
5. positive-positive 대각 입력 전후 SurfaceView pixel crop을 비교한다. root scrollport border `(16,16)-(384,224) CSS px` 바깥 changed pixel은 0, 내부 changed pixel은 0보다 커야 한다.

## 거부 대조와 판정 제한

- 좌우 Control × 수평/수직은 callback에서 `modifiers=2`와 `reason=control-wheel`로 거부되고 queue/process가 0건, revision 1, 초기 offsets를 유지해야 한다.
- virtual mouse는 각 실행에서 `UI_DEV_CREATE` 이후 wheel event를 보낸 다음 `UI_DEV_DESTROY`한다. 실패·종료 후 기기 이름이 input device 목록에 남으면 해당 실행은 실패다.
- 이번 matrix가 통과해도 API 29 Google APIs ARM64 AVD의 opt-in fixture 결과만 입증한다. 실기기, 외장 mouse/trackpad, hardware GPU, Android API 28 이하, iOS callback/sign/adapter, public DOM event, compositor 이후 presentation 및 C13.3 전체는 미검증이다.

## 탐색 실행 기록

2026-10-17에 formal matrix 직전 API 29 AVD의 `H+` 한 건을 harness feasibility 확인용으로 주입했다. callback은 `hfactor=168 px`, `scale=2.625`, `delta=(64,0) CSS px`, sequence 1, target 7, revision 2, outer X 64를 기록했다. 이 한 건은 아래 formal matrix 집계에 포함하지 않으며, 사전 판정 이후의 acceptance result로 사용하지 않는다.
