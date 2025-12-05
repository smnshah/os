.global _start
.extern kernel_main

_start:
    call kernel_main

.hang:
    hlt
    jmp .hang