# 개발·빌드·릴리스 가이드

개발자용 내부 문서입니다. 사용자용 소개는 [README.md](../README.md)（일본어）를 참고하세요.

---

## 릴리스 만들기

태그를 push하면 GitHub Actions가 자동으로 빌드하고, [GitHub Releases](https://github.com/noelfania/key2gksrmf/releases)에 zip을 첨부합니다.

워크플로: [.github/workflows/release.yml](../.github/workflows/release.yml)

```bash
git tag v0.1.0
git push origin v0.1.0
```

- `Cargo.toml`의 `version`과 태그(`v0.1.0`)는 맞춰 둡니다.
- Releases에는 `key2gksrmf-v*-win64.zip`만 올립니다（Edge SmartScreen으로 exe 단독 다운로드가 막히는 경우가 있어 zip만 배포）.

## GitHub Pages (웹 데모)

- 워크플로: [.github/workflows/pages.yml](../.github/workflows/pages.yml)
- 배포 URL: `https://noelfania.github.io/key2gksrmf/`
- `main` 브랜치 push 시 자동 배포

로컬 실행/빌드:

```bash
cd web
npm install
npm run dev
npm run build
```

---

## 빌드 환경

| 항목 | 내용 |
|---|---|
| Rust | edition 2021 |
| 플랫폼 | Windows 10/11 |
| SDK | Win32 빌드 환경（Windows SDK） |

### Rust 설치（Windows）

```bash
winget install Rustlang.Rustup
rustc --version
cargo --version
```

---

## 빌드·테스트

```bash
# 개발 빌드
cargo build

# 릴리스 빌드（권장）
cargo build --release

# 테스트
cargo test
```

출력: `target/release/key2gksrmf.exe`

릴리스 프로필은 `Cargo.toml`에서 `lto`, `strip`, `opt-level = "z"`로 경량화합니다.

---

## 관련 문서

- [상세설계](app-detailed-design.md) — 모듈·API·구현 흐름
- [개발 이슈 정리](issues/issues.md) — 고민·결정·미확인 항목
- [서비스사양](../.cursor/rules/project-standards.mdc) — 목적·범위·기능 요약
