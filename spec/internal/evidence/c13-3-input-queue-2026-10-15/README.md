# C13.3 bounded input queue 사전 판정

## 기준과 범위

- 기준 계약은 `plan/c13-3-scroll-gestures.md`의 입력 queue 항목과 Queue pressure 판정이다.
- 이 구현 조각은 bounded Rust queue의 순서·예약·병합 규칙만 검증한다. Android/iOS callback 대기 시간, FFI, runtime snapshot 게시와 화면 동작을 증명하지 않는다.
- 독립 oracle은 아래 불변 조건과 고정 합성 trace다. 각 입력은 단조 sequence를 쓰며, 사례별 queue capacity는 포화 경로를 재현하도록 regular 1–5개, terminal reservation 1–2개로 제한한다.

## 고정 pass 기준

1. Begin은 regular slot과 terminal reservation을 모두 얻기 전에는 queue나 active reservation에 남지 않는다.
2. 같은 identity 및 snapshot/style/policy epoch의 인접 Move만 마지막 cumulative translation으로 병합한다. 합계 translation과 마지막 sequence가 보존된다.
3. regular queue가 찬 상태에서 병합할 수 없는 Move는 부분 Move를 넣지 않고 그 gesture의 예약 terminal slot에 Cancel을 넣는다. 이미 queue에 있던 입력은 sequence 순으로 먼저 소비된다.
4. End/Cancel은 regular capacity와 무관하게 예약 slot에 한 번만 들어간다. terminal을 dequeue하면 reservation이 정확히 한 번 반환된다. Begin이 거부되면 해당 identity의 대기 입력과 reservation을 제거한다.
5. 같은 CSS point, target, snapshot key, basis scroll revision, policy epoch의 인접 wheel sample만 축별 합산한다. 합산할 수 없고 queue가 찬 경우 sample 전체를 거부하고 카운터를 올리며 gesture 상태는 유지한다.
6. sequence 역행, identity 혼동, 두 번째 terminal, 잘못된 비유한 wheel 입력은 상태를 바꾸지 않는다.
7. 같은 gesture의 Move는 PanBegan 뒤에만 오며, 다른 accepted terminal/control event와 wheel metadata barrier를 넘어 합쳐지거나 재정렬되지 않는다. capacity로 reject된 wheel sample은 queue event가 아니므로 accepted Move 간 coalescing을 끊지 않는다.

## 한계

이 단계의 통과는 순수 Rust queue 모델만 가리킨다. native callback 연결·lock 경합, Android/iOS simulator, rendering worker 응답과 같은 host 측 속성은 다음 단계에서 별도로 검증한다.

## 실행 결과 · 2026-10-15

- Queue focused tests 24/24, `spinon-render` 전체 72/72 통과.
- 전체 Rust workspace 861 passed, 0 failed, 4 ignored.
- Workspace Clippy `--all-targets -- -D warnings`, rustfmt check와 `git diff --check` 통과.
- [focused test log](focused-test.log) · [workspace test log](workspace-test.log) · [workspace Clippy log](workspace-clippy.log) · [rustfmt log](rustfmt.log) · [diff check log](diff-check.log)
- [구현 실패 관점 20개](implementation-review-20.md) · [PR 변경 관점 30개](pr-delta-review-30.md)

이 결과는 queue core unit behavior의 로컬 검증이다. PR CI, native lock/wakeup behavior, Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator, 앱 화면은 아직 실행하지 않았다.
