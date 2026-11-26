class_name Player
extends CharacterBody2D

signal health_changed(value)

enum State { IDLE, RUNNING }

const MAX_HEALTH = 100

var health: int = MAX_HEALTH

func _init():
	health = MAX_HEALTH

func _ready():
	reset_health()

func take_damage(amount: int) -> void:
	var remaining = health - amount
	print(remaining)
	health -= amount
	health_changed.emit(health)

func reset_health():
	health = MAX_HEALTH

class Inventory:
	var items = []

	func add(item):
		items.append(item)

class Backpack extends Inventory:
	func add(item):
		.add(item)
		print(items.size())
