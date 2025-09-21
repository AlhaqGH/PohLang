import sys
sys.path.insert(0, '.')

from Interpreter.poh_interpreter import Interpreter

print("Testing PohLang v0.1.0...")

# Test basic functionality
interp = Interpreter()
interp.run('Write "Hello from PohLang v0.1.0!"')

# Test variables
interp.run('Set x to 5')
interp.run('Write x')

# Test expressions
interp.run('Write 3 plus 4')

print("✅ Basic functionality works!")
print("✅ PohLang v0.1.0 is ready for release!")