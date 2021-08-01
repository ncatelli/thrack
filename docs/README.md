# Thrack Documentation
Thrack is meant to function both as a 3ac implementation to use as a textual intermediary format as well as a set of libraries for generating the 3ac representation.

## Table of Contents
<!-- TOC -->

- [Thrack Documentation](#thrack-documentation)
	- [Table of Contents](#table-of-contents)
	- [Core Components](#core-components)
		- [3AC Format](#3ac-format)
		- [Opcodes](#opcodes)
			- [Arity](#arity)
		- [Registers](#registers)
		- [Register Alias](#register-alias)

<!-- /TOC -->

## Core Components
Three-Address Code (3AC/TAC) is a common representation of IR that maps easily to lower-level assembly representation. Further information on this format can be found via [Wikipedia](https://en.wikipedia.org/wiki/Three-address_code) with some working examples and variations in implementation.

### 3AC Format
Thrack will follow a 3 address code consisting of an opcode, a pointer to a return address and up to 2 operands.

| Opcode | Return Value Pointer | Operand 1 | Operand 2 |
|--------|----------------------|-----------|-----------|
| move   | T 0x0                | 0x2       | n/a       |
| add    | T 0x1                | 0x2       | T 0x0     |

### Opcodes
Opcodes are to be represented by a unique unsigned 16-bit identifier but will be accompanied by a corresponding mnemonic representantion. This will be represented as a single english word. For example:

- move
- add
- divide
- jump
- call
- free

#### Arity
Opcodes will have a fixed arity, encoding their expected operand count at call time. This, however, will not include the type of parameters. Each opcode will be focused on enacting an operation a register or a constant value.

### Registers
Thrack consists of an infinite number of temporary registers, often times these registers, once allocated, can never be changed.

### Register Alias 
Aliases function as a named pointer to a given register. This provides a convenient utility for working with the constantly changing underlying register
