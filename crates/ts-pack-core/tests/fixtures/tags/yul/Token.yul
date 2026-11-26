object "Token" {
    code {
        datacopy(0, dataoffset("runtime"), datasize("runtime"))
        return(0, datasize("runtime"))
    }
    object "runtime" {
        code {
            function balanceOf(account) -> bal {
                bal := sload(account)
            }

            function transfer(to, amount) {
                let from := caller()
                sstore(from, sub(balanceOf(from), amount))
                sstore(to, add(balanceOf(to), amount))
            }

            transfer(calldataload(4), calldataload(36))
        }
    }
}
