extends Node

const SNAP_THRESHOLD = 100.0

var _tween: Tween

func apply(value: Array) -> void:
	var server_pos = Vector2(value[0], value[1])
	var entity = get_parent()

	if entity.position.distance_to(server_pos) > SNAP_THRESHOLD:
		entity.position = server_pos
		return

	if _tween and _tween.is_valid():
		_tween.kill()

	_tween = entity.create_tween()
	_tween.tween_property(entity, "position", server_pos, Constants.INTERP_DURATION)
	_tween.set_ease(Tween.EASE_OUT)
	_tween.set_trans(Tween.TRANS_CUBIC)
