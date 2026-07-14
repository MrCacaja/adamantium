extends Camera2D

const FOLLOW_SPEED = 5.0

func _process(delta: float) -> void:
	if Globals.player_id == "":
		return
	var entity = get_tree().root.get_node_or_null("Server/" + Globals.player_id)
	if !entity:
		return
	global_position = global_position.lerp(entity.global_position, FOLLOW_SPEED * delta)
