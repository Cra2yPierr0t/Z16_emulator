# ZERO 16 bit Instruction Set Architecture

## bit field

`R-Type: | rs2[3:0] | rs1[3:0] | rd[3:0] | opcode[3:0] | `
`I-Type: | imm[7:0] | rd[3:0] | opcode[3:0] |`
`L-Type: | imm[3:0] | rs1[3:0] | rd[3:0] | opcode[3:0] |`
`S-Type: | rs2[3:0] | rs1[3:0] | imm[3:0] | opcode[3:0] |`
`J-Type: | imm[3:0] | rs1[3:0] | rd[3:0] | opcode[3:0] |`
`B-Type: | imm[7:0] | rs2[1:0] | rs1[1:0] | opcode[3:0] |`

## Registers

| addr | name | description |
| ---- | ---- | ----------- |
| `4'h0` | `ZR` | zero register |
| `4'h1` | `B1` | branch register 1 |
| `4'h2` | `B2` | branch register 2 |
| `4'h3` | `B3` | branch register 3 |
| `4'h4` | `G0` | general purpose register |
| `4'h5` | `G1` | general purpose register |
| `4'h6` | `G2` | general purpose register |
| `4'h7` | `G3` | general purpose register |
| `4'h8` | `G4` | general purpose register |
| `4'h9` | `G5` | general purpose register |
| `4'hA` | `G6` | general purpose register |
| `4'hB` | `G7` | general purpose register |
| `4'hC` | `G8` | general purpose register |
| `4'hD` | `G9` | general purpose register |
| `4'hE` | `G10` | general purpose register |
| `4'hF` | `G11` | general purpose register |

## 演算

`R-Type: | rs2[3:0] | rs1[3:0] | rd[3:0] | opcode[3:0] | `

| name | operation | `opcode[3:0]` |
| ---- | --------- | ------------- |
| Add | `rs2 + rs1 ➜ rd` | `4'h0` |
| Sub | `rs2 - rs1 ➜ rd` | `4'h1` |
| Mul | `rs2 * rs1 ➜ rd` | `4'h2` |
| Div | `rs2 / rs1 ➜ rd` | `4'h3` |
| Or | `rs2 \| rs1 ➜ rd` | `4'h4` |
| And | `rs2 & rs1 ➜ rd` | `4'h5` |
| Xor | `rs2 ^ rs1 ➜ rd` | `4'h6` |
| SLL | `rs1 << rs2 ➜ rd` | `4'h7` |
| SRL | `rs1 >> rs2 ➜ rd` | `4'h8` |

`I-Type: | imm[7:0] | rd[3:0] | opcode[3:0] |`

| name | operation | `opcode[3:0]` |
| ---- | --------- | ------------- |
| Addi | `rd + imm ➜ rd` | `4'h9` |
| Subi | `rd - imm ➜ rd` | `4'hA` |

## メモリ

`L-Type: | imm[3:0] | rs1[3:0] | rd[3:0] | opcode[3:0] |`
`S-Type: | rs2[3:0] | rs1[3:0] | imm[3:0] | opcode[3:0] |`

| name | operation | `opcode[3:0]` |
| ---- | --------- | ------------- |
| Load | `[rs1 + imm] ➜ rd` | `4'hB` |
| Store | `rs2 ➜ [rs1 + imm]` | `4'hC` |

## ジャンプ/分岐

`J-Type: | imm[3:0] | rs1[3:0] | rd[3:0] | opcode[3:0] |`

| name | operation | `opcode[3:0]` |
| ---- | --------- | ------------- |
| Jump and link register | `pc + 2 ➜ rd, imm + rs1 + pc ➜ pc` | `4'hD` |

`B-Type: | imm[7:0] | rs2[1:0] | rs1[1:0] | opcode[3:0] |`

| name | operation | `opcode[3:0]` |
| ---- | --------- | ------------- |
| Branch equal | `if rs2 == rs1 then imm + pc ➜ pc` | `4'hE` |
| Branch Less than | `if rs2 > rs1 then imm + pc ➜ pc` | `4'hF` |

## サンプルプログラム

```
Addi 0xA, R0, R1
Add R1, R2, R2
Subi 0x1, R1, R1
BLT R1, R0, -4
```
JALR R0, 0
