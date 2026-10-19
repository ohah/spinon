# Taffy 로컬 수정

MIT 라이선스의 공개 Taffy 0.15.0 소스를 vendoring했습니다. 현재 Spinon 후보는 모든 `spinon-layout` profile에서 이 로컬 의존성을 사용합니다. trace 출력만 opt-in이며 Taffy 버전 변경은 기존 layout profile에도 적용되므로 회귀 검증이 필요합니다.

- upstream: <https://github.com/DioxusLabs/taffy/tree/v0.15.0>
- registry checksum: `3c8480887e8803986d96a7173729565520d4bbb721ad6b7ad6941a22897207aa`
- 로컬 수정:
  - Grid track-sizing 단계 snapshot, track base/growth 상태, item min/max/minimum contribution 관찰값을 제한 trace로 기록합니다. 기존 style의 trace 기본값은 꺼져 있으며 trace buffer 예약 실패나 trace 예산 초과는 부분 진단 결과 대신 오류로 layout을 중단합니다.
  - Spinon compact Grid style의 trace opt-in flag를 `DetailedGridInfo`로 전달해 layout adapter에서 읽을 수 있게 합니다.
  - `content_size`를 쓰지 않는 debug printer도 `writeln!` 실패를 전파하도록 맞춰 인접 구현과 동작을 통일하고 사용하지 않는 `io::Result` 경고를 제거합니다.
  - 현재 Clippy가 요구하는 Grid placement boolean 단순화를 적용합니다.
  - 현재 Clippy가 요구하는 `BlockFormattingContext` 기본 구현을 derive로 바꾸고 선택적 float context도 `Default`로 초기화합니다.
  - Grid 자동 최소 크기에서 replaced item의 authored aspect ratio transferred-size suggestion을 계산합니다. 반대 축의 definite preferred size를 해당 축의 definite min/max로 먼저 clamp하고 비율로 변환한 뒤 대상 축 preferred/max cap과 fixed-track limit을 적용합니다. CSS Grid 최종 정렬에서도 padding/border의 최소 바닥값을 넣기 전에 authored min-size를 비율로 전달해 최종 replaced item 프레임이 자동 최소 기여와 일치하게 합니다. Spinon은 이 조합을 C11.4 직접 HTML image grid item으로 제한합니다.

Taffy Grid 알고리즘의 내부 작업 buffer 일부는 여전히 infallible `Vec::reserve`와 `Vec::with_capacity`를 사용합니다. 이 할당 실패를 Spinon `LayoutError`로 바꾸지 않으며, 모든 내부 할당에 대한 전역 OOM 복구나 완전한 failure atomicity를 보장하지 않습니다.
