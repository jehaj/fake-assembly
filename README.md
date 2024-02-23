# fake-assembly
An interpreter for fake assemply described in the course Computer Architecture at Aarhus University.

Overview over the registers.

| Registers:     |         Notes             |
|----------------|---------------------------|
|R0, R1, ..., R7 | General purpose registers |
|     Flags:     |         Notes             |
| Z              | Z flag will be 1 if the result of an arithmetic or logical operation is zero. |

Overview over the instructions available in this fake assembly language.

| Instruction | Description or Result | Notes |
|-------------|-----------------------|-------|
|    ZERO A   |         A = 0         | Does not change the Z flag |
|  MOV A, B   |         A = B         | Does not change the Z flag |
|  ADD A,B,C  |         A = B + C     | Updates the Z flag         |
|  SUB A,B,C  |         A = B - C     | Updates the Z flag |
|    INC A    |         A = A+1       | Updates the Z flag |
|    DEC A    |         A = A-1       | Updates the Z flag |
|  AND A,B,C  |         A = B AND C   |  bitwise AND; Updates the Z flag |
|  OR A,B,C   |         A = B OR C    | bitwise OR; Updates the Z flag |
|  XOR A,B,C  |         A = B XOR C   | bitwise XOR; Updates the Z flag |
|    NOT A    |         A = NOT A     | bitwise NOT i.e., flips all bits; Updates the Z flag |
|   SHL A,k   |         A = A << k    | Shift A left k times; Does not change the Z flag |
|   SHR A,k   |         A = A >> k    | Shift A right k times; Does not change the Z flag |
|   JZ label  |    Jump to label if Z is set     | Does not change the Z flag |
|  JNZ label  |    Jump to label if Z is not set | Does not change the Z flag |
|    J label  |    Jump to label                 | Does not change the Z flag |
