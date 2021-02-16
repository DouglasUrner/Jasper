// Boot Code

.section ".text._start"

.global _start

_start:
1:  wfe         // Wait For Event
    b       1b  // If it happens, wait again