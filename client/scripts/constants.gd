extends Node

enum Action {
	Spawn,
	Destroy,
	SendState,
	Update
}

const ModelObjs = {
	"player": preload("res://scenes/player.tscn")
}
