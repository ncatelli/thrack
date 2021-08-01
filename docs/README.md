# Thrack Documentation
Thrack is meant to function both as a 3ac implementation to use as a textual intermediary format as well as a set of libraries for generating the 3ac representation.

## Table of Contents
<!-- TOC -->

- [Thrack Documentation](#thrack-documentation)
	- [Table of Contents](#table-of-contents)
	- [Core Components](#core-components)
		- [3AC Format](#3ac-format)

<!-- /TOC -->

## Core Components
Three-Address Code (3AC/TAC) is a common representation of IR that maps easily to lower-level assembly representation. Further information on this format can be found via [Wikipedia](https://en.wikipedia.org/wiki/Three-address_code) with some working examples and variations in implementation.

### 3AC Format
Thrack will follow a 3 address code consisting of an opcode, a pointer to a return address up to 2 operands.

| Opcode | Return Value Pointer | Operand 1 | Operand 2 |
|--------|----------------------|-----------|-----------|
| move   | T 0x0                | 0x1       | n/a       |
| add    | T 0x1                | 0x2       | 0x2       |

