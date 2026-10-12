# C11.1 layout-core 구현 실패 관점 검토 · 2026-10-12

**기록 시점:** 이 검토는 Taffy layout-core 변경만 검토한 초기 단계 기록이다. 이후 Stylo cascade·typed projection이 추가되어 현재 상태와 일치하지 않는 당시 미완료 항목은 [후속 Stylo→layout adapter 검토](./c11-style-grid-projection-implementation-review-2026-10-12.md)에서 다시 확인했다.

**범위:** `spinon-layout`의 Grid 입력 검증과 Taffy 0.14.0 dispatch 골격만 검토한다. Stylo cascade, runtime profile, JS document revision, Android/iOS surface, 공개 CSS 지원은 이 검토의 범위가 아니다.

## 검토 결과

1. **잠긴 계산기와 feature가 일치하는가** — Cargo.lock의 Taffy `0.14.0`에서 `grid` feature를 활성화했다. 다른 버전의 trait/스타일 API로 우회하지 않는다.
2. **기존 custom tree의 owner 경계를 보존하는가** — 기존 `CalcLayoutTree`를 유지하고 grid trait만 구현했다. 새 `TaffyTree`를 만들어 revision·CSS math 경계를 우회하지 않는다.
3. **Grid display dispatch가 실제 Grid algorithm을 호출하는가** — `(Display::Grid, true)`에서 `compute_grid_layout`을 호출한다.
4. **기존 Block/Flex dispatch가 바뀌었는가** — 각 dispatch arm은 유지했고 Grid 안 Flex 자식 회귀를 추가했다.
5. **pixel track 변환이 단위를 바꾸는가** — `LengthPx`를 Taffy `from_length`로 전달하고 100px track 위치를 검사했다.
6. **percentage track이 definite axis를 확인하는가** — 두 container axis의 definiteness를 먼저 요구하고 25%·50% track geometry를 확인했다.
7. **`fr` 분모와 입력 값이 안전한가** — 양의 유한 `fr`만 허용하고 실제 남는 폭·높이 배분을 테스트했다.
8. **row/column gap basis가 섞이는가** — Grid의 두 gap을 각각 width/height definiteness로 검증하고 서로 독립된 row/column gap geometry를 확인했다.
9. **track list 누락이 기본값으로 성공하는가** — visible Grid의 container 입력과 비어 있지 않은 row/column list를 필수화했다.
10. **존재하지 않는 container/item ID가 무시되는가** — 입력 map key가 현재 layout tree에 있는지 선검증한다.
11. **Grid item map이 다른 부모에서 적용되는가** — `grid_items` key가 Grid container의 직계 자식인지 검증한다.
12. **미지원 auto-placement 방향이 조용히 row로 바뀌는가** — row 외 flow mode를 Taffy 호출 전 거부한다.
13. **명시 line/span 값이 자동 배치처럼 처리되는가** — 현재 단계에서는 Auto 외 placement를 거부한다.
14. **source order와 `order`의 의미가 어긋나는가** — order 값이 0이 아닌 item을 거부한다. 첫 검토에서 누락을 찾아 `UnsupportedGridOrder`와 회귀를 추가했다.
15. **positioned item이 일반 item으로 배치되는가** — static이 아닌 Grid 직계 자식을 거부한다.
16. **intrinsic size 경계가 고정 크기 제한을 우회하는가** — item width/height가 둘 다 Fixed가 아니면 거부한다.
17. **implicit track이 기본 track으로 만들어지는가** — auto placement로 필요한 cell 수가 explicit capacity보다 많으면 거부한다.
18. **hidden subtree가 cell 수를 소모하는가** — `display:none` 조상/자손을 item count에서 제외한다.
19. **과대 track이 Taffy의 silent clamp를 성공 결과로 노출하는가** — Taffy 내부 10,000 track 경계를 읽고 초과 입력을 사전 거부하는 검사와 회귀를 추가했다. 이 제한은 공개 CSS 지원이나 Chromium clamp와 같다고 주장하지 않는다.
20. **기존 constructor/test 입력이 신규 map 때문에 깨지는가** — HostDocument·core tree constructor와 기존 `LayoutInput` test literals에 빈 Grid map을 명시하고 workspace compile을 확인했다.

## 수정과 재검증

- 0이 아닌 Grid item `order`는 현재 Taffy child iteration에서 CSS의 order-modified placement를 보장하지 않아 성공시키면 잘못된 cell을 낼 수 있다. 이제 `UnsupportedGridOrder`로 전체 계산을 거부한다.
- Taffy `0.14.0`은 축마다 10,000 explicit track에서 큰 grid를 clamp한다. adapter는 현재 입력을 잘라 계산하지 않고 `GridTrackLimitExceeded`로 거부한다. C11.1 profile의 이 한계는 고정 Chromium 기준 및 전체 CSS 호환을 대신하지 않는다.
- Grid feature 활성화 직후 기본 debug workspace 시험에서 8 MiB `spinon-css-cascade` worker가 1,024-depth runtime fixture에서 stack overflow를 냈다. 격리 재실행도 실패해 시험 병렬성 문제가 아님을 확인했다. Taffy `Style`은 Grid feature 전 240 bytes에서 활성화 후 552 bytes로 커졌다. 임시 16 MiB worker 진단 실행은 통과했으나 해당 변경은 되돌렸다.
- 모든 계산 노드에 Taffy의 unused Grid named-line·area fields를 보관하는 대신 `CompactTaffyStyle` projection을 구현했다. compact size 회귀와 기본 8 MiB debug worker의 최대 depth 1,024 실행이 통과했다. production stack 설정은 바꾸지 않았다.
- `mise exec -- cargo test --offline -p spinon-layout grid::tests -- --nocapture` — 6 passed, 0 failed.
- `mise exec -- cargo clippy --offline -p spinon-layout --all-targets -- -D warnings` — 통과.
- `mise exec -- cargo check --offline --workspace` — 통과.
- 최종 `mise exec -- cargo test --locked --workspace` — 575 passed, 0 failed, 3 ignored; 1,024-depth debug worker 시험을 포함한다. [실행 로그와 나머지 검증](./c11-grid-layout-core-validation-2026-10-12/README.md).
- 최종 CSS reference 193/193, C11 Chrome precomparison 4/4, JS 2/2, Clippy·rustfmt·diff check 통과. 원본 로그는 위 실행 근거에 연결한다.

## 남은 관문

이 검토 시점에는 Stylo typed-value projection, profile allowlist·진단 및 platform runtime 연결이 아직 없었다. 이후 일부 Rust adapter work는 별도 변경으로 추가되었지만, `LayoutInputRevision`의 앱 runtime publication, Chrome 10-scene node별 Rust comparator, Android/iOS V8→WGPU Grid geometry 및 surface pixel 실행은 현재도 남아 있다. 이 검토는 C11.1 완료나 공개 `display:grid` 지원을 승인하지 않는다.
