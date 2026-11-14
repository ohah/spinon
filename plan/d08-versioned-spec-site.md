# D08 버전별 명세 사이트 계획

- **상태:** 계획 초안 · 사이트 구현·배포 미시작 · D08 공식 완료 미표시
- **공식 항목:** [D08 버전별 명세 사이트](../spec/STATUS.md)
- **기준 문서:** [README](../README.md) · [명세 인덱스](../spec/README.md) · [API 명세 목록](../spec/api/README.md) · [지원 완료 API](../spec/api/index.md) · [문서 사이트 설정](../packages/docs/rspress.config.ts) · [수동 Pages workflow](../.github/workflows/docs-pages.yml)
- **범위:** 현재 문서의 버전 출처·콘텐츠 소유·URL·검색·예제·마이그레이션·license·publication 결정과 acceptance 질문을 연결한다. 사이트 구현, Rspress 실행, 빌드, preview, Pages 배포는 포함하지 않는다.

## 목표와 현재 경계

D08은 저장소 Markdown 명세를 검색 가능한 HTML로 생성하고 API·지원표·예제·마이그레이션·license를 버전별로 공개하는 문서 작업이다. 이 초안은 해당 사이트나 버전별 산출물이 이미 구현됐다고 주장하지 않는다.

현재 Rspress 문서 앱은 `spec/`을 root로 쓰고 기본 출력은 `packages/docs/build/site`, base URL은 `/spinon/`이다. `@rspress/core`는 `2.0.22`에 고정돼 있다. 이 계획에서는 기본 build output을 사용하지 않으며, 현재 preview·배포 process나 문서 원본 파일을 바꾸지 않는다. 이후 구현이 승인되면 active preview 경로와 분리한 output을 정하고, build/deploy gate를 별도 검증한다.

`spec/README.md`의 숫자 명세 버전 `0.1.0`은 첫 출시 전 고정된 계약 버전이다. 앱의 release version, D08 문서 snapshot의 version label, 개별 공개 API의 first-supported version과는 서로 다른 버전 축일 수 있으므로 같은 값이라고 가정하지 않는다. 현재 지원 완료 API 목록은 0개다. `spec/` 제안 계약·내부 구현 상태·향후 지원 API를 사이트에서 같은 상태처럼 표시하지 않는다.

## 결정 영역

| 경계 | 현재 원본 | D08 계획에서 남길 결정 |
|---|---|---|
| 버전 출처·표시 | `spec/README.md`의 내부 계약 `0.1.0`은 출시 전 고정이다. `spec/STATUS.md`는 구현 상태의 공식 원본이며 README는 첫 공식 제품 release 미정으로 설명한다. | 문서 snapshot을 어떤 immutable Git tag/release manifest에 결속할지, unreleased draft와 첫/후속 release를 어떤 표식으로 구분할지 정한다. 내부 계약 숫자 버전을 제품·문서 배포 버전으로 승격하지 않는다. |
| URL·canonical 경로 | 현재 GitHub Pages base는 `/spinon/`; 기존 README·preview 링크는 unversioned 경로를 가리킨다. | versioned URL 형식, 기본 경로의 역할, `latest`/draft alias 의미, 과거 URL redirect·canonical 주소 정책을 정한다. 승인 전 구체 경로·alias가 이미 유효한 계약인 것처럼 쓰지 않는다. |
| 콘텐츠 소유 | `spec/`은 제안 동작 계약, `spec/api/`는 구현·근거를 갖춘 지원 API, `STATUS.md`는 공식 구현 상태를 소유한다. 현재 지원 API는 없다. | 각 버전에서 어느 source revision을 읽을지, source section을 API·지원표·예제·마이그레이션·license 페이지에 어떻게 연결할지, 미작성 자료의 owner를 정한다. source를 복사하면 기준 revision과 동기화/폐기 규칙을 둔다. |
| 지원·미지원 배지 | 제안 계약·미정 항목은 구현 또는 지원 선언이 아니며, 완료 판정에는 API/인터페이스 명세와 실제 evidence가 필요하다. | 승인된 API만 `supported`로 내보내고, proposed·unreviewed·unsupported·platform-limited·evidence-pending 상태를 혼동하지 않을 표시 원본과 변환 규칙을 정한다. 로드맵 숫자와 페이지별 지원 상태를 임의로 재계산하지 않는다. |
| 예제·마이그레이션 | API 문서는 지원 버전, 성공·오류·수명, 플랫폼 차이, 실행 예제를 요구한다. | 예제의 source version·dependency lock·검증 platform, migration guide의 이전/대상 release와 지원 기간을 묶는다. 실제 검증되지 않은 snippet을 실행 가능하다고 표시하지 않는다. |
| 검색·탐색 | Rspress는 정적 HTML을 만들고 현재 하나의 `spec/` tree를 입력으로 받는다. | 검색 index, sidebar, version picker, cross-version link의 scope와 기본 선택 규칙을 정한다. 검색 결과가 버전 간 중복·최신 상태 혼합을 일으키지 않게 acceptance를 둔다. |
| License·고지 | D08은 API·예제·migration·license를 버전별로 공개하도록 요구한다. 기존 source와 third-party notice의 책임 위치는 구현 전 확인해야 한다. | license 원본의 owner, third-party dependency notice를 어떤 snapshot에서 가져올지, 보존/표시 형식과 변경 추적 방법을 확인한다. 없는 license 또는 권리 결정을 추정하지 않는다. |
| 산출물·저장소 수명 | 기본 Rspress output은 `packages/docs/build/site`; workflow는 `workflow_dispatch`만 허용한다. | 버전별 빌드 산출물을 저장·보관·삭제하는 위치, atomic publish·rollback, CDN cache와 오래된 링크 보존, 중복 용량을 정한다. active preview output을 덮지 않는다. |
| release·배포 gate | 첫 공식 release 전 자동 publish 금지, Pages workflow는 수동 실행, Rspress `2.0.22` 고정. | 수동 승인자, release tag 검증, preview·production 분리, workflow input 검증, `--delete` 동작, 이전 버전 보존과 재배포 기준을 정한다. 사용자가 요청하지 않은 build/deploy를 이 계획에서 실행하지 않는다. |

## 책임과 선행 결정

| owner | 필요한 연결 | 계획 단계에서 허용하는 결론 |
|---|---|---|
| `spec/`·`spec/api/` maintainers | 제안 계약, 지원 완료 API, 상태 대장, migration/example/license source의 authoritative owner | source-of-truth 목록과 빠진 owner를 식별한다. 계획 초안만으로 어떤 API도 지원으로 게시하지 않는다. |
| Release/version owner | app release tag, internal contract version, API first-supported version 간 관계 | 미정 관계를 선택지와 승인 gate로 기록한다. `0.1.0` 숫자 계약을 변경하지 않는다. |
| Docs owner | Rspress navigation/search/theme, public URL 및 preview URL | versioned routing·search·redirect acceptance를 정의한다. 현재 설정과 lockfile은 변경하지 않는다. |
| Pages/release owner | 수동 workflow, production destination, retention/rollback | first-release 전 배포 금지와 사용자 승인 경계를 보존한다. workflow를 실행하거나 gh-pages를 갱신하지 않는다. |
| D08 | 위 결정을 묶어 reproducible, immutable, source-pinned document set으로 만든다. | 모든 version dimension·source·route·retention·license 결정의 책임자가 확인되기 전 구현을 시작하지 않는다. |

## 계획 acceptance matrix

아래는 구현 전 결정할 실패 경로와 관찰값이다. 현재 사이트가 이 조건을 통과했다는 실행 결과가 아니다.

| 입력·실패 상황 | 계획에서 정할 검증 경계 |
|---|---|
| unreleased working tree와 versioned release source가 혼합 | draft/preview와 released snapshot의 source SHA, label, URL·검색 범위를 분리하고 release source가 바뀌면 생성 작업이 중단되는지 정한다. |
| 문서 version label과 internal contract/API/app version이 서로 다름 | 각 버전 축과 표시 owner를 명시하며 누락·모순된 mapping은 release gate에서 거부하는 조건을 정한다. |
| D08 snapshot에서 proposed API가 supported로 승격 | `spec/api/`의 승인 상태와 구현 evidence를 따라야 하며 `spec/` draft나 STATUS mention만으로 지원 배지를 만들지 않는다. |
| Rspress tree에 source revision 이후 draft 변경 또는 누락 파일이 있음 | source tag/revision와 built pages manifest 연결, dirty source·untracked generated evidence 처리 규칙을 정한다. |
| 오래된 version URL, unversioned 링크 또는 canonical URL이 사라짐 | 기존 published URL inventory, redirect/alias, link checker scope와 오래된 snapshot retention을 정한다. |
| 검색에서 다른 release의 API·migration 결과가 섞임 | selected version filter, cross-version result 표시, draft 제외·포함 정책과 검색 index manifest를 정한다. |
| migration guide 대상 release가 없거나 순서가 건너뜀 | source/target version의 연속성, unsupported jump 안내, dependency requirements와 예제 검증을 정한다. |
| 예제의 dependency·환경이 문서 표기 version과 다름 | lockfile·runtime·platform profile을 source-pinned record로 검증하고 실행 검증 없는 예제를 명확히 구분한다. |
| license/third-party notice가 version snapshot에서 누락 또는 stale | notice source, generation check, attribution, owner 승인과 실패 시 publish block 범위를 정한다. |
| active preview와 production/versioned output이 같은 디렉터리를 사용 | build output이 분리되고 실패·취소 시 기존 preview/published artifact를 보존하도록 격리 acceptance를 둔다. |
| partial build 또는 이전 버전 갱신이 최신 링크를 덮어씀 | version directory atomicity, active alias update order, rollback 대상과 failed publish state invariant를 정한다. |
| Pages workflow가 의도치 않게 자동 실행되거나 delete 범위가 넓음 | workflow_dispatch·승인·source tag·destination allowlist·`rsync --delete` scope를 고정한다. 자동 배포 경로를 추가하지 않는다. |
| 기존 URL의 API 상태가 새 STATUS 수치와 맞지 않음 | 지원 표시는 source revision에 묶고 site generation time·revision·status count freshness를 표시하거나 빌드를 거부한다. |
| 다수 version snapshot으로 산출물·검색 index·license tree가 과도하게 커짐 | 보관 정책·저장 quota·삭제 권한·복구 가능한 backup window를 정하고 build 자원 한도를 확인한다. |

## 단계와 중단 조건

1. 현재 GitHub Pages, Tailscale preview와 로컬 Rspress URL을 inventory하고 각각의 운영 owner·source SHA·기존 링크를 기록한다. inventory를 위해 기존 서버를 종료하거나 output을 갱신하지 않는다.
2. app release version, internal contract version, API support version, docs snapshot label의 관계를 release owner가 승인할 수 있도록 선택지와 migration cost를 정리한다.
3. `spec/`, `spec/api/`, `STATUS.md`, examples, migration, license/third-party notices의 source owner와 snapshot inclusion을 연결한다.
4. versioned path, old-link redirects, `latest`/draft semantics, search isolation, retention와 rollback acceptance를 결정한다.
5. 수동 build·release tag 확인·output 격리·atomic publish 절차를 정하고 Rspress `2.0.22` 및 Pages workflow manual gate를 유지한다.
6. 구현 후에는 source-pinned multi-version fixture, broken-link/redirect/search/license matrix, supported-vs-proposal badge matrix와 browser QA를 별도 구현·검증 작업으로 계획한다.

- 제품/API/support/version 정책의 owner가 확정하지 않은 선택을 D08 계획이 대신하지 않는다.
- 기존 `/spinon/` 주소, Tailscale preview route 또는 published content를 승인 없이 바꾸지 않는다.
- Rspress build가 active `packages/docs/build/site` output을 사용할 수 있거나 untracked `spec/` 입력을 읽을 우려가 있으면 격리 경로와 source allowlist가 준비될 때까지 build를 보류한다.
- 첫 공식 release 전 자동 Pages 배포는 금지하고, 수동 workflow도 별도 명시 요청 없이는 실행하지 않는다.
- 이 계획은 D08 checkbox, 78/214 공식 집계, internal/API version, Rspress dependency, workflow, docs site output을 변경하지 않는다. 버전별 사이트 구현·게시·지원 API·첫 공식 release를 완료로 표시하지 않는다.
