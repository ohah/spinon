# C13.3 Android Control-wheel 재현 계획 적대 검토 20개 · 2026-10-16

검토 대상은 기존 C13.3 native wheel 계획에 추가한 Android API 37 AVD Control-wheel 재현 경로다. 이 검토는 persistent helper 구현 전에 수행했다. 제품 완료 상태 변경은 계획에 포함하지 않았다.

| # | 실패 관점 | 적대 확인 | 판정 및 계획 반영 |
|---:|---|---|---|
| 1 | 실기기 오접속 | 입력 대상은 `emulator-N` 형식, `ro.kernel.qemu=1`, API 37, ARM64를 모두 확인해야 한다. | 포함. 하나라도 다르면 실행하지 않는다. |
| 2 | 물리 device serial | adb 기본 serial이나 `localhost:5555` alias를 선택할 수 있다. | 포함. 명시한 `emulator-N`만 허용하고 alias는 거부한다. |
| 3 | `/dev/uinput` 권한 부족 | shell UID/group가 device node를 열지 못할 수 있다. | 포함. 권한 실패는 입력 실패로 기록하고 권한 변경이나 root 우회를 하지 않는다. |
| 4 | 16 KB page AVD ELF 적재 | 기본 linker segment alignment가 4 KB이면 helper가 시작 직후 종료될 수 있다. | 포함. AVD page size를 기록하고 linker max-page-size를 16 KB로 고정한다. |
| 5 | 가상 장치 기능 오인 | EV_KEY/EV_REL만 허용하면 Android가 pointer wheel source로 분류하지 않을 수 있다. | 포함. pointer property, BTN_LEFT, relative axes와 InputReader 등록을 확인한다. |
| 6 | 포인터 위치 미정 | 시작 좌표가 이전 AVD pointer 위치에 의존하면 앱 SurfaceView 밖에서 거부될 수 있다. | 포함. 화면 corner로 reset한 뒤 ready origin 기준 target으로 이동한다. |
| 7 | 접근성 clip 좌표 혼동 | 화면 좌표를 앱의 clipped accessibility bounds에서 계산하면 SurfaceView 위치가 틀릴 수 있다. | 포함. `SPINON_C133_SURFACE_BOUNDS phase=ready` 원점을 사용한다. |
| 8 | 준비 로그의 조기 캡처 | ready와 첫 GPU presentation 시점 사이에 캡처하면 전체 SurfaceView가 달라 보일 수 있다. | 포함. initial draw revision 1 뒤 안정화 대기를 거쳐 baseline을 캡처한다. |
| 9 | 좌우 Control key 누락 | 좌측 키만 등록하고 우측 Control을 일반 key로 처리할 수 있다. | 포함. left/right 각각의 실제 callback modifiers를 확인한다. |
| 10 | 단일 축만 검증 | horizontal만 검사하면 vertical Control wheel이 queue에 들어갈 수 있다. | 포함. left/right × H/V 네 조합을 독립 fresh launch로 실행한다. |
| 11 | key와 wheel 순서 경합 | key-down과 wheel을 분리한 후 InputReader가 modifier state를 놓칠 수 있다. | 포함. 한 uinput device에서 key-down, wheel, key-up 순서로 보내며 callback meta를 요구한다. |
| 12 | meta bit 의미 혼동 | 좌우 Ctrl의 raw meta state는 다르며 normalized modifier는 같아야 한다. | 포함. left `meta=12288`, right `meta=20480`, 양쪽 `modifiers=2`를 각각 검사한다. |
| 13 | 잘못된 거부 사유 | 잘못된 point/source가 Control 거부처럼 보이는 거짓 통과가 가능하다. | 포함. target CSS point의 screen 좌표를 검증하고 `reason=control-wheel`을 정확히 요구한다. |
| 14 | 거부 뒤 enqueue | reject log 뒤에 queue admission이 발생하는 부분 경로를 놓칠 수 있다. | 포함. native success log와 wheel process log가 모두 없어야 한다. |
| 15 | 조용한 상태 mutation | queue 로그는 없지만 offset/revision이 바뀔 수 있다. | 포함. revision 1과 세 owner의 초기 offsets를 draw log에서 확인한다. |
| 16 | 화면만 보는 oracle | 동일해 보이는 전체 screenshot은 작은 스크롤 경계를 놓칠 수 있다. | 포함. ready bounds에 정렬한 SurfaceView 전체 pixel 차이가 0인지 확인한다. |
| 17 | key-up 유실 | 테스트 종료 뒤 Control modifier가 전역에 남아 positive control을 오염할 수 있다. | 포함. helper가 key-up을 보내고 이후 InputReader meta state 0을 검사한다. |
| 18 | 가상 입력 잔존 | helper 종료 뒤 uinput device가 살아 다음 case에 입력을 전달할 수 있다. | 포함. 매 case 종료 후 device가 InputReader에서 제거됐는지 확인한다. |
| 19 | 과도한 거부 assertion | 모든 입력을 reject해도 테스트가 통과하는 harness 결함이 가능하다. | 포함. no-Control ordinary wheel을 positive control로 실행해 queue admission과 revision 2 draw를 요구한다. |
| 20 | 범위 과장 | AVD 결과를 trackpad, iOS, 제품 지원 또는 C13.3 전체 완료로 확대할 수 있다. | 포함. AVD fixture evidence로만 기록하고 상태 대장 완료 수와 iOS gate를 유지한다. |

계획 확인 결과: 20개 관점에서 재현 입력, no-op oracle, positive control, AVD 전용 경계와 남은 플랫폼 gate를 분리했다. ADB keyboard/mouse 두 명령은 modifier를 공유하지 않았으므로 같은 가상 evdev device에서 key와 wheel을 순서대로 보내는 방식을 사용한다.
