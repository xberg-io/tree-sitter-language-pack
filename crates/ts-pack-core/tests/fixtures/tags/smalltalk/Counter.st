increment: step
	"Add step to the count, up to the limit."
	| next |
	next := count + step.
	count := next min: self limit.
	self changed.
	^ count
