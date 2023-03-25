extends Node

@export var websocket_url = "127.0.0.1:9002"

var socket = WebSocketPeer.new()
var first_poll = true

var player_obj = preload("res://scenes/player.tscn")

func spawn(args: String):
	print(args)

func _ready():
	socket.connect_to_url(websocket_url)

func _process(delta):
	socket.poll()
	var state = socket.get_ready_state()
	if state == WebSocketPeer.STATE_OPEN:
		socket.send(PackedByteArray())
		if first_poll :
			var player_id = socket.get_packet().get_string_from_ascii()
			print(player_id)
			first_poll = false
		while socket.get_available_packet_count():
			var res = socket.get_packet().get_string_from_ascii()
			print(res)
			var event = int(res[0])
			res = res.right(res.length() - 1)
			print(event, res)
			match event:
				Constants.Action.Spawn:
					#TODO: invocar baseado no argumento "model", não apenas jogador
					var player_instance = player_obj.instantiate()
					var player_state = JSON.parse_string(res)
					if player_state.transform.position:
						player_instance.position.x = player_state.transform.position.x
						player_instance.position.y = player_state.transform.position.y
						player_instance.position.z = player_state.transform.position.z
					add_child(player_instance)

	elif state == WebSocketPeer.STATE_CLOSING:
		# Keep polling to achieve proper close.
		pass
	elif state == WebSocketPeer.STATE_CLOSED:
		var code = socket.get_close_code()
		var reason = socket.get_close_reason()
		print("WebSocket closed with code: %d, reason %s. Clean: %s" % [code, reason, code != -1])
		set_process(false) # Stop processing.
