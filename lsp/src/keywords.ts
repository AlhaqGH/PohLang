export interface KeywordDefinition {
	label: string;
	detail: string;
	documentation: string;
	insertText?: string;
}

export const pohLangKeywords: KeywordDefinition[] = [
	{
		label: 'Start Program',
		detail: 'Program block',
		documentation: 'Begin a PohLang program block. Must be paired with "End Program".',
		insertText: 'Start Program\n\t\nEnd Program'
	},
	{
		label: 'End Program',
		detail: 'End program block',
		documentation: 'End the program block started with "Start Program".'
	},
	{
		label: 'Write',
		detail: 'Output statement',
		documentation: 'Output text or variable value to the console.\nExample: Write "Hello World"',
		insertText: 'Write '
	},
	{
		label: 'Set',
		detail: 'Variable assignment',
		documentation: 'Assign a value to a variable.\nExample: Set x to 10',
		insertText: 'Set  to '
	},
	{
		label: 'Make',
		detail: 'Function definition',
		documentation: 'Define a function.\nExample: Make myFunction with x and y:',
		insertText: 'Make  with :\n\t\nEnd'
	},
	{
		label: 'End',
		detail: 'End block',
		documentation: 'End a block (function, if, while, for).'
	},
	{
		label: 'If',
		detail: 'Conditional statement',
		documentation: 'Start a conditional block.\nExample: If x is greater than 5',
		insertText: 'If \n\t\nEnd'
	},
	{
		label: 'Else',
		detail: 'Alternative branch',
		documentation: 'Provide an alternative branch in an if statement.'
	},
	{
		label: 'While',
		detail: 'Loop statement',
		documentation: 'Create a while loop.\nExample: While count is less than 10',
		insertText: 'While \n\t\nEnd'
	},
	{
		label: 'For',
		detail: 'Iteration statement',
		documentation: 'Iterate over a collection.\nExample: For each item in list',
		insertText: 'For each  in \n\t\nEnd'
	},
	{
		label: 'Return',
		detail: 'Return statement',
		documentation: 'Return a value from a function.\nExample: Return result'
	},
	{
		label: 'True',
		detail: 'Boolean true',
		documentation: 'Boolean value representing true.'
	},
	{
		label: 'False',
		detail: 'Boolean false',
		documentation: 'Boolean value representing false.'
	},
	{
		label: 'Ask',
		detail: 'Input statement',
		documentation: 'Get user input.\nExample: Ask "Enter your name:"',
		insertText: 'Ask '
	},
	{
		label: 'Read',
		detail: 'Read input',
		documentation: 'Read a line of input from the user.'
	},
	{
		label: 'Break',
		detail: 'Break loop',
		documentation: 'Exit a loop early.'
	},
	{
		label: 'Continue',
		detail: 'Continue loop',
		documentation: 'Skip to the next iteration of a loop.'
	}
];

export const pohLangOperators: KeywordDefinition[] = [
	{
		label: 'Plus',
		detail: 'Addition',
		documentation: 'Add two numbers.\nExample: x plus y'
	},
	{
		label: 'Minus',
		detail: 'Subtraction',
		documentation: 'Subtract two numbers.\nExample: x minus y'
	},
	{
		label: 'Times',
		detail: 'Multiplication',
		documentation: 'Multiply two numbers.\nExample: x times y'
	},
	{
		label: 'Divided',
		detail: 'Division',
		documentation: 'Divide two numbers.\nExample: x divided by y'
	},
	{
		label: 'Is',
		detail: 'Equality comparison',
		documentation: 'Check if two values are equal.\nExample: x is 5'
	},
	{
		label: 'Is not',
		detail: 'Inequality comparison',
		documentation: 'Check if two values are not equal.\nExample: x is not 5'
	},
	{
		label: 'Greater than',
		detail: 'Greater than comparison',
		documentation: 'Check if left value is greater than right.\nExample: x is greater than 5'
	},
	{
		label: 'Less than',
		detail: 'Less than comparison',
		documentation: 'Check if left value is less than right.\nExample: x is less than 5'
	},
	{
		label: 'And',
		detail: 'Logical AND',
		documentation: 'Logical AND operator.\nExample: x is 5 and y is 10'
	},
	{
		label: 'Or',
		detail: 'Logical OR',
		documentation: 'Logical OR operator.\nExample: x is 5 or y is 10'
	},
	{
		label: 'Not',
		detail: 'Logical NOT',
		documentation: 'Logical NOT operator.\nExample: not flag'
	},
	{
		label: 'To',
		detail: 'Dictionary assignment',
		documentation: 'Assign a value to a dictionary key.\nExample: Set dict[key] to value'
	}
];
