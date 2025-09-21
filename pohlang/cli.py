"""PohLang unified CLI.

Usage examples:
  pohlang path/to/file.poh           # run a .poh program (Python interpreter)
  pohlang --debug path/to/file.poh   # run with debug tracing
  pohlang release [--dry-run]        # run release preparation (tests + build)
"""

from __future__ import annotations
import sys
import argparse
from pathlib import Path

# Reuse existing interpreter runners
from Interpreter.run_poh import main as run_poh_main


def _run_release(args) -> int:
    """Invoke the prepare_release script in the repo."""
    script = Path(__file__).resolve().parents[1] / "prepare_release.py"
    if not script.exists():
        print("Error: prepare_release.py not found next to this package.")
        return 1
    # Defer to the script as a module execution to preserve its logic
    # Equivalent to: python prepare_release.py [--dry-run]
    import runpy
    # Simulate argv for the script
    old_argv = sys.argv[:]
    try:
        sys.argv = [str(script)] + (["--dry-run"] if args.dry_run else [])
        runpy.run_path(str(script), run_name="__main__")
        return 0
    except SystemExit as e:
        # the script may call sys.exit; bubble up its code
        code = int(e.code) if isinstance(e.code, (int,)) else 1
        return code
    finally:
        sys.argv = old_argv


def main() -> int:
    parser = argparse.ArgumentParser(prog="pohlang", description="PohLang command-line interface")
    subparsers = parser.add_subparsers(dest="command")

    # release subcommand
    rel = subparsers.add_parser("release", help="Run tests and build distributables")
    rel.add_argument("--dry-run", action="store_true", help="Run checks without building")

    # If no subcommand is provided, treat first argument as .poh file (and pass-through flags to interpreter)
    # We still accept --debug for convenience.
    parser.add_argument("script", nargs="?", help="Path to .poh script to run")
    parser.add_argument("--debug", action="store_true", help="Enable debug tracing when running a script")

    args, unknown = parser.parse_known_args()

    if args.command == "release":
        return _run_release(args)

    # Fallback: run interpreter on provided script
    if not args.script:
        parser.print_help()
        return 0

    # Reconstruct argv for Interpreter.run_poh: [prog, script, ...]
    sys.argv = [sys.argv[0], args.script] + unknown
    return run_poh_main() or 0


if __name__ == "__main__":  # pragma: no cover
    sys.exit(main())
