extends Control

@onready var messages: VBoxContainer = %Messages
@onready var input_line: LineEdit = %InputLine
@onready var input_panel: PanelContainer = %InputPanel
@onready var scroll_container: ScrollContainer = %ScrollContainer

func _ready() -> void:
	input_panel.hide()
	input_line.text_submitted.connect(func(_text): _submit_message())

func _unhandled_input(event: InputEvent) -> void:
	if !Globals.name_set:
		return
	if event is InputEventKey and event.pressed:
		if event.keycode == KEY_ENTER or event.keycode == KEY_KP_ENTER:
			if Globals.chat_open:
				_submit_message()
			else:
				_open_chat()
			get_viewport().set_input_as_handled()
		elif event.keycode == KEY_ESCAPE and Globals.chat_open:
			_close_chat()
			get_viewport().set_input_as_handled()

func _open_chat() -> void:
	Globals.chat_open = true
	input_panel.show()
	input_line.grab_focus()

func _close_chat() -> void:
	Globals.chat_open = false
	input_line.text = ""
	input_panel.release_focus()
	input_panel.hide()

func _submit_message() -> void:
	var text = input_line.text.strip_edges()
	if text == "":
		_close_chat()
		return
	var message = {"input_type": "Chat", "actor_id": Globals.player_id, "args": text}
	Server.send_data(JSON.stringify(message))
	_close_chat()

func append_message(message_data) -> void:
	var scroll_bar = scroll_container.get_v_scroll_bar()
	var at_bottom = (scroll_bar.value + scroll_bar.page) >= (scroll_bar.max_value - 2)
	
	var message := Label.new()
	message.text = message_data.sender + ": " + message_data.message
	message.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	%Messages.add_child(message)

	await get_tree().process_frame

	if at_bottom:
		scroll_container.scroll_vertical = scroll_bar.max_value
