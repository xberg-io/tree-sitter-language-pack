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

            function require(condition) {
                if iszero(condition) { revert(0, 0) }
            }

            function safeAdd(a, b) -> r {
                r := add(a, b)
                require(iszero(lt(r, a)))
            }

            function transfer(to, amount) {
                let from := caller()
                require(iszero(lt(balanceOf(from), amount)))
                sstore(from, sub(balanceOf(from), amount))
                sstore(to, safeAdd(balanceOf(to), amount))
            }

            transfer(calldataload(4), calldataload(36))
        }
    }
}
