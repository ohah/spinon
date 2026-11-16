# C15.3 Flex intrinsic text test · 독립성 적대 검토 3관점

**검토일:** 2026-11-15 · **범위:** Rust layout 테스트만 추가 · **기준 checkout HEAD:** `401911e6b78b543e3c9553c3e6e162e43ca381b2`

| 관점 | 공격한 실패 경로 | 판정 |
| --- | --- | --- |
| 계획·의존성 | C15.3 `intrinsic-text` acceptance에 Chrome 155 Block/Flex/Grid `56.8125×24px` 기준이 있는지, Rust test-only 범위가 새 제품 계약이나 C15 완료로 잘못 확대되는지 확인했다. | **통과:** 고정 기준과 기존 C15 계획 검토를 재사용한다. HTML/CSS/runtime/paint 증거로 해석하지 않는다. |
| 실제 Flex 측정 경로 | 테스트가 사전 계산한 intrinsic size를 주입하거나 일반 Block text 경로만 실행해도 통과하는지 확인했다. | **통과:** 실제 no-system-font text engine, pinned Noto fixture, Flex 부모와 auto basis/size item을 `TaffyLayoutEngine`에 전달해 최종 frame과 text snapshot을 확인한다. callback 세부 요청은 공개 trace가 아니므로 주장 범위를 frame consumer에 한정한다. |
| 파일·빌드·자원 결합 | C11.4의 dirty source와 겹치는지, test binary 이외 제품 artifact에 Noto bytes가 들어가는지, 병렬 Cargo build가 충돌하는지 확인했다. | **조건부 통과:** 변경은 `#[cfg(test)]` module 등록과 테스트 파일뿐이고 C11.4 product files와 겹치지 않는다. 단, Cargo target은 공유하므로 장시간 C11.4 matrix와 테스트 build는 겹치지 않게 했다. |

이 판정은 C15.3 전체 검토나 runtime evidence를 대신하지 않는다. 독립성 검토와 이후 implementation review는 서로 다른 관문이다.
