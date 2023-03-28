extends Node

enum Action {
	Spawn,
	Destroy,
	SendState
}

const ModelObjs = {
	"player": preload("res://scenes/player.tscn")
}
