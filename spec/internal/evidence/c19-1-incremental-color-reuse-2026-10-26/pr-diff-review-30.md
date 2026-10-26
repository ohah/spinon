# C19.1 incremental color reuse PR diff 적대적 검토 30개

검토 대상은 base `29449e4f794e53255909b58ed8fb2fd4995dd5d2`에서 source commit `2007b120`까지의 전체 diff다. 구현 전 계획 검토 20개 및 구현 실패 관점 검토 20개와 분리된 PR 경계 검토다.

| # | PR diff 실패 경로 | 검토 결과 |
|---:|---|---|
| 1 | 계획 SHA가 코드 시작 전 고정되지 않음 | `97d31e561670a3a3de128bdf84c9a5c0c86ee071` 계획 검토가 code commit보다 앞선 커밋에 있다. |
| 2 | 변경 범위가 C19.1보다 넓어져 완료 계약을 우회 | `C19ComputedColorV1` subtree reuse와 output property 경계만 바꾼다. |
| 3 | full C19.1과 incremental profile이 각기 다른 Longhand list 사용 | 두 경로가 같은 dedicated helper를 호출한다. |
| 4 | C19.1 color snapshot에 layout/font 결과를 불필요하게 포함 | `background-color`, `color` 두 key로 제한하고 regression이 key 수를 검사한다. |
| 5 | 계산 style carrier를 paint-ready `background_color`로 잘못 승격 | C19.1 legacy `background_color` 및 `background_paint`는 계속 `None`이다. |
| 6 | 새 목록 변경이 `RuntimeBlockPaintV1`으로 전파 | 해당 profile의 existing getter와 dispatch branch는 건드리지 않았다. |
| 7 | C19.2 group opacity가 C19.1 color-only dispatch로 섞임 | C19.2는 기존 별도 profile이며 이 diff에서 수정하지 않았다. |
| 8 | `color`·`background-color` 값의 문자열 이름과 Stylo Longhand ID가 다름 | 각 문자열에 대응하는 `LonghandId`를 함께 고정한다. |
| 9 | author CSS admission 허용 범위를 몰래 줄이거나 넓힘 | `RUNTIME_BLOCK_PAINT_AUTHOR_PROPERTIES`는 유지했다. |
| 10 | currentColor의 background가 상속된 새 color 대신 이전 값을 유지 | 변경한 parent와 `currentColor` child를 full cascade와 비교한다. |
| 11 | dirty parent의 inherited color가 dirty descendant에 전달되지 않음 | dirty subtree child의 `computed_color`와 background carrier가 모두 달라진다. |
| 12 | changed subtree 밖의 sibling이 stale하거나 불필요하게 새 output을 받음 | 이전 sibling output을 직접 비교하고 reuse 통계도 검사한다. |
| 13 | parent context 계산 결과를 output reuse와 혼동 | context root style은 다시 계산하되 직렬화 output은 previous와 같음을 확인한다. |
| 14 | 계산·재사용 통계의 합계가 fixture element 수와 어긋남 | recomputed 2, context 1, reused 2를 독립 검사한다. |
| 15 | source color space 및 component tuple이 incremental path에서 손실 | ordered full `ComputedElementStyle` 전체를 비교한다. |
| 16 | missing component/alpha flag 또는 mapped sRGB가 일부만 복사 | 전체 `ComputedCssColor` carrier 동등성에 포함된다. |
| 17 | CSS Color oracle 차이를 incremental reuse 성공으로 과장 | Chrome/Stylo 색 차이 판정은 하지 않고 같은-engine full cascade만 oracle로 쓴다. |
| 18 | snapshot element order나 diagnostics가 reuse 때문에 재정렬·누락 | elements 및 diagnostics 전체를 full cascade와 대조한다. |
| 19 | author stylesheet 존재 시 selector 영향 범위를 추정해 reuse | non-empty stylesheet input은 기존 안전 guard대로 `None`이다. |
| 20 | previous profile이 다르거나 viewport/media environment가 바뀜 | profile mismatch와 viewport mismatch fallback을 테스트한다. |
| 21 | 다른 document generation의 NodeId를 재사용 | 기존 generation guard는 수정되지 않았다. 새 generation 실행은 이번 fixture의 검증 범위 밖이다. |
| 22 | 문서 revision이 증가하지 않은 호출을 재사용 | same document revision 재호출이 `None`인지 테스트한다. |
| 23 | style revision이 너무 멀리 진행된 snapshot으로 dirty set을 오인 | 두 단계 건너뛴 revision은 `None` fallback을 확인한다. |
| 24 | 유효하지 않은 dirty root에서 partial result를 조용히 구성 | `make_reuse_plan()`의 existing fail-closed 경로는 유지되며 별도 잘못된 ID 실행은 하지 않았다. |
| 25 | C19.1 profile이 synthetic wrapper와 GPU/layout 적합성을 증명한다고 문서화 | 계약과 evidence가 color carrier/cache equivalence에 한정한다. |
| 26 | C19 전체 완료 또는 checkbox 집계를 앞당김 | C19 checkbox, 72/213을 수정하지 않고 STATUS에는 nested evidence bullet만 추가했다. |
| 27 | 공개 roadmap을 배포·수정해 공식 집계와 일시 불일치 | public roadmap source/delivery는 이 diff에 없다. |
| 28 | 증거 링크가 계획/계약/실행 로그를 찾지 못함 | STATUS에서 evidence README, README에서 plan 및 logs를 상대 경로로 연결한다. |
| 29 | 실행하지 않은 광범위 테스트나 platform run을 통과로 기재 | 4 C19 tests, 244 crate tests, focused Clippy/fmt만 보고하며 simulator/browser는 미실행이라고 밝힌다. |
| 30 | parent PR review 전에 merge 가능 상태나 제품 지원을 주장 | CI/merge/브라우저/플랫폼 증거는 이 리뷰에 포함되지 않으며 이후 별도로 확인해야 한다. |

**판정:** PASS 26 / PARTIAL 4 / FAIL 0. PARTIAL은 generation mismatch 및 invalid dirty-root 직접 주입, simulator/browser 확인, CI 및 merge 상태다. 이들은 코드가 허용하는 broader claims가 아니며 README에서 이 작업의 범위를 Rust cache equivalence로 제한했다.
