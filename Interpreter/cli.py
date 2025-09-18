"""PohLang Python interpreter CLI.

Usage:
  python -m Interpreter.cli examples/python/hello.poh

This command will execute a .poh script using the Python reference interpreter.
"""

from __future__ import annotations
import sys
import argparse
from .poh_interpreter import Interpreter, RuntimeErrorPoh


def main():
	parser = argparse.ArgumentParser(description="PohLang Python reference interpreter")
	parser.add_argument('script', help='Path to .poh script')
	parser.add_argument('--debug', action='store_true', help='Enable debug tracing')
	args = parser.parse_args()
	path = args.script
	try:
		with open(path, 'r', encoding='utf-8') as f:
			src = f.read()
	except FileNotFoundError:
		print(f"File not found: {path}")
		sys.exit(66)
	interp = Interpreter()
	interp.debug_enabled = args.debug
	try:
		interp.run(src, filename=path)
	except (RuntimeErrorPoh, Exception) as e:  # noqa: BLE001
		print(f"Runtime error: {e}")
		if args.debug:
			import traceback
			traceback.print_exc()
		sys.exit(70)


if __name__ == '__main__':  # pragma: no cover
	main()

