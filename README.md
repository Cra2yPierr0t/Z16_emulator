# ZERO 16 bit Instruction Set Architecture

## bit field

```
R-Type: | rs2[3:0] | rs1[3:0] | rd[3:0] | opcode[3:0] | 
I-Type: | imm[7:0] | rd[3:0] | opcode[3:0] |
L-Type: | imm[3:0] | rs1[3:0] | rd[3:0] | opcode[3:0] |
S-Type: | rs2[3:0] | rs1[3:0] | imm[3:0] | opcode[3:0] |
J-Type: | imm[3:0] | rs1[3:0] | rd[3:0] | opcode[3:0] |
B-Type: | imm[7:0] | rs2[1:0] | rs1[1:0] | opcode[3:0] |
```

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

`OP RS2 RS1 RD`

| name | operation | `opcode[3:0]` | example |
| ---- | --------- | ------------- | ------- |
| Add | `rs2 + rs1 ➜ rd` | `4'h0` | `ADD G2 G1 G0` |
| Sub | `rs2 - rs1 ➜ rd` | `4'h1` | `SUB G2 G1 G0` |
| Mul | `rs2 * rs1 ➜ rd` | `4'h2` | `MUL G2 G1 G0` |
| Div | `rs2 / rs1 ➜ rd` | `4'h3` | `DIV G2 G1 G0` |
| Or | `rs2 \| rs1 ➜ rd` | `4'h4` | `OR G2 G1 G0` |
| And | `rs2 & rs1 ➜ rd` | `4'h5` | `AND G2 G1 G0` |
| Xor | `rs2 ^ rs1 ➜ rd` | `4'h6` | `XOR G2 G1 G0` |
| SLL | `rs1 << rs2 ➜ rd` | `4'h7` | `SLL G2 G1 G0` |
| SRL | `rs1 >> rs2 ➜ rd` | `4'h8` | `SRL G2 G1 G0` |

`I-Type: | imm[7:0] | rd[3:0] | opcode[3:0] |`

`OP IMM RD`

| name | operation | `opcode[3:0]` | example |
| ---- | --------- | ------------- | ------- |
| Addi | `rd + imm ➜ rd` | `4'h9` | `ADDI 42 G4` |
| Subi | `rd - imm ➜ rd` | `4'hA` | `SUBI 42 G4` |

## メモリ

`L-Type: | imm[3:0] | rs1[3:0] | rd[3:0] | opcode[3:0] |`

`S-Type: | rs2[3:0] | rs1[3:0] | imm[3:0] | opcode[3:0] |`

| name | operation | `opcode[3:0]` | example |
| ---- | --------- | ------------- | ------- |
| Load | `[rs1 + imm] ➜ rd` | `4'hB` | `LOAD 42 G5 G6` |
| Store | `rs2 ➜ [rs1 + imm]` | `4'hC` | `STORE G5 G6 42` |

## ジャンプ/分岐

`J-Type: | imm[3:0] | rs1[3:0] | rd[3:0] | opcode[3:0] |`

| name | operation | `opcode[3:0]` | example |
| ---- | --------- | ------------- | ------- |
| Jump and link register | `pc + 2 ➜ rd` <br>`imm + rs1 + pc ➜ pc` | `4'hD` | `JALR 40 G7 G8` |

`B-Type: | imm[7:0] | rs2[1:0] | rs1[1:0] | opcode[3:0] |`

| name | operation | `opcode[3:0]` | example |
| ---- | --------- | ------------- | ------- |
| Branch equal | `if rs2 == rs1 `<br>`then imm + pc ➜ pc` | `4'hE` | `BEQ B1 B2 -4` | 
| Branch Less than | `if rs2 > rs1 `<br>`then imm + pc ➜ pc` | `4'hF` | `BLT B1 B2 -12` |

## サンプルプログラム

0~10の総和

```
ADDI 0xA B1
ADD B1 B2 B2
SUBI 0x1 B1
BLT B1 ZR -4
JALR 0 ZR G11
```
