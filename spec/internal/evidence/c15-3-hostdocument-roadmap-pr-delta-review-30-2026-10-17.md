# C15.3 HostDocument text paint roadmap · PR 변경 적대 검토 30개

**검토 범위:** `spec/public/roadmap.html`에 추가한 PR #264 제한 slice 요약을 기준 PR, `spec/STATUS.md`, 계약 0075, 구현 근거와 대조했다. 사용자에게 보여 주는 상태·수치·링크·플랫폼 경계를 각기 다른 실패 관점에서 확인했다.

| # | 독립 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 타임라인이 아직 열려 있는 PR을 병합됐다고 표시한다 | PR #264 조회 결과 `MERGED`; 노트의 병합 표기가 현재 PR 상태와 같다. |
| 2 | PR #264를 계획 변경으로 잘못 분류한다 | 제목과 계약 0075는 HostDocument text layout→runtime glyph paint 구현을 가리킨다. |
| 3 | C15.3 하위 fixture를 전체 C15.3 완료로 승격한다 | 노트에 `제한 slice`를 명시하고 일반 DOM·style 경계를 분리했다. |
| 4 | 상위 C15 상태 대장을 완료로 바꾸었다고 오해한다 | C15.3/C15 미완료 상태를 문장에 유지한다. |
| 5 | 전체 완료 수가 부분 fixture 때문에 증가한 것처럼 보인다 | 공식 완료 수 `61/197`을 그대로 썼다. |
| 6 | 폭 15px과 text advance 20px을 서로 같은 값으로 표현한다 | paragraph available width와 한 줄 advance를 각각 15px/20px로 적었다. |
| 7 | 긴 단어가 wrap 또는 clamp됐다고 오해한다 | 한 줄 text를 유지하고 owner 경계 밖 ink를 관찰했다고 기술했다. |
| 8 | paragraph 경계 밖 잉크가 paint되지 않는다고 잘못 표현한다 | owner 경계 바깥 visible ink 확인을 명시했다. |
| 9 | 6/6 결과를 좌표 분포 없이 숫자만 과장한다 | 안쪽 2개, 경계 밖 4개로 표본 분포를 명시했다. |
| 10 | Android 실기기 또는 hardware GPU 결과처럼 보인다 | Android 17/API 37 ARM64 AVD와 PixelCopy라고 특정했다. |
| 11 | Android emulator renderer를 hardware GPU로 오해한다 | AVD는 SwiftShader라고 밝히고 hardware GPU는 미검증으로 둔다. |
| 12 | iOS 기기와 simulator를 혼동한다 | iPhone 17 Pro/iOS 26.2 Simulator라고 한정했다. |
| 13 | Metal drawable 읽기를 compositor 이후 화면 결과로 확대한다 | iOS 표본이 compositor 전임을 명시했다. |
| 14 | 6개 sample과 전체 화면 정확도를 혼동한다 | exact 결과를 여섯 고정 표본으로 한정했다. |
| 15 | PixelCopy와 Metal readback을 한 측정 경로처럼 뭉뚱그린다 | Android PixelCopy, iOS Metal drawable을 별도로 적었다. |
| 16 | fixture 실행을 live V8 DOM Text admission으로 오해한다 | live V8 DOM admission을 제외 범위에 적었다. |
| 17 | 고정 Ahem 스타일을 CSS font projection 완료로 해석한다 | CSS font projection을 별도 미검증 범위로 표시했다. |
| 18 | 단일 `XX`를 일반 wrapping 지원으로 해석한다 | 일반 text wrapping/fallback은 검증하지 않았다고 명시했다. |
| 19 | AVD·simulator 결과를 physical-device 결과로 승격한다 | 실기기를 검증 범위에서 제외했다. |
| 20 | Android 그래픽 결과를 물리 GPU 지원으로 표현한다 | Android hardware GPU 제외를 명시했다. |
| 21 | iOS pre-compositor 결과를 final composed frame으로 표현한다 | compositor 이후 화면을 미검증으로 적었다. |
| 22 | 0075 계약 링크가 없거나 다른 내부 계약을 가리킨다 | `spec/internal/0075-c15-3-hostdocument-text-paint-bridge.md`가 존재한다. |
| 23 | 실행 근거 링크가 계획 precomparison을 가리킨다 | implementation README는 실행·화면·제한 내용을 담는 경로다. |
| 24 | 구현 실패 관점 링크가 잘못된 파일명으로 끊긴다 | `implementation-review-20.md` 경로가 현재 evidence 폴더에 존재한다. |
| 25 | PR 변경 실패 관점 링크가 20개 구버전 파일을 가리킨다 | 새 `pr-delta-review-30.md` 경로를 사용하고 실제 문서가 존재한다. |
| 26 | 병합 PR 링크가 다른 작업을 가리킨다 | 외부 링크는 `ohah/spinon/pull/264`로 고정했다. |
| 27 | Tailnet base path에서 내부 route 링크가 root로 벗어난다 | 세 내부 Markdown 링크는 preview의 `/spinon/internal/...` 경로를 쓴다. |
| 28 | Rspress가 만든 `.html` route와 Markdown source name이 달라 페이지가 404가 된다 | 링크 target은 현재 Tailnet의 Rspress route 규칙에 맞춰 기존 문서들과 같은 `.html` 형식이다. |
| 29 | 새 기록이 앞선 C15.3 scroll slice보다 이전 사건처럼 배치된다 | 새 문단은 non-zero scroll ancestor 결과 뒤에 놓여 동일 날짜 기록의 순서를 유지한다. |
| 30 | 로드맵에서 오래된 “Simulator 미확인” 문장이 새 slice에도 적용되는 것처럼 충돌한다 | 새 문단은 PR #264의 두 simulator 범위를 별도로 적고, 과거 단계의 범위는 해당 문단에 남긴다. |

검토는 roadmap HTML 변경에 한정한다. PR #264의 제품 동작은 별도의 simulator 및 구현 검증 자료에 기록되어 있으며 이 문서는 추가 runtime 증거를 주장하지 않는다.
