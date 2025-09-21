"""
Shim package to allow `python -m pohlang`.
Delegates to Interpreter.run_poh.main() which is the canonical CLI.
"""

from .__main__ import main  # re-export

__all__ = ["main"]
