extends Node3D


func _input(event):
	if event is InputEventMouseMotion && Input.is_action_pressed("camera_drag"):
		rotation.y -= event.relative.x * 0.01
		rotation.x -= event.relative.y * 0.01
		rotation.x = clamp(rotation.x, deg_to_rad(-25), deg_to_rad(25) )
