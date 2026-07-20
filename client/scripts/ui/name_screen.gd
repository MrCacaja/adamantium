extends Control

@onready var name_input: LineEdit = %NameInput
@onready var play_button: Button = %PlayButton

func _ready() -> void:
	hide()
	play_button.pressed.connect(func(): _submit_name())
	name_input.text_submitted.connect(func(_text): _submit_name())

func _submit_name() -> void:
	var name_text = name_input.text.strip_edges()
	if name_text == "":
		name_text = "Jogador " + Globals.player_id
	Server.send_name(name_text)
	hide()
