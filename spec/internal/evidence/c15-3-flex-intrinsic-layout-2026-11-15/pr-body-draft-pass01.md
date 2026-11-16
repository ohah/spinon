## 변경한 항목

- [ ] 작업 유형 및 주요 영역 라벨을 지정했습니다. PR 생성 시 `enhancement`, `area:layout`, `area:testing`을 붙입니다.
- 구현 상태 대장 ID: C15.3 Flex intrinsic text layout (Rust layout 회귀 테스트 범위)
- 변경 내용과 지원 범위: 시스템 폰트에 의존하지 않는 Noto Sans KR `한글字` 텍스트 항목을 Taffy Flex의 max-content 입력에 넣는 `#[cfg(test)]` 회귀 테스트 한 개를 추가합니다. 저장된 Chrome 155 rect(`56.8125 × 24px`, 허용 오차 `0.05px`)와 Flex/item 프레임을 비교하고, 최종 text snapshot의 소유 NodeId, 원본·스타일 정보, revision, 줄 수, 높이, 가용 너비를 확인합니다. 제품 runtime, 공개 API, CSS 지원 판정은 변경하지 않습니다.

## 명세와 동작 근거

- [x] 해당 항목의 [상태 대장](../../../STATUS.md) 상태를 갱신했습니다. C15.3/C15는 미완료로 두고 공식 집계 `78/214`를 유지합니다.
- API/인터페이스 명세: 해당 없음. `#[cfg(test)]` 레이아웃 회귀 테스트만 추가하며 제품 또는 FFI 계약은 바뀌지 않습니다.
- 성공·오류·수명 동작, 예제, 플랫폼 차이와 미지원 범위: 제품 API 변경이 없어 해당하지 않습니다. 이 테스트는 앱·플랫폼 동작을 주장하지 않습니다.
- 완료로 표시한 플랫폼의 동작 근거: 플랫폼 완료를 표시하지 않았으므로 해당하지 않습니다.
- 집중 실행 명령: `mise exec -- cargo test --locked -p spinon-layout c15_3_flex_max_content_item_matches_noto_intrinsic_text_oracle -- --test-threads=1 --nocapture` — 테스트 1개 통과, 0개 실패.
- 기준 관찰: Chrome 155, 고정 Noto Sans KR fixture, DPR 1에서 `56.8125 × 24px` ([관찰 JSON](../c15-3-unicode-shaping-precomparison-2026-10-17/chrome155-observation.json)).
- 입력 차이와 한계: Chrome fixture는 `inline-flex`, `width: max-content`, 기본 `height: auto`입니다. Rust 테스트는 Taffy `Flex`의 width와 height를 모두 `max-content`로 지정합니다. 따라서 이 결과는 한 줄 입력의 제한된 프레임 비교이며 CSS `height:auto` 동등성을 입증하지 않습니다. Chrome fixture 전체 파싱, CSS projection, HostDocument/runtime, paint, 시뮬레이터·실기기, 더 넓은 Flex sizing 또는 WPT 적합성도 검증하지 않습니다.
- 근거: [실행 요약](./README.md), [focused 로그](./focused-layout-pass01.log), [Chrome fixture](../../../../tests/fixtures/css/c15/c15-3-unicode-shaping-oracle.html), [Rust 회귀 테스트](../../../../crates/spinon-layout/src/c15_text_layout_tests/flex_intrinsic.rs), [독립성 검토 3관점](./parallel-review-3.md), [구현 검토 20관점](./implementation-review-20.md), [후보 변경 검토 30관점](./pre-pr-candidate-review-30-pass01.md).
- C11.4 pass07은 43개 test 중 36 passed·7 failed였으며 전체 matrix는 green이 아닙니다. 별도로 C15 후보 두 source는 pass07의 1,920개 입력 manifest에서 사전·사후 일치했고 C15 focused pass02 및 현재 hash도 모두 같습니다([hash closure](./source-hash-closure-pass01.json)). 실제 PR 패키지에서는 두 Rust source와 필요한 C15 evidence만 선택하고 C11.4, Cargo.lock sha2, mise.toml toolchain 변경을 제외합니다. 실제 PR 본문을 게시할 때 이 초안의 파일 상대 링크를 공개된 PR head 기준 GitHub 링크로 바꿉니다. 이 후보 검토는 실제 PR diff 검토를 대체하지 않습니다.

## 남은 범위

C15.3/C15는 미완료입니다. 이 변경은 실제 CSS `inline-flex` 동작, intrinsic callback 종류, HTML/CSS 투영, runtime 통합, paint, 플랫폼별 동작 또는 전체 Flex sizing 지원을 완료 처리하지 않습니다. 앱 화면 캡처는 없으며, Chrome 이미지를 첨부한다면 제품 화면이 아닌 기준 캡처라고 표시해야 합니다.
