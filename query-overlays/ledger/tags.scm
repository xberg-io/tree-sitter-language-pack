; Ledger journals declare accounts and commodities with directives, as beancount's `open` and
; `commodity` do (see the beancount tags); a posting refers to an account, which ledger also
; accepts undeclared.

; `account Assets:Checking`
(account_directive . (account) @name) @definition.type

; `commodity EUR`
(commodity_directive . (commodity) @name) @definition.type

; `    Expenses:Groceries    42,50 EUR`
(posting (account) @name) @reference.type
