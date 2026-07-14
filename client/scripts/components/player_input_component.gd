extends Node

var _prev_args := ""

func apply(_value) -> void:
	pass

func _physics_process(_delta: float) -> void:
	var actor_id = Globals.player_id
	var directions := []

	if Input.is_action_pressed("forward"):
		directions.append("0")
	if Input.is_action_pressed("left"):
		directions.append("1")
	if Input.is_action_pressed("back"):
		directions.append("2")
	if Input.is_action_pressed("right"):
		directions.append("3")

	var args = ",".join(directions) if directions.size() > 0 else "stop"
	if args == _prev_args:
		return
	_prev_args = args

	var message = {"input_type": "Move", "actor_id": actor_id, "args": args}
	Server.send_data(JSON.stringify(message))
