property defaultName : "World"

on greet(personName)
	return "Hello, " & personName
end greet

on makeGreeting for personName
	return greet(personName)
end makeGreeting

script Counter
	property total : 0
	on increment()
		set total to total + 1
		return total
	end increment
end script

tell Counter to increment()
on run
	script Logger
		property prefix : "log: "
	end script
	log greet(defaultName)
end run
