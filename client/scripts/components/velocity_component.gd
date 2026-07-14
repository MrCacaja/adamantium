extends Node

var velocity := Vector2.ZERO

func apply(value: Array) -> void:
	velocity = Vector2(value[0], value[1])
