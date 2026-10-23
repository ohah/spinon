# C11.4 `0%` auto-repeat minimum 구현·runtime 적대 검토 20개

구현 코드와 실제 두 Simulator 실행을 계획 검토와 분리해 다시 확인했다. 아래 20개는 서로 다른 실패 경로다.

1. **Floor 적용 단계** — 1px floor가 count 전용인지 확인. Taffy repeat-count 계산의 지역 값에만 적용하고 track sizing input은 원래 함수로 유지한다. 통과.
2. **반복 그룹 내 여러 track** — 그룹당 한 번이 아니라 각 repeated track contribution을 floor하는지 확인. 두-track group 단위 테스트가 60회 count를 확인한다. 통과.
3. **반복 밖 prefix/suffix** — 양옆 explicit 0.5px track에 floor가 새지 않는지 확인. `fill-col-zero-neighbors-subpixel`과 Taffy test에서 반복 23회 및 이웃 track 0.5px를 확인한다. 통과.
4. **Gap 분리** — track floor와 gutter를 중복 합산하지 않는지 확인. gap 4px의 24회 count와 1.1666667px used flex track을 검증한다. 통과.
5. **Column percentage basis** — inline content basis를 쓰는지 확인. 120px content-box fixture에서 120/24 반복이 일치한다. 통과.
6. **Row percentage basis** — block axis로 동일 계산을 수행하는지 확인. row-axis 24회와 auto-fit 120회 장면이 통과한다.
7. **Border-box 변환** — padding/border 차감 뒤 content basis를 쓰는지 확인. 136px border-box에서 content 116px, gap 4px, 24회가 일치한다. 통과.
8. **0 content size** — 0 denominator가 많은 반복 수로 변환되지 않는지 확인. layout preflight가 해당 축을 typed error로 거부한다. 통과.
9. **Indefinite/cyclic basis** — 불확정 축에서 0%를 확정 track으로 승격하지 않는지 확인. 기존 definite-positive-axis gate가 유지된다. 통과.
10. **음수·비유한 computed percentage** — NaN/Infinity/negative 입력이 cascade/layout 경계를 통과하지 않는지 확인. style/layout typed error 및 Taffy generic guard가 확인된다. 코드 diff 검토에서 `.max(1px)`가 Taffy의 generic 음수 repeated size를 count 단계에서 숨길 수 있음을 추가로 찾아, 음수 항은 invalid sentinel로 보내도록 수정했다. 음수 단독·음수와 양수 혼합 회귀 테스트가 repeat count 1을 확인한다. 통과.
11. **Taffy generic zero/nonfinite 경로** — adapter 없이 호출돼도 0·비유한 denominator로 나누지 않는지 확인. 보수적 repeat count 1로 귀결되고 float cast에 invalid 값이 도달하지 않는다. 음수 repeated size도 같은 fail-closed count 경계를 통과한다. focused 회귀 테스트 통과.
12. **큰 count의 정수 변환** — 양의 overflow ratio가 비유한 intermediate나 wrapping으로 이어지지 않는지 확인. Rust float-to-int saturation 뒤 기존 track cap을 적용한다. 통과.
13. **Maximum 선택** — definite fixed maximum이 선택되는 조건을 count floor가 무시하지 않는지 확인. `minmax(0%, 12px)` 제어 테스트에서 7회 count를 확인한다. 통과.
14. **0px control** — `0%` 구현이 기존 `0px` repeat count를 바꾸지 않는지 확인. 120회 대조 테스트가 통과한다.
15. **Auto-fill used size** — count floor가 실제 zero minimum을 1px로 고정하지 않는지 확인. 120 track이 1px씩 채워지고 gap 장면은 남은 크기를 flex로 분배한다. 통과.
16. **빈 auto-fit** — placement 전 count를 유지하면서 placement 후 모두 collapse하는지 확인. `fit-col-empty`의 120 collapsed track이 통과한다.
17. **점유 auto-fit** — 사용된 track 하나만 확장하고 빈 119개가 collapse하는지 확인. `fit-col-first`의 120px/0px histogram과 item frame이 통과한다.
18. **Span과 collapse 경계** — 두 track을 span한 item 주변의 gutter/collapse가 맞는지 확인. `fit-col-span-two`에서 58px × 2와 22 collapsed track을 확인한다.
19. **Cross-platform fixture oracle와 trace 수집 완전성** — Android/iOS allowlist·frame expectation과 모든 scene trace part가 연결되는지 확인. 양쪽 adapter의 누락 frame 기대값을 보완했고, Android log burst에서 사라진 중간 chunk를 찾아 opt-in log pacing 및 capture complete-set gate를 추가했다. Node boundary test가 gate 사용을 확인하며 최종 두 simulator 13/13 trace 전체가 복원됐다. 통과.
20. **제품 노출과 증거 한계** — 새 public ABI, default archive payload 또는 hardware/pixel parity 주장이 생기는지 확인. 새 ABI는 없고 default archive marker는 absent, opt-in positive control은 present다. PixelCopy/Metal equality, hardware GPU, 실기기와 전체 C11.4는 주장하지 않는다. 통과.

**해소한 초기 발견:** 첫 AVD 실행은 `spinon-style` cascade의 별도 positive-only percentage gate에 막혔다. finite zero 허용으로 수정했다. 둘째 실행은 새 scene에 오래된 native 기본 frame 기대값을 적용했다. 양 플랫폼 scene 기대값을 추가하고 boundary test가 모두 검사하도록 고친 뒤 Android API 37 AVD와 iPhone 17/iOS 26.2 Simulator에서 각각 13/13 scene이 통과했다. 세부 원본은 [runtime 근거](README.md)와 같은 폴더의 scene 로그에 있다.
