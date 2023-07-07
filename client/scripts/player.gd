extends CharacterBody3D


const SPEED = 5.0
const JUMP_VELOCITY = 4.5


func _physics_process(delta):
	var actor_id = Globals.player_id;
	var message = {"input_type": "Move", "actor_id": actor_id};
	if Input.is_action_pressed("forward"):
		message.args = "0";
		Server.send_data(JSON.stringify(message))
	if Input.is_action_pressed("left"):
		message.args = "1";
		Server.send_data(JSON.stringify(message))
	if Input.is_action_pressed("back"):
		message.args = "2";
		Server.send_data(JSON.stringify(message))
	if Input.is_action_pressed("right"):
		message.args = "3";
		Server.send_data(JSON.stringify(message))
