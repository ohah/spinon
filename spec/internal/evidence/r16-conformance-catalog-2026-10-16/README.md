# R16 명세·첫 적합성 시나리오 근거 · 2026-10-16

## 판정 범위

R16은 공개 명세 초안과 시나리오 형식 문서 작업으로 완료 판정했다. 공개 문서 `0001`–`0009` 버전은 `0.1.0-draft`이며, API 지원 목록은 계속 0개다. [0009 카탈로그](../../../0009-conformance-scenarios.md)는 SC-UI-001, SC-CSS-001, SC-INPUT-001, SC-ASYNC-001, SC-DIAG-001, SC-LIFE-001, SC-A11Y-001의 입력·oracle·예상 관찰값·negative control·플랫폼별 결과 상태를 제안한다.

일곱 시나리오는 웹·Android·iOS 전부 `미실행`이다. 고정 Chromium JSON, 기존 app/simulator run, unit test 또는 GPU 제출은 이 카탈로그의 통과로 합산하지 않는다. 공개 API·제품 동작·플랫폼 적합성·접근성 지원·WPT 결과는 추가되지 않았다. Android/iOS app build와 simulator 실행은 이 문서 작업에서 하지 않았다.

## 기준과 확인값

| 항목 | 확인값 |
|---|---|
| 비교 기준 | 저장소의 `spec/0001`–`0008`, `spec/api/index.md`, `spec/internal/README.md`, `spec/STATUS.md`와 기존 고정 fixture/evidence |
| 명세 버전 | 0001–0008은 각각 `0.1.0-draft`; 새 0009도 `0.1.0-draft` |
| API 상태 | `spec/api/index.md`: 지원 완료 API 0개 |
| 시나리오 | 7개, 모두 제안·미실행 |
| CSS reference | Chrome `154.0.8037.98`, fixture `C08-block-flow-v1`, viewport `320×240`, DPR 1/2, computed exact·각 geometry field 최대 0.5 CSS px |
| CSS 입력 해시 | HTML `73308088081ac74f356aa2f61ca5240b5c2fa86d5ee848966d4fea8719ec4ed7`; inventory `181fa27f968b26ded1a5ddc1148777e0bda930dbf29dc1ae7fa8f433cf9d2b2a` |
| Chromium reference 해시 | `6f4d4db489379d0d225e4640413cb9f59300176e9a74cf721a5fe4410af676b4` |
| 계획 | [R16 계획](https://github.com/ohah/spinon/blob/main/plan/r16-specification-draft.md) · [계획 검토 20개](../r16-specification-plan-review-20-2026-10-16.md) · [계획 PR 변경 검토 20개](../r16-specification-plan-pr-review-20-2026-10-16.md) |

## 문서 빌드

`mise exec -- bun run docs:build`는 종료 코드 1이었다. R16 변경 전 기준 빌드와 최종 빌드 모두 기존 dead link가 있는 페이지 433개와 `rust,ignore` Shiki 오류 2개를 보고했다. 최종 빌드에는 `0009` 카탈로그와 이 근거 디렉터리의 dead link가 없어 새 R16 링크 오류는 기준 대비 추가되지 않았다. 이 빌드는 시나리오 실행이나 제품 지원을 증명하지 않는다.

## 경계

R16 완료는 시나리오 제안과 미정 계약 등록부를 포함한 문서 산출물에만 적용된다. 성공 기준·오류 서식·플랫폼 version·pixel tolerance 등 표에 `미정`으로 둔 계약은 각 소유 명세에서 결정하기 전까지 지원을 주장하지 않는다.
