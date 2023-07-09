extends Node

@export var websocket_url = "127.0.0.1:9002"

var socket = WebSocketPeer.new()
var first_poll = true
var message_list = []
var tick_secs = Constants.TICK_RATE_SECS

func state_to_instance(state: Dictionary, instance: Node):
	for key in state:
		var value = state[key]
		match key:
			'id': instance.set_name(value)
			_:
				if instance.get(key) != null:
					match key:
						'transform': 
							if instance.action.get('in_progress') == true:
								return
							instance.position = Vector3(
								value.position.x, value.position.y, value.position.z
							)
						'action':
							instance.change_action(value.movement, value.ticks, value.locked)

func load_game_state(res: String):
	for n in get_children():
		remove_child(n)
		n.queue_free()
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

func update_obj(res: String):
	var obj_state = JSON.parse_string(res)
	var obj_instance = get_node(obj_state.id)
	if !obj_instance:
		printerr("Could not find node with ID " + obj_state.id)
	state_to_instance(obj_state, obj_instance)

func _ready():
	socket.connect_to_url(websocket_url)

func _process(delta):
	tick_secs -= delta;
	socket.poll()
	var state = socket.get_ready_state()
	if state == WebSocketPeer.STATE_OPEN:
		var message = message_list.pop_front();
		if (message && tick_secs <= 0):
			socket.send_text(message);
			tick_secs = Constants.TICK_RATE_SECS
		else: socket.send(PackedByteArray())
		if first_poll :
			Globals.player_id = socket.get_packet().get_string_from_ascii()
			first_poll = false
		while socket.get_available_packet_count():
			var res = socket.get_packet().get_string_from_ascii()
			var event = int(res[0])
			res = res.right(res.length() - 1)
			match event:
				Constants.Action.Spawn: spawn(res)
				Constants.Action.SendState: load_game_state(res)
				Constants.Action.Destroy: delete_obj(res)
				Constants.Action.Update: update_obj(res)

	elif state == WebSocketPeer.STATE_CLOSING:
		# Keep polling to achieve proper close.
		pass
	elif state == WebSocketPeer.STATE_CLOSED:
		var code = socket.get_close_code()
		var reason = socket.get_close_reason()
		print("WebSocket closed with code: %d, reason %s. Clean: %s" % [code, reason, code != -1])
		set_process(false) # Stop processing.

func send_data(message: String):
	var msg_json = JSON.parse_string(message);
	for i in message_list:
		var msg = JSON.parse_string(i);
		if msg.input_type == msg_json.input_type:
			msg.args = msg_json.args;
			return;
	message_list.push_back(message);
