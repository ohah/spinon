# C11.4 · Taffy 0.14.0 automatic minimum 소스 감사

**상태:** pinned dependency source audit. Spinon layout result, Chrome differential, WPT, Android/iOS runtime 검증이 아니다.

## 고정 출처

- Spinon `Cargo.lock`: `taffy 0.14.0`, crates.io checksum `639627c87f43b9181c811f40a6296409e093a17bc761214cba3c15df74f86b99`.
- Primary source: [Taffy 0.14.0 `grid_item.rs`](https://github.com/DioxusLabs/taffy/blob/v0.14.0/src/compute/grid/types/grid_item.rs#L629-L650), [Taffy 0.14.0 `track_sizing.rs`](https://github.com/DioxusLabs/taffy/blob/v0.14.0/src/compute/grid/track_sizing.rs#L594-L690).
- 감사한 로컬 source SHA-256: `grid_item.rs` `8263d35f88d2c471160cbacd4a7f63291e1b5c2183c8baebc765d2d8211bc2ee`; `track_sizing.rs` `deabd6f661133e0e6ead0ca510f753901a897c7d7a56b8d7213dc46201a5f19d`.
- Normative source: [CSS Grid Layout Module Level 1 · §6.6, §11.3–§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/).

## 코드에서 확인한 경계

`GridItem::minimum_contribution`은 현재 축 전체 `axis_tracks`에서 automatic-minimum 조건을 검사한다. 같은 함수는 현재 item의 실제 span을 `item_axis_tracks`로 별도 계산해 span 길이를 확인하지만, “해당 축에 auto minimum track이 있는가”와 “span에 flexible track이 있는가” 검사는 전체 track slice에 `.any(...)`를 호출한다. 이 조건은 item이 걸친 track만 살펴야 한다. 따라서 다른 위치의 `auto` 또는 `fr` track이 현재 item의 content-based automatic minimum 선택에 영향을 줄 수 있다.

또한 Taffy `LayoutPartialTree`의 measure callback은 min-content/max-content 측정 요청을 제공하지만, track sizing 단계별 item contribution source 및 automatic-minimum 선택을 외부에 보고하는 observer는 공개되지 않는다. 현재 Spinon callback의 반환값만으로는 specified/transferred/content size suggestion과 automatic minimum 적용 근거를 독립 검증할 수 없다.

## 계획상 조정

- Stage A는 text/image intrinsic measurement 값과 출처·축·revision을 같은 입력에서 보존한다. C14.3 replaced-size callback은 natural image의 현재 재사용 경로다.
- Automatic minimum은 track sizing function과 item placement/span을 모두 알아야 하므로 Stage A의 독립 측정값으로 취급하지 않는다. Stage B의 typed track model 및 placed-item 계산에서 해당 item이 실제로 걸친 track만 검사하고, specified/transferred/content size suggestion과 fixed-track clamp를 기록한다.
- Taffy는 differential 후보로 유지한다. 위 span 범위 오류와 단계별 trace 부재가 해결되기 전에는 Taffy 최종 frame만으로 C11.4 구현 근거를 만들지 않는다.

이 조정은 계획의 기존 Chrome oracle을 바꾸지 않는다. 구현 전에는 적어도 “auto/fr track이 item span 밖에 있는 경우”와 “auto/fr track이 item span 안에 있는 경우”를 서로 독립된 control로 두어야 한다. 이 자료는 C11.4, C11 상위 또는 공개 CSS 지원을 완료로 표시하지 않는다.
