class
	ACCOUNT

inherit
	ANY

create
	make

feature -- Initialization

	make (initial: INTEGER)
		local
			temp: INTEGER
		do
			temp := initial
			balance := temp
		end

feature -- Access

	balance: INTEGER

	count: INTEGER
		attribute
			Result := 0
		end

	Max_balance: INTEGER = 1000

	deposit (amount: INTEGER)
		do
			balance := balance + amount
			report
		end

	report
		do
			print (balance)
		end

end
