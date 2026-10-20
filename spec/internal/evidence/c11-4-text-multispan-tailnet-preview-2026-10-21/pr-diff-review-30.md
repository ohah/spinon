# Tailnet C11.4 문서 PR diff 적대적 검토 30개

검토 대상은 PR #409의 상태·계획·로드맵·실기기 실행 기록과 Tailnet 정적 미리보기 증거다. 모든 점검은 2026-10-21 작업 브랜치와 pass27 산출물에 대해 확인했다.

| # | 별도 실패 관점 | 확인과 결과 |
|---:|---|---|
| 1 | 문서 변경이 구현 파일을 끌어들임 | 변경 목록을 제한된 문서·증거 경로와 대조했다. Android 앱 소스 변경은 없다. |
| 2 | C11.4를 완료로 잘못 표시 | 공식 상태의 C11.4 항목은 미완료 상태다. |
| 3 | 공식 완료 수가 함께 증가 | 이번 문서 변경에서 체크 상태나 집계를 바꾸지 않았다. |
| 4 | PR #408 병합 SHA 오기 | 계획·상태·로드맵의 SHA가 `4b7282116c0aeab12ba10edeae51fcf7f7bb3a33`과 일치한다. |
| 5 | check rollup 부재를 CI 성공으로 확대 | PR #408에 보고된 check가 없다는 한계를 명시했고 성공 주장은 없다. |
| 6 | 로드맵에 오래된 화면 링크가 남음 | pass27 로드맵 링크가 새 실기기 캡처 디렉터리를 가리킨다. |
| 7 | 증거 README가 이전 실행을 최신으로 표시 | 표·화면 삽입 링크가 `20261020T215102621Z-daee998f` 실행과 일치한다. |
| 8 | 테스트 대상을 AVD로 오인 | 기기 metadata에서 `qemu=0`, Samsung SM-S731N, Android 16/API 36을 확인했다. |
| 9 | 다른 기기의 APK가 설치됨 | 로컬 APK와 기기의 `base.apk` SHA-256이 `700db402...f2439c`로 일치한다. |
| 10 | 다중 장면 중 일부만 실행 | 결과에는 4개 scene이 있고 각 scene이 통과했다. |
| 11 | span-2 순서 결과 누락 | forward와 reversed scene에서 `[25, 35, 35, 0]`을 확인했다. |
| 12 | span-2/3 순서 결과 누락 | 두 ordering scene에서 `[53.333335876464844, 63.333335876464844, 63.333335876464844, 0]`을 확인했다. |
| 13 | track 값만 맞고 node frame이 틀림 | 네 scene의 전체 node frame이 결과 JSON에 있으며 runner assertion을 통과했다. |
| 14 | log가 layout 완료만 보여 draw를 놓침 | 모든 관찰에 `drawPresented=true`가 기록됐다. |
| 15 | fixture 결과를 글리프 표시 증거로 과장 | README는 `runtime_text_count=0`, `glyph_batch_draw_count=0`과 paint 한계를 표시한다. |
| 16 | 화면이 이전 실행에서 복사됨 | screenshot은 새 실기기 실행 디렉터리에 있고 scene 실행 후 기기에서 직접 저장됐다. |
| 17 | 화면 크기나 원본 해시가 불일치 | PNG는 1080×2340이며 remote manifest와 동일한 110,147 bytes / SHA-256 `8f7cdc99...61860f5`다. |
| 18 | Markdown 포함 이미지가 빌드에서 빠짐 | RSPress HTML의 해시 이미지 URL이 pass28 산출물에 있고 Agent Browser에서 완전히 로드됐다. |
| 19 | 로드맵의 직접 PNG 링크가 404 | 원본 경로 PNG를 `public/spec/internal/...`에 포함했고 Tailnet 및 Agent Browser에서 200으로 열었다. |
| 20 | image URL이 이전 run을 가리킴 | roadmap HTML의 링크가 새 run ID와 일치한다. |
| 21 | 문서가 잘못된 URL base로 렌더링 | 선택 RSPress config의 `/spinon/` 경로에서 roadmap·STATUS·plan·evidence 페이지를 확인했다. |
| 22 | Markdown 이미지 변환만 되고 공개 route 누락 | 네 문서와 원본 PNG·result JSON URL을 Agent Browser에서 직접 열어 제목·내용을 확인했다. |
| 23 | 상태·계획 페이지가 병합 전 상태를 표시 | 공개 STATUS와 plan에 PR #408 병합 SHA가 보인다. |
| 24 | 상태 체크박스가 시각적으로 바뀜 | 공개 STATUS의 C11.4 checkbox `checked=false`를 확인했다. |
| 25 | pass28 대상 목록에 빌드 외 파일 포함 | preflight 목록은 선택 site output의 1,735개 파일과 일치한다. |
| 26 | 기존 파일이 백업 없이 덮임 | 교체 대상 96개의 이전 bytes·SHA를 별도 backup에 보존하고 검증했다. |
| 27 | 미리보기 파일을 정리하며 사용 자료 삭제 | 동기화 manifest에서 삭제는 0건이다. |
| 28 | 복사 완료만 확인하고 대상 바이트 미검증 | 로컬 대상 1,735/1,735와 백업 96/96이 SHA-256 일치다. |
| 29 | HTTP 200이지만 오래된 파일 제공 | Tailnet 1,735개 경로의 served bytes·SHA가 source manifest와 모두 일치한다. |
| 30 | 브라우저 검증 후 공유 서비스까지 종료 | Agent Browser session만 닫았고 Tailnet preview PID `1191`은 계속 제공 중이다. |

최종 검증 상세는 [Tailnet 미리보기 실행 기록](./README.md), [preflight](./preflight-pass28.json), [로컬·백업 검증](./final-verification-pass28.json), [원격 URL·SHA 검증](./served-verification-pass28.json)에 있다.
