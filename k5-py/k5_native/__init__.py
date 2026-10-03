"""K5 Native — Rust-accelerated K5 hash (PyO3 bindings).
Falls back to pure Python if native module is not available.
"""
try:
    from k5_native import k5_512, k5_1024, k2048, kof, k5_h
except ImportError:
    import sys
    sys.path.insert(0, r"..\src")
    from k5 import k5_512, k5_1024, k2048, kof, k5_h
