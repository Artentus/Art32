#ruledef Env {
    brk => asm { envcall 0 }
    rst => asm { envcall 1 }
    err => asm { envcall 2 }
}
