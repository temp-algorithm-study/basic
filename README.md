# basic

## Rust

```
rust/
├── Cargo.toml        # workspace (members에 패키지 추가)
├── print/            # 주제별 패키지
│   └── src/bin/      # 문제별 파일
└── sorting/          # cargo new sorting 으로 추가
    └── src/bin/
```

```bash
cd rust
cargo new 주제명              # 새 패키지 생성
# Cargo.toml members에 추가
cargo run -p 주제명 --bin 파일명
```