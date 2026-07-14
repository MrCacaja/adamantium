extends Node

const COMPONENT_SCRIPT_BASE_PATH = "res://scripts/components/"

static var sprite_frames_cache: Dictionary = {}

func apply(value: String) -> void:
	var type = value.to_lower()
	var sprite_frames = _get_or_build_sprite_frames(type)
	if sprite_frames == null:
		return

	var animated_sprite = get_parent().get_node_or_null("AnimatedSprite2D")
	if !animated_sprite:
		animated_sprite = AnimatedSprite2D.new()
		animated_sprite.name = "AnimatedSprite2D"
		get_parent().add_child(animated_sprite)

	animated_sprite.sprite_frames = sprite_frames

	if animated_sprite.animation == "" or !animated_sprite.is_playing():
		var default_anim = _get_default_animation_name(type)
		if default_anim != "":
			animated_sprite.play(default_anim)

	var anim_ctrl = get_parent().get_node_or_null("animation_controller")
	if !anim_ctrl:
		anim_ctrl = Node.new()
		anim_ctrl.name = "animation_controller"
		anim_ctrl.set_script(load(COMPONENT_SCRIPT_BASE_PATH + "animation_controller_component.gd"))
		get_parent().add_child(anim_ctrl)


func play_anim(anim_name: String, direction: String, flip_h := false) -> void:
	var anim_key = "%s_%s" % [anim_name, direction]
	var anim_sprite = get_parent().get_node_or_null("AnimatedSprite2D")
	if anim_sprite and anim_sprite.sprite_frames.has_animation(anim_key):
		if anim_sprite.animation != anim_key:
			anim_sprite.play(anim_key)
		anim_sprite.flip_h = flip_h


func _get_or_build_sprite_frames(type: String) -> SpriteFrames:
	if sprite_frames_cache.has(type):
		return sprite_frames_cache[type]

	var config = Constants.SPRITES_CONFIGS.get(type, null)
	if config == null:
		push_error("Sprite config not found for type: %s" % type)
		return null

	var sprite_frames = SpriteFrames.new()
	sprite_frames.remove_animation("default")

	var frame_size: Vector2i = config.frame_size
	var padding: int = config.get("padding", 0)

	for anim_name in config.animations.keys():
		var directions = config.animations[anim_name]
		for direction in directions.keys():
			var dir_config = directions[direction]
			var sheet_path = dir_config.sheet
			var frame_count: int = dir_config.frames
			var speed: float = dir_config.get("speed", 8.0)

			var anim_key = "%s_%s" % [anim_name, direction]

			if sprite_frames.has_animation(anim_key):
				sprite_frames.remove_animation(anim_key)

			sprite_frames.add_animation(anim_key)
			sprite_frames.set_animation_speed(anim_key, speed * frame_count)
			sprite_frames.set_animation_loop(anim_key, true)

			var texture = load("res://" + sheet_path) as Texture2D
			if texture == null:
				push_error("Failed to load sprite sheet: %s" % sheet_path)
				continue

			for i in range(frame_count):
				var region = Rect2(
					padding + i * (frame_size.x + 2 * padding),
					padding,
					frame_size.x,
					frame_size.y
				)
				var atlas = AtlasTexture.new()
				atlas.atlas = texture
				atlas.region = region
				sprite_frames.add_frame(anim_key, atlas)

	sprite_frames_cache[type] = sprite_frames
	return sprite_frames


func _get_default_animation_name(type: String) -> String:
	var config = Constants.SPRITES_CONFIGS.get(type, null)
	if config == null:
		return ""

	var animations = config.animations
	var first_anim = animations.keys()[0]
	var first_dir = animations[first_anim].keys()[0]
	return "%s_%s" % [first_anim, first_dir]
