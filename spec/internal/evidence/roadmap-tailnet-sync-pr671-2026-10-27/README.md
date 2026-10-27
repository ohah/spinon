# PR #671 행 축 span-stage 계획 공개 로드맵 반영

## 변경 범위

`spec/public/roadmap.html`에 병합된 계획 PR #671을 추가했다. 항목은 CSS Grid 2025 CRD §11.5의 두 행 겹침 span 계획, 규범 기대값과 Chrome 156 관찰의 구분, 계획 변경이므로 runtime 근거가 없다는 범위를 설명한다. C11.4는 계속 미완료이고 공식 집계는 `72/213`이다.

근거 문서:

- [계획](../../../../plan/c11-4-row-axis-span-stage-oracle.md)
- [Chrome 156 사전 비교](../c11-4-row-axis-span-stage-precomparison-2026-10-27/README.md)
- [계획 검토 20개](../../../../plan/c11-4-row-axis-span-stage-oracle-plan-review-20.md)
- [계획 PR 변경 검토 30개](../../../../plan/c11-4-row-axis-span-stage-oracle-plan-pr-diff-review-30.md)
- [공식 상태](../../../../spec/STATUS.md)
- [공개 로드맵 화면](./roadmap-pr671-public-row-axis-20261027.png)
- [캡처 메타데이터](./capture-metadata.json) · [SHA-256](./SHA256SUMS)

## 화면 근거

PNG는 Agent Browser에서 이 변경의 `spec/public/roadmap.html`을 로컬 HTTP origin으로 열어 캡처했다. 화면에는 #669 다음의 #671 계획 항목과 공식 `72/213` 문구가 보인다. 이 화면은 정적 로드맵 문구의 렌더링만 증명하며 Spinon runtime·Android·iOS 동작 근거가 아니다. SHA-256은 `8d16d7ed4af9d2aa8c1115bc254424add39d1cf22c1d11f175a647d7294ee4cc`다.

## 빌드 한계

전체 `bun run docs:build`는 이번 변경과 무관한 기존 근거 Markdown의 JSON·로그·폴더 링크를 dead link로 판정해 종료 코드 1로 끝났다. 전체 문서 빌드 성공으로 보고하지 않는다. 공개 HTML은 Agent Browser에서 직접 렌더링했고, 새 로드맵 링크가 가리키는 Markdown·이미지 원본 경로를 확인했다. 이 변경을 병합한 뒤 Tailnet 미리보기에는 선택 경로만 별도로 빌드·동기화한다.
