at: index put: value
	"Store value at index, clamped to the limits, and notify dependents."
	| slot clamped |
	slot := index max: 1 min: self limit.
	clamped := value + step.
	items at: slot put: clamped.
	self changed; announce: #stored.
	Transcript show: 'stored'; cr.
	^ clamped
