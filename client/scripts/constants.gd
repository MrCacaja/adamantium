extends Node

enum Action {
	Spawn,
	Destroy,
	SendState,
	Update
}

enum Movement {
	Idle,
	WalkNorth,
	WalkWest,
	WalkSouth,
	WalkEast,
	WalkNorthwest,
	WalkNortheast,
	WalkSouthwest,
	WalkSoutheast,
}

const ModelObjs = {
	"player": preload("res://scenes/player.tscn")
}

#those shall be the same in the server code:
const TICK_RATE_SECS = 0.2;
