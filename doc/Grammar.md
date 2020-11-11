Here is the full grammar for supported arithmetic expressions:

```
expr ::= term (+ expr | - expr | epsilon)
term ::= factor (* term | / term | epsilon)
factor ::= ( expr ) | integer
integer ::= ... -2 | -1 | 0 | 1 | 2 | ...

```