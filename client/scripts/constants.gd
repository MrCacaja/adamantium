extends Node

enum Action {
	SyncEntity,
	SyncId,
	Disconnect,
	Chat,
	SyncChunk,
	TileUpdate
}

enum Movement {
	Idle,
	Walk
}

enum Direction {
	Down,
	Left,
	Right,
	Up
}

const TILE_TYPE_NAMES = [
	"grass", "dirt", "sand", "water", "stone", "deep_water",
	"snow"
]

const SPRITES_CONFIGS = {
	"human": {
		"frame_size": Vector2i(32, 32),
		"padding": 16,
		"animations": {
			"idle": {
				"down": {
					"sheet": "sprites/Entities/Characters/Body_A/Animations/Idle_Base/Idle_Down-Sheet.png",
					"frames": 4,
					"speed": 1.0
				},
				"side": {
					"sheet": "sprites/Entities/Characters/Body_A/Animations/Idle_Base/Idle_Side-Sheet.png",
					"frames": 4,
					"speed": 1.0
				},
				"up": {
					"sheet": "sprites/Entities/Characters/Body_A/Animations/Idle_Base/Idle_Up-Sheet.png",
					"frames": 4,
					"speed": 1.0
				}
			},
			"walk": {
				"down": {
					"sheet": "sprites/Entities/Characters/Body_A/Animations/Walk_Base/Walk_Down-Sheet.png",
					"frames": 6,
					"speed": 1.0
				},
				"side": {
					"sheet": "sprites/Entities/Characters/Body_A/Animations/Walk_Base/Walk_Side-Sheet.png",
					"frames": 6,
					"speed": 1.0
				},
				"up": {
					"sheet": "sprites/Entities/Characters/Body_A/Animations/Walk_Base/Walk_Up-Sheet.png",
					"frames": 6,
					"speed": 1.0
				}
			}
		}
	}
}

const TICK_RATE_SECS = 0.2
const INTERP_DURATION = 0.08
const CHAT_RADIUS = 300.0
const CHAT_MAX_MESSAGES = 50

const CHUNK_SIZE = 32
const TILE_SIZE_PX = 16

const TILE_ATLAS_MAP = {
	0: {"source": "floors", "atlas": Vector2i(1, 10)}, # grass
	1: {"source": "floors", "atlas": Vector2i(11, 10)}, # dirt
	2: {"source": "floors", "atlas": Vector2i(5, 22)}, # sand
	3: {"source": "water", "atlas": Vector2i(1, 11)}, # water
	4: {"source": "floors", "atlas": Vector2i(9, 0)}, # stone
	5: {"source": "water", "atlas": Vector2i(6, 11)}, # deep_water
	6: {"source": "floors", "atlas": Vector2i(2, 15)}, # snow
}
