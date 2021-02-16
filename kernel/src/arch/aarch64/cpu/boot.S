// Boot Code

.section ".text._start"

.global _start

_start:
    mrs     x1, mpidr_el1   // Read Multiprocessor Affinity Register
    and     x1, x1, #3      // Mask off all but core id (bits [1:0])
    cbz     x1, 2f          // Jump to 2: if core 0
1:                          // Idle other cores
    wfe                     // Wait For Event
    b       1b              // If it happens, wait again
2:                          // Core 0 continues on
    ldr     x1, =_start     // Load our address (_start)
    mov     sp, x1          // Set top of stack below us
    bl      runtime_init    // Jump to runtime_init()
    b       1b              // Should never get here, but
                            // just in case, we join the
                            // idle cores.
