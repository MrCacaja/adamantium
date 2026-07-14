extends Node2D

@export var websocket_url = "127.0.0.1:9002"

var socket = WebSocketPeer.new()
var first_poll = true
var message_list = []
var tick_secs = Constants.TICK_RATE_SECS

func sync_entity(res: String) -> void:
	var state = JSON.parse_string(res)
	var entity = get_node_or_null(String.num_uint64(state.id))
	var spawned = !entity
	if spawned:
		entity = preload("res://scripts/entity.gd").new()
	entity.apply_state(state)
	if spawned:
		add_child(entity)

func delete_obj(id: String) -> void:
	remove_child(get_node(id))

func _ready() -> void:
	y_sort_enabled = true
	socket.connect_to_url(websocket_url)

func _process(delta: float) -> void:
	tick_secs -= delta
	socket.poll()
	var state = socket.get_ready_state()
	if state == WebSocketPeer.STATE_OPEN:
		if message_list.size() > 0 and tick_secs <= 0:
			socket.send_text(message_list.pop_front())
			tick_secs = Constants.TICK_RATE_SECS
		while socket.get_available_packet_count():
			var res = socket.get_packet().get_string_from_ascii()
			var event = int(res[0])
			res = res.right(res.length() - 1)
			match event:
				Constants.Action.SyncEntity: sync_entity(res)
				Constants.Action.SyncId: Globals.player_id = res

	elif state == WebSocketPeer.STATE_CLOSING:
		pass
	elif state == WebSocketPeer.STATE_CLOSED:
		var code = socket.get_close_code()
		var reason = socket.get_close_reason()
		print("WebSocket closed with code: %d, reason %s. Clean: %s" % [code, reason, code != -1])
		set_process(false)

func send_data(message: String) -> void:
	var msg_json = JSON.parse_string(message)
	for i in range(message_list.size()):
		var msg = JSON.parse_string(message_list[i])
		if msg.input_type == msg_json.input_type:
			message_list[i] = message
			return
	message_list.push_back(message)
