extends CharacterBody3D


const SPEED = 5.0
const JUMP_VELOCITY = 4.5
@export var action = {
	"movement": Constants.Movement.Idle,
	"ticks": 0,
	"locked": false,
	"done": false,
}

func change_action(movement, ticks, locked):
	action.movement = int(movement);
	action.ticks = ticks;
	action.locked = locked;
	action.done = false;
	match action.movement:
		Constants.Movement.Idle: action.done = true
		Constants.Movement.WalkNorth: tween_walk(position + Vector3.FORWARD, ticks)
		Constants.Movement.WalkWest: tween_walk(position + Vector3.LEFT, ticks)
		Constants.Movement.WalkSouth: tween_walk(position + Vector3.BACK, ticks)
		Constants.Movement.WalkEast: tween_walk(position + Vector3.RIGHT, ticks)
	print(action)

func change_to_idle():
	change_action(Constants.Movement.Idle, 0, true)

func tween_walk(direction, ticks):
	var tween = get_tree().create_tween()
	tween.tween_property(self, "position", direction, ticks * Constants.TICK_RATE_SECS).set_trans(Tween.TRANS_LINEAR)
	tween.tween_callback(change_to_idle)

func _process(delta):
	var actor_id = Globals.player_id;
	var message = {"input_type": "Move", "actor_id": actor_id};
	var cam_rotation = $CameraHolder.global_rotation_degrees.y;
	
	var cam_direction = 0
	if cam_rotation >= -45 && cam_rotation <= 45:
		cam_direction = 0
	elif cam_rotation >= -135 && cam_rotation < -45:
		cam_direction = 3
	elif cam_rotation > 45 && cam_rotation < 135:
		cam_direction = 1
	else:
		cam_direction = 2

	if Input.is_action_pressed("forward"):
		message.args = str(cam_direction)
		Server.send_data(JSON.stringify(message))

	if Input.is_action_pressed("left"):
		message.args = str((cam_direction + 1) % 4)

	if Input.is_action_pressed("back"):
		message.args = str((cam_direction + 2) % 4)
		Server.send_data(JSON.stringify(message))

	if Input.is_action_pressed("right"):
		message.args = str((cam_direction + 3) % 4)
		Server.send_data(JSON.stringify(message))

	if message.has("args"):
		Server.send_data(JSON.stringify(message))
