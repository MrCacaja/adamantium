extends Node2D
class_name Entity

const COMPONENT_SCRIPT_BASE_PATH = "res://scripts/components/"

const COMPONENT_MAP = {
	"position": COMPONENT_SCRIPT_BASE_PATH + "position_component.gd",
	"sprite": COMPONENT_SCRIPT_BASE_PATH + "sprite_component.gd",
	"player_id": COMPONENT_SCRIPT_BASE_PATH + "player_input_component.gd",
	"velocity": COMPONENT_SCRIPT_BASE_PATH + "velocity_component.gd",
	"direction": COMPONENT_SCRIPT_BASE_PATH + "direction_component.gd",
	"anim_state": COMPONENT_SCRIPT_BASE_PATH + "anim_state_component.gd",
	"scale": COMPONENT_SCRIPT_BASE_PATH + "scale_component.gd",
}

func _get_or_create_component(key: String) -> Node:
	var component = get_node_or_null(key)
	if !component:
		component = Node.new()
		component.name = key
		component.set_script(load(COMPONENT_MAP[key]))
		add_child(component)
	return component

func apply_state(state: Dictionary) -> void:
	for key in state:
		if key == "id":
			name = String.num_uint64(state[key])
			continue
		if key == "player_id":
			var server_id = state[key].get("id", -1)
			if Globals.player_id != "" and server_id == int(Globals.player_id):
				_get_or_create_component(key).apply(state[key])
			continue
		if COMPONENT_MAP.has(key):
			_get_or_create_component(key).apply(state[key])
