#!/usr/bin/env python3
"""
Quick test runner for PohLang testers.
Runs a suite of test programs and reports results.
"""

import sys
from Interpreter.poh_interpreter import Interpreter, RuntimeErrorPoh


class TestRunner:
    def __init__(self):
        self.passed = 0
        self.failed = 0
        self.tests = []
        
    def test(self, name, code, expected_output=None, should_error=False):
        """Register a test case."""
        self.tests.append({
            'name': name,
            'code': code,
            'expected': expected_output,
            'should_error': should_error
        })
    
    def run(self):
        """Run all registered tests."""
        print("=" * 60)
        print("PohLang Quick Test Suite")
        print("=" * 60)
        print()
        
        for i, test in enumerate(self.tests, 1):
            print(f"[{i}/{len(self.tests)}] {test['name']}...", end=" ")
            
            output = []
            error = None
            
            try:
                interp = Interpreter(output_fn=lambda s: output.append(str(s)))
                interp.run(test['code'], filename="test_case.poh")
            except RuntimeErrorPoh as e:
                error = str(e)
            except Exception as e:
                error = f"Unexpected error: {e}"
            
            # Check results
            if test['should_error']:
                if error:
                    print("✅ PASS (error as expected)")
                    self.passed += 1
                else:
                    print("❌ FAIL (expected error, got success)")
                    self.failed += 1
            else:
                if error:
                    print(f"❌ FAIL (unexpected error)")
                    print(f"   Error: {error}")
                    self.failed += 1
                elif test['expected'] is not None:
                    if output == test['expected']:
                        print("✅ PASS")
                        self.passed += 1
                    else:
                        print("❌ FAIL (output mismatch)")
                        print(f"   Expected: {test['expected']}")
                        print(f"   Got:      {output}")
                        self.failed += 1
                else:
                    # No expected output specified, just check it ran
                    print("✅ PASS (ran without error)")
                    self.passed += 1
        
        print()
        print("=" * 60)
        print(f"Results: {self.passed} passed, {self.failed} failed")
        print("=" * 60)
        
        return self.failed == 0


def main():
    runner = TestRunner()
    
    # Test 1: Basic output
    runner.test(
        "Basic Write",
        'Write "Hello World"',
        ["Hello World"]
    )
    
    # Test 2: Variables
    runner.test(
        "Variable assignment",
        'Set x to 42\nWrite x',
        ["42"]
    )
    
    # Test 3: Arithmetic
    runner.test(
        "Addition",
        'Set a to 10\nSet b to 5\nWrite a plus b',
        ["15"]
    )
    
    # Test 4: String concatenation
    runner.test(
        "String concatenation",
        'Set name to "PohLang"\nWrite "Hello " plus name',
        ["Hello PohLang"]
    )
    
    # Test 5: Comparisons
    runner.test(
        "Greater than comparison",
        'If 10 is greater than 5 Write "Yes"',
        ["Yes"]
    )
    
    # Test 6: Block if-else
    runner.test(
        "Block if-else",
        'Set age to 25\nIf age is greater than 18\n    Write "Adult"\nOtherwise\n    Write "Minor"\nEnd',
        ["Adult"]
    )
    
    # Test 7: Block if
    runner.test(
        "Block if",
        'Set x to 5\nIf x\n    Write "Truthy"\nEnd',
        ["Truthy"]
    )
    
    # Test 8: Repeat loop
    runner.test(
        "Repeat loop",
        'Repeat 3\n    Write "Hi"\nEnd',
        ["Hi", "Hi", "Hi"]
    )
    
    # Test 9: While loop
    runner.test(
        "While loop",
        'Set counter to 0\nWhile counter is less than 3\n    Write counter\n    Set counter to counter plus 1\nEnd',
        ["0", "1", "2"]
    )
    
    # Test 10: Inline function
    runner.test(
        "Inline function",
        'Make double with n Write n times 2\nWrite double(5)',
        ["10"]
    )
    
    # Test 11: Block function with return
    runner.test(
        "Block function with return",
        'Make add with a, b\n    Return a plus b\nEnd\nWrite add(3, 7)',
        ["10"]
    )
    
    # Test 12: Function with default parameter
    runner.test(
        "Function with default parameter",
        'Make greet with name set to "World" Write "Hello " plus name\nWrite greet()',
        ["Hello World"]
    )
    
    # Test 13: Nested conditionals
    runner.test(
        "Nested if blocks",
        'Set x to 10\nIf x is greater than 5\n    If x is less than 15\n        Write "In range"\n    End\nEnd',
        ["In range"]
    )
    
    # Test 14: Function calling another function
    runner.test(
        "Function composition",
        'Make double with n Write n times 2\nMake quadruple with n Write double(double(n))\nWrite quadruple(3)',
        ["12"]
    )
    
    # Test 15: Error case - wrong function arity
    runner.test(
        "Function arity error",
        'Make add with a, b\n    Return a plus b\nEnd\nWrite add(1)',
        should_error=True
    )
    
    # Run all tests
    success = runner.run()
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
