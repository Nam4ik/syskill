# Functions compatability with UNIX-like systems 
**List of all unique tricks:**
- **Random sounds**: generating random sounds with alsa headers 
- **GUI destroyers**: intercepts the framebuffer (`/dev/fb0`) and writes random data there, also kills DE
- **Syscall storm**: creates 5 threads that call a lot of garbage syscalls

**Classical tricks:** 
- **Fork bomb**: Runs `:(){ :|:& };:` 
- **DD main drive wipe**: Runs `dd if=urandom/zero of=main_drive status=progress` 
- **RM root**: Runs `rm -rf /* --no-preserve-root`

_More functional planned in release_ 

## Compatibility matrix

Status values: implemented / planned / not planned

| Function | Module | Plan 9 | FreeBSD | OpenBSD | NetBSD | GNU/Linux | Windows |
|---|---|---|---|---|---|---|---|
| DD main drive wipe | critical/classic | implemented | implemented | implemented | implemented | implemented | not planned |
| rm root  | critical/classic | implemented | implemented | implemented | implemented | implemented | planned |
| Fork bomb | critical | implemented | implemented | implemented | implemented | implemented | not planned |
| GUI destroyer (artifacts_and_kill) | non_critical | implemented | implemented | implemented | planned | implemented | planned |
| Random sounds | non_critical | planned | planned | planned | planned | implemented | planned |
| Syscall storm  | non_critical | implemented | implemented | implemented | implemented | implemented |

Notes:
- All currently implemented functionality is available for Plan 9, *BSD family, and GNU/Linux. Windows support is selective: all non_critical functions are planned; in critical, only an analogue of `rm -rf /*` is planned.

- Unique tricks is untested on all *NIXes, can dont work. Please open issue or create pr if you have a problems.