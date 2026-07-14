extends Node

var current_state := "idle"

func apply(value: String) -> void:
	current_state = value
