# C15.3 Tailnet 로드맵 변경 PR 적대적 검토 20개 관점

**검토 범위:** spec/public/roadmap.html의 C15.3 진행 문구와 내부 근거 인덱스 연결. C15 구현 결과나 제품 동작을 승인하는 검토가 아니다.

| # | 실패 관점 | 대조한 근거와 판정 |
|---:|---|---|
| 1 | C15.3 계획을 구현 완료로 오인하게 되는가 | 문구가 “구현 전 계획 단계”라고 명시한다. C15 runtime 구현 완료로 읽히지 않는다. |
| 2 | 이미 병합된 작업의 PR 상태가 틀렸는가 | PR #245는 현재 MERGED; 로드맵은 계획 병합만 기록한다. |
| 3 | C15.3 계획을 C15.2b 구현과 혼합하는가 | 새 문단을 C15.2b 설명 뒤에 별도 배치하고 C15.3으로 표기한다. |
| 4 | 날짜가 근거와 맞지 않는가 | 문구 날짜는 2026-10-17이며 현재 변경일과 일치한다. |
| 5 | Hangul oracle이 계획 자료에 실제 있는가 | 고정 Noto Sans KR 샘플과 DOM 관찰이 precomparison README에 있다. |
| 6 | CJK 줄바꿈 문구가 관찰을 과장하는가 | line-break:strict fixture와 관찰된 줄 묶음이 기록돼 있다. 로드맵은 전체 CJK 적합성을 주장하지 않는다. |
| 7 | 결합 문자 지원 완료로 오인되는가 | 로드맵은 관찰 범주만 열거하고 Spinon 구현 완료를 주장하지 않는다. |
| 8 | ligature 관찰을 Canvas 값으로 잘못 일반화하는가 | 요약은 ligature oracle 범위만 말한다. 세부 한계는 연결한 README가 Canvas/DOM 차이를 분리한다. |
| 9 | Arabic/Bidi 범위가 전체 RTL 지원을 암시하는가 | 문구는 샘플 범주만 표기하며 RTL 지원 상태를 체크하지 않는다. |
| 10 | Emoji ZWJ 결과가 glyph ID 확인으로 과장되는가 | 같은 문단에 실제 glyph ID 미확인을 명시한다. |
| 11 | Block/Flex/Grid를 제품 레이아웃 성공으로 읽을 수 있는가 | Chrome 155 DOM 관찰로 한정하고 Spinon shaping 결과가 없다고 적는다. |
| 12 | overflow 사례가 화면 바깥 픽셀 검증을 암시하는가 | 실제 앱 실행이 없다고 밝히며 WGPU paint도 미확인으로 분리한다. |
| 13 | clip을 compositor pixel 증거로 해석하게 되는가 | 문구는 oracle 범주만 말하고 Android/iOS 앱 화면을 검증하지 않았다고 한다. |
| 14 | hit-test가 Spinon 입력 경로 완료로 보이는가 | Chrome DOM oracle로 한정했고 Spinon 실행 결과를 주장하지 않는다. |
| 15 | paint order가 WGPU pixel exact로 오인되는가 | WGPU paint 미검증을 명시한다. |
| 16 | 시스템 font fallback을 고정 oracle로 오인하는가 | 고정 Noto 글꼴 자산을 명시하고 연결 README에 자산 provenance가 있다. |
| 17 | Chrome 155 자료가 WPT/Spinon 적합성으로 표현되는가 | DOM 관찰이라고 한정하고 WPT pass나 conformance 주장은 없다. |
| 18 | Android/iOS 검증 범위를 과장하는가 | AVD·Simulator 앱 실행이 아직 없다고 정확히 적는다. 실기기 통과를 암시하지 않는다. |
| 19 | 공식 상태 체크나 완료 수를 몰래 바꾸는가 | 변경 경로는 roadmap HTML과 evidence index뿐이며 spec/STATUS.md는 수정하지 않는다. 동적 완료 수 계산도 변경하지 않는다. |
| 20 | 새 evidence 링크가 Rspress route를 가리키지 못하는가 | 대상 Markdown 파일은 spec/internal/evidence/.../README.md에 존재하고 기존 preview는 같은 internal/evidence/.../README.html 평탄 경로를 쓴다. 실제 제공은 Tailnet sync 후 Agent Browser로 별도 확인한다. |

## 확인 결과

문구·근거 링크의 범위 오류는 발견되지 않았다. Tailnet 제공 여부와 렌더 화면은 이 PR 병합 후 별도 copy-only 동기화 PR에서 검증한다.
