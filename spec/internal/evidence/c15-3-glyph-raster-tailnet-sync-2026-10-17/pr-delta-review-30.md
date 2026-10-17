# C15.3 glyph raster Tailnet 동기화 PR 변경 적대 검토 30개

검토 대상은 PR에 넣을 로드맵·내부 색인·동기화 근거와 별도 Tailnet 정적 미리보기 복사 절차다. 각 항목은 다른 실패 경로나 경계에서 현재 source, 생성 페이지, 제공 바이트와 화면을 대조했다.

1. **완료 상태 부풀림:** roadmap 문구가 구현 PR #248/#250과 준비 단계를 설명하며 C15 완료를 주장하지 않는지 확인 — 통과.
2. **상위 작업 경계:** C15, C16, C19가 미완료로 남고 완료 체크 수가 증가하지 않는지 확인 — 통과.
3. **시뮬레이터 주장:** 이번 문서 sync가 Android/iOS Simulator 앱 검증을 주장하지 않는지 확인 — 통과.
4. **수치 일치:** 35/35, 154/154, Clippy, rustfmt 수치가 raster 실행 로그와 맞는지 확인 — 통과.
5. **명세 범위:** glyph 순서, source mapping, CSS 위치, DPR 및 fractional phase가 계약 0071과 모순되지 않는지 확인 — 통과.
6. **paint 경계:** roadmap이 WGPU draw나 앱 runtime text가 연결되었다고 말하지 않는지 확인 — 통과.
7. **clip·hit-test 경계:** 부모 clip, overflow ink, hit-test 미검증을 누락하거나 완료로 돌리지 않았는지 확인 — 통과.
8. **color glyph 경계:** color glyph compositing을 지원한다고 암시하지 않는지 확인 — 통과.
9. **PR 참조:** shaping #248과 raster #250의 역할을 뒤바꾸거나 병합 범위를 합쳐 과장하지 않는지 확인 — 통과.
10. **source 기준점:** 문서와 manifest의 시작 commit이 PR #250 병합 뒤 `a9a8a5117892514ded6fdb0d8a22ab931af643f6`인지 확인 — 통과.
11. **공식 상태 원본:** `STATUS.md`의 C15 문구와 선택 빌드 출력이 같은 branch source에서 왔는지 확인 — 통과.
12. **로드맵 색상 진단:** Chrome 표본에 한정하고 Finch 상태 미보존을 밝히며 Local MINDE exact 일치를 요구하지 않는지 확인 — 통과.
13. **Tailnet 색인 링크:** 내부 색인에서 새 sync 증거로 가는 상대 링크가 실제 파일에 연결되는지 확인 — 통과.
14. **미러 페이지 동등성:** `roadmap.html`과 `public/roadmap.html`, `internal/`과 `spec/internal/`의 복사 대상이 같은 source bytes를 쓰는지 확인 — 최종 hash 검사 통과.
15. **Rspress 버전:** 문서 작업이 고정된 Rspress `2.0.22`를 바꾸지 않는지 확인 — 통과.
16. **전체 빌드 실패 공개:** 전체 빌드의 기존 dead link, `rust,ignore` fence 및 이미지 경로 실패를 숨기지 않는지 확인 — 통과.
17. **선택 렌더 한계:** 선택 빌드 성공을 전체 문서 링크 검사 또는 제품 검증으로 확대하지 않는지 확인 — 통과.
18. **생성 페이지 범위:** 상태, 계획, 색인, 계약, raster 근거, sync README와 review가 빠지지 않는지 확인 — 선택 산출물 목록 통과.
19. **정적 asset 완전성:** 생성 페이지가 참조하는 CSS·JS와 아이콘 파일이 복사 목록에 포함되는지 확인 — 최종 manifest와 HTTP 검증 통과.
20. **캡처 누락:** 로드맵·상태 대장·계약 0071 캡처가 README 및 Tailnet 경로에 포함되는지 확인 — 통과.
21. **캡처 진위:** 각 PNG가 Agent Browser에서 현재 Tailnet listener를 연 뒤 저장한 파일이며 임의 재구성 이미지가 아닌지 확인 — 통과.
22. **상대 링크 해석:** sync README의 상대 PNG·JSON·MD 링크가 새 evidence 디렉터리 기준으로 유효한지 확인 — 통과.
23. **base 경로:** 생성 문서와 custom roadmap 경로가 `/spinon/` 아래에서 정상 제공되는지 확인 — HTTP 및 Agent Browser 통과.
24. **manifest 계수:** 최종 manifest의 route·byte·created/replaced/unchanged 집계가 preflight 및 copy verification과 맞는지 확인 — 통과.
25. **백업 우선:** 기존 파일 중 바뀌는 대상은 전용 `backup/` 경로에 덮어쓰기 전에 보존하는지 확인 — 통과.
26. **백업 무결성:** 모든 이전 파일 backup SHA-256이 preflight의 이전 값과 같은지 확인 — 통과.
27. **삭제 범위:** 복사 절차가 기존 Tailnet 미리보기 파일을 삭제하지 않고 최신 build에 없는 hash asset도 그대로 두는지 확인 — 통과.
28. **HTTP parity:** 모든 manifest route가 HTTP 200과 expected SHA-256을 반환하고 `served-verification.json`의 flat/spec 두 경로도 hash 확인을 통과하는지 확인 — 통과.
29. **프로세스 수명:** Agent Browser 주 세션과 빈 보조 세션을 모두 닫고 browser process가 남지 않는지, Tailnet listener PID `1191`은 유지되는지 확인 — 통과.
30. **변경 범위 격리:** diff가 roadmap 문구·내부 색인·sync 증거에 한정되고 Rust 제품 코드나 완료 체크를 건드리지 않는지, 더러운 주 worktree와 분리된 전용 worktree에서 진행하는지 확인 — 통과.

초기 초안의 임시 `generated_at`과 20개 검토 표기는 최종 copy 시각을 기록한 manifest와 서로 다른 실패 관점을 다룬 30개 검토로 교체했다.
