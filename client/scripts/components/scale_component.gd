extends Node

func apply(value: Array) -> void:
	get_parent().scale = Vector2(value[0], value[1])
