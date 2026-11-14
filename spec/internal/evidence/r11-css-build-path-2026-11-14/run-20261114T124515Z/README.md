# R11 · 모바일 CSS 변환 경로 증거 범위 확인

**판정:** 현재 R10 sample을 converter에 넣어 기존 versioned JSON과 byte-exact 일치하는 산출물을 재현했다. 이 확인은 warning 완전성, CSS 오류 진단의 구분, 강제 종료 중 partial write 안전성 또는 R11 고유 완료 기준을 검증하지 않는다. 따라서 R11 공식 상태는 미완료다.

**공식 상태:** [R11은 계속 미완료](../../../../STATUS.md) · **기준 HEAD:** `401911e6b78b543e3c9553c3e6e162e43ca381b2` · **branch:** `codex/r11-css-build-path-plan-2026-11-14` · **run ID:** `run-20261114T124515Z`

**PR 변경분 검토:** [서로 다른 30개 실패 관점](./pr-diff-review-30.md)

## 관찰 결과

- [R10 converter](../../../../../spikes/style-layout/compile-css.mjs)의 `sample.css`를 현재 HEAD에서 변환해 새 JSON 출력을 만들었다. 결과는 `version: 1`이고 두 class의 style field를 담으며, tracked [`styles.json`](../../../../../spikes/style-layout/styles.json)과 byte-exact 일치했다. input SHA-256은 `f4d103…5a53`, output SHA-256은 `83c2f7…3ae5a`이다. 원본과 output은 저장소 밖 임시 디렉터리에 두었다.
- 현재 verify.mjs는 12개 converter 경계를 통과했다(exit 0). 알 수 없는 속성·선택자·중복 규칙·at-rule·important·백분율·음수 값·문법 오류 거부, 평범하지 않은 class명, input/output 동일 경로 거부를 포함한다. 보존된 로그는 합계 한 줄뿐이어서 개별 parser 오류와 visitor 거부의 진단을 비교했다고 주장하지 않는다.
- `display:flex`와 `color:red`를 한 규칙에 둔 mixed positive/unsupported 입력은 exit 1로 실패했고 output JSON은 만들어지지 않았다. `color` 거부 원문은 [`cases/mixed-unsupported/stderr.txt`](./cases/mixed-unsupported/stderr.txt)에 있다.
- [R14 converter 결과](../../r14-tailwind-css-2026-10-16/README.md)의 보존된 JSON은 전체 Tailwind CSS 거부, 분리한 `.flex` positive control 성공, spacing/color token·hover·media·Preflight negative controls의 결과와 output 존재 여부를 기록한다. 해당 R14 source revision의 converter·lockfile·generated CSS·results JSON hash를 확인했다.

## 실행 환경과 경계

- repository realpath는 `/private/tmp/spinon-r11-css-build-path-plan-20261114`, HEAD는 위 commit이다. 실행 전 전체 status에는 untracked R11 계획 초안과 20점 계획 검토 파일 두 개만 있었다. `spikes/style-layout/`, R10/R14 근거와 R14 계획의 staged/unstaged status와 diff는 비어 있었고, 실행 전후 converter input·source·기존 output hash가 HEAD와 같았다. 상세 값은 [`initial-source-snapshot.txt`](./initial-source-snapshot.txt)와 [`post-run-source-snapshot.txt`](./post-run-source-snapshot.txt)에 있다.
- 실행 임시 경로 /private/tmp/spinon-r11-css-build-path-2026-11-14.VBV0nL은 realpath 비교로 worktree 밖임을 확인했다. evidence run-id 경로가 비어 있음을 확인한 뒤 새로 만들었다. 최초 복사 manifest는 supplemental R14 hash가 historical-source-hashes.txt에 추가되고 toolchain.txt의 끝 공백이 정규화되기 전 값이다. 그 두 파일 hash가 현재 bytes와 다르므로 staged-files.sha256은 역사적 자료이며 현재 file verifier가 아니다. 현재 final-evidence-sha256.txt는 그 자체를 제외한 나머지 35개 파일을 hash한다. 저장소 root에서 `cd spec/internal/evidence/r11-css-build-path-2026-11-14/run-20261114T124515Z && shasum -a 256 -c final-evidence-sha256.txt`를 실행해 다시 확인할 수 있다. 최초 33개 항목 검사 출력은 [manifest-verification-before-log.txt](./manifest-verification-before-log.txt)에 보존했다.
- Bun 1.4.2, Node v24.20.0, 설치된 Lightning CSS 1.33.0; resolved module은 기록한 worktree의 node_modules 경로였다. version·resolution probe와 raw output은 [toolchain.txt](./toolchain.txt), [resolution-probe.stdout](./resolution-probe.stdout), [install.stdout](./install.stdout)에 보존했다. 해당 ignored node_modules의 실행 전 존재 여부는 기록하지 못해 보존했다.

## 역사적 증거 경계

- R10 converter·package·lockfile·sample·`styles.json`은 R10 source commit `5ad70525de730b6ad1e354fbf4a466034120346d`부터 현재 HEAD까지 같은 SHA-256이다. R10 commit은 현재 HEAD의 조상이다. R10 요약 Markdown은 그 당시 blob `eb3fd7…54c3a`에서 현재 `476b60…c1d901`로 후속 편집됐다. 따라서 보고서의 최신 설명과 당시 원문 hash를 구분했다.
- R14 evidence commit d72606f7ee76b4ac122722d7d74af03335002f20은 현재 HEAD의 조상은 아니지만, 조상인 7ff0430550cdfdd604ccc591244dabed2d113182와 같은 parent·tree를 갖는다. R14 converter, Tailwind generator package/lock/input/fixture/verify source, generated CSS와 converter results JSON blob은 두 revision과 현재 HEAD에서 동일하다. R14 README는 과거 hash에서 후속 편집됐다. 전체 hash·revision 기록은 [historical-source-hashes.txt](./historical-source-hashes.txt)에 있다. 역사 실행의 개별 control raw logs는 보존돼 있지 않다.

## 남은 한계

- verify.mjs의 12개 input case가 통과했지만 case별 stderr/stdout과 parser-vs-visitor 진단 결과는 보존되지 않았다. 강제 종료, 파일 쓰기 실패, partial JSON 및 실행 전 output lstat/symlink preflight도 재현 자료가 없다.
- 원본 초기 snapshot에는 temp path가 포함돼 있어 snapshot과 해당 temp directory 생성의 선후를 입증하지 못한다. original run temp의 archived copy를 모두 SHA-256 확인한 뒤 [cleanup-follow-up.txt](./cleanup-follow-up.txt)에 기록하고 삭제했다.
- R14 generated CSS·result JSON과 Tailwind generator package/lock/input/fixture/verification source hash를 history/current HEAD 사이에서 확인했다. 역사적 생성 command의 per-control raw log와 tool invocation output은 복원되지 않아 남은 제한이다.

이 결과는 정적 R10/Taffy 개발 실험의 좁은 converter 가능성만 확인한다. JSON은 공개 앱 API나 제품 bundle 계약이 아니다. architecture는 Lightning CSS 변환을 제품 CSS 경로로 선택하지 않았고 CSS 계산에는 Stylo를 둔다([architecture](../../../../../docs/architecture.md)). 이 조사에서는 Rust consumer tests, browser, Android/iOS app, simulator, WPT 또는 성능을 실행하지 않았다.

compile-css.mjs는 transform 반환값을 변수에 보관하지 않는다. 이번 sample의 stdout/stderr에 경고가 보이지 않았다는 사실은 반환 metadata의 warning 완전성을 입증하지 않아 미확인이다. 과거 R14 JSON은 당시 결과를 기록한 증거이며 이 run에서 다시 실행한 결과로 보지 않는다.

R11 상태 문구는 별도 조사 범위를 R10보다 좁게 또는 넓게 요구하는지 정하지 않는다. 따라서 이 결과만으로 checkbox·공식 완료 수·제품 CSS 지원을 바꾸지 않는다. 구체적인 R11 scope가 결정되면 현재 증거를 재사용하고 필요한 차이만 별도 검증해야 한다.
