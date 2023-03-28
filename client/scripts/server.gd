extends Node

@export var websocket_url = "127.0.0.1:9002"

var socket = WebSocketPeer.new()
var first_poll = true

func state_to_instance(state: Dictionary, instance: Node):
	for key in state:
		var value = state[key]
		match key:
			'id': instance.set_name(value)
			_:
				if instance.get(key) != null:
					match key:
						'transform': instance.position = Vector3(
							value.position.x, value.position.y, value.position.z
						)

func load_game_state(res: String):
	var GAME_STATE = JSON.parse_string(res)
	for key in GAME_STATE:
		spawn(JSON.stringify(GAME_STATE[key]))

func delete_obj(id: String):
	remove_child(get_node(id))

func spawn(res: String):
	var obj_state = JSON.parse_string(res)
	var obj_instance = Constants.ModelObjs[obj_state.model.to_lower()].instantiate()
	state_to_instance(obj_state, obj_instance)
	add_child(obj_instance)

func _ready():
	socket.connect_to_url(websocket_url)

func _process(delta):
	socket.poll()
	var state = socket.get_ready_state()
	if state == WebSocketPeer.STATE_OPEN:
		socket.send(PackedByteArray())
		if first_poll :
			var player_id = socket.get_packet().get_string_from_ascii()
			first_poll = false
		while socket.get_available_packet_count():
			var res = socket.get_packet().get_string_from_ascii()
			var event = int(res[0])
			res = res.right(res.length() - 1)
			match event:
				Constants.Action.Spawn: spawn(res)
				Constants.Action.SendState: load_game_state(res)
				Constants.Action.Destroy: delete_obj(res)

	elif state == WebSocketPeer.STATE_CLOSING:
		# Keep polling to achieve proper close.
		pass
	elif state == WebSocketPeer.STATE_CLOSED:
		var code = socket.get_close_code()
		var reason = socket.get_close_reason()
		print("WebSocket closed with code: %d, reason %s. Clean: %s" % [code, reason, code != -1])
		set_process(false) # Stop processing.
