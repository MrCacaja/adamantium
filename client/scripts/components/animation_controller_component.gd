extends Node

var _current_anim := ""
var _current_dir := ""

func _process(_delta: float) -> void:
	var sprite = get_parent().get_node_or_null("sprite")
	var anim_comp = get_parent().get_node_or_null("anim_state")
	var dir_comp = get_parent().get_node_or_null("direction")
	if not sprite or not anim_comp or not dir_comp:
		return

	var anim = anim_comp.current_state
	var dir = dir_comp.current_direction

	var sprite_dir = "side" if dir in ["left", "right"] else dir
	var flip_h = dir == "left"

	if anim != _current_anim or dir != _current_dir:
		_current_anim = anim
		_current_dir = dir
		sprite.play_anim(anim, sprite_dir, flip_h)
