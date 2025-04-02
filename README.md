# xlsz

## リファクタリング中

以下のような構造にする

root
- README.md
- src
  - Rust 版 xlsz
  - pyo3 に依存させない
- python/xlsz
  - Python 版 xlsz

xlsx reader/writer for Python & Rust.
